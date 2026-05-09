# Data Model: Card Polish

## CardStructure

| Field | Description | Validation |
| ----- | ----------- | ---------- |
| Layers | Background, frame, foreground, title | Must remain in this apparent-depth order |
| Parallax source | Current smoothed card tilt | Must not use raw pointer position directly |
| Frame aperture | Central cutout stencil region defined by CardStructure frame geometry | Background must not render outside the aperture |
| Frame mesh | One frame object with a center cutout and full-card UVs | Theme texture must read as one continuous image rather than four separate border textures |

## CardTheme

| Field | Description | Validation |
| ----- | ----------- | ---------- |
| Theme id | Stable theme identifier | Initial values are `skybolt` and `tar`; asset folders are `cards/CardThemes/CardTheme_SkyBolt/` and `cards/CardThemes/CardTheme_Tar/` |
| Display name | HUD-facing name | Initial value is `SKYBOLT` |
| Background visual style | Static texture and material settings | Blue/white repeated cloud texture |
| Frame visual style | Static texture and material settings applied to the CardStructure frame mesh | Grey/off-grey 45-degree pinstripe texture plus frame shine; must not define frame geometry |
| Foreground visual style | Static texture and material settings | Original superhero-inspired transparent PNG |
| Title visual style | Static texture and material settings | `SKYBOLT` transparent PNG |

## CardThemeRegistry

| Field | Description | Validation |
| ----- | ----------- | ---------- |
| Slots | Two theme slots | Slot 0 populated, slot 1 reserved |
| Active index | Current selected slot | Must always resolve to a populated theme |

## Theme Toggle Input

| Field | Description | Validation |
| ----- | ----------- | ---------- |
| Key | HUD key `T` | Pressing cycles between available themes and keeps the active theme valid |

## AppScene Reload Input

| Field | Description | Validation |
| ----- | ----------- | ---------- |
| Key | HUD key `R` | Non-toggle operation; each press rebuilds reloadable card scene content |
| Reload scope | Primary camera and card structure | Must not add gameplay, model browser, or deck behavior |

## Hot Reload Auto-Restart Toggle

| Field | Description | Validation |
| ----- | ----------- | ---------- |
| Key | HUD key `H` | Toggle; defaults to disabled |
| Persistence | `data/local_storage/debug-hud-input.json` | Uses `bevy-persistent`; local runtime state only |
| Effect | Desktop hot-reload patch handling | Enabled invokes the same AppScene reload path as `R`; disabled does not |
