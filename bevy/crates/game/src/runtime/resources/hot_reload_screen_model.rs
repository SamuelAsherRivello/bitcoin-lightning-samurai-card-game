use bevy::prelude::Resource;

use crate::runtime::resources::ActiveView;

/// HUMAN: Tracks hot-patch events that may reset the active conceptual screen.
/// AI: Keep script/tool startup in 001-project-setup; this model is runtime-only.
#[derive(Clone, Debug, Default, Eq, PartialEq, Resource)]
pub struct HotReloadScreenModel {
    pub last_observed_patch_count: u64,
    pub pending_screen_reset: bool,
    pub last_rebuilt_screen: Option<ActiveView>,
}

impl HotReloadScreenModel {
    pub fn observe_patch_count(&mut self, patch_count: u64) -> bool {
        if patch_count == self.last_observed_patch_count {
            return false;
        }

        self.last_observed_patch_count = patch_count;
        self.pending_screen_reset = true;
        true
    }

    pub fn take_screen_reset_request(
        &mut self,
        is_enabled: bool,
        active_view: ActiveView,
    ) -> Option<ActiveView> {
        if !self.pending_screen_reset {
            return None;
        }

        self.pending_screen_reset = false;
        if !is_enabled {
            return None;
        }

        self.last_rebuilt_screen = Some(active_view);
        Some(active_view)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observes_only_new_patch_counts() {
        let mut model = HotReloadScreenModel::default();

        assert!(!model.observe_patch_count(0));
        assert!(model.observe_patch_count(1));
        assert!(!model.observe_patch_count(1));
        assert_eq!(model.last_observed_patch_count, 1);
        assert!(model.pending_screen_reset);
    }

    #[test]
    fn enabled_hot_reload_consumes_reset_request_for_active_screen() {
        let mut model = HotReloadScreenModel::default();
        model.observe_patch_count(3);

        assert_eq!(
            model.take_screen_reset_request(true, ActiveView::DeckScene),
            Some(ActiveView::DeckScene)
        );
        assert!(!model.pending_screen_reset);
        assert_eq!(model.last_rebuilt_screen, Some(ActiveView::DeckScene));
    }

    #[test]
    fn disabled_hot_reload_consumes_patch_without_screen_reset() {
        let mut model = HotReloadScreenModel::default();
        model.observe_patch_count(4);

        assert_eq!(
            model.take_screen_reset_request(false, ActiveView::GameScene),
            None
        );
        assert!(!model.pending_screen_reset);
        assert_eq!(model.last_rebuilt_screen, None);
    }
}
