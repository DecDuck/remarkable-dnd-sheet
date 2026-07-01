//! Zero-cost typed wrappers for D&D 5e concepts.
//!
//! All types have the same memory layout as the plain integer types they
//! replace — no boxing, no heap allocation, no vtables.
//!
//! Skill indices follow the PHB alphabetical order used throughout the
//! project (QML `skillNames`, `skillProfs`/`classSkillChoices` bitmasks):
//!
//!   0  Acrobatics        (DEX)    9  Medicine          (WIS)
//!   1  Animal Handling   (WIS)   10  Nature            (INT)
//!   2  Arcana            (INT)   11  Perception        (WIS)
//!   3  Athletics         (STR)   12  Performance       (CHA)
//!   4  Deception         (CHA)   13  Persuasion        (CHA)
//!   5  History           (INT)   14  Religion          (INT)
//!   6  Insight           (WIS)   15  Sleight of Hand   (DEX)
//!   7  Intimidation      (CHA)   16  Stealth           (DEX)
//!   8  Investigation     (INT)   17  Survival          (WIS)

// ─── Ability ─────────────────────────────────────────────────────────────────

/// The six core ability scores. `repr(u8)` so they can be used as array
/// indices and bit positions with zero runtime cost.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ability {
    Str = 0,
    Dex = 1,
    Con = 2,
    Int = 3,
    Wis = 4,
    Cha = 5,
}

impl Ability {
    pub const ALL: [Ability; 6] = [
        Ability::Str, Ability::Dex, Ability::Con,
        Ability::Int, Ability::Wis, Ability::Cha,
    ];

    pub const fn abbrev(self) -> &'static str {
        match self {
            Ability::Str => "STR",
            Ability::Dex => "DEX",
            Ability::Con => "CON",
            Ability::Int => "INT",
            Ability::Wis => "WIS",
            Ability::Cha => "CHA",
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Ability::Str => "Strength",
            Ability::Dex => "Dexterity",
            Ability::Con => "Constitution",
            Ability::Int => "Intelligence",
            Ability::Wis => "Wisdom",
            Ability::Cha => "Charisma",
        }
    }

    /// Construct from a 0-based index. Returns `None` if out of range.
    pub const fn from_index(i: u8) -> Option<Self> {
        match i {
            0 => Some(Ability::Str),
            1 => Some(Ability::Dex),
            2 => Some(Ability::Con),
            3 => Some(Ability::Int),
            4 => Some(Ability::Wis),
            5 => Some(Ability::Cha),
            _ => None,
        }
    }
}

// ─── Skill ───────────────────────────────────────────────────────────────────

/// The 18 PHB skills in alphabetical order. `repr(u8)` so variants double as
/// bit positions in [`SkillSet`] and array indices.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Skill {
    Acrobatics     =  0,
    AnimalHandling =  1,
    Arcana         =  2,
    Athletics      =  3,
    Deception      =  4,
    History        =  5,
    Insight        =  6,
    Intimidation   =  7,
    Investigation  =  8,
    Medicine       =  9,
    Nature         = 10,
    Perception     = 11,
    Performance    = 12,
    Persuasion     = 13,
    Religion       = 14,
    SleightOfHand  = 15,
    Stealth        = 16,
    Survival       = 17,
}

