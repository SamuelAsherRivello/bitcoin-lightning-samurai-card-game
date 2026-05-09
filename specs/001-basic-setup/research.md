# Research: Basic Setup

| Decision | Rationale | Alternatives Considered |
| -------- | --------- | ----------------------- |
| Store placement as ignored local JSON | Human-readable local runtime state is easy to inspect and keep out of source control under `generated/` | Registry or global app data was rejected because project guidance keeps generated local state inside the repo unless explicitly asked otherwise |
| Save placement only on normal close | Matches clarification and avoids unnecessary file writes during every drag or resize | Continuous save was rejected because it contradicts the clarified save timing |
| Restore exact placement when same screen is available | Supports repeated two-screen review workflows with minimal friction | OS automatic placement was rejected because it does not meet exact restore requirements |
| Center primary screen at 800x600 for invalid/off-screen placement | Gives a predictable visible fallback and satisfies the clarified default | Nearest screen fallback was rejected because the clarified fallback is primary screen centered |
| Use VS Code task entries for integrated terminal output | The user specifically wants command output in VS Code rather than detached popup PowerShell windows | Detached process launch was rejected because output is not visible in the integrated terminal |
