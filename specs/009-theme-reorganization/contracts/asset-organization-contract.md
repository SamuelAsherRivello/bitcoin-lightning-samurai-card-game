# Contract: Theme Asset Organization

## Theme Root

| Contract Item | Requirement |
| ------------- | ----------- |
| Root path | `bevy/crates/game/assets/themes/theme_japan` |
| Required categories | `cards`, `locations`, `worlds` |
| Theme identity | Present in `theme_japan`; not repeated by owned card, location, or world asset names. |
| Shared assets | Reusable non-theme assets remain outside the category folders. |

## Scene And View Naming

| Contract Item | Requirement |
| ------------- | ----------- |
| Persistent scene | `AppScene` is the always-present app-level scene. |
| Active sub-screen views | `GameScene`, `DeckScene`, and `DebugScene` are loaded on top of `AppScene`; at most one is active at a time. |
| Naming | Use `Scene` for the persistent app-level container and `View` for rendered sub-screen presentations. |

## Cards

| Contract Item | Requirement |
| ------------- | ----------- |
| Category path | `bevy/crates/game/assets/themes/theme_japan/cards` |
| Card folders | `card_kage_ren`, `card_lord_daichi`, `card_sister_hotaru`, `card_yokai_placeholder` |
| Model/View naming | Card data is named `CardModel`; rendered card presentation is named `CardView`. |
| View bundle contents | `CardViewBundle` creates the root visual entity for one rendered card; card view systems spawn the front layers and back presentation below that root. |
| Naming | Card folders start with `card_` and do not include `japan`; Rust data/rendering names use `Model` and `View` to disambiguate ownership. |
| Behavior | Existing bottom-row display, click-to-DeckScene navigation, `DeckScene` viewing, and flip behavior remain unchanged. |

## Locations

| Contract Item | Requirement |
| ------------- | ----------- |
| Category path | `bevy/crates/game/assets/themes/theme_japan/locations` |
| Location folders | `location_fortress_gate`, `location_bamboo_crossing`, `location_shrine_ruins`, `location_battlefield`, `location_spirit_well`, `location_market_square` |
| Naming | Location folders start with `location_` and do not include `japan`. |
| Behavior | Existing visible tactical location presentation remains unchanged. |

## Worlds

| Contract Item | Requirement |
| ------------- | ----------- |
| Category path | `bevy/crates/game/assets/themes/theme_japan/worlds` |
| World folders | `world_bamboo_forest`, `world_coastal_harbor`, `world_suji_swamp`, and future `world_<name>` folders. |
| Naming | World folders start with `world_` and do not include `japan`. |
| Background art layout | World background image creation follows `specs/008-game-theme-poc` world slot-treatment requirements: exactly six diegetic slot treatments aligned to the runtime slot rectangles, with no extra slot-like clearings outside those areas. |
| Behavior | Existing active world display and world toggle behavior remain unchanged. |

## Verification

| Contract Item | Requirement |
| ------------- | ----------- |
| Static path verification | Tests or review confirm all runtime card, location, and world paths resolve below `themes/theme_japan`. |
| Naming verification | Tests or review confirm no owned card, location, or world asset name repeats `japan` outside the theme root. |
| Runtime verification | Existing desktop and browser workflows are checked after path migration. |
