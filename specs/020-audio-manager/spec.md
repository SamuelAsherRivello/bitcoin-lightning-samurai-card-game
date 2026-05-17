# Feature Specification: Audio Manager

**Feature Branch**: `020-audio-manager`  
**Created**: 2026-05-15  
**Status**: Draft  
**Input**: User description: "Make an audio manager with SFX and Music channels driven by existing settings values. The game can play arbitrary sound or music through that API. Add a shared button_ui_bundle for all buttons, play Click01 on button clicks, play Slide01 for deck-to-hand and hand-to-location card movement, play Tamborine01 when a location changes from closed to open, and play Upgrade01 when a location's winning side changes to a new non-tied side."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Respect Audio Settings (Priority: P1)

A player can toggle SFX and Music settings and trust that all game audio respects those preferences without changing gameplay behavior.

**Why this priority**: The audio manager must use the existing player-facing settings before any individual sound trigger can behave correctly.

**Independent Test**: Toggle SFX off, trigger each listed SFX event, and verify no SFX is heard; toggle SFX back on and verify the same events can be heard. Toggle Music off and verify music playback is muted independently from SFX.

**Acceptance Scenarios**:

1. **Given** SFX is enabled, **When** a game event requests a sound effect, **Then** the corresponding sound effect is audible.
2. **Given** SFX is disabled, **When** a game event requests a sound effect, **Then** no sound effect is audible and gameplay continues normally.
3. **Given** Music is enabled, **When** a game event requests music, **Then** the requested music can be heard.
4. **Given** Music is disabled, **When** a game event requests music, **Then** music is not audible and SFX behavior is unchanged.

---

### User Story 2 - Hear Consistent Button Feedback (Priority: P1)

A player hears the same click feedback whenever they activate any game button that uses the shared button presentation.

**Why this priority**: Button feedback is the broadest audio interaction and requires a shared button path so future screens do not drift.

**Independent Test**: Open every currently reachable screen, click each visible button, and verify that each active button uses the shared default button style and plays the button-click sound when SFX is enabled.

**Acceptance Scenarios**:

1. **Given** any current game button is visible, **When** the button is inspected, **Then** it uses the shared default button style.
2. **Given** SFX is enabled and a current game button is clicked, **When** the click is accepted, **Then** `AudioEnum.ButtonClick` plays `Click01.wav`.
3. **Given** SFX is disabled and a current game button is clicked, **When** the click is accepted, **Then** no button-click sound is audible.

---

### User Story 3 - Hear Card Movement Feedback (Priority: P2)

A player hears a slide sound when cards move from player deck zones into player hand zones, and from player hand zones into location slots.

**Why this priority**: Card movement is a core gameplay action, and audio should reinforce meaningful movement rather than every visual update.

**Independent Test**: Simulate or play card movement from near and far player decks into near and far hands, then from near and far hands to a location slot, and verify each accepted movement plays one slide sound when SFX is enabled.

**Acceptance Scenarios**:

1. **Given** SFX is enabled, **When** a card moves from the near player deck to the near player hand, **Then** `AudioEnum.CardSlide` plays `Slide01.wav` once.
2. **Given** SFX is enabled, **When** a card moves from the far player deck to the far player hand, **Then** `AudioEnum.CardSlide` plays `Slide01.wav` once.
3. **Given** SFX is enabled, **When** a card moves from the near player hand to a location slot, **Then** `AudioEnum.CardSlide` plays `Slide01.wav` once.
4. **Given** SFX is enabled, **When** a card moves from the far player hand to a location slot, **Then** `AudioEnum.CardSlide` plays `Slide01.wav` once.

---

### User Story 4 - Hear Location State Feedback (Priority: P2)

A player hears a distinct sound when a location opens and when location control changes to a new winning side.

**Why this priority**: Location reveals and control shifts are high-signal match events that should be recognizable without adding extra UI.

