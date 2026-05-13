# Contract: GameView Round Flow

This contract defines externally visible GameView behavior for the local near-player loop. It is a UI/runtime contract, not a network API.

## Startup And Restart

| Event | Expected Behavior |
| ----- | ----------------- |
| GameView opens fresh | Round displays `1/6`; lower-right End Round is visible; lower-left Restart and Undo are visible; left location is open; middle and right locations are closed; Undo is disabled until a current-round placement exists |
| Restart pressed | Active GameView play state is cleared; a new randomized 12-card deck is created; round returns to `1/6`; energy returns to `1/1`; location open/closed state resets to round 1; current-round history is empty |
| Restart during animation | Final visible and model state is a clean fresh game with no old cards, energy, or move history mixed in |

## Round Schedule

| Round | Cards Dealt From Player Deck | Energy Label At Start |
| ----- | ---------------------------- | --------------------- |
| 1 | 1 | `Energy 1/1` |
| 2 | 2 | `Energy 2/2` |
| 3 | 3 | `Energy 3/3` |
| 4 | 1 | `Energy 4/4` |
| 5 | 1 | `Energy 5/5` |
| 6 | 1 | `Energy 6/6` |

## Deal Rules

| Rule | Expected Behavior |
| ---- | ----------------- |
| Preserve deck order | Remaining deck cards are selected from the randomized deck order |
| No energy gate | A card may be dealt regardless of whether its energy equals the current round's energy grant |
| Every round | Rounds 1 through 6 each deal cards from deck to hand according to the round schedule |
| Partial deck | If fewer cards remain than requested, only remaining deck cards are dealt |

## Card Values

| Card | Power | Energy Cost |
| ---- | ----- | ----------- |
| kage | 1 | 1 |
| sister | 2 | 1 |
| Lord | 3 | 2 |
| test | 4 | 3 |

## Location Display And Effects

| Location | Open Round | Closed Title | Closed Body | Open Title | Open Body | Effect While Open |
| -------- | ---------- | ------------ | ----------- | ---------- | --------- | ----------------- |
| Left | 1 | `Closed Until Round 1` before open | Empty | `Fortress Gate` | `+2 Energy to each card here` | Add `+2` to each placed card's effective energy |
| Middle | 2 | `Closed Until Round 2` before open | Empty | `Bamboo Crossing` | `-2 Energy to each card here` | Add `-2` to each placed card's effective energy |
| Right | 3 | `Closed Until Round 3` before open | Empty | `Normal` | `(No Ability)` | No effect |

| Rule | Expected Behavior |
| ---- | ----------------- |
| Text layout | Each location shows a horizontally centered title text area and body text area; title is larger, up to two lines, and placed about 30% from the location top; body is up to three lines below it |
| Closed state | Closed locations show only `Closed Until Round X` as the title and no body text |
| Open state | Open locations show their own title and ability body |
| Ability application | Only open locations apply their ability to cards placed there |
| Add card to open location | The location ability updates the placed card immediately |
| Location opens with cards already there | The location ability updates those placed cards immediately |
| Remove card from open location | The location ability is removed immediately, including when Undo returns the card to hand |

## Visible Controls

| Control | Position | Enabled Rule | Label Rule |
| ------- | -------- | ------------ | ---------- |
| End Round | Lower right safe-area HUD | Enabled through round 6 | Shows `End Round` and the current round fraction |
| Restart | Lower left safe-area HUD, above Undo | Always enabled during GameView play | Shows `Restart` |
| Undo | Lower left safe-area HUD, below Restart | Enabled only when current-round move history is non-empty | Shows `Energy current/max` newline `Undo` |

## Placement And Undo

| Action | Expected Behavior |
| ------ | ----------------- |
| Place affordable card | Card moves from hand to legal local location slot; energy decreases by card energy cost; open location ability updates the card's effective energy; move record is added to current-round history |
| Place unaffordable card | Card remains in hand; energy is unchanged; current-round history is unchanged |
| Undo with current-round moves | Only current-round moved cards return to hand; active location ability effects are removed from those cards; energy spent by those moves is restored; hand group recenters |
| Undo with no current-round moves | Button is disabled or visually greyed out and performs no state change |
| End Round | Current-round undo history clears; prior-round placed cards remain placed |

## Final Winner Calculation

| Rule | Expected Behavior |
| ---- | ----------------- |
| Location score | Each team's score at a location is the sum of that team's placed-card power at that location |
| Location win | The team with the higher location score wins that location |
| Tied location | Equal location scores award no location win to either team |
| First comparison | The final winner is the team with more locations won |
| First tie-breaker | If locations won are tied, the team with higher total power points across all locations wins |
| Second tie-breaker | If locations won and total power points are tied, the team with more cards played during the game wins |
| Full tie | If locations won, total power points, and cards played are all tied, the game result is a draw |

## Hand Layout And Dealing

| Situation | Expected Behavior |
| --------- | ----------------- |
| Card dealt | Card starts below screen center at x = `screen_width / 2`, animates into hand, and lines up to the right of existing hand cards |
| Empty deck | No deal animation plays and the hand remains unchanged |
| Hand changes | Entire hand group recenters within the hand area |
| Wide hand | More than four or five cards may exceed the hand area's width while preserving centered group alignment |
