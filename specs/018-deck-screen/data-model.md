# Data Model: DeckScreen Meta Game UI

## `TopNavigationModel`

| Field | Type | Description |
| ----- | ---- | ----------- |
| `selected` | `TopNavigationDestination` | Current selected destination. |
| `items` | `Vec<TopNavigationDestination>` | Ordered destinations rendered by the top nav. |

## `TopNavigationDestination`

| Variant | Label | Initial Scope |
| ------- | ----- | ------------- |
| `PlayGame` | `Play Game` | Destination label only for this feature. |
| `MyDecks` | `My Decks` | Selected and active on DeckScreen. |
| `Settings` | `Settings` | Destination label only for this feature. |
| `Debug` | `Debug` | Destination label only for this feature. |

## `DeckScreenModel`

| Field | Type | Description |
| ----- | ---- | ----------- |
| `mode` | `DeckScreenMode` | Deck selection or selected deck editor. |
| `selected_deck_id` | `Option<String>` | Persisted deck currently being edited. |
| `editor_tab` | `DeckEditorTabModel` | Active `Not In Deck` tab. |
| `modal` | `Option<DeckScreenCardModalModel>` | Current fullscreen card overlay state. |

## `DeckScreenMode`

| Variant | Description |
| ------- | ----------- |
| `DeckSelection` | Shows `New Deck` and deck tiles. |
| `Editor` | Shows selected deck cards and available cards. |

## `DeckEditorTabModel`

| Variant | Description |
| ------- | ----------- |
| `Library` | Shows owned cards not in selected deck. |
| `Shop` | Shows an empty shop state in this feature. |

## `DeckEditableCardModel`

| Field | Type | Description |
| ----- | ---- | ----------- |
| `entry_id` | `String` | Stable editable entry identity. |
| `card_id` | `String` | Card model ID. |
| `zone` | `DeckEditableZoneModel` | `Deck`, `Library`, or `Shop`. |
| `is_owned` | `bool` | Whether the entry is owned by the player. |
| `is_selectable` | `bool` | Real cards are selectable; empty slots are not. |

## `DeckScreenCardModalModel`

| Field | Type | Description |
| ----- | ---- | ----------- |
| `entry_id` | `String` | Selected card entry. |
| `card_id` | `String` | Selected card model ID. |
| `source_zone` | `DeckEditableZoneModel` | Zone the modal opened from. |
| `actions` | `DeckScreenModalActionsModel` | Enabled/disabled action flags. |

## Validation Rules

| Rule | Requirement |
| ---- | ----------- |
| Editor deck | `Editor` requires a selected deck. |
| Modal selection | Modal cannot reference an empty slot. |
| Modal input | Any open modal blocks lower DeckScreen and top nav input. |
| Duplicate safety | One persisted card entry cannot appear in both deck and library. |
| Shop mutation | Shop entries are absent until a future purchase feature. |

## Deck View Constraint

| Requirement | Detail |
| ----------- | ------ |
| DeckViewBundle | Implementation MUST create a DeckViewBundle that renders a deck tile using the existing card back asset and the deck name only. |