**Independent Test**: Change a location from closed to open and verify the open sound; then step through score changes `0,0 -> 0,3 -> 0,5 -> 6,5` and verify the control-change sound plays only on `0,3` and `6,5`.

**Acceptance Scenarios**:

1. **Given** SFX is enabled and a location is closed, **When** the location changes to open, **Then** `AudioEnum.LocationOpen` plays `Tamborine01.wav` once.
2. **Given** SFX is enabled and a tied location changes from `0,0` to far winning `0,3`, **When** the winning side is evaluated, **Then** `AudioEnum.LocationLeadChange` plays `Upgrade01.wav` once.
3. **Given** SFX is enabled and the same side remains winning while the score changes from `0,3` to `0,5`, **When** the winning side is evaluated, **Then** no lead-change sound plays.
4. **Given** SFX is enabled and the winning side changes from far winning `0,5` to near winning `6,5`, **When** the winning side is evaluated, **Then** `AudioEnum.LocationLeadChange` plays `Upgrade01.wav` once.
5. **Given** SFX is enabled and a location changes from one tied score to another tied score, **When** the winning side is evaluated, **Then** no lead-change sound plays.

### Edge Cases

| Edge Case | Expected Handling |
| --------- | ----------------- |
| An audio request references an unmapped enum value | The game remains stable and reports the missing mapping for diagnosis. |
| A mapped audio file is missing or cannot load | The game remains stable and reports the failed asset load without blocking gameplay. |
| SFX is toggled off while a sound is requested | The request is ignored or silenced according to the current SFX setting. |
| Music is toggled off while music is playing | Music becomes inaudible without muting SFX. |
| A button click is rejected because another modal or screen owns input | No button-click sound plays for the rejected click. |
| A card movement is previewed, animated, or redrawn without changing zones | No slide sound plays unless the accepted source and destination zones match the movement rules. |
| Location totals recalculate but the winning side remains the same | No lead-change sound plays. |
| Location totals recalculate to a tie | No lead-change sound plays for the tie result. |

## Requirements *(mandatory)*

### Functional Requirements

| ID | Requirement |
| -- | ----------- |
| FR-001 | The game MUST provide an audio manager with two independent channels named SFX and Music. |
| FR-002 | The SFX channel MUST be audible only when the existing SFX setting is enabled. |
| FR-003 | The Music channel MUST be audible only when the existing Music setting is enabled. |
| FR-004 | The audio manager MUST expose a game-facing way to request arbitrary mapped sound effects and music without callers loading files directly. |
| FR-005 | Audio requests MUST use named enum values for known sounds, with a one-to-one mapping from each enum value to exactly one audio file. |
| FR-006 | `AudioEnum.ButtonClick` MUST map one-to-one to the confirmed file `Click01.wav`. |
| FR-007 | `AudioEnum.CardSlide` MUST map one-to-one to the confirmed file `Slide01.wav`. |
| FR-008 | `AudioEnum.LocationOpen` MUST map one-to-one to the confirmed file `Tamborine01.wav`. |
| FR-009 | `AudioEnum.LocationLeadChange` MUST map one-to-one to the confirmed file `Upgrade01.wav`. |
| FR-010 | The game MUST provide a shared `button_ui_bundle` for game buttons. |
| FR-011 | The shared `button_ui_bundle` MUST support multiple future styles while providing exactly one default style in this feature. |
| FR-012 | Every current game button MUST use the shared `button_ui_bundle` default style after this feature. |
| FR-013 | Accepted clicks on buttons using the shared bundle MUST request `AudioEnum.ButtonClick`. |
| FR-014 | Moving a card from the near player deck to the near player hand MUST request `AudioEnum.CardSlide`. |
| FR-015 | Moving a card from the far player deck to the far player hand MUST request `AudioEnum.CardSlide`. |
| FR-016 | Moving a card from the near player hand to any location slot MUST request `AudioEnum.CardSlide`. |
| FR-017 | Moving a card from the far player hand to any location slot MUST request `AudioEnum.CardSlide`. |
| FR-018 | Changing a location from closed to open MUST request `AudioEnum.LocationOpen`. |
| FR-019 | Reopening, redrawing, or refreshing an already open location MUST NOT request `AudioEnum.LocationOpen`. |
| FR-020 | When a location calculates which side is winning, the game MUST request `AudioEnum.LocationLeadChange` only if the winning side is a new non-tied side compared with the previous winning side for that location. |
| FR-021 | Score changes that keep the same winning side MUST NOT request `AudioEnum.LocationLeadChange`. |
| FR-022 | Tied location states MUST NOT request `AudioEnum.LocationLeadChange`. |
| FR-023 | Audio behavior MUST be testable without relying on human hearing by observing accepted audio requests and channel settings. |
| FR-024 | Audio failures MUST NOT block input, card movement, location reveal, location scoring, or screen navigation. |

