# Codex And Specify Workflow

Use this rule when planning, implementing, reviewing, or documenting work in this template or a generated project.

## Principles

| Principle | Requirement |
| --------- | ----------- |
| Spec first | When behavior is ambiguous, update or create a spec before implementation. |
| Project neutral | Do not add stack-specific assumptions to this template unless the task explicitly changes the template scope. |
| Local context | Prefer repo-local instructions, specs, and memory over generic assumptions. |
| Template reference | Use `bevy/crates/template-crate` as the proper Bevy reference when planning folders, representative files, asset folders, or Rust coding standards. |
| Clear verification | Every implementation plan should name the commands or checks that prove the work is complete. |
| Documentation parity | README, specs, and images should match the actual root structure and workflow. |

## Default File Ownership

| Path | Owner |
| ---- | ----- |
| `.agents/skills` | Specify workflow skills. |
| `.codex/skills` | Codex workflow skills. |
| `.specify` | Specify templates, scripts, memory, and workflow state. |
| `.specs` | Template/reference specifications. |
| `specs` | Active project specifications. |
| `bevy/crates/game/assets` | Runtime game assets and fixtures. |
| `bevy/crates/template-crate` | Proper Bevy crate, asset, file, and Rust coding standards reference. |
| `scripts` | Repeatable project-local commands. |
| `documentation` | Supporting documentation and README images. |
