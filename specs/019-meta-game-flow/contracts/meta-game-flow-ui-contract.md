# UI Contract: Meta Game Flow

## Screens

| Screen | Active View | Top Nav Selection | Required Content |
| ---- | ---- | ---- | ---- |
| MainMenuScreen | MainMenuScene | Play Game | Login with Lightning, Start Game |
| LightningScreen | LightningScene | Play Game | QR placeholder, Back, Learn About Lightning |
| MatchmakingScreen | MatchmakingScene | None or Back-only | Player 01, versus, opponent panel, phase status text |
| GameScreen | GameScene | Play Game | Existing gameplay screen without mode toggle |
| DeckScreen | DeckScene | My Decks | Existing deck selection/editor/modal |
| SettingsScreen | SettingsScene | Settings | CPU Brain, Mode, SFX, Music controls |
| DebugScreen | DebugScene | Debug | Existing debug card preview and Card UI |

## Navigation Behavior

| Action | Expected Result |
| ---- | ---- |
| Click Play Game from MainMenuScreen | Reload MainMenuScreen |
| Click Play Game from GameScreen | Load MainMenuScreen |
| Click My Decks from any screen | Load or reload DeckScreen |
| Click Settings from any screen | Load or reload SettingsScreen |
| Click Debug from any screen | Load or reload DebugScreen |
| Click Login with Lightning | Load LightningScreen |
| Click Lightning Back | Load MainMenuScreen |
| Click Start Game | Load MatchmakingScreen; show Searching, Found, Loading, Preparing; enter GameScreen after preload and timers |

## Matchmaking Presentation

| Phase | Player 1 Panel | Player 2 Panel | Status Text | Gate |
| ---- | ---- | ---- | ---- | ---- |
| Searching | Player 01 | Searching | Searching... | 0.5 seconds |
| Found | Player 01 | Player 02 | Matching... | 0.5 seconds |
| Loading | Player 01 | Player 02 | Loading... | Match assets loaded with dependencies |
| Preparing | Player 01 | Player 02 | Preparing... | 0.5 seconds |

## Button Functionality

| Button Type | Clickable | Functional In 019 |
| ---- | ---- | ---- |
| Start Game | ✅ | ✅ |
| Login with Lightning | ✅ | ✅ navigates to placeholder login |
| Learn About Lightning | ✅ | ✅ opens public browser URL |
| Lightning purchase/transfer buttons | ✅ | ❌ |
| Shop tab | ✅ | ❌ no purchase layout |
| Selected top navigation button | ✅ | ✅ reloads current destination |

## Persistence Contract

| Setting | Persisted | Notes |
| ---- | ---- | ---- |
| CPU Brain Level | ✅ | Level1 only; click remains operational. |
| Match Mode | ✅ | Moves from GameScreen to SettingsScreen. |
| SFX Enabled | ✅ | Stored for future audio use. |
| Music Enabled | ✅ | Stored for future audio use. |
