use super::*;

#[test]
fn window_placement_serializes_position_size_and_screen_identity() {
    let placement = WindowPlacement {
        window_position: IVec2::new(100, 200),
        window_size: UVec2::new(800, 600),
        monitor_name: Some("Display 1".to_string()),
        monitor_position: IVec2::ZERO,
        monitor_size: UVec2::new(1920, 1080),
        relative_position: IVec2::new(100, 200),
    };

    let serialized = serde_json::to_string(&placement).unwrap();
    let deserialized: WindowPlacement = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized, placement);
}

#[test]
fn window_placement_requires_positive_size() {
    let placement = WindowPlacement {
        window_position: IVec2::new(100, 200),
        window_size: UVec2::ZERO,
        monitor_name: None,
        monitor_position: IVec2::ZERO,
        monitor_size: UVec2::new(1920, 1080),
        relative_position: IVec2::new(100, 200),
    };

    assert_eq!(valid_window_placement(Some(placement)), None);
}

#[test]
fn window_placement_uses_workspace_local_storage() {
    let path = window_placement_path();
    assert!(
        path.ends_with(
            Path::new("data")
                .join("local_storage")
                .join("window-placement.json")
        )
    );
    assert!(!path.components().any(|component| {
        component.as_os_str() == "game" && path.to_string_lossy().contains("game\\data")
    }));
}

#[test]
fn debug_hud_input_uses_workspace_local_storage() {
    let path = debug_hud_input_path();
    assert!(
        path.ends_with(
            Path::new("data")
                .join("local_storage")
                .join("debug-hud-input.json")
        )
    );
}

#[test]
fn card_settings_uses_workspace_local_storage() {
    let path = card_settings_path();
    assert!(
        path.ends_with(
            Path::new("data")
                .join("local_storage")
                .join("card-settings.json")
        )
    );
}

#[test]
fn debug_hud_input_defaults_all_toggles_off() {
    let store = DebugHudInputStore::default();

    assert!(!store.is_fps_visible);
    assert!(!store.is_fullscreen);
    assert!(!store.is_inspector_visible);
    assert!(!store.is_hot_reload_autorestart_enabled);
    assert_eq!(store.debug_draw_mode, DebugDrawMode::Off);
    assert!(!store.is_debug_drawing_visible);
}

#[test]
fn game_deck_deals_requested_cards_in_remaining_deck_order() {
    let mut deck = GameDeckModel {
        cards: vec![
            YOKAI_PLACEHOLDER_CARD_MODEL_ID.to_string(),
            KAGE_REN_CARD_MODEL_ID.to_string(),
            LORD_DAICHI_CARD_MODEL_ID.to_string(),
            SISTER_HOTARU_CARD_MODEL_ID.to_string(),
        ],
    };
    let mut hand = GameHandModel::default();

    let dealt = deck.deal_to_hand(3, &mut hand);

    assert_eq!(
        dealt,
        vec![
            YOKAI_PLACEHOLDER_CARD_MODEL_ID.to_string(),
            KAGE_REN_CARD_MODEL_ID.to_string(),
            LORD_DAICHI_CARD_MODEL_ID.to_string(),
        ]
    );
    assert_eq!(hand.cards, dealt);
    assert_eq!(deck.cards, vec![SISTER_HOTARU_CARD_MODEL_ID.to_string()]);
}

