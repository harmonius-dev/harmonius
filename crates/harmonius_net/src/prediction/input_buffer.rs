//! Redundant input windows for loss recovery.

/// Last-K inputs carried on each packet (newest last).
#[derive(Clone, Debug, Default)]
pub struct InputRing {
    buf: Vec<(u32, i32)>,
    cap: usize,
}

impl InputRing {
    /// Ring with capacity `cap` (typically 3).
    pub fn new(cap: usize) -> Self {
        Self {
            buf: Vec::new(),
            cap,
        }
    }

    /// Push one tick's input.
    pub fn push(&mut self, tick: u32, value: i32) {
        self.buf.push((tick, value));
        if self.buf.len() > self.cap {
            self.buf.remove(0);
        }
    }

    /// Serialize last up to `cap` as redundant payload.
    pub fn redundant_window(&self) -> &[(u32, i32)] {
        &self.buf
    }

    /// Recover missing tick if present in a later redundant window.
    pub fn recover_missing(server_ticks: &[(u32, &[(u32, i32)])], missing: u32) -> Option<i32> {
        for &(_pkt, win) in server_ticks {
            for &(t, v) in win {
                if t == missing {
                    return Some(v);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.4.2.1 — later packet carries missing tick in redundant tail.
    #[test]
    fn test_input_buffer_redundancy() {
        let mut a = InputRing::new(3);
        a.push(49, 490);
        a.push(50, 500);
        a.push(51, 510);
        let w51 = a.redundant_window();
        let server = vec![(51u32, w51)];
        assert_eq!(InputRing::recover_missing(&server, 50), Some(500));
    }
}