impl Skill {
    /// The ability that governs this skill.
    pub const fn ability(self) -> Ability {
        match self {
            Skill::Athletics => Ability::Str,
            Skill::Acrobatics | Skill::SleightOfHand | Skill::Stealth => Ability::Dex,
            Skill::AnimalHandling | Skill::Insight | Skill::Medicine
            | Skill::Perception   | Skill::Survival => Ability::Wis,
            Skill::Arcana | Skill::History | Skill::Investigation
            | Skill::Nature | Skill::Religion => Ability::Int,
            Skill::Deception | Skill::Intimidation | Skill::Performance
            | Skill::Persuasion => Ability::Cha,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Skill::Acrobatics     => "Acrobatics",
            Skill::AnimalHandling => "Animal Handling",
            Skill::Arcana         => "Arcana",
            Skill::Athletics      => "Athletics",
            Skill::Deception      => "Deception",
            Skill::History        => "History",
            Skill::Insight        => "Insight",
            Skill::Intimidation   => "Intimidation",
            Skill::Investigation  => "Investigation",
            Skill::Medicine       => "Medicine",
            Skill::Nature         => "Nature",
            Skill::Perception     => "Perception",
            Skill::Performance    => "Performance",
            Skill::Persuasion     => "Persuasion",
            Skill::Religion       => "Religion",
            Skill::SleightOfHand  => "Sleight of Hand",
            Skill::Stealth        => "Stealth",
            Skill::Survival       => "Survival",
        }
    }

    /// Construct from a 0-based index. Returns `None` if `i >= 18`.
    pub const fn from_index(i: u8) -> Option<Self> {
        match i {
            0  => Some(Skill::Acrobatics),
            1  => Some(Skill::AnimalHandling),
            2  => Some(Skill::Arcana),
            3  => Some(Skill::Athletics),
            4  => Some(Skill::Deception),
            5  => Some(Skill::History),
            6  => Some(Skill::Insight),
            7  => Some(Skill::Intimidation),
            8  => Some(Skill::Investigation),
            9  => Some(Skill::Medicine),
            10 => Some(Skill::Nature),
            11 => Some(Skill::Perception),
            12 => Some(Skill::Performance),
            13 => Some(Skill::Persuasion),
            14 => Some(Skill::Religion),
            15 => Some(Skill::SleightOfHand),
            16 => Some(Skill::Stealth),
            17 => Some(Skill::Survival),
            _  => None,
        }
    }
}

// ─── AbilitySet ───────────────────────────────────────────────────────────────

/// A set of abilities stored in 6 bits of a `u8`. Identical memory footprint
/// to the raw `u8` bitmask it replaces — the type system enforces correct use
/// at zero runtime cost.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct AbilitySet(pub u8);

impl AbilitySet {
    pub const fn new() -> Self { Self(0) }

    /// Return a new set with `ab` added (usable in `const` contexts).
    pub const fn set(self, ab: Ability) -> Self {
        Self(self.0 | (1 << ab as u8))
    }

    pub fn contains(self, ab: Ability) -> bool {
        self.0 & (1 << ab as u8) != 0
    }

    /// Raw bits for QML / C++ interop (maps 1:1 to the old `u8` bitmask).
    pub const fn bits(self) -> u8 { self.0 }

    pub fn iter(self) -> impl Iterator<Item = Ability> {
        Ability::ALL.into_iter().filter(move |&ab| self.contains(ab))
    }
}

// ─── SkillSet ─────────────────────────────────────────────────────────────────

/// A set of skills stored in 18 bits of a `u32`. Same memory layout as the
/// old `u32` bitmask — just typed.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct SkillSet(pub u32);

impl SkillSet {
    pub const fn new() -> Self { Self(0) }

    /// Return a new set with `sk` added (usable in `const` contexts).
    pub const fn set(self, sk: Skill) -> Self {
        Self(self.0 | (1 << sk as u32))
    }

    pub fn contains(self, sk: Skill) -> bool {
        self.0 & (1 << sk as u32) != 0
    }

    pub fn contains_index(self, idx: u8) -> bool {
        idx < 18 && self.0 & (1 << idx) != 0
    }

    /// Raw bits for QML / C++ interop (maps 1:1 to the old `u32` bitmask).
    pub const fn bits(self) -> u32 { self.0 }

    pub fn iter(self) -> impl Iterator<Item = Skill> {
        (0u8..18).filter_map(move |i| {
            if self.contains_index(i) { Skill::from_index(i) } else { None }
        })
    }
}

// ─── AsiBonus ─────────────────────────────────────────────────────────────────