### Key Entities *(include if feature involves data)*

| Entity | Description |
| ------ | ----------- |
| Audio Manager | The game-facing service that accepts sound and music requests and applies channel mute settings. |
| Audio Channel | SFX or Music, each independently muted by the existing settings values. |
| Audio Enum | Named audio identifiers used by game code instead of direct file references. |
| Audio Mapping | One-to-one association between a named audio identifier and a confirmed audio asset file. |
| Button UI Bundle | Shared button presentation and interaction bundle used by all current game buttons. |
| Button Style | A named button visual style; this feature includes exactly one default style while allowing future styles. |
| Card Movement Event | Accepted movement from player deck to player hand or player hand to location slot, from either near or far side. |
| Location Open Event | A location transition from closed to open. |
| Location Winning Side | The current non-tied side winning a location: near, far, or none for ties. |

## Success Criteria *(mandatory)*

### Measurable Outcomes

| ID | Measurable Outcome |
| -- | ------------------ |
| SC-001 | With SFX enabled, all four required SFX triggers produce observable audio requests mapped to the confirmed files. |
| SC-002 | With SFX disabled, the same four required SFX triggers produce no audible SFX while gameplay state still updates. |
| SC-003 | With Music disabled, requested music is inaudible while button, card, and location SFX still follow the SFX setting. |
| SC-004 | Every current game button is traceable to the shared default button style after implementation. |
| SC-005 | The score sequence `0,0 -> 0,3 -> 0,5 -> 6,5` produces exactly two lead-change audio requests: one for far taking the lead and one for near taking the lead. |
| SC-006 | Missing or failed audio assets never prevent the player from completing the triggering button, card, or location action. |

## Assumptions

| Topic | Assumption |
| ----- | ---------- |
| Feature name | The feature is named `audio-manager`, not audio system. |
| Existing settings | The current persisted SFX and Music settings are the source of truth for channel mute behavior. |
| Confirmed SFX files | `Click01.wav`, `Slide01.wav`, `Tamborine01.wav`, and `Upgrade01.wav` exist in the game crate audio assets. |
| Music assets | Specific music tracks are not assigned in this feature; the manager must support music requests once mappings are added. |
| Button scope | All current interactive game buttons are migrated to the shared bundle; non-button card clicks or drag gestures are not reclassified as buttons. |
| Rejected input | Sounds play only for accepted user actions and accepted game-state transitions. |
| Location winner memory | Each location remembers its previous winning side so repeated recalculations do not replay the lead-change sound. |
| Bevy structure | New Rust files and folders follow typical lowercase Rust naming conventions. |
| Template reference | Bevy crate folders, representative files, asset folders, and Rust coding standards use `bevy/crates/template-crate` as the proper local reference. |
| Runtime naming | Changed Bevy runtime files use one primary concept per file, Scene/Model/View naming, and HUMAN/AI purpose comments. |
| Layout | Visible button UI remains inside the aspect-ratio-safe game view. |
