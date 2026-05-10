# Bevy Card Game Agent Instructions

This repository is a Bevy ECS card game built from the Codex Project Template. Keep Codex and Specify infrastructure at the root, and keep game implementation under the Bevy workspace.

## Git Safety

- Never run destructive Git operations.
- Git use is limited to inspection and additive work.
- Do not delete repositories, branches, tags, commits, or history.
- Do not rewrite history, squash commits, amend commits, rebase, or force-push.
- Do not run commands that discard local file changes.
- Allowed Git operations are `git status`, `git diff`, `git log`, `git show`, `git fetch`, `git add`, `git commit`, and normal non-force `git push`.
- If a task appears to require destructive Git, stop and explain that the operation is not permitted.

## Secret And Credential Safety

- Never ask the user to paste passwords, private keys, seed phrases, API keys, cookies, session tokens, or database credentials into chat.
- Never print secrets from local files, remote files, environment variables, service settings, or config files.
- If a task needs a secret, instruct the user to enter it directly into the target app, terminal, secret manager, or hosting provider.
- Do not copy secrets into generated files, logs, commits, pull requests, screenshots, or Markdown notes.

## Workspace Scope

- Stay inside the current repository unless the user explicitly names another path or repository in the current request.
- Keep generated files, scratch files, downloaded assets, caches, and temporary outputs inside this repository.
- Do not change global editor, shell, Git, SSH, service, or machine configuration unless the user explicitly asks for that class of change.
- Do not expose local-only services, admin tools, databases, or app internals to the public internet without explicit approval.

## Project Workflow

- Inspect current files before editing; keep changes scoped to the requested behavior.
- Prefer specs before implementation when behavior is not already defined.
- Keep active specs under `specs/`.
- Keep Specify configuration and templates under `.specify/`.
- Keep Codex guidance, reusable skills, rules, and memory under `.codex/`.
- Keep reusable non-game project assets under `bevy/crates/game/assets/`.
- Keep Bevy runtime assets under `bevy/crates/game/assets/`.
- Keep the Bevy workspace directory casing lowercase as `bevy/crates/...`; do not introduce `Bevy/Crates/...` paths.
- Keep card structure assets under `bevy/crates/game/assets/cards/card_structure/`, card type assets under `bevy/crates/game/assets/cards/card_types/card_type_<card_type_name>/`, world assets under `bevy/crates/game/assets/worlds/<world_name>/`, and custom shaders under `bevy/crates/game/assets/shaders/`.
- Keep repeatable project scripts under `scripts/`.
- Keep README-visible images under `documentation/images/`.
- Keep Bevy source organized by ECS role under `bevy/crates/game/src/runtime/`.
- Follow `.codex/rules/bevy-runtime-structure.md` for Bevy crate ownership, ECS module boundaries, feature plugins, runtime assets, hot-reloadable systems, and tests.
- Keep persistent overlays such as DebugHUD and Card UI inside the aspect-ratio safe area. For Bevy UI, spawn under the `bevy_aspect_ratio_mask::Hud` root; for egui windows, offset anchors by the 1280x800 safe-area margins.
- Keep desktop window defaults in `bevy/crates/shared/src/window.rs`; update `DEFAULT_WINDOW_WIDTH` and `DEFAULT_WINDOW_HEIGHT` there when changing the project-approved launch/fallback size.
- Use `scripts/main/InstallDependencies.ps1` once per machine to verify Rust setup, then use `scripts/other/RunTests.ps1`, `scripts/main/RunAppDesktop.ps1`, and `scripts/other/StopApp.ps1` for repeatable local workflows.

<!-- SPECKIT START -->
Active implementation plan: `specs/006-card-flip/plan.md`
<!-- SPECKIT END -->

## Markdown Format Rules

| Rule | Requirement |
| ---- | ----------- |
| Listed information | When Markdown content lists multiple related items, prefer a table over bullets or numbered lists unless the content is procedural code guidance or a short nested explanation. |
| Boolean table cells | When a table cell would otherwise say `Yes` or `No`, use ✅ for yes and ❌ for no. |