/// Ability-score improvements: `[STR, DEX, CON, INT, WIS, CHA]` as signed
/// bytes. Same memory layout as `[i8; 6]`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AsiBonus(pub [i8; 6]);

impl AsiBonus {
    pub const ZERO: Self = Self([0; 6]);

    pub fn iter(&self) -> core::slice::Iter<i8> {
        self.0.iter()
    }
}

impl Default for AsiBonus {
    fn default() -> Self { Self::ZERO }
}

/// Index by `Ability` — e.g. `feat.asi[Ability::Con]`.
impl core::ops::Index<Ability> for AsiBonus {
    type Output = i8;
    fn index(&self, ab: Ability) -> &i8 {
        &self.0[ab as usize]
    }
}

/// Index by raw `usize` — backward-compatible with the old `[i8;6]` access
/// pattern used in charsheet.rs.
impl core::ops::Index<usize> for AsiBonus {
    type Output = i8;
    fn index(&self, i: usize) -> &i8 {
        &self.0[i]
    }
}

// ─── Data structs ─────────────────────────────────────────────────────────────

/// A single named feature granted at a class level (PHB description included).
pub struct LevelFeature {
    pub name: &'static str,
    pub description: &'static str,
}

/// One selectable option within a `ChoiceFeature` (e.g. a specific Fighting Style).
pub struct FeatureOption {
    pub name: &'static str,
    pub description: &'static str,
}

/// A feature that requires the player to pick one or more options from a list.
/// Used for both class choices (tied to a class level) and race choices.
pub struct ChoiceFeature {
    pub name: &'static str,
    pub description: &'static str,
    /// All available options to choose from.
    pub options: &'static [FeatureOption],
    /// How many options the player must pick (e.g. 1 for Fighting Style, 2 for Metamagic L3).
    pub num_picks: u8,
}

/// A group of `ChoiceFeature`s that become available at a specific class level.
pub struct ClassChoiceSet {
    /// 1-based class level at which these choices become available.
    pub class_level: u8,
    pub features: &'static [ChoiceFeature],
}

/// A racial trait — either purely informational or requiring a player choice.
pub struct RaceTrait {
    pub name: &'static str,
    pub description: &'static str,
    /// Empty = no choice required.
    pub options: &'static [FeatureOption],
    /// 0 = informational only; >0 = must pick this many options.
    pub num_picks: u8,
}

pub struct ClassData {
    pub name: &'static str,
    /// Hit die size (6, 8, 10, or 12).
    pub hit_die: u8,
    /// Saving-throw proficiencies.
    pub save_profs: AbilitySet,
    /// Skills available to choose from at character creation.
    pub skill_pool: SkillSet,
    /// Number of skills to pick from `skill_pool`.
    pub skill_choices: u8,
    /// Official subclass names (PHB + Xanathar's + Tasha's).
    pub subclasses: &'static [&'static str],
    /// Character levels at which an ASI-or-feat slot opens.
    pub asi_levels: &'static [u8],
    /// The class level (1-based) at which the subclass is chosen.
    pub subclass_level: u8,
    /// PHB features gained at each class level.
    /// `level_features[i]` lists the features gained at class level `i+1`.
    pub level_features: &'static [&'static [LevelFeature]],
    /// Player-facing choices grouped by the class level at which they're made.
    pub class_choices: &'static [ClassChoiceSet],
}

pub struct RaceData {
    pub name: &'static str,
    /// Racial ability-score improvements.
    pub asi: AsiBonus,
    /// Base walking speed in feet.
    pub speed: u8,
    /// Short trait summary (kept for backwards compatibility).
    pub traits: &'static str,
    /// Structured racial traits (informational + choice-bearing).
    pub race_traits: &'static [RaceTrait],
}

pub struct BackgroundData {
    pub name: &'static str,
    /// Skill proficiencies granted.
    pub skill_profs: SkillSet,
}

pub struct FeatData {
    pub name: &'static str,
    /// Score bonus applied when this feat is selected (`ZERO` for most feats).
    pub asi: AsiBonus,
    pub description: &'static str,
}
