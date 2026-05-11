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
| Flip button/action | Flip only the currently viewed card for animation testing. | Do not persist flip state or apply it to other cards. |
| Return to GameScene | Preserve active world and global CardUI settings. | Do not reset the world because the card changed. |

## Art Contract

| Asset Type | Required Direction | Excluded Direction |
| ---------- | ------------------ | ------------------ |
| Cards | 9:16 vertical Japan Realism character-poster composition, mostly full-body, 70-80% character height. | Comic-book splash framing, arcade fantasy, magic glow. |
| Worlds | Grounded cinematic environments with mist, rain, smoke, torch or lantern fire where appropriate. | Supernatural energy effects or glowing fantasy lighting. |
| Weapons/materials | Believable silhouettes, realistic armor, cloth, wood, metal, rope, smoke, and fire. | Non-believable weapons or exaggerated magical materials. |
