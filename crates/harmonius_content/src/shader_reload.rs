//! Shader PSO reload simulation.

/// Compile or overlay errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShaderCompileError {
    /// Human-readable message.
    pub message: String,
}

/// Tracks PSO versions per shader id.
#[derive(Debug, Default)]
pub struct ShaderReloader {
    /// Current good PSO generation per shader.
    pub pso_generation: Vec<u32>,
    /// Versions queued for retirement after tick.
    pub retirement_queue: Vec<u32>,
    /// Last overlay error.
    pub last_overlay: Option<ShaderCompileError>,
}

impl ShaderReloader {
    /// New reloader with `n` shader slots.
    pub fn new(n: usize) -> Self {
        Self {
            pso_generation: vec![1; n],
            retirement_queue: Vec::new(),
            last_overlay: None,
        }
    }

    /// Successful recompile bumps generation and queues old.
    pub fn recompile_ok(&mut self, shader_id: usize) {
        let g = self.pso_generation[shader_id];
        self.retirement_queue.push(g);
        self.pso_generation[shader_id] = g + 1;
        self.last_overlay = None;
    }

    /// Failed compile leaves PSO unchanged and sets overlay.
    pub fn recompile_err(&mut self, shader_id: usize, msg: impl Into<String>) {
        let _ = shader_id;
        self.last_overlay = Some(ShaderCompileError {
            message: msg.into(),
        });
    }

    /// Frame boundary: move retirements.
    pub fn frame_boundary_tick(&mut self) {
        self.retirement_queue.clear();
    }
}
