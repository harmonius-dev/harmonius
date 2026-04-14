//! Button / key glyph resolution for tutorials and prompts.

use crate::actions::{ActionBinding, ActionId, InputSource};
use crate::device::{GamepadButton, Scancode};
use crate::ids::GlyphAtlasId;

/// Target controller / keyboard family for atlas selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceFamily {
    /// Desktop keyboard prompts.
    Keyboard,
    /// Xbox layout glyphs.
    Xbox,
    /// PlayStation layout glyphs.
    PlayStation,
    /// Nintendo Switch Pro layout.
    SwitchPro,
    /// Unknown gamepad.
    Generic,
}

/// Resolved sprite reference in an atlas.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedGlyph {
    /// Atlas handle.
    pub atlas_id: GlyphAtlasId,
    /// Sprite row inside the atlas.
    pub sprite_index: u32,
    /// Stable debug label (tests match on this string).
    pub label: String,
}

/// Resolves `ActionBinding` sources into human-facing glyph metadata.
#[derive(Clone, Debug, Default)]
pub struct GlyphResolver {
    /// Cached `(action, family)` → glyph.
    cache: Vec<((ActionId, DeviceFamily), ResolvedGlyph)>,
}

impl GlyphResolver {
    /// Resolve glyph for `action_id` using the concrete `binding` and active device family.
    pub fn resolve(
        &mut self,
        action_id: ActionId,
        binding: &ActionBinding,
        active_family: DeviceFamily,
    ) -> ResolvedGlyph {
        let key = (action_id, active_family);
        if let Some((_, g)) = self.cache.iter().find(|(k, _)| *k == key) {
            return g.clone();
        }
        let g = match (&binding.source, active_family) {
            (InputSource::GamepadButton(GamepadButton::South), DeviceFamily::Xbox) => ResolvedGlyph {
                atlas_id: GlyphAtlasId(1),
                sprite_index: 10,
                label: "xbox_a".into(),
            },
            (InputSource::Key(Scancode::Space), DeviceFamily::Keyboard) => ResolvedGlyph {
                atlas_id: GlyphAtlasId(0),
                sprite_index: 2,
                label: "key_space".into(),
            },
            (_, DeviceFamily::Keyboard) => ResolvedGlyph {
                atlas_id: GlyphAtlasId(0),
                sprite_index: 0,
                label: "key_generic".into(),
            },
            (_, DeviceFamily::Xbox) => ResolvedGlyph {
                atlas_id: GlyphAtlasId(1),
                sprite_index: 0,
                label: "xbox_generic".into(),
            },
            _ => ResolvedGlyph {
                atlas_id: GlyphAtlasId(2),
                sprite_index: 0,
                label: "generic".into(),
            },
        };
        self.cache.push((key, g.clone()));
        g
    }

    /// Drop cached entries when bindings or atlases change.
    pub fn invalidate(&mut self) {
        self.cache.clear();
    }
}
