use bevy::{
    ecs::system::SystemParam,
    prelude::*,
    text::{Underline, UnderlineColor},
    ui::UiScale,
    window::{PrimaryWindow, WindowResized},
};
use bevy_card_game_shared::GameTitle;
use bevy_inspector_egui::{
    bevy_egui::{EguiContext, PrimaryEguiContext, egui},
    bevy_inspector,
    bevy_inspector::EntityFilter,
};

use crate::runtime::components::{
    DebugHudFpsText, DebugHudKeyText, DebugHudText, InspectorState, Player,
};
use crate::runtime::resources::{DebugHudState, GameTicks};

const FPS_UPDATE_INTERVAL_SECONDS: f32 = 0.5;
const SCREEN_PADDING_TOP: f32 = 24.0;
const SCREEN_PADDING_LEFT: f32 = 24.0;
const TARGET_WIDTH: f32 = 1280.0;
const TARGET_HEIGHT: f32 = 720.0;

pub fn setup_game(mut commands: Commands) {
    commands.spawn((Player, Name::new(GameTitle::DISPLAY)));
}

pub fn advance_ticks(mut ticks: ResMut<GameTicks>) {
    ticks.0 += 1;
}

pub fn setup_inspector(mut commands: Commands) {
    commands.spawn((Name::new("Bevy Inspector"), InspectorState::default()));
}

pub fn setup_debug_hud(mut commands: Commands) {
    commands
        .spawn((
            Text::new("Card Inspection POC\nFrame: 0\nKEYS: "),
            TextFont {
                font_size: 22.0,
                ..Default::default()
            },
            TextColor(Color::WHITE),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(SCREEN_PADDING_LEFT),
                top: Val::Px(SCREEN_PADDING_TOP),
                width: Val::Px(210.0),
                align_items: AlignItems::Center,
                padding: UiRect {
                    left: Val::Px(40.0),
                    right: Val::Px(12.0),
                    top: Val::Px(8.0),
                    bottom: Val::Px(8.0),
                },
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.02, 0.02, 0.02, 0.72)),
            DebugHudText,
        ))
        .with_children(|parent| {
            spawn_key_span(parent, "W", KeyCode::KeyW, false);
            spawn_key_span(parent, "A", KeyCode::KeyA, false);
            spawn_key_span(parent, "S", KeyCode::KeyS, false);
            spawn_key_span(parent, "D", KeyCode::KeyD, false);
            parent.spawn(TextSpan::new("\nKEYS: "));
            spawn_key_span(parent, "F", KeyCode::KeyF, true);
            parent.spawn(TextSpan::new(" "));
            spawn_key_span(parent, "I", KeyCode::KeyI, true);
            parent.spawn((TextSpan::new(""), DebugHudFpsText));
        });
}

#[derive(SystemParam)]
pub struct DebugHudUpdateParams<'w, 's> {
    keys: Res<'w, ButtonInput<KeyCode>>,
    time: Res<'w, Time>,
    ticks: Res<'w, GameTicks>,
    hud_state: ResMut<'w, DebugHudState>,
    inspector_query: Query<'w, 's, &'static InspectorState>,
    text_query: Query<'w, 's, &'static mut Text, With<DebugHudText>>,
    fps_text_query: Query<'w, 's, &'static mut TextSpan, With<DebugHudFpsText>>,
    key_text_query: Query<'w, 's, (&'static DebugHudKeyText, &'static mut UnderlineColor)>,
}

