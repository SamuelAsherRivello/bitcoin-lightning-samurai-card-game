# Tasks: Card Bundle

**Input**: Consolidated design documents from `specs/006-card-bundle/`

## Phase 1: Spec Consolidation

| ID | Task | Status |
| -- | ---- | ------ |
| T001 | Create consolidated `specs/006-card-bundle/spec.md` from card inspection, polish, and flip scope | ✅ |
| T002 | Create consolidated data model for geometry, inspection, CardStructure, CardType, CardBack, and flip state | ✅ |
| T003 | Create consolidated contract for prototype surfaces, inspection, CardFront, debug controls, and flip behavior | ✅ |
| T004 | Create consolidated quickstart and verification guidance | ✅ |
| T005 | Create consolidation research notes and requirements checklist | ✅ |

## Phase 2: Link And Dependency Cleanup

| ID | Task | Status |
| -- | ---- | ------ |
| T006 | Update references from `004-card-inspection-poc` to `006-card-bundle` | ✅ |
| T007 | Update references from `005-card-polish` to `006-card-bundle` | ✅ |
| T008 | Update references from `006-card-flip` to `006-card-bundle` | ✅ |
| T009 | Verify no downstream spec depends on removed folders | ✅ |

## Phase 3: Removal

| ID | Task | Status |
| -- | ---- | ------ |
| T010 | Remove obsolete `specs/004-card-inspection-poc` directory | ✅ |
| T011 | Remove obsolete `specs/005-card-polish` directory | ✅ |
| T012 | Remove obsolete `specs/006-card-flip` directory | ✅ |

## Phase 4: Verification

| ID | Task | Status |
| -- | ---- | ------ |
| T013 | Run link search for old spec names | ✅ |
| T014 | Review git status for intended spec-only changes and pre-existing unrelated worktree changes | ✅ |
