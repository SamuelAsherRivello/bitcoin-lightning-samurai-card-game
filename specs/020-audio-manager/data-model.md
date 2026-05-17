# Data Model: Audio Manager

## AudioManagerModel

| Field | Description | Validation |
| ----- | ----------- | ---------- |
| SFX channel state | Whether SFX requests are audible based on the existing SFX setting. | Mirrors the current settings value at request time. |
| Music channel state | Whether music requests are audible based on the existing Music setting. | Mirrors the current settings value at request time. |
| Audio mappings | One-to-one map from audio enum values to asset files. | Each known enum value maps to exactly one file. |
| Loaded handles | Cached handles for mapped audio assets. | Missing assets produce diagnostics and do not block gameplay. |

## AudioChannelModel

| Value | Meaning |
| ----- | ------- |
| SFX | Short sound effects for UI, card movement, and location state changes. |
| Music | Longer music playback controlled independently from SFX. |

## AudioEnum

| Value | Channel | File | Trigger |
| ----- | ------- | ---- | ------- |
| `ButtonClick` | SFX | `audio/sfx/Click01.wav` | Accepted shared button click. |
| `CardSlide` | SFX | `audio/sfx/Slide01.wav` | Accepted card movement from deck to hand or hand to location. |
| `LocationOpen` | SFX | `audio/sfx/Tamborine01.wav` | Location transition from closed to open. |
| `LocationLeadChange` | SFX | `audio/sfx/Upgrade01.wav` | Location winning side changes to a new non-tied side. |

## ButtonUiBundle

| Field | Description | Validation |
| ----- | ----------- | ---------- |
| Button interaction | Standard button input surface. | Click sound plays only after accepted action handling. |
| Default style | The single current button visual style. | All current game buttons use this style. |
| Action metadata | Existing button action data attached by each screen or system. | Existing routing and modal blocking behavior remains unchanged. |

## ButtonStyleModel

| Value | Meaning |
| ----- | ------- |
| Default | The only style provided in this feature; future styles may be added without replacing the bundle. |

## CardMovementAudioEvent

| Field | Description | Validation |
| ----- | ----------- | ---------- |
| Source zone | Near deck, far deck, near hand, or far hand. | Must be an accepted source from the spec. |
| Destination zone | Matching hand or any location slot. | Must be an accepted destination from the spec. |
| Audio result | `AudioEnum.CardSlide`. | Emits once per accepted movement. |

## LocationAudioState

| Field | Description | Validation |
| ----- | ----------- | ---------- |
| Open state | Whether a location was closed or open before the latest update. | Closed-to-open emits `LocationOpen`; open-to-open does not. |
| Previous winning side | Last known non-tied or tied winner state for the location. | Stored per location. |
| Current winning side | Winner after the latest total calculation. | Near, far, or none for tie. |
| Audio result | `AudioEnum.LocationLeadChange` when current side is new and non-tied. | Same-side changes and ties emit no sound. |

## State Transitions

| Transition | Audio Request |
| ---------- | ------------- |
| SFX on -> SFX off | Future SFX requests become inaudible. |
| Music on -> Music off | Future or current music becomes inaudible; SFX unchanged. |
| Accepted button click | `ButtonClick` if SFX is enabled. |
| Near/far deck -> matching hand | `CardSlide` if SFX is enabled. |
| Near/far hand -> location slot | `CardSlide` if SFX is enabled. |
| Location closed -> open | `LocationOpen` if SFX is enabled. |
| Location tie or no winner -> near/far winner | `LocationLeadChange` if SFX is enabled. |
| Location far winner -> near winner, or near winner -> far winner | `LocationLeadChange` if SFX is enabled. |
| Location winner score changes but side remains same | No audio request. |
| Location winner -> tie | No audio request. |