pub fn update_debug_hud(mut params: DebugHudUpdateParams) {
    if params.keys.just_pressed(KeyCode::KeyF) {
        params.hud_state.is_fps_visible = !params.hud_state.is_fps_visible;
    }

    params.hud_state.fps_accumulated_seconds += params.time.delta_secs();
    params.hud_state.fps_accumulated_frames += 1;

    if params.hud_state.fps_accumulated_seconds >= FPS_UPDATE_INTERVAL_SECONDS {
        params.hud_state.fps_display_value = if params.hud_state.fps_accumulated_seconds > 0.0 {
            params.hud_state.fps_accumulated_frames as f32
                / params.hud_state.fps_accumulated_seconds
        } else {
            0.0
        };

        params.hud_state.fps_accumulated_seconds = 0.0;
        params.hud_state.fps_accumulated_frames = 0;
    }

    let fps_on = params.hud_state.is_fps_visible;
    let inspector_on = params
        .inspector_query
        .single()
        .map(|inspector| inspector.is_visible)
        .unwrap_or(false);

    for (key_text, mut underline_color) in &mut params.key_text_query {
        let is_active = if key_text.is_toggle {
            match key_text.key_code {
                KeyCode::KeyF => fps_on,
                KeyCode::KeyI => inspector_on,
                _ => false,
            }
        } else {
            params.keys.pressed(key_text.key_code)
        };

        underline_color.0 = if is_active {
            Color::WHITE
        } else {
            Color::srgba(1.0, 1.0, 1.0, 0.0)
        };
    }

    let full_text = format!("Card Inspection POC\nFrame: {}\nKEYS: ", params.ticks.0);
    for mut text in &mut params.text_query {
        *text = Text::new(full_text.clone());
    }

    let fps_line = if params.hud_state.is_fps_visible {
        format!("\nFPS: {:.1}", params.hud_state.fps_display_value)
    } else {
        String::new()
    };

    for mut fps_text in &mut params.fps_text_query {
        *fps_text = TextSpan::new(fps_line.clone());
    }
}

pub fn toggle_inspector(
    keys: Res<ButtonInput<KeyCode>>,
    mut inspector_query: Query<&mut InspectorState>,
) {
    if !keys.just_pressed(KeyCode::KeyI) {
        return;
    }

    let Ok(mut inspector) = inspector_query.single_mut() else {
        return;
    };

    inspector.is_visible = !inspector.is_visible;
}

pub fn scale_debug_hud(
    mut window_resized_events: Option<MessageReader<WindowResized>>,
    primary_window_query: Query<(Entity, &Window), With<PrimaryWindow>>,
    mut ui_scale: Option<ResMut<UiScale>>,
) {
    let Some(ref mut window_resized_events) = window_resized_events else {
        return;
    };
    let Some(ref mut ui_scale) = ui_scale else {
        return;
    };
    let Ok((primary_window_entity, primary_window)) = primary_window_query.single() else {
        return;
    };

    let mut primary_window_resized = false;
    for resized_event in window_resized_events.read() {
        if resized_event.window == primary_window_entity {
            primary_window_resized = true;
        }
    }

    if !primary_window_resized {
        return;
    }

    let width_scale = primary_window.resolution.width() / TARGET_WIDTH;
    let height_scale = primary_window.resolution.height() / TARGET_HEIGHT;
    ui_scale.0 = width_scale.min(height_scale).max(0.1);
}

pub fn inspector_ui(world: &mut World) {
    let Some((is_visible, x, y, width, height)) = inspector_window_settings(world) else {
        return;
    };

    if !is_visible {
        return;
    }

    let Ok(mut egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world)
        .cloned()
    else {
        return;
    };

    egui::Window::new("Bevy Inspector")
        .default_pos(egui::pos2(x, y))
        .default_size(egui::vec2(width, height))
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.heading("Card Inspection POC");
                bevy_inspector::ui_for_entities_filtered(world, ui, true, &InspectorEntityFilter);
                ui.allocate_space(ui.available_size());
            });
        });
}

fn spawn_key_span(
    parent: &mut ChildSpawnerCommands,
    text: &'static str,
    key_code: KeyCode,
    is_toggle: bool,
) {
    parent.spawn((
        TextSpan::new(text),
        Underline,
        UnderlineColor(Color::srgba(1.0, 1.0, 1.0, 0.0)),
        DebugHudKeyText::new(key_code, is_toggle),
    ));
}

struct InspectorEntityFilter;

impl EntityFilter for InspectorEntityFilter {
    type StaticFilter = ();

    fn filter_entity(&self, world: &mut World, entity: Entity) -> bool {
        world.get::<Name>(entity).is_some()
    }
}

fn inspector_window_settings(world: &mut World) -> Option<(bool, f32, f32, f32, f32)> {
    let mut query = world.query::<&InspectorState>();
    let inspector = query.iter(world).next()?;
    Some((
        inspector.is_visible,
        inspector.x,
        inspector.y,
        inspector.width,
        inspector.height,
    ))
}
