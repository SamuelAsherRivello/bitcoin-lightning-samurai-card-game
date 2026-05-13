# Bevy Card Game Agent Instructions

This repository is a Bevy ECS card game built from the Codex Project Template. Keep Codex and Specify infrastructure at the root, and keep game implementation under the Bevy workspace.

Default assistant model for this project: `gpt-5.5`.

## Git Safety

- Never run destructive Git operations.
- Git use is limited to inspection and additive work.
- Do not delete repositories, branches, tags, commits, or history.
- Do not rewrite history, squash commits, amend commits, rebase, or force-push.
- Do not run commands that discard local file changes.
- Allowed Git operations are `git status`, `git diff`, `git log`, `git show`, `git fetch`, `git branch`, `git switch`, `git switch -c`, `git checkout -b`, `git add`, `git commit`, and normal non-force `git push`.
- Branch creation and branch switching, including creating and switching to a new feature branch, are allowed when they do not discard local file changes.
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
- Keep theme-owned card, location, and world assets under `bevy/crates/game/assets/themes/theme_<theme_name>/{cards,locations,worlds}/` with category-prefixed folders such as `card_<card_name>`, `location_<location_name>`, and `world_<world_name>`; keep shared shaders under `bevy/crates/game/assets/shaders/`.
- Keep repeatable project scripts under `scripts/`.
- Keep README-visible images under `documentation/images/`.
- Keep Bevy source organized by ECS role under `bevy/crates/game/src/runtime/`.
- Use `bevy/crates/template-crate` as the proper reference for Bevy crate folders, representative files, and Rust coding standards.
- Keep changed Bevy runtime files focused on one primary concept per file; use purposeful names such as `FooPlugin`, `FooComponent`, `BarScene`, `BazView`, `QuxModel`, or `TempSystem`.
- Use `Scene` for the persistent app-level scene, `Model` for data, and `View` for rendering/presentation. `AppScene` is always present; active sub-screen presentations are views such as `GameView`, `DeckBuilderScene`, and `DebugSettingsScene`.
- Name changed runtime system functions as `[domain]_[schedule]_system`, for example `player_update_system`.
- Add a terse two-line purpose comment immediately above each changed or new primary runtime item: one `HUMAN:` line and one `AI:` line.
- Follow `.codex/rules/bevy-runtime-structure.md` for Bevy crate ownership, ECS module boundaries, feature plugins, runtime assets, hot-reloadable systems, and tests.
- Keep persistent overlays such as DebugHUD and Card UI inside the aspect-ratio safe area. For Bevy UI, spawn under the `bevy_aspect_ratio_mask::Hud` root; for egui windows, offset anchors by the 1280x800 safe-area margins.
- Keep desktop window defaults in `bevy/crates/shared/src/window.rs`; update `DEFAULT_WINDOW_WIDTH` and `DEFAULT_WINDOW_HEIGHT` there when changing the project-approved launch/fallback size.
- Use `scripts/main/InstallDependencies.ps1` once per machine to verify Rust setup, then use `scripts/other/RunTests.ps1`, `scripts/main/RunAppDesktop.ps1`, and `scripts/other/StopApp.ps1` for repeatable local workflows.
- If the user says to "peek" at the app, running app, game, or desktop runtime, use the AI runtime workflow: query the local Bevy Remote Protocol endpoint at `http://localhost:15702` when available, capture a screenshot through `bevy_debugger/screenshot` to `target/ai-runtime-screenshots/`, inspect the image, and report both runtime facts and visual observations. If the endpoint is unavailable, say so and ask the user to start `scripts/main/RunAppDesktopHotReload.ps1` or `scripts/other/RunAppDesktop.ps1 -AiRuntime`.
- Treat `AppScene` as the always-present app-level scene and report the currently active view, such as `GameView`, `DeckBuilderScene`, or `DebugSettingsScene`, when describing runtime scene state.

<!-- SPECKIT START -->
Active implementation plan: `specs/017-card-selected/plan.md`
<!-- SPECKIT END -->

## Markdown Format Rules

| Rule | Requirement |
| ---- | ----------- |
| Listed information | When Markdown content lists multiple related items, prefer a table over bullets or numbered lists unless the content is procedural code guidance or a short nested explanation. |
| Boolean table cells | When a table cell would otherwise say `Yes` or `No`, use ✅ for yes and ❌ for no. |