#[test]
fn game_deck_deals_every_round_from_initial_deck_schedule_without_energy_gate() {
    let mut deck = GameDeckModel {
        cards: vec![
            YOKAI_PLACEHOLDER_CARD_MODEL_ID.to_string(),
            KAGE_REN_CARD_MODEL_ID.to_string(),
            LORD_DAICHI_CARD_MODEL_ID.to_string(),
            SISTER_HOTARU_CARD_MODEL_ID.to_string(),
            YOKAI_PLACEHOLDER_CARD_MODEL_ID.to_string(),
            LORD_DAICHI_CARD_MODEL_ID.to_string(),
            KAGE_REN_CARD_MODEL_ID.to_string(),
            SISTER_HOTARU_CARD_MODEL_ID.to_string(),
            LORD_DAICHI_CARD_MODEL_ID.to_string(),
            KAGE_REN_CARD_MODEL_ID.to_string(),
            SISTER_HOTARU_CARD_MODEL_ID.to_string(),
            YOKAI_PLACEHOLDER_CARD_MODEL_ID.to_string(),
        ],
    };
    let mut hand = GameHandModel::default();

    let dealt_counts: Vec<usize> = (1..=6)
        .map(|round| {
            deck.deal_to_hand(requested_cards_for_round(round), &mut hand)
                .len()
        })
        .collect();

    assert_eq!(dealt_counts, vec![1, 2, 3, 1, 1, 1]);
    assert_eq!(hand.cards.len(), 9);
    assert_eq!(deck.cards.len(), 3);
}

#[test]
fn game_deck_deal_to_hand_deals_only_remaining_cards() {
    let mut deck = GameDeckModel {
        cards: vec![KAGE_REN_CARD_MODEL_ID.to_string()],
    };
    let mut hand = GameHandModel::default();

    let dealt = deck.deal_to_hand(3, &mut hand);

    assert_eq!(dealt, vec![KAGE_REN_CARD_MODEL_ID.to_string()]);
    assert_eq!(hand.cards, vec![KAGE_REN_CARD_MODEL_ID.to_string()]);
    assert!(deck.cards.is_empty());
}

#[test]
fn debug_hud_input_store_persists_debug_drawing_toggle() {
    let state = DebugHudState {
        debug_draw_mode: DebugDrawMode::OnSolo,
        ..Default::default()
    };

    let store = DebugHudInputStore::from_state(&state);
    let mut restored_state = DebugHudState::default();
    store.apply_to_state(&mut restored_state);

    assert_eq!(store.debug_draw_mode, DebugDrawMode::OnSolo);
    assert!(store.is_debug_drawing_visible);
    assert_eq!(restored_state.debug_draw_mode, DebugDrawMode::OnSolo);
    assert!(restored_state.is_debug_drawing_visible());
    assert!(restored_state.is_debug_drawing_solo());
}

#[test]
fn debug_hud_input_store_migrates_legacy_debug_drawing_bool() {
    let store = DebugHudInputStore {
        is_debug_drawing_visible: true,
        ..Default::default()
    };
    let mut restored_state = DebugHudState::default();

    store.apply_to_state(&mut restored_state);

    assert_eq!(restored_state.debug_draw_mode, DebugDrawMode::On);
}

#[test]
fn card_defaults_match_japan_realism_card_ratio() {
    let defaults = CardInspectionDefaults::default();
    let expected_ratio = 1.0 / CARD_RENDER_ASPECT_RATIO_WIDTH_OVER_HEIGHT;
    let tolerance = expected_ratio * 0.02;

    assert!((defaults.height_width_ratio() - expected_ratio).abs() <= tolerance);
    assert_eq!(
        defaults.width / defaults.height,
        CARD_RENDER_ASPECT_RATIO_WIDTH_OVER_HEIGHT
    );
    assert_eq!(
        defaults.max_tilt_radians,
        CARD_MAX_TILT_DEGREES.to_radians()
    );
    assert_eq!(
        defaults.smoothing_response_seconds,
        CARD_SMOOTHING_RESPONSE_SECONDS
    );
}

#[test]
fn card_defaults_fit_inside_unit_bounds() {
    let defaults = CardInspectionDefaults::default();

    assert!(defaults.width <= 1.0);
    assert!(defaults.height <= 1.0);
    assert!(defaults.thickness <= 1.0);
    assert_eq!(defaults.height, 1.0);
}

#[test]
fn card_model_registry_has_japan_realism_characters() {
    let registry = CardModelRegistry::default();
    let active_card_model = ActiveCardModel::default();

    assert_eq!(registry.slot_count(), CARD_MODEL_SLOT_COUNT);
    assert_eq!(registry.available_count(), 4);
    assert_eq!(
        registry
            .active_card_model(&active_card_model)
            .map(|card_model| card_model.id),
        Some(KAGE_REN_CARD_MODEL_ID)
    );
}

