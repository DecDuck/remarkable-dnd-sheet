//! D&D 5e static content — PHB + Xanathar's Guide + Tasha's Cauldron.
//!
//! Module layout:
//!   `types`          — zero-cost enums and bitset wrappers + data structs
//!   `class_features` — PHB feature tables for all 12 classes
//!   `class_choices`  — player-facing choice data (Fighting Style, Metamagic, etc.)
//!   `race_features`  — structured racial traits + choice options for all 31 races
//!   `classes`        — `CLASSES: &[ClassData]`
//!   `races`          — `RACES: &[RaceData]`
//!   `backgrounds`    — `BACKGROUNDS: &[BackgroundData]`
//!   `feats`          — `FEATS: &[FeatData]`

pub mod types;
pub mod class_features;
pub mod class_choices;
pub mod race_features;
pub mod classes;
pub mod races;
pub mod backgrounds;
pub mod feats;

// Re-export everything so callers can just `use crate::dnd_data::*;`.
pub use types::*;
pub use classes::CLASSES;
pub use races::RACES;
pub use backgrounds::BACKGROUNDS;
pub use feats::FEATS;

// ── Accessor functions ────────────────────────────────────────────────────────

pub fn num_classes() -> usize { CLASSES.len() }
pub fn num_races() -> usize { RACES.len() }
pub fn num_backgrounds() -> usize { BACKGROUNDS.len() }
pub fn num_feats() -> usize { FEATS.len() }

pub fn class(idx: usize) -> Option<&'static ClassData> { CLASSES.get(idx) }
pub fn race(idx: usize) -> Option<&'static RaceData> { RACES.get(idx) }
pub fn background(idx: usize) -> Option<&'static BackgroundData> { BACKGROUNDS.get(idx) }
pub fn feat(idx: usize) -> Option<&'static FeatData> { FEATS.get(idx) }
