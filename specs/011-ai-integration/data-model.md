# Data Model: AI Integration

## Entity: AI Integration Profile

| Field | Type | Description |
| ----- | ---- | ----------- |
| `name` | string | Stable profile name, initially `ai_integration_dev`. |
| `scope` | enum | `desktop_runtime`, `browser_qa`, or `spec_planning`. |
| `enabled_by_default` | bool | Must be `false` for runtime MCP tooling. |
| `allowed_targets` | list | Target surfaces covered by the profile. |
| `artifact_policy` | string | Where screenshots, snapshots, and reports may be written. |

**Validation Rules**:

| Rule | Requirement |
| ---- | ----------- |
| Default enablement | Runtime MCP profiles MUST NOT be enabled by default. |
| Target coverage | Desktop and browser coverage MUST be documented separately. |
| Artifact paths | Generated artifacts MUST remain project-local. |

## Entity: MCP Runtime Bridge

| Field | Type | Description |
| ----- | ---- | ----------- |
| `server_name` | string | Local MCP server name, recommended `bevy-debugger`. |
| `implementation` | string | Selected implementation, `bevy_debugger_mcp`. |
| `transport` | enum | MCP stdio from Codex to server; BRP HTTP from server to game. |
| `bevy_host` | string | Default `localhost`. |
| `bevy_port` | number | Default `15702` unless a future task changes it. |
| `operation_classes` | list | Supported classes: observational, screenshot-only, mutating. |

**Validation Rules**:

| Rule | Requirement |
| ---- | ----------- |
| Host scope | Host MUST default to `localhost`. |
| Global config | Repo tasks MUST NOT edit global Codex MCP config unless explicitly requested. |
| Availability | Tooling MUST report clear connection failures when the game or server is unavailable. |

## Entity: Bevy Remote Endpoint

| Field | Type | Description |
| ----- | ---- | ----------- |
| `feature_flag` | string | Development feature or script mode that enables BRP. |
| `plugins` | list | Bevy plugins required for remote access and custom methods. |
| `reflected_types` | list | Components/resources safe and useful for AI inspection. |
| `custom_methods` | list | Repo-defined BRP methods such as screenshot capture. |

**Validation Rules**:

| Rule | Requirement |
| ---- | ----------- |
| Opt-in | Endpoint MUST only start when explicitly enabled. |
| Reflection | Only safe, relevant diagnostics should be reflected for AI use. |
| Browser target | Native-only remote support MUST be documented as a browser gap. |

**Future Extension Notes**:

| Area | Guidance |
| ---- | -------- |
| Reflected card state | Future card model, card instance, and card view diagnostics should expose safe reflected fields needed for AI inspection. |
| Reflected location state | Future location totals, control state, and reveal state should be queryable when development tooling is enabled. |
| Reflected point state | Cost, power, effective power, and location total state should be represented through stable reflected components or resources when implemented. |
| Diagnostic summaries | Prefer concise read-only resources for high-level AI queries before exposing large or noisy internal structures. |

## Entity: Runtime Observation

| Field | Type | Description |
| ----- | ---- | ----------- |
| `query_name` | string | Human-readable query purpose. |
| `brp_method` | string | BRP method such as `world.query`, `world.get_components`, or `world.get_resources`. |
| `requested_types` | list | Fully qualified component or resource type names. |
| `result_summary` | string | Agent-readable summary of the returned runtime state. |
| `operation_class` | enum | Always `observational`. |
| `source_confidence` | enum | `reflected_state`, `screenshot_text`, or `visual_inference`. |

**State Transitions**:

| From | To | Trigger |
| ---- | -- | ------- |
| `planned` | `requested` | Codex invokes the MCP observation tool. |
| `requested` | `captured` | MCP returns structured runtime data. |
| `requested` | `failed` | Game endpoint, MCP server, or reflection data is unavailable. |

## Entity: Screenshot Capture

| Field | Type | Description |
| ----- | ---- | ----------- |
| `capture_target` | enum | `desktop_runtime` or `browser_viewport`. |
| `tool` | string | Selected MCP for desktop or browser automation for web. |
| `path` | string | Project-local image path. |
| `spec_reference` | string | Spec or task that requested the screenshot. |
| `timestamp` | string | Capture time for traceability. |
| `peek_request` | bool | True when produced because the user asked to "peek" at the app or running app. |

## Entity: Scene Observation

| Field | Type | Description |
| ----- | ---- | ----------- |
| `app_scene_present` | bool | Whether the always-present `AppScene` should be assumed present by architecture. |
| `active_view` | string | Current user-facing view, such as `GameScene`, `DeckScene`, or `DebugScene`, from reflected state or screenshot DebugHUD text. |
| `evidence` | enum | `reflected_state`, `screenshot_text`, or `visual_inference`. |

**Validation Rules**:

| Rule | Requirement |
| ---- | ----------- |
| Path scope | Screenshot path MUST stay inside the repository. |
| Target clarity | Desktop MCP screenshots and browser viewport screenshots MUST be labeled distinctly. |
| Responsive layout evidence | Captures used for visual QA MUST include enough viewport context to judge whether visible positions and scales respect the aspect-ratio-safe game view. |
| Promotion | Transient captures are not committed unless a future task promotes them. |

## Entity: Tool Operation Class

| Value | Meaning | Required Control |
| ----- | ------- | ---------------- |
| `observational` | Reads state without changing the running game. | Prefer for routine debugging. |
| `screenshot_only` | Captures visual output without changing game state. | Store project-local artifacts. |
| `mutating` | Alters ECS state, resources, or events. | Require reason plus reset, rollback, or restart plan. |

## Entity: AI QA Artifact

| Field | Type | Description |
| ----- | ---- | ----------- |
| `artifact_type` | enum | `screenshot`, `runtime_snapshot`, `log_excerpt`, or `qa_report`. |
| `path` | string | Project-local file path. |
| `source_tool` | string | Tool or command that produced it. |
| `retention` | enum | `transient`, `documentation`, or `release_evidence`. |
| `responsive_layout_checked` | bool | Whether the artifact was reviewed for aspect-ratio-safe positions and scales. |

## Entity: AI Safety Boundary

| Field | Type | Description |
| ----- | ---- | ----------- |
| `network_scope` | string | Must default to `localhost`. |
| `secret_policy` | string | No secrets are requested, printed, copied, or committed. |
| `production_policy` | string | Runtime MCP is development-only. |
| `mutation_policy` | string | Mutation requires explicit task-level reset or rollback guidance. |
