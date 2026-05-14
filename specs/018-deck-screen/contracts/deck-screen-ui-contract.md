# Contract: DeckScreen UI And Interaction

## Navigation Contract

| Requirement | Required Behavior |
| ----------- | ----------------- |
| Top nav labels | Render exactly `Play Game`, `My Decks`, `Settings`, and `Debug`. |
| Selected tab | `My Decks` is selected while DeckScreen is active. |
| Overlay blocking | While the DeckScreen card modal is open, lower nav tabs do not receive hover, click, focus, or activation. |
| Screen retention | Closing the modal returns to DeckScreen, not another scene or view. |

## Layout Contract

| Region | Required Behavior |
| ------ | ----------------- |
| Safe area | All visible DeckScreen UI stays inside the 16:10 safe area. |
| Main split | Main content has a left deck collection panel and right editing panel. |
| Left panel | Shows `Player 01`, selected `Deck01`, and `New Deck`. |
| Right panel | Uses two columns: `Deck 01` and `Not In Deck`. |
| Available tabs | `Not In Deck` contains `Library` and `Shop` sub-tabs with selected and disabled states. |
| Spacing | Text, controls, card tiles, empty slots, and modal actions do not overlap at supported desktop and browser sizes. |

## Card Tile Contract

| Tile Type | Required Behavior |
| --------- | ----------------- |
| Real deck card | Shows card title, art area, cost metadata, and power metadata; selectable. |
| Real available card | Shows card title, art area, cost metadata, and power metadata; selectable. |
| Empty slot | Shows an explicit empty-slot visual; no card title, art, cost, or power; not selectable as a card. |
| Selected source | Card selected for modal can be visually indicated in the underlying list but lower list remains non-interactive while modal is open. |
| Disabled card/action | Uses a distinct disabled state and does not mutate model state on input. |

## Modal Contract

| Requirement | Required Behavior |
| ----------- | ----------------- |
| Trigger | Selecting a real card tile opens the modal. |
| Preview | Modal shows a large card preview centered inside the safe area. |
| Dimming | Modal dims lower DeckScreen content. |
| Input capture | Modal consumes pointer/input outside the action rail and preview so lower content cannot react. |
| Actions | Side rail shows `Move To Deck 01`, `Move To Library`, `Transfer Out`, and `Back`. |
| Back | Closes modal without changing deck data. |
| Gameplay separation | Modal does not use gameplay selected-card inspection controls. |

## Action Enablement Contract

| Action | Enabled When | Disabled When |
| ------ | ------------ | ------------- |
| `Move To Deck 01` | Selected card is in `Library` or implemented `Shop`, and adding it would not duplicate the same entry ID in `Deck01`. | Selected card is already in `Deck01`, empty slot is selected, destination is unavailable, or move would duplicate a card entry. |
| `Move To Library` | Selected card is in `Deck01` or implemented `Shop` and can be stored as owned library content. | Selected card is already in `Library`, empty slot is selected, or card is not an owned transferable copy. |
| `Transfer Out` | Transfer-out behavior is implemented for the selected card source. | Transfer-out behavior is not implemented, no real card is selected, or transfer would corrupt persisted ownership. |
| `Back` | Modal is open. | Never disabled while modal is open. |

## Persistence Contract

| Requirement | Required Behavior |
| ----------- | ----------------- |
| Data source | DeckScreen reads from persisted player deck collection. |
| Active gameplay copy | DeckScreen does not read from or mutate active in-game hand/deck copies. |
| Immediate UI update | Accepted moves update visible `Deck 01` and `Not In Deck` lists immediately. |
| Save | Accepted moves are persisted so restart restores membership and order. |
| Order | Existing `Deck01` order is preserved except where an accepted move intentionally inserts or removes a card. |
| Duplicate safety | A persisted card entry appears in exactly one editable zone after every accepted move. |

## QA Contract

| Test Area | Verification |
| --------- | ------------ |
| Navigation | `My Decks` opens DeckScreen and selected state is visible. |
| Deck list | `Player 01`, `Deck01`, and `New Deck` are readable and selectable. |
| Right columns | `Deck 01` count and ordering match persisted data; `Not In Deck` excludes deck entries. |
| Tabs | `Library` and `Shop` selected states change predictably without changing selected deck. |
| Modal | Background is dimmed, lower UI is blocked, actions are visible, `Back` closes modal. |
| Moves | Move actions update model and UI immediately and survive restart. |
| Resize | Desktop and browser layouts remain within safe area with no overlapping text or controls. |


## Deck View Constraint

| Requirement | Detail |
| ----------- | ------ |
| DeckViewBundle | Implementation MUST create a DeckViewBundle that renders a deck tile using the existing card back asset and the deck name only. |
