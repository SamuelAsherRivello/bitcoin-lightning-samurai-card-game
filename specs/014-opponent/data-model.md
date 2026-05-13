# Data Model: Opponent Modes and Two-Player Resolution

## MatchModeModel

| Field | Type | Validation |
| ----- | ---- | ---------- |
| `mode` | enum | Must be `HumanVersusCpu` or `CpuVersusCpu`. |

**Transitions**:

| From | Event | To |
| ---- | ----- | -- |
| `HumanVersusCpu` | Mode button activated | `CpuVersusCpu` and fresh game reset |
| `CpuVersusCpu` | Mode button activated | `HumanVersusCpu` and fresh game reset |

## ModePreferenceModel

| Field | Type | Validation |
| ----- | ---- | ---------- |
| `selected_mode` | `MatchModeModel` | Defaults to `HumanVersusCpu` when no saved preference exists. |

**Persistence rules**:

| Event | Rule |
| ----- | ---- |
| Startup with no saved preference | Use `HumanVersusCpu`. |
| Startup with saved preference | Load and apply the saved selected mode. |
| Mode changed by user | Save the selected mode to disk before or as part of fresh game reset. |

## PlayerSide

| Value | Meaning |
| ----- | ------- |
| `Near` | Bottom player using bottom hand/slot presentation. |
| `Far` | Top player using off-screen hand and top slots. |

## PlayerControllerModel

| Field | Type | Validation |
| ----- | ---- | ---------- |
| `controller` | enum | `PlayerController` or `CpuController`. |
| `brain_level` | optional enum | Present only for `CpuController`; only `CpuBrainLevel::Level1` is valid. |

**Rules**:

| Controller | Rule |
| ---------- | ---- |
| `PlayerController` | Dispatches mouse, keyboard, and tap choices to shared game logic. |
| `CpuController` | Uses CPU Brain to dispatch choices to the same shared game logic; never dispatches Undo. |

**Mode mapping**:

| Match Mode | Near Controller | Far Controller |
| ---------- | --------------- | -------------- |
| `Human versus CPU` | PlayerController | CpuController with CPU Brain Level 1 |
| `CPU versus CPU` | CpuController with CPU Brain Level 1 | CpuController with CPU Brain Level 1 |

## CpuBrainModel

| Field | Type | Validation |
| ----- | ---- | ---------- |
| `level` | enum | Must be `Level1`. |
| `state` | enum | `Idle`, `Thinking`, `MovingCard`, or `Ready`. |
| `next_decision_at` | time/tick value | Used only for CPU pacing; no human timer. |
| `random_seed` | integer or seed value | May be supplied for deterministic tests and controlled simulations. |

**Rules**:

| Rule | Requirement |
| ---- | ----------- |
| Runtime AI | CPU Brain is authored game code and does not call a runtime generative AI model. |
| Goal | CPU Brain attempts to win the game. |
| Move choice | Level 1 chooses legal affordable drag-equivalent moves that improve win chances when it can evaluate them. |
| Randomness | When multiple acceptable legal moves are available at one decision point, Level 1 may choose among them randomly. |
| Determinism | The same state and same random seed must produce the same move sequence in tests. |
| Pacing | Every CPU move or non-move readiness decision is delayed by 0.5 to 1 second. |
| Stop condition | Marks Next when no energy remains or no legal affordable move exists. |
| Visibility | Hidden from user-facing labels and mode button text. |
| Reconsideration | CPU Brain does not undo or reconsider legal moves once dispatched. |

## CpuBrainKnowledgeModel

| Field | Type | Visibility Rule |
| ----- | ---- | --------------- |
| `own_hand_cards` | card list | Known to the CPU Brain for that CPU player. |
| `own_deck_next_cards` | none | Not available; Brain cannot see what is next in its own deck. |
| `open_locations` | location list | Known when locations are open; includes location abilities. |
| `revealed_slots` | slot/card list | Known for both player sides after cards are revealed. |
| `opposing_current_turn_hidden_placements` | hidden count/slots only | Card identity and values are not available until end-of-turn reveal. |

## MatchPlayerModel

| Field | Type | Validation |
| ----- | ---- | ---------- |
| `side` | `PlayerSide` | Exactly one `Near` and one `Far` per match. |
| `controller` | `PlayerControllerModel` | Must match active mode. |
| `deck` | `PlayerDeck` / active game deck | Independent per player; copied from the same 12-card master deck for this feature. |
| `hand` | `PlayerHand` / active game hand | Cards must originate from that player's deck. |
| `energy_available` | integer | Follows the current turn's energy rules. |
| `ready_for_next` | boolean | Reset to false at the start of each turn. |

## MasterDeckModel

| Field | Type | Validation |
| ----- | ---- | ---------- |
| `cards` | card list | Exactly 12 cards for this feature. |

