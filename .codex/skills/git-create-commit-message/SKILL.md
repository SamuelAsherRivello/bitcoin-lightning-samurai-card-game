---
name: git-create-commit-message
description: Create a Git commit message from repository changes, especially the staged diff by default. Use when Codex is asked to draft, create, prepare, write into Git, or show a commit message without committing.
---

# Git Create Commit Message

## Workflow

1. Inspect staged content by default with `git diff --cached --stat`, `git diff --cached --name-only`, and targeted `git diff --cached -- <paths>` as needed.
2. If the user explicitly asks for unstaged, untracked, or all content, inspect only that requested scope.
3. Never run `git commit` unless the user separately asks to commit.
4. When asked to put the message in Git without committing, write it to `.git/COMMIT_EDITMSG`.
5. Put useful scratch notes in `.temp/git-create-commit-message-temp.txt`.
6. Show the final message in chat in a fenced `text` block.

## Message Shape

Use this default structure:

```text
subject

Details:

- Detail 1.
- Detail 2.
- Detail 3.

Test:
- Test 1.
- Test 2.
- Test 3.
```

Keep detail bullets to 3 maximum and test bullets to 3 maximum. Prefer fewer bullets when the change is small.
Assume every commit compiles and the app window is already open; do not include compile commands or window-open confirmation in details or tests.
Write `Test:` bullets as user-facing gestures or observations in the running app, such as mouse press, drag, drop, click, keyboard navigation, or visible UI state checks.

If the staged content is spec-related, start the subject with:

```text
009-theme-organization: ...
```

Use the body details to explain implementation-facing changes and the `Test:` section to describe how to verify the user-facing feature.
