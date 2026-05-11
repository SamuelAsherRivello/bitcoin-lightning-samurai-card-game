# Research: AI Integration

## Decision: Use `bevy_debugger_mcp` as the primary Bevy runtime MCP

**Rationale**: `bevy_debugger_mcp` is purpose-built as an MCP server for AI-assisted Bevy debugging. Its README describes an MCP bridge between an AI agent and a Bevy game through Bevy Remote Protocol, including real-time entity/component/resource observation, experiments, anomaly/performance workflows, session recording, and screenshot capture. It also documents Codex-compatible MCP-style setup concepts through a stdio server configuration and a Bevy-side screenshot method.

The core architecture matches this repo's need: Codex needs structured runtime state and screenshots, and Bevy already has a native remote protocol for ECS inspection. Bevy 0.18.1 documentation says `RemotePlugin` sets up the remote protocol and `RemoteHttpPlugin` enables HTTP clients to inspect and alter ECS state; the protocol includes `world.query`, `world.get_components`, resources, registry schema, and custom methods. That means `bevy_debugger_mcp` is likely the shortest path to an agent-facing runtime bridge without inventing a new protocol.

The main adoption risks are that the upstream project labels itself experimental and uses a GPL-3.0 license. This plan therefore treats it as an external development tool to install and evaluate locally, not source to vendor into this repository.

**Sources**:

| Source | Relevant Finding |
| ------ | ---------------- |
| [`bevy_debugger_mcp` README](https://github.com/ladvien/bevy_debugger_mcp) | Describes MCP-to-BRP bridge, runtime observation, screenshot capture, and AI debugging workflows. |
| [Bevy Remote Protocol crate docs](https://docs.rs/bevy_remote/latest/bevy_remote/) | Documents `RemotePlugin`, JSON-RPC basis, ECS inspection/mutation methods, resources, registry schema, and custom methods. |
| [Bevy Remote HTTP docs](https://docs.rs/bevy/latest/bevy/remote/http/index.html) | Documents HTTP transport, default port `15702`, native-only availability, and requirement for `RemotePlugin`. |

**Alternatives considered**:

| Alternative | Strengths | Weaknesses | Decision |
| ----------- | --------- | ---------- | -------- |
| Direct custom BRP scripts | Minimal dependency, fully controlled, can be tailored to repo scripts. | Codex would need wrapper scripts or ad hoc commands rather than first-class MCP tools; screenshot and tool schemas would need custom work. | Keep as fallback or smoke-test support, not primary MCP. |
| Playwright or in-app browser screenshots | Strong for browser WebGPU viewport screenshots and visual regression checks. | Does not inspect Bevy ECS state directly and is not a Bevy runtime MCP. | Use as complementary browser QA, not primary MCP. |
| `bevy-inspector-egui` | Already in the project and useful for human interactive inspection. | Visual/manual tool, not an agent-facing MCP; screenshots alone do not expose structured ECS state. | Keep as human/debug overlay support. |
| Terminal logs and repository tests only | Already available, safe, repeatable, and CI-friendly. | Logs/tests cannot answer many live runtime or visual-state questions without adding custom probes. | Keep as baseline verification, not sufficient for AI runtime integration. |
| Build a new repo-local MCP | Maximum control and licensing flexibility. | More design, maintenance, security, and protocol work before any value; duplicates much of `bevy_debugger_mcp`. | Reconsider only if `bevy_debugger_mcp` fails evaluation. |

## Decision: Treat the selected MCP as desktop-first and browser QA as complementary

**Rationale**: Bevy Remote HTTP documentation states the HTTP transport is available on non-wasm targets. This makes the selected MCP a good fit for Windows desktop runtime inspection, but not a full browser WebGPU substitute. The project constitution requires browser parity or explicit gap documentation, so the plan keeps browser screenshots and smoke testing in the existing served-web browser workflow.

**Alternatives considered**:

| Alternative | Rationale For Rejection |
| ----------- | ----------------------- |
| Require MCP support for browser WebGPU before adoption | Blocks useful desktop runtime inspection and screenshots even though browser QA can be handled separately. |
| Ignore browser target for AI integration | Violates the constitution's desktop/browser parity principle. |
| Use browser automation as the only AI tool | Captures pixels but loses structured ECS/runtime state. |

## Decision: Keep runtime tooling opt-in, local-only, and classified by operation type

**Rationale**: Bevy Remote Protocol can inspect and alter ECS state. `bevy_debugger_mcp` also advertises experiment and rollback-style workflows. That is useful, but it must not become a default endpoint or production behavior. The integration contract classifies operations as observational, screenshot-only, or mutating so future tasks can prefer read-only workflows and document mutation/reset steps when needed.

**Alternatives considered**:

| Alternative | Rationale For Rejection |
| ----------- | ----------------------- |
| Enable BRP/MCP in all dev runs by default | Increases accidental exposure and overhead. |
| Forbid all mutation tools | Removes useful hypothesis testing; safer policy is to require explicit reset/rollback guidance. |
| Configure global Codex MCP settings from repo tasks | Violates workspace scope; global config should only be changed after explicit user request. |

## Decision: Use project-local generated-output paths for screenshots and snapshots

**Rationale**: The repo guidance keeps generated files, scratch files, and temporary outputs inside the repository. Screenshot capture should therefore write to a documented project-local path such as `documentation/images/` only when promoted or a future `.gitignored` scratch path when transient.

**Alternatives considered**:

| Alternative | Rationale For Rejection |
| ----------- | ----------------------- |
| Save screenshots to arbitrary global paths | Harder to review and conflicts with workspace scope. |
| Commit every screenshot | Creates noisy repository churn. |
| Never save screenshots | Prevents visual evidence for AI QA. |