**Rules**:

| Rule | Requirement |
| ---- | ----------- |
| Fresh game copy | Near and far players each receive their own copy of the same master deck. |
| Independence | Dealing from one player's deck must not remove cards from the other player's deck. |
| Future scope | Different player-specific decks are deferred to a later feature. |

## MatchTurnModel

| Field | Type | Validation |
| ----- | ---- | ---------- |
| `turn` | integer | Clamped to 1 through 6. |
| `max_turns` | integer | Always 6 for this feature. |
| `near_ready` | boolean | Round cannot advance until true. |
| `far_ready` | boolean | Round cannot advance until true. |
| `winner_state` | optional result | Empty until both players are ready on turn 6. |

**Transitions**:

| State | Event | Result |
| ----- | ----- | ------ |
| Turn 1-5, one player ready | Other player not ready | Stay on current turn. |
| Turn 1-5, both ready | Resolve readiness | Lock current placements, deal next turn, reset readiness. |
| Turn 6, both ready | Resolve readiness | Evaluate winner and stop turn advancement. |
| Any turn | Restart | Reset to turn `1/6`, clear winner, clear readiness. |
| End of turn | Both players ready | Reveal all current-turn hidden placements before the next turn or final scoring. |
| `CPU versus CPU`, no human input | CPU pacing elapses | CPU controllers continue moves, readiness, turn advancement, and final scoring automatically. |

## PlacementVisibilityModel

| Field | Type | Validation |
| ----- | ---- | ---------- |
| `owner` | `PlayerSide` | The player who owns the placed card. |
| `placement_turn` | integer | The turn when the card entered a location. |
| `visibility` | enum | `CurrentTurnHidden` or `Revealed`. |
| `owner_can_view_front` | boolean | True for usability when the owning human views own current-turn cards. |

**Visibility rules**:

| Situation | Human Near Player Sees | CPU Brain Sees |
| --------- | ---------------------- | -------------- |
| Near hand card | Card front | Not applicable unless CPU owns that hand. |
| Far CPU hand card | Face-down card | CPU Brain knows its own hand identity and values. |
| Near current-turn placement | Card front to near human | Hidden from far CPU Brain until reveal. |
| Far current-turn placement | Face-down to near human | Known to owning far CPU Brain. |
| Prior-turn placement | Face up | Known if revealed. |

## CardRenderInteractionModel

| Field | Type | Validation |
| ----- | ---- | ---------- |
| `owner` | `PlayerSide` | The player who owns the rendered card. |
| `controller` | `PlayerControllerModel` | The active controller for that owner. |
| `cursor_responsive` | boolean | True only for local human-owned cards that are currently interactable. |
| `allows_cursor_rotation` | boolean | False for all CPU-owned cards. |
| `allows_drag_affordance` | boolean | False for all CPU-owned cards. |

**Rules**:

| Rule | Requirement |
| ---- | ----------- |
| CPU-owned card rendering | CPU-owned cards are passive to mouse cursor hover, drag affordances, and cursor-facing rotation effects. |
| Human-owned card rendering | Local human-owned cards may keep existing hover, drag, and rotation behavior only while they are legal/interactable. |

## LocationSlotSideModel

| Field | Type | Validation |
| ----- | ---- | ---------- |
| `location_index` | integer | Must be 0, 1, or 2. |
| `side` | enum | `Near` maps to bottom slots; `Far` maps to top slots. |
| `slot_index` | integer | Must fit the per-player location capacity. |
| `card_id` | optional card identity | Present only when occupied. |

## MatchScoreModel

| Field | Type | Validation |
| ----- | ---- | ---------- |
| `location_results` | array | Exactly three location results, evaluated left to right. |
| `near_locations_won` | integer | 0 through 3. |
| `far_locations_won` | integer | 0 through 3. |
| `winner` | enum | Must be `Near` or `Far`; never draw. |

## MatchStatusTextModel

| Field | Type | Validation |
| ----- | ---- | ---------- |
| `status_prefix` | string | Must render as `Status:`. |
| `winner_player_number` | integer | `1` for near player, `2` for far player. |
| `winner_controller_type` | enum | `Human` or `CPU` based on the winning player's active controller. |
| `visible_after_final_result` | boolean | True after final winner evaluation; hidden or neutral before a final result exists. |

**Presentation rule**: After turn 6 final scoring in any mode, GameView shows status text above the Mode button, for example `Status: Winner is Player 1 (CPU)`.

**Winner rules**:

| Step | Rule |
| ---- | ---- |
| 1 | Compare near bottom-slot total power vs far top-slot total power for each location. |
| 2 | Award each location to the higher total power. |
| 3 | If a location is tied, apply a deterministic tiebreaker so one player wins the location. |
| 4 | Award match to the player with two or more location wins. |
