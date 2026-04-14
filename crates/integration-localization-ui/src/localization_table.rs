//! In-memory localization table with resolve + fallback counters.
//!
//! The integration design diagram shows sorted maps and a table-owned current locale; this crate
//! keeps a locale argument on `resolve` and uses `BTreeMap` for deterministic test ordering.

use std::collections::BTreeMap;

use crate::format::format_message;
use crate::types::{
    ArgValue, FallbackCounters, FontChainResolver, LocalizedStringId, LocaleId, MessageTemplate,
    ResolvedText, ScriptTag, TextDirection,
};

/// Named interpolation arguments for `resolve`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResolveArgs {
    pairs: Vec<(String, ArgValue)>,
}

impl ResolveArgs {
    /// Empty args.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds or replaces `key`.
    pub fn insert(&mut self, key: impl Into<String>, value: ArgValue) {
        let key = key.into();
        if let Some((_, v)) = self.pairs.iter_mut().find(|(k, _)| *k == key) {
            *v = value;
            return;
        }
        self.pairs.push((key, value));
    }

    /// Borrows the internal pairs slice.
    #[must_use]
    pub fn pairs(&self) -> &[(String, ArgValue)] {
        &self.pairs
    }
}

/// Fake localization table backing integration tests.
#[derive(Clone, Debug)]
pub struct LocalizationTable {
    locales: BTreeMap<LocaleId, BTreeMap<LocalizedStringId, MessageTemplate>>,
    fallback: LocaleId,
    /// Diagnostic counters.
    pub counters: FallbackCounters,
    resolver: FontChainResolver,
}

impl LocalizationTable {
    /// Creates a table with the given fallback locale.
    #[must_use]
    pub fn new(fallback: LocaleId) -> Self {
        Self {
            locales: BTreeMap::new(),
            fallback,
            counters: FallbackCounters::default(),
            resolver: FontChainResolver,
        }
    }

    /// Inserts a template for `(locale, id)`.
    pub fn insert_template(
        &mut self,
        locale: LocaleId,
        id: LocalizedStringId,
        template: MessageTemplate,
    ) {
        self.locales.entry(locale).or_default().insert(id, template);
    }

    /// Resolves `id` for `locale` using `args`.
    #[must_use]
    pub fn resolve(
        &mut self,
        id: LocalizedStringId,
        locale: LocaleId,
        args: &ResolveArgs,
    ) -> ResolvedText {
        let template = self
            .locales
            .get(&locale)
            .and_then(|m| m.get(&id))
            .or_else(|| {
                self.locales
                    .get(&self.fallback)
                    .and_then(|m| m.get(&id))
                    .inspect(|_| {
                        self.counters.fm1 += 1;
                    })
            });

        let Some(template) = template else {
            self.counters.fm2 += 1;
            return ResolvedText {
                display: format!("[missing:{}]", id.0),
                direction: TextDirection::Ltr,
                script: ScriptTag::LATN,
                fonts: self.resolver.resolve(ScriptTag::LATN),
            };
        };

        let display = format_message(
            &template.pattern,
            args.pairs(),
            &locale.0,
            &mut self.counters,
        );
        let direction = resolve_auto_direction(template.direction, &display);
        let fonts = self.resolver.resolve(template.script);
        ResolvedText {
            display,
            direction,
            script: template.script,
            fonts,
        }
    }
}

fn resolve_auto_direction(template_dir: TextDirection, text: &str) -> TextDirection {
    match template_dir {
        TextDirection::Auto => first_strong_direction(text).unwrap_or(TextDirection::Ltr),
        other => other,
    }
}

fn first_strong_direction(text: &str) -> Option<TextDirection> {
    for ch in text.chars() {
        if ('\u{0590}'..='\u{05FF}').contains(&ch) || ('\u{0600}'..='\u{06FF}').contains(&ch) {
            return Some(TextDirection::Rtl);
        }
        if ch.is_ascii_alphabetic() {
            return Some(TextDirection::Ltr);
        }
    }
    None
}
