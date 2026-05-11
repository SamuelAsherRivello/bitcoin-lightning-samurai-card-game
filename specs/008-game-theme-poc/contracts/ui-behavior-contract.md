# UI Behavior Contract: Game Theme POC

## GameScene Contract

| Interaction | Required Behavior | Must Not Happen |
| ----------- | ----------------- | --------------- |
| Initial view | Show one active world background, three centered locations, and four bottom cards. | Do not show the old desert background or old SkyBolt/Tar-only card lineup. |
| Press `T` | Cycle active world between Bamboo Forest and Coastal Harbor. | Do not change CardUI settings or card identities. |
| World changes | Randomly select and render three tactical locations from the six reusable locations. | Do not render fewer or more than three active locations. |
| Move cursor/touch | Bottom cards lean subtly toward pointer/touch while remaining readable. | Do not create exaggerated tilt that obscures names or silhouettes. |
| Click/tap a card | Open Card Browser focused on that clicked card. | Do not always open a default card regardless of click target. |

## Card Browser Contract

| Interaction | Required Behavior | Must Not Happen |
| ----------- | ----------------- | --------------- |
| Open browser | Enlarge the selected card only. | Do not alter the active GameScene world. |
| Press `T` | Change global CardUI settings and update the visible card presentation. | Do not cycle GameScene world themes. |
| Change CardUI settings | Apply settings to all cards. | Do not store settings per card. |
| Toggle `Show Safe Area` | Show or hide the green safe-area reference layer and persist the setting globally. | Do not alter the art assets, layer scale values, active card, flip state, or active world. |
| Click a layer-scale reset control | Reset only that layer scale to `1.0` and persist the global CardUI settings. | Do not reset other layer scales, depth factor, safe-area visibility, flip state, active card, or active world. |
| Flip button/action | Flip only the currently viewed card for animation testing. | Do not persist flip state or apply it to other cards. |
| Return to GameScene | Preserve active world and global CardUI settings. | Do not reset the world because the card changed. |

## Art Contract

| Asset Type | Required Direction | Excluded Direction |
| ---------- | ------------------ | ------------------ |
| Cards | 7:12 tarot-style vertical Japan Realism character-poster composition, mostly full-body, 70-80% character height unless breakout layers intentionally exceed the frame. | Comic-book splash framing, arcade fantasy, magic glow. |
| Worlds | Grounded cinematic environments with mist, rain, smoke, torch or lantern fire where appropriate. | Supernatural energy effects or glowing fantasy lighting. |
| Weapons/materials | Believable silhouettes, realistic armor, cloth, wood, metal, rope, smoke, and fire. | Non-believable weapons or exaggerated magical materials. |

## Card Front Layer Contract

| Layer | Required Composition | Allowed Breakout | Must Not Happen |
| ----- | -------------------- | ---------------- | --------------- |
| Background | Full `840 x 1440` opaque environment-only artwork that establishes atmosphere and material context. | Runtime frame-aperture masking is allowed when the card design wants the visible background constrained by the frame. | Do not bake in the character, title, frame, alpha padding, or safe-area guide. |
| Frame | Alpha layer drawn primarily inside the 40 px safe-area guide and used as the card's material and structural identity. | Slightly angled corners, irregular rectangles, asymmetric trim, inner/outer line treatments, and card-specific frame language are allowed. | Do not force every card to share the same frame treatment; do not include character or title art. |
| Safe Area | Transparent reference overlay with a green guide rectangle inset 40 px on all sides. | May be shown during art tuning and hidden through CardUI. | Do not treat the guide as final card art, and do not bake it into card-type frame, foreground, title, or background textures. |
| Foreground | Character-only alpha layer, mostly inside the safe-area guide, with the primary stance readable at both card sizes. | Intentional pose accents may break out of the safe-area guide. | Do not clip any opaque or antialiased pixels at the `840 x 1440` image border; do not include environment, frame, or title art. |
| Title | Character-name-only alpha layer, readable and compositionally integrated. | May sit at bottom, top, or slightly off-center; may break out of the safe-area guide when designed intentionally. | Do not clip at image borders; do not add subtitles, plaques, UI text, or extra copy. |
