# Data Model: Gameplay Concepts

## CardDefinitionModel

Represents static card data used to instantiate the near player's runtime deck.

| Field | Type | Validation |
| ----- | ---- | ---------- |
| `id` | enum/string | One of `kage`, `sister`, `Lord`, `test` for this feature |
| `power` | integer | Matches the Card Definition Values table |
| `energy` | integer | Matches the Card Definition Values table and is used as placement cost |

## CardInstanceModel

Represents one runtime copy of a card definition in the near player's deck, hand, or location.

| Field | Type | Validation |
| ----- | ---- | ---------- |
| `instance_id` | stable runtime id | Unique within a fresh game |
| `definition_id` | card definition id | Must reference a known `CardDefinitionModel` |
| `zone` | enum | `Deck`, `Hand`, or `Location` |
| `effective_energy` | integer | Starts from the card definition energy and includes active location ability modifiers while placed |
| `location_energy_delta` | integer | `0` unless an open location ability currently modifies this card |
| `hand_index` | optional integer | Present only while card is in hand |
| `location_index` | optional integer | Present only while card is placed |
| `slot_index` | optional integer | Present only while card is placed |

## LocationDefinitionModel

Represents static location text, open timing, and ability metadata for the three GameView locations.

| Field | Type | Validation |
| ----- | ---- | ---------- |
| `location_index` | integer | 0 through 2 |
| `opens_on_round` | integer | Left opens on round 1, middle on round 2, right on round 3 |
| `title` | string | Open-state title, rendered as up to two centered lines |
| `body` | string | Open-state ability body, rendered as up to three centered lines |
| `ability` | enum | `EnergyDelta(+2)`, `EnergyDelta(-2)`, or `NoAbility` |

## LocationModel

Represents runtime state for one shared location in GameView.

| Field | Type | Validation |
| ----- | ---- | ---------- |
| `location_index` | integer | 0 through 2 |
| `state` | enum | `Closed` or `Open`, derived from current round and `opens_on_round` |
| `display_title` | string | Closed: `Closed Until Round X`; open: location title |
| `display_body` | string | Closed: empty; open: location ability text or `(No Ability)` |
| `placed_cards` | ordered list of card instance ids | Contains only cards whose zone is `Location` at this location |

## Location Definition Values

| Location | Opens On Round | Title | Body | Ability |
| -------- | -------------- | ----- | ---- | ------- |
| Left | 1 | `Fortress Gate` | `+2 Energy to each card here` | Add `+2` to each placed card's effective energy while open and placed here |
| Middle | 2 | `Bamboo Crossing` | `-2 Energy to each card here` | Add `-2` to each placed card's effective energy while open and placed here |
| Right | 3 | `Normal` | `(No Ability)` | No effective-energy change |

## GameDeckModel

Runtime deck state for the near human player.

| Field | Type | Validation |
| ----- | ---- | ---------- |
| `cards` | ordered list of `CardInstanceModel` ids | Exactly 12 cards after fresh game or Restart |
| `remaining_cards` | ordered list of `CardInstanceModel` ids | Cards not yet dealt, kept in randomized deck order for deal selection |
| `random_seed` | optional seed/source | May be stored for tests or diagnostics |

## GameHandModel

Runtime hand state and layout source for visible local cards.

| Field | Type | Validation |
| ----- | ---- | ---------- |
| `cards` | ordered list of `CardInstanceModel` ids | Contains only cards whose zone is `Hand` |
| `layout_center` | safe-view coordinate | Derived from aspect-ratio-safe GameView |
| `layout_spacing` | safe-view distance | May allow more than four or five cards to exceed the hand area's width |

## GameRoundModel

Authoritative six-round progression and energy state.

| Field | Type | Validation |
| ----- | ---- | ---------- |
| `round` | integer | 1 through 6 |
| `max_rounds` | integer | Always 6 |
| `energy_available` | integer | Starts at the current round's maximum and decreases by placement cost |
| `energy_maximum` | integer | Round schedule value: 1, 2, 3, 4, 5, 6 |
| `requested_cards_to_deal` | integer | Round schedule value: 1, 2, 3, 1, 1, 1 |
| `end_round_resolved` | boolean | Allows End Round on round 6 without dealing more cards |

## CurrentRoundMoveRecord

Records the undoable moves made since the current round began.

| Field | Type | Validation |
| ----- | ---- | ---------- |
| `card_instance_id` | runtime card id | Must reference a card currently placed in a location |
| `from_hand_index` | integer | Original hand identity used for return/recenter behavior |
| `location_index` | integer | 0 through 2 |
| `slot_index` | integer | Valid local slot index for that location |
| `energy_cost` | integer | Equals the card definition energy at placement time |
| `location_energy_delta` | integer | Effective-energy modifier applied by the destination location at placement time, if any |

## GameControlView

Presentation state for lower-left and lower-right controls.

| Field | Type | Validation |
| ----- | ---- | ---------- |
| `restart_enabled` | boolean | Always true during GameView play |
| `undo_enabled` | boolean | True only when current-round move history is non-empty |
| `undo_label` | string | `Energy current/max` newline `Undo` |
| `end_round_label` | string | Shows `End Round` and current round fraction |

## State Transitions

| Trigger | From | To | Notes |
| ------- | ---- | -- | ----- |
| Fresh game or Restart | Any state | Round 1, new 12-card deck, empty hand/locations/history | Deal round 1 after reset |
| Start round | Round N | Energy set and scheduled cards dealt | Deal the requested number of cards from the remaining deck order to the hand, regardless of card energy |
| Location opens | Closed location | Open location | Left opens in round 1, middle in round 2, right in round 3; open locations show title/body and apply abilities immediately to cards already there |
| Deal card | Deck | Hand | Animate from below screen center, then recenter hand group |
| Fewer remaining deck cards than requested | Remaining deck cards | Hand | Deal only the cards that remain; do not create extra cards |
| Place card | Hand | Location | Allowed only when energy is sufficient and slot is legal; open location ability applies immediately to the placed card's effective energy |
| Undo | Current-round Location placements | Hand | Remove active location ability deltas, restore cards and their energy deductions, and clear current-round history |
| End Round on rounds 1-5 | Round N | Round N+1 | Clear current-round history, deal next round, reset energy |
| End Round on round 6 | Round 6 | Round 6 resolved | No additional deal |
