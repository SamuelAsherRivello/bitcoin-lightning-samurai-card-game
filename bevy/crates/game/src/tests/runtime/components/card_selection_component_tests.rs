use crate::runtime::components::{CardSelectionMovementState, CardSelectionSource, SelectableCard};
use crate::runtime::resources::ActiveView;

#[test]
fn selectable_card_defaults_to_stationary() {
    let card = SelectableCard::new(CardSelectionSource::ScreenCard {
        view: ActiveView::DebugSettingsScene,
    });

    assert!(card.is_stationary());
}

#[test]
fn moving_selection_state_is_not_stationary() {
    let card = SelectableCard::with_movement_state(
        CardSelectionSource::ScreenCard {
            view: ActiveView::DebugSettingsScene,
        },
        CardSelectionMovementState::Moving,
    );

    assert!(!card.is_stationary());
}
