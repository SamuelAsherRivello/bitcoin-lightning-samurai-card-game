use bevy::prelude::*;

use crate::runtime::components::{
    DebugHudFpsText, DebugHudKeyLabel, DebugHudParent, DebugHudRoot, DebugHudText,
};
use crate::runtime::resources::DebugSafeArea;

/// HUMAN: Spawns the shared DebugHUD panel when a host app opts in.
/// AI: Parent under DebugHudParent so the host can keep it inside the aspect-ratio-safe HUD root.
pub fn debug_hud_setup_system(
    mut commands: Commands,
    parent_query: Query<Entity, With<DebugHudParent>>,
    safe_area: Res<DebugSafeArea>,
    existing_hud_query: Query<Entity, With<DebugHudRoot>>,
) {
    if !existing_hud_query.is_empty() {
        return;
    }

    let rect = safe_area.debug_hud_rect();
    let hud = commands
        .spawn((
            Name::new("Shared DebugHUD"),
            DebugHudRoot,
            Text::new("Scene: GameView\nFrame: 0\nKEYS: "),
            TextFont {
                font_size: 14.0,
                ..Default::default()
            },
            TextColor(Color::WHITE),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(rect.left),
                top: Val::Px(rect.top),
                width: Val::Px(rect.width),
                padding: UiRect::all(Val::Px(8.0)),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.02, 0.02, 0.02, 0.72)),
            DebugHudText,
        ))
        .with_children(|parent| {
            for key_code in [KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD] {
                parent.spawn((
                    TextSpan::new(format!("{key_code:?} ")),
                    DebugHudKeyLabel::new(key_code, false),
                ));
            }
            parent.spawn((TextSpan::new("\nKEYS: "),));
            parent.spawn((
                TextSpan::new("F "),
                DebugHudKeyLabel::new(KeyCode::KeyF, true),
            ));
            parent.spawn((
                TextSpan::new("I"),
                DebugHudKeyLabel::new(KeyCode::KeyI, true),
            ));
            parent.spawn((TextSpan::new(""), DebugHudFpsText));
        })
        .id();

    if let Ok(parent) = parent_query.single() {
        commands.entity(parent).add_child(hud);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_spawns_exactly_one_debug_hud_panel() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<DebugSafeArea>()
            .add_systems(Startup, debug_hud_setup_system);

        app.update();
        app.update();

        let count = app
            .world_mut()
            .query_filtered::<Entity, With<DebugHudRoot>>()
            .iter(app.world())
            .count();
        assert_eq!(count, 1);
    }

    #[test]
    fn setup_can_parent_debug_hud_under_safe_hud_root() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<DebugSafeArea>()
            .add_systems(Startup, debug_hud_setup_system);
        let parent = app.world_mut().spawn(DebugHudParent).id();

        app.update();

        let hud = app
            .world_mut()
            .query_filtered::<Entity, With<DebugHudRoot>>()
            .single(app.world())
            .unwrap();
        assert!(app.world().entity(parent).contains::<Children>());
        assert!(
            app.world()
                .entity(parent)
                .get::<Children>()
                .unwrap()
                .contains(&hud)
        );
    }
}
