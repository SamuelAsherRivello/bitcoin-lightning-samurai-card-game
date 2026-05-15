# Data Model: Game Theme POC

## Entities

| Entity | Fields | Relationships | Validation Rules |
| ------ | ------ | ------------- | ---------------- |
| `WorldTheme` | `id`, `display_name`, `background_texture`, `lighting_profile`, `location_treatment`, `slot_treatment_layout` | Active GameScene has one `WorldTheme`. | Supported values are `bamboo_forest`, `coastal_harbor`, and later registered world IDs such as `suji_swamp`; paths use lowercase `snake_case`; changing world does not change CardUI settings or card identities; each world background uses the required six-slot diegetic treatment layout. |
| `TacticalLocation` | `id`, `display_name`, `base_art`, `world_variant_treatment` | GameScene displays three selected locations from six total. | Supported values are `fortress_gate`, `bamboo_crossing`, `shrine_ruins`, `battlefield`, `spirit_well`, and `market_square`; each selection shows exactly three locations. |
| `CardCharacter` | `id`, `name`, `title`, `visual_family`, `background_texture`, `foreground_texture`, `frame_texture`, `title_texture`, `composition` | GameScene displays four cards; Deck focuses one selected card. | Supported values are `kage_ren`, `lord_daichi`, `sister_hotaru`, and `yokai_placeholder`; every card uses a shared `840 x 1440` front-layer canvas, runtime renders the card silhouette at 2:3, and card fronts render in background, frame, safe-area reference, foreground, title order. |
| `CardUiSettings` | `depth_factor`, `show_safe_area`, `background_layer_scale`, `frame_layer_scale`, `foreground_layer_scale`, `title_layer_scale` | Applied globally to all cards; persisted through existing card settings storage. | Settings are not world-specific and not card-specific; `show_safe_area` only toggles the reference overlay; layer scales default to `1.0`, clamp to `0.0..=2.0`, apply uniformly to x/y, preserve each layer center point, and can be reset individually to `1.0`. |
| `CardFlipSessionState` | `current_y_rotation`, `target_y_rotation`, `visible_face` | Applies to the currently viewed Deck card. | State is temporary and is not written to CardUI settings or card identity data. |
| `ActiveScene` | `game`, `deck` | Determines whether `T` controls world theme or CardUI settings. | In GameScene, `T` cycles world theme; in Deck, `T` changes CardUI settings. |

## State Transitions

| Trigger | Starting State | Result | Persistence |
| ------- | -------------- | ------ | ----------- |
| Open game | No active scene visible | GameScene shows one active world, three locations, and four bottom cards. | Active world initial value is runtime state only unless implementation explicitly persists it later. |
| Press `T` in GameScene | Active world is Bamboo Forest or Coastal Harbor | Active world switches to the other supported world and three locations are selected/rendered. | Does not modify CardUI settings. |
| Click/tap bottom card | GameScene visible | Deck opens focused on clicked card. | Focused card is navigation state; not a durable card setting. |
| Press `T` in Deck | Deck visible | Global CardUI settings change and visible card presentation updates. | CardUI settings are stored globally. |
| Flip card in Deck | Deck visible | Current viewed card flips front/back for animation testing. | Flip state is temporary and not stored. |
| Return to GameScene | Deck visible | GameScene returns with active world unchanged. | CardUI settings continue applying globally to cards. |

## Asset Identity

| Asset Group | Required IDs | Root Path |
| ----------- | ------------ | --------- |
| Card structure | `card_back_japan_realism`, `safe_area` | `bevy/crates/game/assets/cards/card_structure/` |
| Card types | `card_type_kage_ren`, `card_type_lord_daichi`, `card_type_sister_hotaru`, `card_type_yokai_placeholder` | `bevy/crates/game/assets/cards/card_types/` |
| Worlds | `bamboo_forest`, `coastal_harbor` | `bevy/crates/game/assets/worlds/` |
| Locations | `fortress_gate`, `bamboo_crossing`, `shrine_ruins`, `battlefield`, `spirit_well`, `market_square` | `bevy/crates/game/assets/locations/` |

## World Background Slot Treatment Layout

Every new `WorldTheme.background_texture` must be authored against the runtime logical `CardSlotBoardModel` layout, including the background mesh bleed used by GameScene. The texture must have exactly six diegetic card-placement treatments in these areas and no slot-like treatments elsewhere.

| Slot Area | GameScene Logical Rect | 1600 x 1000 Texture Rect With 1.18 Bleed |
| --------- | ---------------------- | ---------------------------------------- |
| Top left | `x=364, y=44, w=184, h=180` | `x=508, y=123, w=195, h=191` |
| Top center | `x=548, y=44, w=184, h=180` | `x=703, y=123, w=195, h=191` |
| Top right | `x=732, y=44, w=184, h=180` | `x=897, y=123, w=195, h=191` |
| Bottom left | `x=364, y=432, w=184, h=180` | `x=508, y=534, w=195, h=191` |
| Bottom center | `x=548, y=432, w=184, h=180` | `x=703, y=534, w=195, h=191` |
| Bottom right | `x=732, y=432, w=184, h=180` | `x=897, y=534, w=195, h=191` |

| Rule | Requirement |
| ---- | ----------- |
| Diegetic placement | Slot treatments must be in-world scene materials such as shallow planks, stones, silt, reeds, moss, shadow, or water depth. |
| Exclusivity | The location band, hand area, and surrounding environment must not contain extra rectangular clearings or card-placement cues. |
| Runtime source | If `CardSlotBoardModel`, `GAME_SCENE_WORLD_BACKGROUND_BLEED`, or the world texture dimensions change, regenerate this table from runtime constants before creating new world art. |

| Card Front Layer | Asset Field | Canvas | Layer Requirement |
| ---------------- | ----------- | ------ | ----------------- |
| Background | `background_texture` | `840 x 1440`, opaque | Environment-only art, no character, no frame, no title text, no alpha padding. Runtime may mask this layer through the frame aperture while the source remains full-canvas. |
| Frame | `frame_texture` | `840 x 1440`, alpha | Card structure treatment between the environment and character; primarily inside the 40 px safe-area guide; can be approximately rectangular, angled, asymmetric, or use one or more line treatments; must not become one universal frame style for every card. |
| Safe Area | `safe_area` card structure texture | `840 x 1440`, alpha | Transparent reference overlay with a green rectangle inset 40 px from every edge; rendered immediately in front of the frame; visibility controlled by persisted `show_safe_area`; not part of final card art identity. |
| Foreground | `foreground_texture` | `840 x 1440`, alpha | Character-only layer; mostly inside the safe area; may selectively break out of the guide for expressive pose elements; must never clip against the image border; AI source images use `#ff00ff` chroma key before alpha extraction. |
| Title | `title_texture` | `840 x 1440`, alpha | Character-name-only stylized raster art; mostly inside the safe area but may break out; can be bottom, top, or off-center when composition benefits; must never clip against the image border; AI source images use `#ff00ff` chroma key before alpha extraction. |