#[test]
fn card_model_registry_exposes_cost_and_base_power_for_every_card() {
    let registry = CardModelRegistry::default();
    let card_ids: Vec<&str> = registry
        .card_models()
        .map(|card_model| card_model.id)
        .collect();

    assert_eq!(
        card_ids,
        vec![
            KAGE_REN_CARD_MODEL_ID,
            LORD_DAICHI_CARD_MODEL_ID,
            SISTER_HOTARU_CARD_MODEL_ID,
            YOKAI_PLACEHOLDER_CARD_MODEL_ID,
        ]
    );
    assert!(registry.card_models().all(|card_model| {
        card_model.cost.is_in_display_contract() && card_model.base_power.is_in_display_contract()
    }));
}

#[test]
fn card_model_creation_assigns_in_range_cost_and_base_power() {
    let generated_values: Vec<(i32, i32)> = (0..64)
        .map(|_| {
            let card_model = CardModel::kage_ren();
            (card_model.cost.value, card_model.base_power.value)
        })
        .collect();

    assert!(generated_values.iter().all(|(cost, base_power)| {
        (POINT_VIEW_DISPLAY_MIN..=POINT_VIEW_DISPLAY_MAX).contains(cost)
            && (POINT_VIEW_DISPLAY_MIN..=POINT_VIEW_DISPLAY_MAX).contains(base_power)
    }));
}

#[test]
fn card_model_textures_match_asset_directory_casing() {
    let registry = CardModelRegistry::default();
    let asset_root = game_asset_root_path();

    for card_model in registry.slots.iter().flatten() {
        for texture_path in [
            card_model.background_texture,
            card_model.frame_texture,
            card_model.foreground_texture,
            card_model.title_texture,
        ] {
            assert!(
                asset_root.join(texture_path).is_file(),
                "missing card model texture at {}",
                texture_path
            );
        }
    }
}

#[test]
fn game_font_master_list_contains_four_existing_assets() {
    let asset_root = game_asset_root_path();
    let fonts = GameFont::all();

    assert_eq!(fonts.len(), GAME_FONT_COUNT);
    for font in fonts {
        assert!(
            asset_root.join(font.asset_path()).is_file(),
            "missing font asset at {}",
            font.asset_path()
        );
    }
}

#[test]
fn game_button_and_point_view_use_different_font_choices() {
    assert_ne!(GAME_BUTTON_FONT, POINT_VIEW_FONT);
    assert_eq!(GAME_BUTTON_FONT.asset_path(), "fonts/kamikaze/Kamikaze.ttf");
    assert_eq!(
        POINT_VIEW_FONT.asset_path(),
        "fonts/blast-dragon/Blast Dragon D.otf"
    );
}

#[test]
fn theme_asset_root_contains_current_japan_cards_locations_and_worlds() {
    let asset_root = game_asset_root_path();
    for relative_path in [
        "themes/theme_japan/cards/card_kage_ren/background.png",
        "themes/theme_japan/cards/card_lord_daichi/background.png",
        "themes/theme_japan/cards/card_sister_hotaru/background.png",
        "themes/theme_japan/cards/card_yokai_placeholder/background.png",
        "themes/theme_japan/cards/card_back.png",
        "themes/theme_japan/cards/safe_area.png",
        "themes/theme_japan/locations/location_fortress_gate/location.png",
        "themes/theme_japan/locations/location_bamboo_crossing/location.png",
        "themes/theme_japan/locations/location_shrine_ruins/location.png",
        "themes/theme_japan/locations/location_battlefield/location.png",
        "themes/theme_japan/locations/location_spirit_well/location.png",
        "themes/theme_japan/locations/location_market_square/location.png",
        "themes/theme_japan/worlds/world_bamboo_forest/world_background.png",
        "themes/theme_japan/worlds/world_coastal_harbor/world_background.png",
    ] {
        assert!(
            asset_root.join(relative_path).is_file(),
            "missing theme asset at {relative_path}"
        );
    }
}

