use bevy::prelude::*;

use crate::runtime::resources::TopNavigationDestination;

/// HUMAN: Root marker for the reusable top navigation view.
/// AI: DeckScreen mounts this first; later screens can share it.
#[derive(Component, Debug, Default)]
pub struct TopNavigationRoot;

/// HUMAN: Button marker for one top navigation destination.
/// AI: Destination activation can be wired per screen without changing button rendering.
#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopNavigationButton {
    pub destination: TopNavigationDestination,
}

impl TopNavigationButton {
    pub const fn new(destination: TopNavigationDestination) -> Self {
        Self { destination }
    }
}
