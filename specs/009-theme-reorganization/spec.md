# Feature Specification: Theme Reorganization

**Feature Branch**: `[009-theme-reorganization]`  
**Created**: 2026-05-11  
**Status**: Draft  
**Input**: User description: "Let's do a 009 reorganization. While the game is set to have only one theme, I'd like to organize it in case we have more. So update the structure to be like this: bevy/crates/game/assets/themes/theme_japan/cards, bevy/crates/game/assets/themes/theme_japan/locations, bevy/crates/game/assets/themes/theme_japan/worlds. Start stuff within each folder with 'card_', 'location_', and 'world_' without mentioning japan in the naming. That is understood based on its location. Then use a bundle for loading the card. Refer to the card concept as a card bundle in the docs. The card bundle contains the front and back and all layers and all behavior."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Organize Theme Assets for Growth (Priority: P1)

A developer can locate all Japan theme cards, locations, and worlds under one theme-specific asset area, making the current one-theme game ready for additional themes later without changing the meaning of existing content.

**Why this priority**: The main value of this feature is reducing asset ambiguity now while preserving a clean path for future theme additions.

**Independent Test**: Can be fully tested by inspecting the asset library and confirming that Japan theme cards, locations, and worlds live under one theme root with clear category folders.

**Acceptance Scenarios**:

1. **Given** the project contains the current Japan theme assets, **When** a developer inspects the theme asset library, **Then** cards, locations, and worlds are grouped under one Japan theme root.
2. **Given** a developer searches for Japan theme card assets, **When** they open the theme's card category, **Then** they can find every current card asset without needing to inspect non-theme card folders.
3. **Given** a future theme is considered, **When** a developer compares the theme organization, **Then** the categories make it clear where another theme's cards, locations, and worlds would belong.

---

### User Story 2 - Use Theme-Local Naming (Priority: P1)

A developer can identify an asset's category from its name while relying on its containing theme folder for the theme identity, avoiding repeated theme names in each asset.

**Why this priority**: Theme-local naming keeps asset names shorter, easier to scan, and less brittle when themes are added or renamed.

**Independent Test**: Can be fully tested by reviewing all moved or renamed theme assets and confirming that card, location, and world assets use category prefixes without embedding the Japan theme name.

**Acceptance Scenarios**:

1. **Given** an asset belongs to the Japan theme card category, **When** a developer reads its name, **Then** the name begins with `card_` and does not include `japan`.
2. **Given** an asset belongs to the Japan theme location category, **When** a developer reads its name, **Then** the name begins with `location_` and does not include `japan`.
3. **Given** an asset belongs to the Japan theme world category, **When** a developer reads its name, **Then** the name begins with `world_` and does not include `japan`.

---

### User Story 3 - Load Cards as Card Bundles (Priority: P2)

A developer can reason about each playable card as a single card bundle that includes its front, back, visual layers, and behavior contract, rather than as disconnected assets.

**Why this priority**: The card bundle concept makes card loading and documentation easier to extend as cards gain more layers, back art, and card-specific behavior.

**Independent Test**: Can be fully tested by reading the updated documentation and validating that each card is described as a card bundle with its required contents and expected behavior ownership.

**Acceptance Scenarios**:

1. **Given** a developer reads card documentation, **When** the card concept is introduced, **Then** it is called a card bundle.
2. **Given** a card bundle is described, **When** a developer reviews its contents, **Then** the bundle includes front presentation, back presentation, all visual layers, and card behavior.
3. **Given** a card is loaded by the game, **When** a developer traces the card's assets and behavior, **Then** the relationship is represented as one card bundle rather than separate unrelated pieces.

### Edge Cases

- If only the Japan theme exists, the structure still uses a theme root so the current theme does not need a later migration when another theme is added.
- If an existing asset name already starts with the correct category prefix, it remains valid only if it also avoids repeating the theme name.
- If a card has placeholder art or incomplete behavior, it is still represented as a card bundle so placeholder and final cards follow the same concept.
- If non-theme shared assets remain in the project, they must not be confused with theme-owned cards, locations, or worlds.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The asset library MUST group all current Japan theme cards, locations, and worlds under a single Japan theme root.
- **FR-002**: The Japan theme root MUST contain separate category areas for cards, locations, and worlds.
- **FR-003**: Current Japan theme card assets MUST be discoverable in the Japan theme card category.
- **FR-004**: Current Japan theme location assets MUST be discoverable in the Japan theme location category.
- **FR-005**: Current Japan theme world assets MUST be discoverable in the Japan theme world category.
- **FR-006**: Theme-owned card asset names MUST begin with `card_`.
- **FR-007**: Theme-owned location asset names MUST begin with `location_`.
- **FR-008**: Theme-owned world asset names MUST begin with `world_`.
- **FR-009**: Theme-owned card, location, and world asset names MUST NOT repeat `japan` because theme identity is provided by the containing theme root.
- **FR-010**: Documentation MUST refer to the loadable card concept as a card bundle.
- **FR-011**: A card bundle MUST be documented as the complete card package containing front presentation, back presentation, all visual layers, and all card behavior needed for loading and play presentation.
- **FR-012**: Card loading documentation MUST describe cards as being loaded through their card bundle rather than through unrelated individual assets.
- **FR-013**: Existing card-facing user behavior from the current proof-of-concept MUST remain unchanged after reorganization, including bottom-row card display, card selection, card browser viewing, and card flipping.
- **FR-014**: Existing world and location presentation from the current proof-of-concept MUST remain unchanged after reorganization, including active world display and visible tactical locations.
- **FR-015**: The reorganization MUST preserve a clear distinction between theme-owned assets and reusable shared assets that are not specific to a theme.

### Key Entities

- **Theme**: A top-level asset grouping that owns one coherent visual setting and contains its cards, locations, and worlds.
- **Theme Card Category**: The theme-owned collection of card bundles and related card assets for that theme.
- **Theme Location Category**: The theme-owned collection of tactical location assets for that theme.
- **Theme World Category**: The theme-owned collection of world assets for that theme.
- **Card Bundle**: The complete loadable card concept, containing front presentation, back presentation, all visual layers, and all behavior associated with that card.
- **Shared Asset**: A reusable asset that does not belong to one theme and therefore remains outside theme-owned card, location, and world categories.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A developer can locate the Japan theme's cards, locations, and worlds within 30 seconds by starting from the theme asset root.
- **SC-002**: 100% of Japan theme card, location, and world assets use the required category prefix for their asset category.
- **SC-003**: 0 Japan theme card, location, or world asset names include `japan` outside the theme root.
- **SC-004**: A developer can identify the required contents of a card bundle from documentation within 2 minutes.
- **SC-005**: A tester can complete the existing card browsing and card flipping flow after the reorganization with no visible behavior regression.
- **SC-006**: A tester can view the existing world and tactical location presentation after the reorganization with no visible behavior regression.

## Assumptions

- The current theme identity is Japan, and the theme root name remains explicit so future sibling themes can follow the same pattern.
- Existing proof-of-concept gameplay and presentation behavior is retained; this feature focuses on organization, naming, loading concepts, and documentation.
- Shared assets that are not specific to cards, locations, or worlds may remain outside the theme root when they are genuinely reusable across themes.
- Placeholder cards and final cards use the same card bundle concept so the documentation does not need separate temporary terminology.
