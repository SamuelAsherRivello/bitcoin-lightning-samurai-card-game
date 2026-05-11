use bevy::prelude::*;

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
    SingleCardArea,
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
        model.request_reference_layout();
        model
    }
}

impl DebugDrawingModel {
    pub fn request_reference_layout(&mut self) {
        self.replace(
            DebugDrawingTarget::GameArea,
            "game area",
            DebugDrawingTarget::GameArea.quantized_rect(),
        );
        self.replace(
            DebugDrawingTarget::LocationAreaTwo,
            "location area",
            DebugDrawingTarget::LocationAreaTwo.quantized_rect(),
        );
        self.replace(
            DebugDrawingTarget::LocationAreaThree,
            "location area",
            DebugDrawingTarget::LocationAreaThree.quantized_rect(),
        );
        self.replace(
            DebugDrawingTarget::LocationAreaFour,
            "location area",
            DebugDrawingTarget::LocationAreaFour.quantized_rect(),
        );
        for target in LOCATION_CARD_SLOT_QUADRANT_TARGETS {
            self.replace_with_color(
                target,
                "location area card slot",
                target.quantized_rect(),
                DebugDrawingColor::blue(),
            );
        }
        self.replace(
            DebugDrawingTarget::HandArea,
            "hand area",
            DebugDrawingTarget::HandArea.quantized_rect(),
        );
        self.replace(
            DebugDrawingTarget::SingleCardArea,
            "single card area",
            DebugDrawingTarget::SingleCardArea.quantized_rect(),
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
    pub const fn quantized_rect(self) -> DebugDrawingRect {
        match self {
            DebugDrawingTarget::GameArea => DebugDrawingRect::new(304.0, 0.0, 672.0, 800.0),
            DebugDrawingTarget::LocationAreaTwo => {
                DebugDrawingRect::new(364.0, 224.0, 184.0, 208.0)
            }
            DebugDrawingTarget::LocationAreaThree => {
                DebugDrawingRect::new(548.0, 224.0, 184.0, 208.0)
            }
            DebugDrawingTarget::LocationAreaFour => {
                DebugDrawingRect::new(732.0, 224.0, 184.0, 208.0)
            }
            DebugDrawingTarget::LocationCardSlotTopLeftUpperLeft => {
                DebugDrawingRect::new(364.0, 44.0, 92.0, 90.0)
            }
            DebugDrawingTarget::LocationCardSlotTopLeftUpperRight => {
                DebugDrawingRect::new(456.0, 44.0, 92.0, 90.0)
            }
            DebugDrawingTarget::LocationCardSlotTopLeftLowerLeft => {
                DebugDrawingRect::new(364.0, 134.0, 92.0, 90.0)
            }
            DebugDrawingTarget::LocationCardSlotTopLeftLowerRight => {
                DebugDrawingRect::new(456.0, 134.0, 92.0, 90.0)
            }
            DebugDrawingTarget::LocationCardSlotTopCenterUpperLeft => {
                DebugDrawingRect::new(548.0, 44.0, 92.0, 90.0)
            }
            DebugDrawingTarget::LocationCardSlotTopCenterUpperRight => {
                DebugDrawingRect::new(640.0, 44.0, 92.0, 90.0)
            }
            DebugDrawingTarget::LocationCardSlotTopCenterLowerLeft => {
                DebugDrawingRect::new(548.0, 134.0, 92.0, 90.0)
            }
            DebugDrawingTarget::LocationCardSlotTopCenterLowerRight => {
                DebugDrawingRect::new(640.0, 134.0, 92.0, 90.0)
            }
            DebugDrawingTarget::LocationCardSlotTopRightUpperLeft => {
                DebugDrawingRect::new(732.0, 44.0, 92.0, 90.0)
            }
            DebugDrawingTarget::LocationCardSlotTopRightUpperRight => {
                DebugDrawingRect::new(824.0, 44.0, 92.0, 90.0)
            }
            DebugDrawingTarget::LocationCardSlotTopRightLowerLeft => {
                DebugDrawingRect::new(732.0, 134.0, 92.0, 90.0)
            }
            DebugDrawingTarget::LocationCardSlotTopRightLowerRight => {
                DebugDrawingRect::new(824.0, 134.0, 92.0, 90.0)
            }
            DebugDrawingTarget::LocationCardSlotBottomLeftUpperLeft => {
                DebugDrawingRect::new(364.0, 432.0, 92.0, 90.0)
            }
            DebugDrawingTarget::LocationCardSlotBottomLeftUpperRight => {
                DebugDrawingRect::new(456.0, 432.0, 92.0, 90.0)
            }
            DebugDrawingTarget::LocationCardSlotBottomLeftLowerLeft => {
                DebugDrawingRect::new(364.0, 522.0, 92.0, 90.0)
            }
            DebugDrawingTarget::LocationCardSlotBottomLeftLowerRight => {
                DebugDrawingRect::new(456.0, 522.0, 92.0, 90.0)
            }
            DebugDrawingTarget::LocationCardSlotBottomCenterUpperLeft => {
                DebugDrawingRect::new(548.0, 432.0, 92.0, 90.0)
            }
            DebugDrawingTarget::LocationCardSlotBottomCenterUpperRight => {
                DebugDrawingRect::new(640.0, 432.0, 92.0, 90.0)
            }
            DebugDrawingTarget::LocationCardSlotBottomCenterLowerLeft => {
                DebugDrawingRect::new(548.0, 522.0, 92.0, 90.0)
            }
            DebugDrawingTarget::LocationCardSlotBottomCenterLowerRight => {
                DebugDrawingRect::new(640.0, 522.0, 92.0, 90.0)
            }
            DebugDrawingTarget::LocationCardSlotBottomRightUpperLeft => {
                DebugDrawingRect::new(732.0, 432.0, 92.0, 90.0)
            }
            DebugDrawingTarget::LocationCardSlotBottomRightUpperRight => {
                DebugDrawingRect::new(824.0, 432.0, 92.0, 90.0)
            }
            DebugDrawingTarget::LocationCardSlotBottomRightLowerLeft => {
                DebugDrawingRect::new(732.0, 522.0, 92.0, 90.0)
            }
            DebugDrawingTarget::LocationCardSlotBottomRightLowerRight => {
                DebugDrawingRect::new(824.0, 522.0, 92.0, 90.0)
            }
            DebugDrawingTarget::HandArea => DebugDrawingRect::new(360.0, 576.0, 560.0, 208.0),
            DebugDrawingTarget::SingleCardArea => DebugDrawingRect::new(570.0, 604.0, 128.0, 168.0),
        }
    }
}

#[cfg(test)]
mod tests {
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

        assert_eq!(model.requests().len(), 30);
        assert_eq!(
            model
                .request_for(DebugDrawingTarget::SingleCardArea)
                .unwrap()
                .rect,
            DebugDrawingRect::new(570.0, 604.0, 128.0, 168.0)
        );
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
                .request_for(DebugDrawingTarget::LocationCardSlotTopCenterUpperLeft)
                .unwrap()
                .rect,
            DebugDrawingRect::new(548.0, 44.0, 92.0, 90.0)
        );
        assert_eq!(
            model
                .request_for(DebugDrawingTarget::LocationCardSlotTopCenterLowerRight)
                .unwrap()
                .rect,
            DebugDrawingRect::new(640.0, 134.0, 92.0, 90.0)
        );
        assert_eq!(
            model
                .request_for(DebugDrawingTarget::LocationCardSlotTopCenterLowerRight)
                .unwrap()
                .color,
            DebugDrawingColor::blue()
        );
    }
}
