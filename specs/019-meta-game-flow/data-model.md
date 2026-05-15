# Data Model: Meta Game Flow

## ActiveView

Represents the active child view/screen layered with persistent `AppScene`.

| Field | Type | Validation |
| ---- | ---- | ---- |
| Variant | Enum | Must be one of MainMenuScene, LightningScene, MatchmakingScene, GameScene, DeckScene, SettingsScene, DebugScene. |

## TopNavigationDestination

Represents user-facing top navigation destinations.

| Field | Type | Validation |
| ---- | ---- | ---- |
| Destination | Enum | Play Game, My Decks, Settings, Debug in that exact order. |
| Selected | Derived boolean | Selected when it maps to the current active screen. |

## MatchmakingModel

Tracks fake matchmaking progress after Start Game.

| Field | Type | Validation |
| ---- | ---- | ---- |
| phase | Enum | Searching, Found, Loading, Preparing. |
| elapsed_seconds | f32 | Non-negative. |
| preload_handles | UntypedHandle[] | Strong handles for match card, world, location, and font assets during Loading. |
| MATCH_ASSETS_PRELOAD_ENABLED | const bool | Defaults to true; false skips Loading and enters Preparing after Found. |

### State Transitions

| From | Event | To |
| ---- | ---- | ---- |
| Searching | elapsed >= 0.5 seconds | Found |
| Found | elapsed >= 0.5 seconds | Loading |
| Loading | all preload handles loaded with dependencies | Preparing |
| Preparing | elapsed >= 0.5 seconds | GameScene |
| Any | screen reload | Searching with 0.0 elapsed |

## MatchModel

Represents one complete runtime game between two players.

| Field | Type | Validation |
| ---- | ---- | ---- |
| mode | MatchModeModel | HumanVersusCpu or CpuVersusCpu. |
| world | MatchWorldModel | Must reference a valid `WorldModelRegistry` index. |
| locations | MatchLocationSelectionModel | Must contain exactly three active location registry indices. |
| near | MatchPlayerModel | Player 1 runtime state. |
| far | MatchPlayerModel | Player 2 runtime state. |
| round | MatchRoundModel | Round 1 through max rounds; winner optional until match end. |
| placements | PlacementVisibilityModel[] | Hidden/revealed placement state for cards on the board. |
| pending_cpu_placements | CpuBrainMoveModel[] | Temporary CPU move queue for the current planning/resolution step. |
| resolution_phase | MatchResolutionPhase | Planning, CpuPlacementsMoving, or CpuPlacementsRevealing. |

## MatchPlayerModel

Represents one player inside a match, not the long-lived deck library owner.

| Field | Type | Validation |
| ---- | ---- | ---- |
| side | MatchPlayerSide | Near for Player 1, Far for Player 2. |
| controller | PlayerControllerModel | Human or CPU. |
| deck | Card id[] | Runtime shuffled deck, 12 cards at match start. |
| deck_instance_ids | CardInstanceId[] | Same length and order as `deck`. |
| hand | Card id[] | Cards drawn from the runtime deck. |
| hand_instance_ids | CardInstanceId[] | Same length and order as `hand`. |
| energy_available | i32 | Non-negative during legal play. |
| ready_for_next | bool | True only after the player commits the current round. |

## MetaGameSettingsModel

Stores pre-game settings that must persist locally.

| Field | Type | Validation |
| ---- | ---- | ---- |
| cpu_brain_level | Enum | Level1 only for this pass. |
| selected_mode | MatchModeModel | HumanVersusCpu or CpuVersusCpu. |
| sfx_enabled | bool | Defaults to true. |
| music_enabled | bool | Defaults to true. |

## LightningModel

The LightningScreen has no authenticated state in this pass.

| Field | Type | Validation |
| ---- | ---- | ---- |
| qr_placeholder | Display-only | Must not encode secrets or real credentials. |
| learn_url | String | Public non-secret URL about Bitcoin Lightning nodes. |