#[test]
fn runtime_model_paths_start_with_theme_root() {
    let card_registry = CardModelRegistry::default();
    for card_model in card_registry.card_models() {
        for texture_path in [
            card_model.background_texture,
            card_model.frame_texture,
            card_model.foreground_texture,
            card_model.title_texture,
        ] {
            assert!(texture_path.starts_with("themes/theme_japan/cards/"));
        }
    }

    let world_registry = WorldModelRegistry::default();
    for world_model in &world_registry.themes {
        assert!(
            world_model
                .background_texture
                .starts_with("themes/theme_japan/worlds/")
        );
    }

    let location_registry = LocationModelRegistry::default();
    for location_model in &location_registry.locations {
        assert!(
            location_model
                .texture
                .starts_with("themes/theme_japan/locations/")
        );
    }
}

#[test]
fn theme_owned_folder_names_use_category_prefixes() {
    let card_registry = CardModelRegistry::default();
    for card_model in card_registry.card_models() {
        assert!(theme_owned_name(card_model.background_texture).starts_with("card_"));
    }

    let world_registry = WorldModelRegistry::default();
    for world_model in &world_registry.themes {
        assert!(theme_owned_name(world_model.background_texture).starts_with("world_"));
    }

    let location_registry = LocationModelRegistry::default();
    for location_model in &location_registry.locations {
        assert!(theme_owned_name(location_model.texture).starts_with("location_"));
    }
}

#[test]
fn theme_owned_paths_do_not_repeat_japan_after_theme_root() {
    for path in theme_owned_runtime_paths() {
        let after_root = path
            .strip_prefix("themes/theme_japan/")
            .expect("theme-owned path should start with theme root");
        assert!(
            !after_root.contains("japan"),
            "theme-owned path repeats theme name: {path}"
        );
    }
}

#[test]
fn card_model_registry_paths_cover_card_view_bundle_presentation_assets() {
    let registry = CardModelRegistry::default();
    for card_model in registry.card_models() {
        assert!(card_model.background_texture.ends_with("/background.png"));
        assert!(card_model.frame_texture.ends_with("/frame.png"));
        assert!(
            card_model
                .foreground_texture
                .ends_with("/foreground_character.png")
        );
        assert!(card_model.title_texture.ends_with("/title.png"));
        assert!(CARD_BACK_TEXTURE_PATH.ends_with("/card_back.png"));
        assert!(CARD_SAFE_AREA_TEXTURE_PATH.ends_with("/safe_area.png"));
    }
}

#[test]
fn card_model_toggle_cycles_between_four_japan_realism_cards() {
    let registry = CardModelRegistry::default();
    let mut active_card_model = ActiveCardModel::default();

    active_card_model.toggle(&registry);
    assert_eq!(active_card_model.index, 1);
    assert_eq!(
        registry
            .active_card_model(&active_card_model)
            .map(|card_model| card_model.display_name),
        Some(LORD_DAICHI_CARD_MODEL_NAME)
    );

    active_card_model.toggle(&registry);
    assert_eq!(active_card_model.index, 2);
    assert_eq!(
        registry
            .active_card_model(&active_card_model)
            .map(|card_model| card_model.display_name),
        Some(SISTER_HOTARU_CARD_MODEL_NAME)
    );

    active_card_model.toggle(&registry);
    assert_eq!(active_card_model.index, 3);
    assert_eq!(
        registry
            .active_card_model(&active_card_model)
            .map(|card_model| card_model.display_name),
        Some(YOKAI_PLACEHOLDER_CARD_MODEL_NAME)
    );

    active_card_model.toggle(&registry);

    assert_eq!(active_card_model.index, 0);
    assert_eq!(
        registry
            .active_card_model(&active_card_model)
            .map(|card_model| card_model.display_name),
        Some(KAGE_REN_CARD_MODEL_NAME)
    );
}

