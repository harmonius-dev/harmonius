//! Gesture-driven UI interactions (pan, swipe, pinch).

use crate::types::Vec2;

/// Phase of a continuous gesture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GesturePhase {
    /// Gesture recognized and started.
    Began,
    /// Gesture updated.
    Changed,
    /// Gesture finished normally.
    Ended,
    /// Gesture cancelled by the platform.
    Cancelled,
}

/// Swipe direction for drag initiation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SwipeDirection {
    /// Swipe upward.
    Up,
    /// Swipe downward.
    Down,
    /// Swipe left.
    Left,
    /// Swipe right.
    Right,
}

/// Gesture classification.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GestureType {
    /// No-op placeholder.
    Tap,
    /// Two-finger pan.
    Pan,
    /// Directional swipe.
    Swipe {
        /// Swipe direction.
        direction: SwipeDirection,
    },
    /// Pinch zoom.
    Pinch,
}

/// Gesture payload delivered to UI systems.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GestureEvent {
    /// Gesture kind.
    pub gesture_type: GestureType,
    /// Gesture phase.
    pub phase: GesturePhase,
    /// Anchor position in logical pixels.
    pub position: Vec2,
    /// Multiplicative scale delta for pinch gestures.
    pub scale: f32,
    /// Pan delta in logical pixels.
    pub delta: Vec2,
}

/// Simple scroll model for pan + pinch mapping tests.
#[derive(Clone, Debug, PartialEq)]
pub struct ScrollView {
    /// Scroll translation in logical pixels.
    pub offset: Vec2,
    /// Content scale multiplier.
    pub content_scale: f32,
    /// Minimum allowed zoom.
    pub min_zoom: f32,
    /// Maximum allowed zoom.
    pub max_zoom: f32,
    /// Pinch focal anchor in panel-local pixels.
    pub focal_point: Vec2,
    /// Whether a pinch gesture is active.
    pub zoom_active: bool,
    /// Snapshot of `content_scale` at pinch began.
    pinch_began_scale: f32,
}

impl ScrollView {
    /// Creates a scroll view with sane zoom clamps.
    pub fn new(min_zoom: f32, max_zoom: f32) -> Self {
        Self {
            offset: Vec2::new(0.0, 0.0),
            content_scale: 1.0,
            min_zoom,
            max_zoom,
            focal_point: Vec2::new(0.0, 0.0),
            zoom_active: false,
            pinch_began_scale: 1.0,
        }
    }

    /// Applies a gesture to this scroll view.
    pub fn apply_gesture(&mut self, gesture: GestureEvent) {
        match gesture.gesture_type {
            GestureType::Pan => {
                if gesture.phase == GesturePhase::Changed || gesture.phase == GesturePhase::Began {
                    self.offset = Vec2::new(
                        self.offset.x + gesture.delta.x,
                        self.offset.y + gesture.delta.y,
                    );
                }
            }
            GestureType::Pinch => {
                self.focal_point = gesture.position;
                match gesture.phase {
                    GesturePhase::Began => {
                        self.zoom_active = true;
                        self.pinch_began_scale = self.content_scale;
                    }
                    GesturePhase::Changed => {
                        self.content_scale = (self.content_scale * gesture.scale)
                            .clamp(self.min_zoom, self.max_zoom);
                    }
                    GesturePhase::Ended => {
                        self.zoom_active = false;
                    }
                    GesturePhase::Cancelled => {
                        self.content_scale = self.pinch_began_scale;
                        self.zoom_active = false;
                    }
                }
            }
            _ => {}
        }
    }
}

/// Drag initiation tracker for swipe gestures.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DragDropManager {
    /// Last initiated drag direction, if any.
    pub last_drag: Option<SwipeDirection>,
}

impl DragDropManager {
    /// Starts a drag when a swipe gesture is recognized.
    pub fn on_swipe(&mut self, gesture: GestureEvent) {
        if let GestureType::Swipe { direction } = gesture.gesture_type {
            if gesture.phase == GesturePhase::Began {
                self.last_drag = Some(direction);
            }
        }
    }
}
