/// HUMAN: Reusable top-navigation destinations for screen-level navigation.
/// AI: Mount this on DeckScreen first; other screens can reuse the same model later.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TopNavigationDestination {
    PlayGame,
    #[default]
    MyDecks,
    Settings,
    Debug,
}

impl TopNavigationDestination {
    pub const fn label(self) -> &'static str {
        match self {
            Self::PlayGame => "Play Game",
            Self::MyDecks => "My Decks",
            Self::Settings => "Settings",
            Self::Debug => "Debug",
        }
    }

    pub const fn all() -> [Self; 4] {
        [Self::PlayGame, Self::MyDecks, Self::Settings, Self::Debug]
    }
}

/// HUMAN: Current top-navigation presentation state.
/// AI: Keep destination state reusable and independent from any one screen's content model.
#[derive(bevy::prelude::Resource, Clone, Debug, Eq, PartialEq)]
pub struct TopNavigationModel {
    pub selected: TopNavigationDestination,
}

impl Default for TopNavigationModel {
    fn default() -> Self {
        Self {
            selected: TopNavigationDestination::MyDecks,
        }
    }
}

impl TopNavigationModel {
    pub fn is_selected(&self, destination: TopNavigationDestination) -> bool {
        self.selected == destination
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_navigation_destinations_keep_mockup_order() {
        let labels: Vec<&'static str> = TopNavigationDestination::all()
            .iter()
            .map(|destination| destination.label())
            .collect();

        assert_eq!(labels, ["Play Game", "My Decks", "Settings", "Debug"]);
    }

    #[test]
    fn top_navigation_defaults_to_my_decks() {
        let model = TopNavigationModel::default();

        assert!(model.is_selected(TopNavigationDestination::MyDecks));
        assert!(!model.is_selected(TopNavigationDestination::PlayGame));
    }
}
