//! Client-side prediction scaffolding.

use crate::entity::Entity;
use crate::input::InputFrame;

/// Length of the rolling input buffer stored on the predictor.
pub const INPUT_BUFFER_LEN: usize = 32;

/// Applies local inputs immediately against a predicted physics body.
#[derive(Debug)]
pub struct ClientPredictor {
    /// Entity whose physics state is predicted locally.
    pub predicted_entity: Entity,
    input_buffer: [Option<InputFrame>; INPUT_BUFFER_LEN],
    input_head: usize,
    input_len: usize,
    /// Last acknowledged server tick.
    pub last_server_tick: u64,
}

impl ClientPredictor {
    /// Constructs a predictor targeting `predicted_entity`.
    #[must_use]
    pub fn new(predicted_entity: Entity) -> Self {
        Self {
            predicted_entity,
            input_buffer: std::array::from_fn(|_| None),
            input_head: 0,
            input_len: 0,
            last_server_tick: 0,
        }
    }

    /// Records `input` and forwards it to `integrate`, which should advance local physics.
    pub fn apply_input(&mut self, input: InputFrame, mut integrate: impl FnMut(&InputFrame)) {
        integrate(&input);
        if self.input_len < INPUT_BUFFER_LEN {
            let idx = (self.input_head + self.input_len) % INPUT_BUFFER_LEN;
            self.input_buffer[idx] = Some(input);
            self.input_len += 1;
        } else {
            self.input_buffer[self.input_head] = Some(input);
            self.input_head = (self.input_head + 1) % INPUT_BUFFER_LEN;
        }
        self.last_server_tick = input.tick;
    }

    /// Returns recorded inputs in tick order (oldest first) for rollback replay.
    #[must_use]
    pub fn input_history(&self) -> Vec<InputFrame> {
        let mut out = Vec::with_capacity(self.input_len);
        for i in 0..self.input_len {
            let idx = (self.input_head + i) % INPUT_BUFFER_LEN;
            if let Some(frame) = self.input_buffer[idx] {
                out.push(frame);
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec3;

    #[test]
    fn tc_ir_4_5_2_1_prediction_applies_local_input() {
        let mut pos = Vec3::ZERO;
        let mut pred = ClientPredictor::new(Entity::new(1, 0));
        let input = InputFrame {
            tick: 7,
            move_x: 1.0,
            move_y: 0.0,
            move_z: 0.0,
        };
        pred.apply_input(input, |inp| {
            pos += Vec3::new(inp.move_x, inp.move_y, inp.move_z);
        });
        assert_eq!(pos, Vec3::X);
        assert_eq!(pred.last_server_tick, 7);
    }
}