#[test]
fn card_flip_state_defaults_to_front_idle() {
    let state = CardFlipState::default();

    assert_eq!(state.current_y_rotation, 0.0);
    assert_eq!(state.target_y_rotation, 0.0);
    assert_eq!(state.elapsed_seconds, 0.0);
    assert_eq!(state.visible_face, CardFace::Front);
    assert!(!state.is_animating());
}

#[test]
fn card_flip_state_targets_180_degrees_per_request() {
    let mut state = CardFlipState::default();

    state.request_flip();

    assert_eq!(state.target_y_rotation, std::f32::consts::PI);
    assert!(state.is_animating());
}

#[test]
fn card_flip_state_switches_face_after_midpoint() {
    assert_eq!(CardFlipState::face_for_angle(0.0), CardFace::Front);
    assert_eq!(
        CardFlipState::face_for_angle(std::f32::consts::FRAC_PI_2 - 0.01),
        CardFace::Front
    );
    assert_eq!(
        CardFlipState::face_for_angle(std::f32::consts::FRAC_PI_2 + 0.01),
        CardFace::Back
    );
    assert_eq!(
        CardFlipState::face_for_angle(std::f32::consts::PI),
        CardFace::Back
    );
}

#[test]
fn card_flip_state_reverses_mid_animation_from_current_progress() {
    let mut state = CardFlipState::default();

    state.request_flip();
    state.current_y_rotation = std::f32::consts::FRAC_PI_2;
    state.request_flip();

    assert_eq!(state.target_y_rotation, 0.0);
}

#[test]
fn card_flip_state_uses_half_second_ease_out() {
    let mut state = CardFlipState::default();

    state.request_flip();
    state.advance(CARD_FLIP_DURATION_SECONDS * 0.5);

    assert!(state.current_y_rotation > std::f32::consts::PI * 0.5);
    assert!(state.is_animating());

    state.advance(CARD_FLIP_DURATION_SECONDS * 0.5);

    assert_eq!(state.current_y_rotation, std::f32::consts::PI);
    assert!(!state.is_animating());
}

#[test]
fn card_back_texture_uses_theme_card_asset_path() {
    assert_eq!(
        CARD_BACK_TEXTURE_PATH,
        "themes/theme_japan/cards/card_back.png"
    );
    assert!(!CARD_BACK_TEXTURE_PATH.contains("card_model_"));
}

#[test]
fn world_model_registry_cycles_between_bamboo_forest_and_coastal_harbor() {
    let registry = WorldModelRegistry::default();
    let mut active_world_model = ActiveWorldModel::default();

    assert_eq!(registry.len(), WORLD_MODEL_COUNT);
    assert_eq!(
        registry.active_world_model(&active_world_model).id,
        BAMBOO_FOREST_WORLD_ID
    );

    active_world_model.toggle(&registry);
    assert_eq!(
        registry.active_world_model(&active_world_model).id,
        COASTAL_HARBOR_WORLD_ID
    );

    active_world_model.toggle(&registry);
    assert_eq!(
        registry.active_world_model(&active_world_model).id,
        BAMBOO_FOREST_WORLD_ID
    );
}

#[test]
fn active_locations_selects_three_locations_from_six() {
    let registry = LocationModelRegistry::default();
    let mut active_locations = ActiveLocations::default();
    let active_world_model = ActiveWorldModel::default();

    assert_eq!(registry.len(), LOCATION_MODEL_COUNT);
    assert_eq!(
        registry.selected_locations(&active_locations).len(),
        ACTIVE_LOCATION_COUNT
    );

    active_locations.reroll(&registry, &active_world_model);

    assert_eq!(
        registry.selected_locations(&active_locations).len(),
        ACTIVE_LOCATION_COUNT
    );
    assert!(
        active_locations
            .indices
            .iter()
            .all(|index| *index < LOCATION_MODEL_COUNT)
    );
    for (position, index) in active_locations.indices.iter().enumerate() {
        assert!(
            !active_locations.indices[(position + 1)..].contains(index),
            "active location index {index} should appear only once"
        );
    }
}

