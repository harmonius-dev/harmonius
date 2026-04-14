//! ECS-style command buffer for structural changes (dismemberment).

/// Structural animation command.
#[derive(Clone, Debug, PartialEq)]
pub enum EcsCommand {
    /// Sever a limb subtree.
    Sever {
        /// Owning entity.
        entity: u64,
        /// Limb identifier.
        limb: u32,
    },
}

/// Command queue processed into world mutations.
#[derive(Clone, Debug, Default)]
pub struct CommandBuffer {
    cmds: Vec<EcsCommand>,
}

impl CommandBuffer {
    /// Pushes a command (preferred over direct world mutation).
    pub fn push(&mut self, c: EcsCommand) {
        self.cmds.push(c);
    }

    /// Drains all commands.
    pub fn drain(&mut self) -> Vec<EcsCommand> {
        std::mem::take(&mut self.cmds)
    }
}
