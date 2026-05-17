# Game Event Sequence

## One-Round Event Hierarchy

| # | Major event | Sub events |
|---|---|---|
| 1 | Game Start | Load app resources; enter `GameScene`; reset match state; choose world and locations; spawn world, locations, controls, and card presentation roots. |
| 2 | Round Start | Set active round number and energy; open any newly available location state; complete round-start presentation gates; only after the round-start sequence is complete, deal round cards. |
| 3 | Location Intro | On game start only, reveal location 1, wait, reveal location 2, wait, reveal location 3; each location fades from 0% to 100% opacity and scales from 150% to 100%. |
| 4 | Deal Near | Add near-player cards to the authoritative hand model; spawn or sync near hand card views from the deal source. |
| 5 | Deal Far | Add far-player cards to the match hand model; spawn or sync far passive hand card views from the deal source. |
| 6 | Planning | Human may move current-round hand cards into local location slots; CPU waits for hand presentation, chooses legal moves, and marks ready; the human presses End Round and marks ready. |
| 7 | Resolution Start | When both players are ready, commit pending CPU moves; instantly flip all human cards moved during this round from front to back; request card flip audio. |
| 8 | Human Reveal | Reveal current-round human cards first; process human cards left-to-right by location and by side-specific slot order; each reveal uses the swan flip and only updates location power after that card's reveal completes. |
| 9 | CPU Reveal | Reveal current-round CPU cards after human reveals; process CPU cards left-to-right by location and by side-specific slot order; each reveal uses the swan flip and only updates location power after that card's reveal completes. |
| 10 | Round Finish | Lock revealed current-round cards; if this was not round 6, advance to the next round and return to Round Start; if this was round 6, compute final location control and finish the game. |
| 11 | Game Finish | Store winner state; update match status; stop round advancement. |

## Animation Types

| Animation type | Trigger | Visual behavior | Audio | Gameplay timing rule |
|---|---|---|---|---|
| Insta-flip | Resolution starts after both players are ready. | Non-animated face change from front to back for human cards moved during the current round. | Requests `CardFlip`. | Happens before any current-round reveal animation starts. |
| Flip | Generic card face change or non-swan card flip behavior. | Rotates between front and back using the standard card flip timing. | Requests `CardFlip` when triggered by a gameplay action. | Presentation-only unless a gameplay system explicitly changes reveal state. |
| Swan flip | Current-round card reveal. | Animated back-to-front reveal with the swan-style scale/rotation flourish. | Requests `CardFlip` at reveal start and swan timing cues where configured. | Location power updates only after the reveal animation completes and placement state becomes `Revealed`. |
| Deal slide | Round deal spawns hand cards. | Cards move from a deal source into the near or far hand layout. | Requests slide/deal audio through the audio manager when hand counts increase. | Must wait until the Round Start sequence has completed. |
| Card move to slot | CPU commits a planned placement. | Passive CPU card moves from hand to the selected location slot while face-down. | Movement audio may be requested by audio manager state changes. | Does not update location power by itself. |
| Drag/place tween | Human drags or places a card. | Local card follows pointer, scales while dragged, then tweens into hand or location slot. | Movement audio may be requested by accepted placement systems. | Does not update location power by itself. |
| Location intro | Game start location presentation. | Whole location bundle fades 0% to 100% opacity and scales 150% to 100%, one location at a time. | No required gameplay audio. | Round-one deal waits until location 3 intro is complete. |
