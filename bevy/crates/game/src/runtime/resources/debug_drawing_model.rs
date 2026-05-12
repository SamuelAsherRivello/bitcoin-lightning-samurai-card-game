use bevy::prelude::*;

use super::card_slot_model::{CardSlotBoardModel, CardSlotRect, CardSlotSide};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DebugDrawingTarget {
    GameArea,
    LocationAreaTwo,
    LocationAreaThree,
    LocationAreaFour,
    LocationCardSlotTopLeftUpperLeft,
    LocationCardSlotTopLeftUpperRight,
    LocationCardSlotTopLeftLowerLeft,
    LocationCardSlotTopLeftLowerRight,
    LocationCardSlotTopCenterUpperLeft,
    LocationCardSlotTopCenterUpperRight,
    LocationCardSlotTopCenterLowerLeft,
    LocationCardSlotTopCenterLowerRight,
    LocationCardSlotTopRightUpperLeft,
    LocationCardSlotTopRightUpperRight,
    LocationCardSlotTopRightLowerLeft,
    LocationCardSlotTopRightLowerRight,
    LocationCardSlotBottomLeftUpperLeft,
    LocationCardSlotBottomLeftUpperRight,
    LocationCardSlotBottomLeftLowerLeft,
    LocationCardSlotBottomLeftLowerRight,
    LocationCardSlotBottomCenterUpperLeft,
    LocationCardSlotBottomCenterUpperRight,
    LocationCardSlotBottomCenterLowerLeft,
    LocationCardSlotBottomCenterLowerRight,
    LocationCardSlotBottomRightUpperLeft,
    LocationCardSlotBottomRightUpperRight,
    LocationCardSlotBottomRightLowerLeft,
    LocationCardSlotBottomRightLowerRight,
    HandArea,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DebugDrawingRect {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
}

impl DebugDrawingRect {
    pub const fn new(left: f32, top: f32, width: f32, height: f32) -> Self {
        Self {
            left,
            top,
            width,
            height,
        }
    }

    pub const fn from_card_slot_rect(rect: CardSlotRect) -> Self {
        Self::new(rect.left, rect.top, rect.width, rect.height)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DebugDrawingColor {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl DebugDrawingColor {
    pub const fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub const fn turquoise() -> Self {
        Self::new(0.0, 0.82, 0.74, 1.0)
    }

    pub const fn blue() -> Self {
        Self::new(0.0, 0.32, 1.0, 1.0)
    }

    pub fn border_color(self) -> Color {
        Color::srgba(self.red, self.green, self.blue, self.alpha)
    }

    pub fn fill_color(self) -> Color {
        Color::srgba(self.red, self.green, self.blue, 0.06)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DebugDrawingRequest {
    pub target: DebugDrawingTarget,
    pub label: String,
    pub rect: DebugDrawingRect,
    pub color: DebugDrawingColor,
    pub generation: u64,
}

/// HUMAN: Requested debug drawing annotations that persist until removed or replaced.
/// AI: Treat these as temporary runtime discussion aids, not permanent feature state.
#[derive(Resource, Debug)]
pub struct DebugDrawingModel {
    requests: Vec<DebugDrawingRequest>,
    next_generation: u64,
}

impl Default for DebugDrawingModel {
    fn default() -> Self {
        let mut model = Self {
            requests: Vec::new(),
            next_generation: 0,
        };
        let slot_board = CardSlotBoardModel::default();
        model.request_reference_layout(&slot_board);
        model
    }
}

impl DebugDrawingModel {
    /// HUMAN: Rebuild default debug annotations from runtime layout data.
    /// AI: Keeps request geometry sourced from layout models instead of hardcoded coordinates.
    pub fn request_reference_layout(&mut self, slot_board: &CardSlotBoardModel) {
        self.replace(
            DebugDrawingTarget::GameArea,
            "game area",
            DebugDrawingTarget::GameArea.quantized_rect(),
        );
        for (target, location_index, label) in [
            (DebugDrawingTarget::LocationAreaTwo, 0, "location area 1"),
            (DebugDrawingTarget::LocationAreaThree, 1, "location area 2"),
            (DebugDrawingTarget::LocationAreaFour, 2, "location area 3"),
        ] {
            let rect = slot_board
                .location_area_rect(location_index)
                .map(DebugDrawingRect::from_card_slot_rect)
                .unwrap_or(DebugDrawingRect::new(0.0, 0.0, 0.0, 0.0));
            self.replace(target, label, rect);
        }
        for target in LOCATION_CARD_SLOT_QUADRANT_TARGETS {
            self.replace_with_color(
                target,
                "",
                target.quantized_rect(),
                DebugDrawingColor::blue(),
            );
        }
        self.replace(
            DebugDrawingTarget::HandArea,
            "hand area",
            DebugDrawingTarget::HandArea.quantized_rect(),
        );
    }

    pub fn request_hand_area(&mut self, label: impl Into<String>) {
        self.replace(
            DebugDrawingTarget::HandArea,
            label,
            DebugDrawingTarget::HandArea.quantized_rect(),
        );
    }

    pub fn replace(
        &mut self,
        target: DebugDrawingTarget,
        label: impl Into<String>,
        rect: DebugDrawingRect,
    ) {
        self.replace_with_color(target, label, rect, DebugDrawingColor::turquoise());
    }

    pub fn replace_with_color(
        &mut self,
        target: DebugDrawingTarget,
        label: impl Into<String>,
        rect: DebugDrawingRect,
        color: DebugDrawingColor,
    ) {
        self.next_generation += 1;
        let request = DebugDrawingRequest {
            target,
            label: label.into(),
            rect,
            color,
            generation: self.next_generation,
        };

        if let Some(existing) = self
            .requests
            .iter_mut()
            .find(|existing| existing.target == target)
        {
            *existing = request;
        } else {
            self.requests.push(request);
        }
    }

    pub fn remove(&mut self, target: DebugDrawingTarget) {
        self.requests.retain(|request| request.target != target);
    }

    pub fn request_for(&self, target: DebugDrawingTarget) -> Option<&DebugDrawingRequest> {
        self.requests
            .iter()
            .find(|request| request.target == target)
    }

    pub fn requests(&self) -> &[DebugDrawingRequest] {
        &self.requests
    }
}

const LOCATION_CARD_SLOT_QUADRANT_TARGETS: [DebugDrawingTarget; 24] = [
    DebugDrawingTarget::LocationCardSlotTopLeftUpperLeft,
    DebugDrawingTarget::LocationCardSlotTopLeftUpperRight,
    DebugDrawingTarget::LocationCardSlotTopLeftLowerLeft,
    DebugDrawingTarget::LocationCardSlotTopLeftLowerRight,
    DebugDrawingTarget::LocationCardSlotTopCenterUpperLeft,
    DebugDrawingTarget::LocationCardSlotTopCenterUpperRight,
    DebugDrawingTarget::LocationCardSlotTopCenterLowerLeft,
    DebugDrawingTarget::LocationCardSlotTopCenterLowerRight,
    DebugDrawingTarget::LocationCardSlotTopRightUpperLeft,
    DebugDrawingTarget::LocationCardSlotTopRightUpperRight,
    DebugDrawingTarget::LocationCardSlotTopRightLowerLeft,
    DebugDrawingTarget::LocationCardSlotTopRightLowerRight,
    DebugDrawingTarget::LocationCardSlotBottomLeftUpperLeft,
    DebugDrawingTarget::LocationCardSlotBottomLeftUpperRight,
    DebugDrawingTarget::LocationCardSlotBottomLeftLowerLeft,
    DebugDrawingTarget::LocationCardSlotBottomLeftLowerRight,
    DebugDrawingTarget::LocationCardSlotBottomCenterUpperLeft,
    DebugDrawingTarget::LocationCardSlotBottomCenterUpperRight,
    DebugDrawingTarget::LocationCardSlotBottomCenterLowerLeft,
    DebugDrawingTarget::LocationCardSlotBottomCenterLowerRight,
    DebugDrawingTarget::LocationCardSlotBottomRightUpperLeft,
    DebugDrawingTarget::LocationCardSlotBottomRightUpperRight,
    DebugDrawingTarget::LocationCardSlotBottomRightLowerLeft,
    DebugDrawingTarget::LocationCardSlotBottomRightLowerRight,
];

impl DebugDrawingTarget {
    pub fn quantized_rect(self) -> DebugDrawingRect {
        match self {
            DebugDrawingTarget::GameArea => DebugDrawingRect::new(304.0, 0.0, 672.0, 800.0),
            DebugDrawingTarget::LocationAreaTwo
            | DebugDrawingTarget::LocationAreaThree
            | DebugDrawingTarget::LocationAreaFour => self
                .runtime_rect(&CardSlotBoardModel::default())
                .unwrap_or(DebugDrawingRect::new(0.0, 0.0, 0.0, 0.0)),
            DebugDrawingTarget::LocationCardSlotTopLeftUpperLeft
            | DebugDrawingTarget::LocationCardSlotTopLeftUpperRight
            | DebugDrawingTarget::LocationCardSlotTopLeftLowerLeft
            | DebugDrawingTarget::LocationCardSlotTopLeftLowerRight
            | DebugDrawingTarget::LocationCardSlotTopCenterUpperLeft
            | DebugDrawingTarget::LocationCardSlotTopCenterUpperRight
            | DebugDrawingTarget::LocationCardSlotTopCenterLowerLeft
            | DebugDrawingTarget::LocationCardSlotTopCenterLowerRight
            | DebugDrawingTarget::LocationCardSlotTopRightUpperLeft
            | DebugDrawingTarget::LocationCardSlotTopRightUpperRight
            | DebugDrawingTarget::LocationCardSlotTopRightLowerLeft
            | DebugDrawingTarget::LocationCardSlotTopRightLowerRight
            | DebugDrawingTarget::LocationCardSlotBottomLeftUpperLeft
            | DebugDrawingTarget::LocationCardSlotBottomLeftUpperRight
            | DebugDrawingTarget::LocationCardSlotBottomLeftLowerLeft
            | DebugDrawingTarget::LocationCardSlotBottomLeftLowerRight
            | DebugDrawingTarget::LocationCardSlotBottomCenterUpperLeft
            | DebugDrawingTarget::LocationCardSlotBottomCenterUpperRight
            | DebugDrawingTarget::LocationCardSlotBottomCenterLowerLeft
            | DebugDrawingTarget::LocationCardSlotBottomCenterLowerRight
            | DebugDrawingTarget::LocationCardSlotBottomRightUpperLeft
            | DebugDrawingTarget::LocationCardSlotBottomRightUpperRight
            | DebugDrawingTarget::LocationCardSlotBottomRightLowerLeft
            | DebugDrawingTarget::LocationCardSlotBottomRightLowerRight => self
                .runtime_rect(&CardSlotBoardModel::default())
                .unwrap_or(DebugDrawingRect::new(0.0, 0.0, 0.0, 0.0)),
            DebugDrawingTarget::HandArea => DebugDrawingRect::new(364.0, 612.0, 552.0, 188.0),
        }
    }

    pub fn runtime_rect(self, slot_board: &CardSlotBoardModel) -> Option<DebugDrawingRect> {
        if let Some(location_index) = self.location_area_index() {
            return slot_board
                .location_area_rect(location_index)
                .map(DebugDrawingRect::from_card_slot_rect);
        }
        let (location_index, side, slot_index) = self.card_slot_identity()?;
        slot_board
            .slot_rect(location_index, side, slot_index)
            .map(DebugDrawingRect::from_card_slot_rect)
    }

    fn location_area_index(self) -> Option<usize> {
        match self {
            DebugDrawingTarget::LocationAreaTwo => Some(0),
            DebugDrawingTarget::LocationAreaThree => Some(1),
            DebugDrawingTarget::LocationAreaFour => Some(2),
            _ => None,
        }
    }

    fn card_slot_identity(self) -> Option<(usize, CardSlotSide, usize)> {
        let index = LOCATION_CARD_SLOT_QUADRANT_TARGETS
            .iter()
            .position(|target| *target == self)?;
        let side = if index < 12 {
            CardSlotSide::Opponent
        } else {
            CardSlotSide::LocalPlayer
        };
        let side_index = index % 12;
        let location_index = side_index / 4;
        let slot_index = side_index % 4;

        Some((location_index, side, slot_index))
    }
}

#[cfg(test)]
#[path = "../../tests/runtime/resources/debug_drawing_model_tests.rs"]
mod debug_drawing_model_tests;
