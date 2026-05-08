#![cfg_attr(windows, windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_card_game::{
    GamePlugin,
    runtime::resources::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH, load_window_placement},
};

fn main() {
    let saved_window_placement = load_window_placement();
    let window_resolution = saved_window_placement
        .as_ref()
        .map(|placement| WindowResolution::new(placement.window_size.x, placement.window_size.y))
        .unwrap_or_else(|| WindowResolution::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT));
    let window_position = saved_window_placement
        .map(|placement| WindowPosition::At(placement.window_position))
        .unwrap_or(WindowPosition::Centered(MonitorSelection::Primary));

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Card Game".to_string(),
                resolution: window_resolution,
                position: window_position,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .run();
}
