# Data Model: Card Bundle

## Runtime Ownership

| Entity | Fields | Validation | Lifecycle |
| ------ | ------ | ---------- | --------- |
| Game Card Runtime | Card components, resources, systems, plugin wiring, tests under `bevy/crates/game` | Owns card-specific geometry, structure, artwork selection, reload integration, and flip behavior | Composed during app startup with shared window, camera, DebugHUD, inspector, and diagnostics |
| Fixed Camera | Existing primary camera transform, projection, clear color | Transform remains stable during pointer inspection and flip | Spawned by camera setup; observed but not mutated by card inspection systems |

## CardGeometry

| Field | Meaning | Validation |
| ----- | ------- | ---------- |
| Width | `0.063` world units for poker-card width | Used consistently by front and back faces |
| Height | `0.088` world units for poker-card height | Height-to-width ratio matches `88:63` within 2% |
| Thickness | Slight visible slab thickness | No bevel and no visible layered z stack |
| Center transform | Card placement at current prototype focus | Card remains centered during inspection and flip |
| Max tilt | Pointer-driven tilt limit | Does not exceed 20 degrees from neutral per supported axis |

## CardInspectionState

| Field | Meaning | Validation |
| ----- | ------- | ---------- |
| Last pointer target | Last valid normalized pointer coordinates in `[-1, 1]` | Missing pointer data keeps last valid target |
| Target rotation | Rotation derived from pointer target | Clamped to max tilt |
| Smoothed rotation | Runtime card inspection orientation | Moves toward target without snapping |
| Response target | Smoothing feel | Reaches target within 100 ms |

## CardStructure

| Field | Meaning | Validation |
| ----- | ------- | ---------- |
| Layers | Background, frame, foreground, title | Maintains apparent-depth order |
| Parallax source | Current smoothed card tilt | Does not use raw pointer directly |
| Frame aperture | Central cutout region | Background renders only through aperture |
| Frame mesh | One frame object with continuous full-card UV mapping | Frame texture reads as one continuous image |
| Frame shine | Tilt-reactive holographic/foil treatment | Bound to frame and preserves layer readability |
| Shared back slot | Series/CardStructure backface presentation | Does not vary by active CardFront |

## CardType

| Field | Meaning | Validation |
| ----- | ------- | ---------- |
| Card type id | Stable artwork identity | Resolves to an available asset set |
| Display name | DebugHUD/Card UI facing label | Never blank for available entries |
| Background visual style | Static/generated texture and material settings | Blue/white repeated icon-like cloud style for initial front |
| Frame visual style | Static/generated texture and material settings | Subtle grey/off-grey 45-degree pinstripe plus shine response |
| Foreground visual style | Static/generated texture and material settings | Flat superhero-inspired foreground art with premium breakout composition |
| Title visual style | Static/generated texture and material settings | Frontmost title art that may break over frame |

## CardTypeRegistry

| Field | Meaning | Validation |
| ----- | ------- | ---------- |
| Slots | Available CardType/CardFront entries | Sized for at least two prototype entries when available |
| Active index | Current selected front | Always resolves to populated artwork |
| Hidden front update | Active front changes while CardBack is visible | Visible CardBack remains unchanged until face-up |

## CardFlipState

| Field | Meaning | Validation |
| ----- | ------- | ---------- |
| Current angle | Current y-axis side-selection flip angle | Advances smoothly and remains finite |
| Target angle | Flip angle target | Changes by 180 degrees for each accepted flip activation |
| Direction | Active animation direction | Reverses from current progress on mid-animation `Flip` |
| Active side | CardFront or CardBack | Changes only at the flip midpoint |
| Is animating | Whether current angle is still moving toward target | Clears when target is reached |

## CardFace

| Value | Meaning | Validation |
| ----- | ------- | ---------- |
| CardFront | Active CardType/CardDefinition front presentation using CardStructure layers | Visible before midpoint in front-to-back flips and after midpoint in back-to-front flips |
| CardBack | Shared card-series superhero-pattern rectangular backface | Visible after midpoint in front-to-back flips and before midpoint in back-to-front flips |

## CardBackVisual

| Field | Meaning | Validation |
| ----- | ------- | ---------- |
| Texture path | Shared backface asset under CardStructure/card-series ownership | Not duplicated under individual front-art folders |
| Dimensions | Rectangular face matching established card proportions | Preserves silhouette during flip |
| Front relationship | Independent from active CardFront | Active front toggles do not change this visual |
| Art direction | Abstract superhero-game pattern compatible with current fronts | Contains no words, readable letters, characters, logos, or clear symbols |

## Debug And Prototype Inputs

| Input | Surface | Meaning | Validation |
| ----- | ------- | ------- | ---------- |
| `T` | DebugHUD | Cycle active CardType/CardFront | Always resolves to valid front artwork |
| `R` | DebugHUD | Reload AppScene card content | Non-toggle operation that preserves DebugHUD state |
| `H` | DebugHUD | Toggle hot-reload auto-restart | Defaults false and persists through local runtime state |
| `Flip` | Card UI | Toggle CardFront/CardBack animation | Activates from temporary Card UI and supports reversal |

## State Transitions

| From | Event | To |
| ---- | ----- | -- |
| Neutral front | Pointer move | Smoothed inspection tilt toward target |
| Front idle | Click `Flip` | Animating toward back |
| Back idle | Click `Flip` | Animating toward front |
| Animating toward back | Click `Flip` | Animating toward front from current progress |
| Animating toward front | Click `Flip` | Animating toward back from current progress |
| Animating reaches midpoint | Frame update | Active side switches according to direction |
| Animating reaches target | Frame update | Front idle or back idle |
| CardFront visible | Press `T` | Active and visible CardFront update |
| CardBack visible | Press `T` | Hidden CardFront update; visible CardBack unchanged |
