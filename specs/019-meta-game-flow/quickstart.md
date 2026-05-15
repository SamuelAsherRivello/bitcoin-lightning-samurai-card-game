# Quickstart: Meta Game Flow

## Verify Unit Tests

```powershell
scripts/other/RunTests.ps1
```

## Verify Desktop Runtime

```powershell
scripts/main/RunAppDesktop.ps1 -AiRuntime
```

Expected checks:

| Step | Expected Result |
| ---- | ---- |
| Launch app | MainMenuScreen is active, Play Game selected. |
| Click Login with Lightning | LightningScreen shows QR placeholder and Back. |
| Click Back | MainMenuScreen returns. |
| Click Start Game | Matchmaking shows Searching for about 1 second, then Player 02 for about 1 second. |
| Wait for matchmaking | Existing GameScreen loads. |
| Click Play Game in top nav from GameScreen | MainMenuScreen loads. |
| Click Settings | SettingsScreen loads. |
| Toggle mode/audio | Values update and persist. |
| Click My Decks | Existing DeckScreen loads. |
| Click a left or right card | Fullscreen card overlay appears centered with actions. |
| Click Debug | DebugScreen shows debug card and Card UI. |

## Verify Browser Runtime

```powershell
scripts/other/RunAppWeb.ps1
```

Use the same click path as desktop. Browser verification may require user interaction for opening the Learn About Lightning external page depending on browser popup policy.
