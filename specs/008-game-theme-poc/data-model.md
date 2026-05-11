# Data Model: Game Theme POC

## Entities

| Entity | Fields | Relationships | Validation Rules |
| ------ | ------ | ------------- | ---------------- |
| `WorldTheme` | `id`, `display_name`, `background_texture`, `lighting_profile`, `location_treatment` | Active GameScene has one `WorldTheme`. | Supported values are `bamboo_forest` and `coastal_harbor`; paths use lowercase `snake_case`; changing world does not change CardUI settings or card identities. |
| `TacticalLocation` | `id`, `display_name`, `base_art`, `world_variant_treatment` | GameScene displays three selected locations from six total. | Supported values are `fortress_gate`, `bamboo_crossing`, `shrine_ruins`, `battlefield`, `spirit_well`, and `market_square`; each selection shows exactly three locations. |
| `CardCharacter` | `id`, `name`, `title`, `visual_family`, `background_texture`, `foreground_texture`, `frame_texture`, `title_texture`, `composition` | GameScene displays four cards; Card Browser focuses one selected card. | Supported values are `kage_ren`, `lord_daichi`, `sister_hotaru`, and `yokai_placeholder`; every card uses 9:16 composition and Japan Realism constraints; card fronts render in background, frame, foreground, title order. |
| `CardUiSettings` | `depth_factor`, `background_layer_scale`, `frame_layer_scale`, `foreground_layer_scale`, `title_layer_scale` | Applied globally to all cards; persisted through existing card settings storage. | Settings are not world-specific and not card-specific; layer scales default to `1.0`, clamp to `0.0..=2.0`, apply uniformly to x/y, and preserve each layer center point. |
| `CardFlipSessionState` | `current_y_rotation`, `target_y_rotation`, `visible_face` | Applies to the currently viewed Card Browser card. | State is temporary and is not written to CardUI settings or card identity data. |
| `ActiveScene` | `game`, `card_browser` | Determines whether `T` controls world theme or CardUI settings. | In GameScene, `T` cycles world theme; in Card Browser, `T` changes CardUI settings. |

## State Transitions

| Trigger | Starting State | Result | Persistence |
| ------- | -------------- | ------ | ----------- |
| Open game | No active scene visible | GameScene shows one active world, three locations, and four bottom cards. | Active world initial value is runtime state only unless implementation explicitly persists it later. |
| Press `T` in GameScene | Active world is Bamboo Forest or Coastal Harbor | Active world switches to the other supported world and three locations are selected/rendered. | Does not modify CardUI settings. |
| Click/tap bottom card | GameScene visible | Card Browser opens focused on clicked card. | Focused card is navigation state; not a durable card setting. |
| Press `T` in Card Browser | Card Browser visible | Global CardUI settings change and visible card presentation updates. | CardUI settings are stored globally. |
| Flip card in Card Browser | Card Browser visible | Current viewed card flips front/back for animation testing. | Flip state is temporary and not stored. |
| Return to GameScene | Card Browser visible | GameScene returns with active world unchanged. | CardUI settings continue applying globally to cards. |

## Asset Identity

| Asset Group | Required IDs | Root Path |
| ----------- | ------------ | --------- |
| Card structure | `card_back_japan_realism` | `bevy/crates/game/assets/cards/card_structure/` |
| Card types | `card_type_kage_ren`, `card_type_lord_daichi`, `card_type_sister_hotaru`, `card_type_yokai_placeholder` | `bevy/crates/game/assets/cards/card_types/` |
| Worlds | `bamboo_forest`, `coastal_harbor` | `bevy/crates/game/assets/worlds/` |
| Locations | `fortress_gate`, `bamboo_crossing`, `shrine_ruins`, `battlefield`, `spirit_well`, `market_square` | `bevy/crates/game/assets/locations/` |

| Card Front Layer | Asset Field | Layer Requirement |
| ---------------- | ----------- | ----------------- |
| Background | `background_texture` | Environment-only art, no character, no frame, no title text. |
| Frame | `frame_texture` | Card structure treatment between the environment and character. |
| Foreground | `foreground_texture` | Character-only alpha layer; may selectively overlap or extend beyond the frame; AI source images use `#ff00ff` chroma key before alpha extraction. |
| Title | `title_texture` | Character-name-only stylized raster art with alpha around the title shape; AI source images use `#ff00ff` chroma key before alpha extraction. |
