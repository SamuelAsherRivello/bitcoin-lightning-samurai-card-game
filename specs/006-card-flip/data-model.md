# Data Model: Card Flip

## CardFlipState

| Field | Meaning | Validation |
| ----- | ------- | ---------- |
| Current angle | The current y-axis flip angle applied to the card side-selection animation | Advances smoothly and remains finite |
| Target angle | The y-axis flip angle the current animation is moving toward | Changes by 180 degrees for each accepted flip activation |
| Direction | Whether the active animation is moving toward front or back | Reverses from current progress when `Flip` is clicked mid-animation |
| Active side | The side whose graphics are currently visible: CardFront or CardBack | Changes only at the flip midpoint |
| Is animating | Whether current angle is still moving toward target angle | Clears when the target angle is reached |

## CardFace

| Value | Meaning | Validation |
| ----- | ------- | ---------- |
| CardFront | The current multi-layer front presentation from `005-card-polish` | Visible before midpoint in front-to-back flips and after midpoint in back-to-front flips |
| CardBack | The shared card-series superhero-pattern rectangular backface | Visible after midpoint in front-to-back flips and before midpoint in back-to-front flips |

## CardBackVisual

| Field | Meaning | Validation |
| ----- | ------- | ---------- |
| Texture path | Shared superhero-pattern backface texture under `bevy/crates/game/assets/cards/card_structure/` | Exists as a runtime asset and is not in an individual front-art folder |
| Dimensions | Rectangular card face matching established card proportions | Preserves current card silhouette during flip |
| Front relationship | CardSeries/CardStructure-level visual independent of active card front | Active front toggles do not change this visual |
| Art direction | Bold abstract superhero-game pattern compatible with existing front palettes | Contains no words, readable letters, characters, logos, or clear symbols |
| Future brand relationship | Visual may inform future box cover or main menu theme later | Does not define those future surfaces in this feature |

## FlipButtonAction

| Field | Meaning | Validation |
| ----- | ------- | ---------- |
| Source | Existing Card UI `Flip` button | Activates from Card UI only |
| Front/back toggle | Determines whether the next target is front or back | Initial side is CardFront |
| Mid-animation behavior | Reverses direction from current progress | No queued or ignored flip state |

## ActiveCardFrontSelection

| Field | Meaning | Validation |
| ----- | ------- | ---------- |
| Active front index | The currently selected prototype card front entry | Changes when `T` is pressed |
| Visible effect when face up | Front artwork changes immediately | Applies only while CardFront is visible |
| Visible effect when face down | CardBack stays visible | Changed front appears after flipping face up |

## State Transitions

| From | Event | To |
| ---- | ----- | -- |
| Front idle | Click `Flip` | Animating toward back |
| Back idle | Click `Flip` | Animating toward front |
| Animating toward back before midpoint | Click `Flip` | Animating toward front from current progress |
| Animating toward front before midpoint | Click `Flip` | Animating toward back from current progress |
| Animating reaches midpoint | Frame update | Active side switches according to direction |
| Animating reaches target | Frame update | Front idle or back idle |
