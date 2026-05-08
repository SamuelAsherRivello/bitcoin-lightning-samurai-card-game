# Codex Project Template

This is a project-neutral starter repository for Codex and Specify workflows.

Use it when you want a clean agent-ready foundation before choosing a product stack, application framework, or deployment target.

## TOC

- [Codex Project Template](#codex-project-template)
  - [TOC](#toc)
  - [Pics](#pics)
  - [Getting Started](#getting-started)
  - [Details](#details)
  - [Structure](#structure)
  - [Features](#features)
- [Credits](#credits)

<BR>

## Pics

<div style="display: flex; align-items: flex-start; gap: 16px;">
  <a href="./documentation/images/Overview01.png" style="display: inline-block;"><img src="./documentation/images/Overview01.png" width="500" alt="Codex project template overview" style="display: block;"></a>
  <a href="./documentation/images/Workflow01.png" style="display: inline-block;"><img src="./documentation/images/Workflow01.png" width="500" alt="Codex and Specify workflow" style="display: block;"></a>
</div>

<!-- AI: Section purpose: show visual proof of the repository layout and the intended Codex plus Specify workflow. Refresh these images in place when root folders, workflow stages, or documentation conventions change. -->

<BR>

## Getting Started

Clone or copy this repository, then update the project identity and project-specific instructions before adding implementation code.

### Common Setup

| Step | Required? | Description |
| ---- | --------- | ----------- |
| Rename project identity | ✅ | Update `README.md`, `AGENTS.md`, `.codex/project-identity.md`, and any active specs with the new repository name and display name. |
| Write the first spec | ✅ | Use the Specify skills in `.agents/skills` to create or refine `specs/<feature>/spec.md`. |
| Add project scripts | ❌ | Put repeatable local commands in `project/scripts` after the generated project chooses its toolchain. |
| Add project assets | ❌ | Put seed images, fixtures, diagrams, and other reusable project files in `project/assets` until a generated project defines a more specific location. |

<!-- AI: Section purpose: explain the generic starting workflow without assuming a language, package manager, app framework, or deployment target. -->

<BR>

# Details

## Structure

### Root Folders

| # | Name | In Git? | Purpose |
| - | ---- | ------- | ------- |
| 01 | [`.agents`](./.agents) | ✅ | Specify skills used for specification, planning, task generation, implementation, analysis, and checklist workflows. |
| 02 | [`.codex`](./.codex) | ✅ | Repo-local Codex guidance, reusable skills, project identity notes, memory, and rules. |
| 03 | [`.github`](./.github) | ✅ | Placeholder for repository automation that generated projects can define later. |
| 04 | [`.playwright-cli`](./.playwright-cli) | ✅ | Placeholder for browser automation artifacts when a generated project needs visual or browser QA. |
| 05 | [`.specify`](./.specify) | ✅ | Specify configuration, templates, scripts, workflow registry, constitution, and active feature state. |
| 06 | [`.specs`](./.specs) | ✅ | Template/reference specs and generated-project seed material. |
| 07 | [`documentation`](./documentation) | ✅ | README images and supporting documentation. |
| 08 | [`project`](./project) | ✅ | Project-neutral home for reusable scripts and assets before a generated project defines its own source layout. |
| 09 | [`specs`](./specs) | ✅ | Working specifications for features in this repository or generated projects. |

<!-- AI: Section purpose: explain only root-level folders and their intended ownership. Keep this aligned with the actual root tree. -->

### Project Folders

| Path | Description |
| ---- | ----------- |
| [`project/assets`](./project/assets) | Reusable project assets, fixtures, reference media, and diagrams. |
| [`project/scripts`](./project/scripts) | Repeatable local scripts chosen by the generated project. |
| [`documentation/images`](./documentation/images) | README-visible images that summarize the template and workflow. |

<BR>

## Features

### Codex AI Features

This repo includes Codex context and Specify workflows so agent prompts can stay short, repeatable, and grounded in repository files.

Related tech: [OpenAI Codex](https://openai.com/codex)

| File | Purpose |
| ---- | ------- |
| [`AGENTS.md`](./AGENTS.md) | Global and project-local safety, workflow, and formatting instructions for agents. |
| [`.aiignore`](./.aiignore) | AI-agent ignore file for version control internals, generated output, runtime data, logs, and local caches. |
| [`.agents/skills`](./.agents/skills) | Specify skills for constitution, specification, clarification, planning, tasks, implementation, analysis, and issue conversion. |
| [`.codex/skills`](./.codex/skills) | Codex skills for planning, README refreshes, images, reviews, QA, release preparation, memory, and project creation. |
| [`.codex/skills/add-tech-stack`](./.codex/skills/add-tech-stack/SKILL.md) | Shared technology-stack overlay workflow. Use the Rust, Bevy, or Dioxus variants when a generated project chooses one of those stacks. |
| [`.codex/rules`](./.codex/rules) | Repo-local rules that generated projects can extend with stack-specific guidance. |
| [`.specify`](./.specify) | Specify configuration, templates, workflow scripts, and constitution. |
| [`specs`](./specs) | Active project specs. |

### Specify Workflow

| Stage | Artifact | Purpose |
| ----- | -------- | ------- |
| Constitution | [`.specify/memory/constitution.md`](./.specify/memory/constitution.md) | Durable project principles and constraints. |
| Specification | `specs/<feature>/spec.md` | User goals, success criteria, requirements, and scenarios. |
| Plan | `specs/<feature>/plan.md` | Implementation approach, dependencies, structure, and verification strategy. |
| Tasks | `specs/<feature>/tasks.md` | Ordered work items ready for implementation. |
| Analysis | Skill output | Cross-checks spec, plan, and tasks for gaps or contradictions. |

### Project-Neutral Conventions

| Convention | Description |
| ---------- | ----------- |
| No default stack | The template does not assume a language, runtime, package manager, frontend framework, backend framework, or deployment host. |
| Local scripts | Generated projects place repeatable commands under `project/scripts` until they choose a more specific convention. |
| Local assets | Generated projects place reusable seed assets under `project/assets` until they choose a more specific convention. |
| Documentation images | README images live under `documentation/images` and should be refreshed when the root structure or workflow changes. |

<BR>

# Credits

**Created By**

Samuel Asher Rivello. Over 25 years XP with game development (2025); over 10 years XP with Unity (2025).

**Contact**

| Channel | Link |
| ------- | ---- |
| Twitter | [@srivello](https://twitter.com/srivello) |
| Git | [Github.com/SamuelAsherRivello](https://github.com/SamuelAsherRivello) |
| Resume & Portfolio | [SamuelAsherRivello.com](https://www.SamuelAsherRivello.com) |
| LinkedIn | [Linkedin.com/in/SamuelAsherRivello](https://www.linkedin.com/in/samuelrivello/) |

**License**

Provided as-is under [MIT License](./LICENSE) | Copyright (c) 2006 - 2026 Rivello Multimedia Consulting, LLC

<!-- AI: Section purpose: preserve creator, contact, and license information. Generated projects can replace this section when ownership changes. -->