#[test]
fn card_ui_depth_factor_defaults_to_current_parallax_strength() {
    let state = CardUiState::default();

    assert_eq!(state.depth_factor, CARD_DEPTH_FACTOR_DEFAULT);
    assert_eq!(state.depth_multiplier(), 1.0);
    assert!(state.show_safe_area);
    assert_eq!(state.background_layer_scale, CARD_LAYER_SCALE_DEFAULT);
    assert_eq!(state.frame_layer_scale, CARD_LAYER_SCALE_DEFAULT);
    assert_eq!(state.foreground_layer_scale, CARD_LAYER_SCALE_DEFAULT);
    assert_eq!(state.title_layer_scale, CARD_LAYER_SCALE_DEFAULT);
}

#[test]
fn card_ui_depth_factor_scales_from_coplanar_to_double_strength() {
    let mut state = CardUiState {
        depth_factor: CARD_DEPTH_FACTOR_MIN,
        ..Default::default()
    };

    assert_eq!(state.depth_multiplier(), 0.0);

    state.depth_factor = CARD_DEPTH_FACTOR_MAX;

    assert_eq!(state.depth_multiplier(), 2.0);
}

#[test]
fn card_settings_applies_depth_factor_to_card_ui_state() {
    let settings = CardSettingsStore {
        depth_factor: 7.5,
        show_safe_area: false,
        background_layer_scale: 0.5,
        frame_layer_scale: 0.75,
        foreground_layer_scale: 1.25,
        title_layer_scale: 1.5,
    };
    let mut state = CardUiState::default();

    settings.apply_to_state(&mut state);

    assert_eq!(state.depth_factor, 7.5);
    assert!(!state.show_safe_area);
    assert_eq!(state.background_layer_scale, 0.5);
    assert_eq!(state.frame_layer_scale, 0.75);
    assert_eq!(state.foreground_layer_scale, 1.25);
    assert_eq!(state.title_layer_scale, 1.5);
}

#[test]
fn card_settings_clamps_depth_factor_to_supported_range() {
    let settings = CardSettingsStore {
        depth_factor: CARD_DEPTH_FACTOR_MAX + 1.0,
        show_safe_area: false,
        background_layer_scale: CARD_LAYER_SCALE_MIN - 1.0,
        frame_layer_scale: CARD_LAYER_SCALE_MAX + 1.0,
        foreground_layer_scale: CARD_LAYER_SCALE_MAX + 1.0,
        title_layer_scale: CARD_LAYER_SCALE_MIN - 1.0,
    };
    let mut state = CardUiState::default();

    settings.apply_to_state(&mut state);

    assert_eq!(state.depth_factor, CARD_DEPTH_FACTOR_MAX);
    assert_eq!(state.background_layer_scale, CARD_LAYER_SCALE_MIN);
    assert_eq!(state.frame_layer_scale, CARD_LAYER_SCALE_MAX);
    assert_eq!(state.foreground_layer_scale, CARD_LAYER_SCALE_MAX);
    assert_eq!(state.title_layer_scale, CARD_LAYER_SCALE_MIN);
}

fn game_asset_root_path() -> PathBuf {
    workspace_root_path()
        .join("bevy")
        .join("crates")
        .join("game")
        .join("assets")
}

fn theme_owned_name(path: &str) -> &str {
    path.split('/')
        .nth(3)
        .expect("theme-owned path should include category-owned folder")
}

fn theme_owned_runtime_paths() -> Vec<&'static str> {
    let card_registry = CardModelRegistry::default();
    let world_registry = WorldModelRegistry::default();
    let location_registry = LocationModelRegistry::default();
    let mut paths = vec![CARD_BACK_TEXTURE_PATH, CARD_SAFE_AREA_TEXTURE_PATH];
    for card_model in card_registry.card_models() {
        paths.extend([
            card_model.background_texture,
            card_model.frame_texture,
            card_model.foreground_texture,
            card_model.title_texture,
        ]);
    }
    for world_model in &world_registry.themes {
        paths.push(world_model.background_texture);
    }
    for location_model in &location_registry.locations {
        paths.push(location_model.texture);
    }
    paths
}
