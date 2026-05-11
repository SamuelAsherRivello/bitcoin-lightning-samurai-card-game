# Research: Game Theme POC

## Decisions

| Topic | Decision | Rationale | Alternatives Considered |
| ----- | -------- | --------- | ----------------------- |
| Runtime architecture | Extend the existing Bevy ECS scene, card, and Card Browser systems. | The project already has `ActiveScene`, card click navigation, card flip animation, card settings persistence, card parallax layers, and GameScene/CardBrowser roots. Reusing them reduces risk and keeps the permanent change aligned with the current app. | A separate prototype-only scene was rejected because the spec requires permanent replacement, not a temporary overlay. |
| Card identity model | Replace the current two-card registry with four Japan Realism card identities: Kage Ren, Lord Daichi, Sister Hotaru, and Yokai placeholder. | The bottom game scene must show exactly one of each card, and the Card Browser must focus the clicked card. A registry with four identities supports both flows. | Keeping SkyBolt/Tar as hidden legacy card types was rejected because the feature permanently replaces pre-008 cards. |
| CardUI settings | Keep global CardUI settings persisted through the existing card settings store. | The clarification states settings are stored and applied to all cards. Existing `CardUiState` and `CardSettingsStore` already provide global persistence behavior. | Per-card visual settings were rejected because they contradict the clarification. |
| Flip state | Keep flip state temporary and scoped to the current Card Browser card/session. | The clarification states the flip button only tests the animation and does not store state. | Persisting flip state or applying it to all cards was rejected because it would create durable card state outside the spec. |
| World themes | Add a world theme resource for Bamboo Forest and Coastal Harbor, with `T` cycling only while in GameScene. | The spec requires `T` to cycle worlds in GameScene and CardUI settings in Card Browser, preserving separation between world themes and CardUI settings. | Using one global theme state for both world and cards was rejected because it violates the explicit separation rule. |
| Tactical locations | Model six reusable location identities and select three on each world change. | The spec names six reusable gameplay spaces and requires three visible locations after each world change. | Hard-coding three static UI cards was rejected because it would not exercise random selection from the reusable pool. |
| Asset strategy | Create new source-controlled bitmap assets under lowercase `snake_case` directories. | The feature requires new art and permanent visual replacement. Runtime assets belong under `bevy/crates/game/assets`, and the constitution requires lowercase paths under `bevy/`. | Downloading or linking remote assets at runtime was rejected because the app should remain local and reproducible. |
| Card composition | Use 9:16 vertical assets for front card art and keep 70-80% character height. | The spec defines card composition as a core acceptance criterion. This affects generated assets, card dimensions, and Browser presentation. | Reusing current poker-ratio assets unchanged was rejected because the spec requires 9:16 vertical composition. |
| Verification | Use unit/system tests for registry/state behavior and repository scripts for desktop/web checks. | Existing tests already cover resources, card textures, scene transitions, and flip behavior; scripts are the project-approved verification route. | Manual-only verification was rejected because the permanent scene replacement needs regression coverage. |

## Resolved Clarifications

| Question | Resolution |
| -------- | ---------- |
| Is 008 temporary or permanent? | Permanent game change replacing existing cards and world background. |
| Does 008 create new art? | Yes, new Japan Realism card and world/location assets are part of scope. |
| Are CardUI settings per-card? | No, they are global and persisted for all cards. |
| Is flip state stored? | No, flip state only tests the current Card Browser animation. |
| Are mist, smoke, and torch fire allowed? | Yes, grounded real-world atmospheric effects are allowed; magic glow is not. |
