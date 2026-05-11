# Data Model: Theme Reorganization

| Entity | Fields | Relationships | Validation Rules |
| ------ | ------ | ------------- | ---------------- |
| `ThemeModel` | `id`, `root_path`, `display_name` | Owns card models, location models, and world models. | Current value is `theme_japan`; root path is `bevy/crates/game/assets/themes/theme_japan`; folder name uses lowercase `snake_case`. |
| `AppScene` | `root_entity`, `active_view`, `persistent_ui_roots` | Always-present app-level scene that hosts one active sub-screen view. | Remains present while `GameView` or `CardBrowserView` is loaded on top. |
| `GameView` | `world_view`, `location_views`, `card_views`, `hud_state` | Gameplay sub-screen presentation loaded on top of `AppScene`. | Renames the active gameplay sub-screen from `GameScene`; behavior remains unchanged. |
| `CardBrowserView` | `active_card_model_id`, `card_view`, `card_ui_state`, `flip_state` | Card browser sub-screen presentation loaded on top of `AppScene`. | Renames the active browser sub-screen from `CardBrowserScene`; behavior remains unchanged. |
| `CardModel` | `id`, `display_name`, `root_path`, `front_layers`, `back_texture`, `view_behavior` | Belongs to one theme card category; provides the data used to create a `CardViewBundle`. | Root folder starts with `card_`; folder name does not include `japan`; front layers include background, frame, foreground, and title; back presentation is present; behavior remains existing card selection, browsing, flipping, and layer controls. |
| `CardView` | `card_model_id`, `visible_face`, `layer_state`, `interaction_state` | Rendered presentation created from one `CardModel` through a `CardViewBundle`. | Does not own durable card data; visual state such as flip face remains temporary unless another spec makes it persistent. |
| `CardViewBundle` | `root_entity`, `front_layers`, `back_layer`, `view_components`, `view_behavior` | Bevy bundle that creates the visual card view from a `CardModel`. | Contains front presentation, back presentation, all visual layers, and view behavior required for loading and play presentation. |
| `CardFrontLayerModel` | `role`, `texture_path`, `canvas_size`, `depth_order` | Part of one `CardModel`; consumed by `CardViewBundle`. | Supported roles are background, frame, foreground, and title; texture paths resolve below the owning card folder; layer order remains compatible with current rendering. |
| `LocationModel` | `id`, `display_name`, `texture_path` | Belongs to one theme location category; participates in tactical location presentation. | Root folder starts with `location_`; folder name does not include `japan`; texture path resolves below `themes/theme_japan/locations`. |
| `LocationView` | `location_model_id`, `texture_handle`, `placement`, `visibility` | Rendered tactical location presentation created from one `LocationModel`. | Does not own durable location identity or asset path data. |
| `WorldModel` | `id`, `display_name`, `background_texture` | Belongs to one theme world category; active world selects one theme world background. | Root folder starts with `world_`; folder name does not include `japan`; texture path resolves below `themes/theme_japan/worlds`. |
| `WorldView` | `world_model_id`, `background_handle`, `lighting_state` | Rendered world presentation created from one `WorldModel`. | Does not own durable world identity or asset path data. |
| `SharedAsset` | `id`, `path`, `purpose` | Can be referenced by theme-owned entities but is not owned by one theme. | Must remain outside `themes/theme_japan/{cards,locations,worlds}` only when reusable across themes, such as shader support. |

## Naming Proposal

| Existing Concept | Proposed Name | Reason |
| ---------------- | ------------- | ------ |
| `CardType` | `CardModel` | Existing struct stores card identity, display name, asset paths, and visual tuning data. |
| `CardTypeRegistry` | `CardModelRegistry` | Registry owns card data models, not rendered views. |
| `ActiveCardType` | `ActiveCardModel` | Active selection points at data used to create a rendered card. |
| `ActiveScene` | `ActiveView` | Active selection points at the sub-screen view loaded on top of `AppScene`. |
| `GameSceneRoot` / `GameSceneEntity` | `GameViewRoot` / `GameViewEntity` | Names gameplay presentation as a view because `AppScene` is the persistent scene. |
| `CardBrowserSceneRoot` / `CardBrowserSceneEntity` | `CardBrowserViewRoot` / `CardBrowserViewEntity` | Names card browser presentation as a view because it is loaded on top of `AppScene`. |
| Card visual spawn result | `CardView` | Names the rendered presentation separately from card data. |
| Card visual Bevy bundle | `CardViewBundle` | Bundle creates card visuals: front layers, back layer, view components, and view behavior. |
| `WorldTheme` | `WorldModel` | Existing struct stores world identity and background asset data. |
| `WorldThemeRegistry` | `WorldModelRegistry` | Registry owns world data models. |
| `ActiveWorldTheme` | `ActiveWorldModel` | Active selection points at world data used to render a view. |
| World background presentation | `WorldView` | Names rendered world presentation separately from world data. |
| `TacticalLocation` | `LocationModel` | Existing struct stores location identity and texture data. |
| `TacticalLocationRegistry` | `LocationModelRegistry` | Registry owns location data models. |
| Tactical location presentation | `LocationView` | Names rendered location presentation separately from location data. |
| Card asset folder | `card_<card_name>` | Filesystem name remains category-prefixed and does not include `model` or `view`. |

## Asset Path Mapping

| Current Asset Group | New Theme Path | Notes |
| ------------------- | -------------- | ----- |
| `assets/cards/card_structure/card_back_japan_realism.png` | `assets/themes/theme_japan/cards/card_back.png` | Theme identity moves to path; filename no longer repeats `japan`. |
| `assets/cards/card_structure/safe_area.png` | `assets/themes/theme_japan/cards/safe_area.png` | Safe-area guide remains part of `CardViewBundle` presentation support. |
| `assets/cards/card_types/card_type_kage_ren/*` | `assets/themes/theme_japan/cards/card_kage_ren/*` | Folder contains assets referenced by `CardModel` and consumed by `CardViewBundle`. |
| `assets/cards/card_types/card_type_lord_daichi/*` | `assets/themes/theme_japan/cards/card_lord_daichi/*` | Folder contains assets referenced by `CardModel` and consumed by `CardViewBundle`. |
| `assets/cards/card_types/card_type_sister_hotaru/*` | `assets/themes/theme_japan/cards/card_sister_hotaru/*` | Folder contains assets referenced by `CardModel` and consumed by `CardViewBundle`. |
| `assets/cards/card_types/card_type_yokai_placeholder/*` | `assets/themes/theme_japan/cards/card_yokai_placeholder/*` | Placeholder follows the same `CardModel` and `CardViewBundle` model. |
| `assets/locations/<name>/location.png` | `assets/themes/theme_japan/locations/location_<name>/location.png` | Location folders use `location_` prefix. |
| `assets/worlds/<name>/world_background.png` | `assets/themes/theme_japan/worlds/world_<name>/world_background.png` | World folders use `world_` prefix. |
| `assets/shaders/card_background_mask.wgsl` | unchanged | Shared shader, not theme-owned content. |

## State And Behavior

| State | Before 009 | After 009 |
| ----- | ---------- | --------- |
| Active card selection | Index into explicit card registry | Same behavior; registry points at `CardModel` data with card asset paths. |
| Card browser flip state | Temporary `CardBrowserView` session state | Unchanged. |
| CardUI settings | Global persisted settings | Unchanged. |
| Active world | Runtime world registry index | Same behavior; registry points at theme world asset paths. |
| Active locations | Three selected tactical location indices | Same behavior; registry points at theme location asset paths. |
