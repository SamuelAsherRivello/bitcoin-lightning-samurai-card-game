use super::*;

#[test]
fn hand_area_request_persists_until_removed() {
    let mut model = DebugDrawingModel {
        requests: Vec::new(),
        next_generation: 0,
    };

    model.request_hand_area("hand area");

    assert!(model.request_for(DebugDrawingTarget::HandArea).is_some());
    model.remove(DebugDrawingTarget::HandArea);
    assert!(model.request_for(DebugDrawingTarget::HandArea).is_none());
}

#[test]
fn replacing_target_updates_generation() {
    let mut model = DebugDrawingModel {
        requests: Vec::new(),
        next_generation: 0,
    };

    model.request_hand_area("first");
    let first_generation = model
        .request_for(DebugDrawingTarget::HandArea)
        .unwrap()
        .generation;
    model.request_hand_area("second");
    let request = model.request_for(DebugDrawingTarget::HandArea).unwrap();

    assert_eq!(model.requests().len(), 1);
    assert_eq!(request.label, "second");
    assert!(request.generation > first_generation);
}

#[test]
fn default_model_requests_reference_debug_drawing_layout() {
    let model = DebugDrawingModel::default();

    assert_eq!(model.requests().len(), 11);
    assert_eq!(
        model
            .request_for(DebugDrawingTarget::LocationAreaTwo)
            .unwrap()
            .rect
            .left
            + 184.0,
        model
            .request_for(DebugDrawingTarget::LocationAreaThree)
            .unwrap()
            .rect
            .left
    );
    assert_eq!(
        model
            .request_for(DebugDrawingTarget::LocationAreaThree)
            .unwrap()
            .rect
            .left
            + 184.0,
        model
            .request_for(DebugDrawingTarget::LocationAreaFour)
            .unwrap()
            .rect
            .left
    );
    assert_eq!(
        model
            .request_for(DebugDrawingTarget::LocationAreaTwo)
            .unwrap()
            .label,
        "location area 1"
    );
    assert_eq!(
        model
            .request_for(DebugDrawingTarget::LocationAreaThree)
            .unwrap()
            .label,
        "location area 2"
    );
    assert_eq!(
        model
            .request_for(DebugDrawingTarget::LocationAreaFour)
            .unwrap()
            .label,
        "location area 3"
    );
    assert_eq!(
        model
            .request_for(DebugDrawingTarget::LocationCardSlotsTopCenter)
            .unwrap()
            .label,
        "Slots Area"
    );
    assert_eq!(
        model
            .request_for(DebugDrawingTarget::HandArea)
            .unwrap()
            .label,
        "hand area"
    );
    assert_eq!(
        model
            .request_for(DebugDrawingTarget::GameArea)
            .unwrap()
            .label,
        "game area"
    );
    assert_eq!(
        model
            .request_for(DebugDrawingTarget::LocationAreaTwo)
            .unwrap()
            .rect,
        DebugDrawingRect::new(364.0, 224.0, 184.0, 208.0)
    );
    assert_eq!(
        model
            .request_for(DebugDrawingTarget::LocationAreaThree)
            .unwrap()
            .rect,
        DebugDrawingRect::new(548.0, 224.0, 184.0, 208.0)
    );
    assert_eq!(
        model
            .request_for(DebugDrawingTarget::LocationAreaFour)
            .unwrap()
            .rect,
        DebugDrawingRect::new(732.0, 224.0, 184.0, 208.0)
    );
    assert_eq!(
        model
            .request_for(DebugDrawingTarget::LocationCardSlotsTopCenter)
            .unwrap()
            .rect,
        DebugDrawingRect::new(548.0, 44.0, 184.0, 180.0)
    );
    assert_eq!(
        model
            .request_for(DebugDrawingTarget::LocationCardSlotsBottomCenter)
            .unwrap()
            .rect,
        DebugDrawingRect::new(548.0, 432.0, 184.0, 180.0)
    );
    assert_eq!(
        model
            .request_for(DebugDrawingTarget::LocationCardSlotsBottomCenter)
            .unwrap()
            .color,
        DebugDrawingColor::blue()
    );
    assert_eq!(
        model
            .request_for(DebugDrawingTarget::LocationCardSlotsBottomCenter)
            .unwrap()
            .color
            .alpha,
        0.9
    );
    assert_eq!(
        model
            .request_for(DebugDrawingTarget::LocationCardSlotsBottomCenter)
            .unwrap()
            .color
            .fill_alpha,
        0.1
    );
}

#[test]
fn location_slot_debug_targets_read_runtime_slot_rects() {
    let board = CardSlotBoardModel::default();

    assert_eq!(
        DebugDrawingTarget::LocationCardSlotsBottomRight.runtime_rect(&board),
        Some(DebugDrawingRect::new(732.0, 432.0, 184.0, 180.0))
    );
}
