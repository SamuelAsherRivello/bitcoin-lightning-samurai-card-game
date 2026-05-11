# Implementation Plan: [FEATURE]

**Branch**: `[###-feature-name]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit-plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary

[Extract from feature spec: primary requirement + technical approach from research]

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: [e.g., chosen project language and version or NEEDS CLARIFICATION]  
**Primary Dependencies**: [chosen project dependencies or NEEDS CLARIFICATION]  
**Storage**: [if applicable, chosen persistence approach or N/A]  
**Testing**: [chosen verification command or NEEDS CLARIFICATION]  
**Target Platform**: [chosen runtime platform or NEEDS CLARIFICATION]
**Project Type**: [chosen project type or NEEDS CLARIFICATION]  
**Performance Goals**: [domain-specific target or NEEDS CLARIFICATION]  
**Constraints**: [domain-specific constraints or NEEDS CLARIFICATION]  
**Scale/Scope**: [domain-specific scope or NEEDS CLARIFICATION]

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- Confirm the feature follows the active spec, constitution, and repo-local agent guidance.
- Confirm source, assets, scripts, docs, and tests stay in the locations defined by this project.
- Confirm Rust workspace folders and files use typical Rust naming conventions, including lowercase crate, module, and asset directories.
- Confirm changed runtime files are organized around one primary concept per file with purposeful Plugin/Component/Scene/View/Model/System names.
- Confirm changed runtime items include a terse `HUMAN:` and `AI:` purpose comment immediately above the primary item.
- Confirm changed runtime system functions follow `[domain]_[schedule]_system`.
- Confirm Bevy runtime naming uses `Scene` for the persistent app-level scene, `Model` for data, and `View` for rendering/presentation where those distinctions apply.
- Confirm theme-owned card, location, and world assets use `assets/themes/theme_<theme_name>/{cards,locations,worlds}/` with category-prefixed folders when a feature touches theme assets.
- Confirm visible loading or toast-style feedback remains for template data loading, cache reads, cache writes, refreshes, and database creation.
- Confirm browser builds keep localStorage snapshots and do not introduce browser SQLite or OPFS worker startup.
- Confirm first-time native database/schema/seed setup remains in `create_database_if_missing()` and normal reads do not recreate or reseed existing data.
- Confirm browser-visible changes have a practical served-web verification path.
- Confirm the feature follows the selected language and framework standards.
- Confirm all on-screen 2D and 3D positions derive from the aspect-ratio-safe game view and are recalculated when the viewport changes.
- Confirm any framework-specific API constraints are documented in the plan.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit-plan command output)
├── research.md          # Phase 0 output (/speckit-plan command)
├── data-model.md        # Phase 1 output (/speckit-plan command)
├── quickstart.md        # Phase 1 output (/speckit-plan command)
├── contracts/           # Phase 1 output (/speckit-plan command)
└── tasks.md             # Phase 2 output (/speckit-tasks command - NOT created by /speckit-plan)
```

### Source Code (repository root)
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths for the generated project. The delivered plan must
  not include Option labels.
-->

```text
# [REMOVE IF UNUSED] Option 1: Single project (DEFAULT)
src/
├── models/
├── services/
├── cli/
└── lib/

tests/
├── contract/
├── integration/
└── unit/

# [REMOVE IF UNUSED] Option 2: Multi-surface project
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/

# [REMOVE IF UNUSED] Option 3: Client + service project
api/
└── [same as backend above]

client/
└── [client-specific structure: feature modules, user flows, client tests]
```

**Structure Decision**: [Document the selected structure and reference the real
directories captured above]

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
