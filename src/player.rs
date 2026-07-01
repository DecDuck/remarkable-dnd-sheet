//! Pure D&D 5e character computation — no QObject, no Pin, no allocations beyond
//! the `Vec` inside `CharacterState`.
//!
//! # Model
//!
//! A character is modelled as an **ordered list of levels** (`Vec<ClassLevel>`).
//! Each entry records which class that level was taken in, and every discrete
//! choice made at that level: subclass selection, ASI/feat, class skills, and
//! an optional custom HP roll.
//!
//! `compute(&state)` is a **pure function** that derives every stat from
//! scratch.  Because no mutable state is threaded through the computation,
//! changing any choice anywhere in the level list requires nothing more than
//! calling `compute` again — no undo stacks, no caches to invalidate.
//!
//! # Key types
//!
//! | Type | Purpose |
//! |------|---------|
//! | `ClassLevel`     | One level in a specific class with its choices     |
//! | `LevelChoice`    | Subclass / ASI-or-feat / class skill / HP roll      |
//! | `AsiChoice`      | What was picked for an ASI slot                    |
//! | `CharacterState` | All inputs: level list + base scores + race/bg data |
//! | `ComputedStats`  | All derived values; ready to push into Q_PROPERTYs  |

use crate::dnd_data::{self, Ability, AbilitySet, Skill, SkillSet};

// ─── Level entry ──────────────────────────────────────────────────────────────

/// One character level gained in a specific class, with every choice attached.
#[derive(Clone, Debug)]
pub struct ClassLevel {
    /// Index into [`dnd_data::CLASSES`].
    pub class_idx: usize,
    /// All choices recorded when this level was taken.
    pub choices: Vec<LevelChoice>,
}

impl ClassLevel {
    pub fn new(class_idx: usize) -> Self {
        Self { class_idx, choices: Vec::new() }
    }

    /// Return the ASI/feat slot choice stored at this level, if any.
    pub fn asi_choice(&self) -> Option<&AsiChoice> {
        self.choices.iter().find_map(|c| {
            if let LevelChoice::AsiSlot(ch) = c { Some(ch) } else { None }
        })
    }

    /// The subclass chosen at this level, if any.
    pub fn subclass(&self) -> Option<usize> {
        self.choices.iter().find_map(|c| {
            if let LevelChoice::Subclass(si) = c { Some(*si) } else { None }
        })
    }

    /// HP roll stored for this level (`None` → use average `hd/2+1`).
    pub fn hp_roll(&self) -> Option<u8> {
        self.choices.iter().find_map(|c| {
            if let LevelChoice::HpRoll(r) = c { Some(*r) } else { None }
        })
    }

    /// Class skills picked at this level.
    pub fn class_skills(&self) -> impl Iterator<Item = Skill> + '_ {
        self.choices.iter().filter_map(|c| {
            if let LevelChoice::ClassSkill(sk) = c { Some(*sk) } else { None }
        })
    }
}

/// A discrete choice recorded when a character gains a level.
#[derive(Clone, Debug)]
pub enum LevelChoice {
    /// Subclass selected at this level.  Only the first `Subclass` choice per
    /// class is honoured by `compute`; duplicates are silently ignored.
    Subclass(usize),

    /// An ASI/feat slot used at this level.
    AsiSlot(AsiChoice),

    /// A class skill proficiency chosen at level 1 of this class.
    ClassSkill(Skill),

    /// Custom HP roll for this level (1..=hit_die).
    /// If absent, `hd / 2 + 1` (the "take average" rule) is used.
    HpRoll(u8),

    /// A choice made for a `ChoiceFeature` at this level.
    /// `choice_set_idx`: index into `ClassData::class_choices` for this class.
    /// `feature_idx`: index within `ClassChoiceSet::features`.
    /// `option_idx`: which option was selected.
    /// Multiple entries with the same `(choice_set_idx, feature_idx)` represent
    /// multi-pick selections (e.g. choosing 2 Metamagic options).
    FeaturePick { choice_set_idx: u8, feature_idx: u8, option_idx: u8 },
}

/// What a character chose for an ASI/feat slot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AsiChoice {
    /// +2 to one ability (`ability2 = None`) or +1 to two abilities.
    Scores {
        ability1: Ability,
        /// `None` → ability1 gets +2.  `Some(a)` → ability1 and `a` each get +1.
        ability2: Option<Ability>,
    },
    /// Took a feat (index into [`dnd_data::FEATS`]).
    Feat(usize),
}

/// Encode an `AsiChoice` as the QML-compatible integer:
///   -2  = Scores chosen,  ≥0 = feat index.
pub fn asi_choice_to_slot_value(ch: &AsiChoice) -> i32 {
    match ch {
        AsiChoice::Scores { .. } => -2,
        AsiChoice::Feat(fi) => *fi as i32,
    }
}

// ─── Input state ─────────────────────────────────────────────────────────────

/// All raw inputs needed to derive a character's stats.
///
/// Build by pushing levels via [`CharacterState::add_level`], then modify
/// individual entries to record choices.  Pass a reference to [`compute`] to
/// get [`ComputedStats`].
#[derive(Clone, Debug, Default)]
pub struct CharacterState {
    /// All levels gained in order (index 0 = character level 1, max 20).
    /// Each entry records the class taken and every choice made.
    pub levels: Vec<ClassLevel>,

    /// Base ability scores **before** race bonuses or ASI/feat bonuses.
    /// Rolled or point-buy values: [STR, DEX, CON, INT, WIS, CHA].
    pub base_scores: [i32; 6],

    /// Racial ability-score bonuses applied when a race is selected.
    pub race_asi: [i8; 6],

    /// Skill proficiencies granted by the chosen background.
    pub background_skill_profs: SkillSet,

    /// Extra skill proficiencies (Half-Elf bonus, Prodigy feat, etc.).
    pub extra_skill_profs: SkillSet,

    /// Extra saving-throw proficiencies beyond class grants and Resilient feats.
    pub extra_save_profs: AbilitySet,

    /// Player choices for racial traits that require selection (e.g. Draconic Ancestry).
    /// Each entry: `(trait_idx, option_idx)` where `trait_idx` is an index into
    /// the selected race's `race_traits` slice.
    pub race_feature_picks: Vec<(u8, u8)>,
}

impl CharacterState {
    /// Total character level (number of levels in `levels`).
    pub fn total_level(&self) -> usize {
        self.levels.len()
    }

    /// Index of the primary (first-taken) class, or `None` if no levels.
    pub fn primary_class_idx(&self) -> Option<usize> {
        self.levels.first().map(|l| l.class_idx)
    }

    /// Number of levels taken in a specific class.
    pub fn class_level_count(&self, class_idx: usize) -> usize {
        self.levels.iter().filter(|l| l.class_idx == class_idx).count()
    }

    /// Append a level in `class_idx`.  Returns the new total level, or `None`
    /// if already at the maximum of 20.
    pub fn add_level(&mut self, class_idx: usize) -> Option<usize> {
        if self.levels.len() >= 20 {
            return None;
        }
        self.levels.push(ClassLevel::new(class_idx));
        Some(self.levels.len())
    }

    /// Remove and return the last level, or `None` if already empty.
    pub fn remove_last_level(&mut self) -> Option<ClassLevel> {
        self.levels.pop()
    }

    /// Set (or clear) the ASI/feat choice at a specific 1-based character level.
    ///
    /// Any existing `AsiSlot` choice at that level is replaced.
    pub fn set_asi_choice(&mut self, level: usize, choice: Option<AsiChoice>) {
        if let Some(entry) = self.levels.get_mut(level.saturating_sub(1)) {
            entry.choices.retain(|c| !matches!(c, LevelChoice::AsiSlot(_)));
            if let Some(ch) = choice {
                entry.choices.push(LevelChoice::AsiSlot(ch));
            }
        }
    }

    /// Set the subclass choice at a specific 1-based character level.
    pub fn set_subclass(&mut self, level: usize, subclass_idx: usize) {
        if let Some(entry) = self.levels.get_mut(level.saturating_sub(1)) {
            entry.choices.retain(|c| !matches!(c, LevelChoice::Subclass(_)));
            entry.choices.push(LevelChoice::Subclass(subclass_idx));
        }
    }

    /// Toggle a class skill at a specific 1-based character level.
    ///
    /// Returns `true` if added, `false` if removed.
    pub fn toggle_class_skill(&mut self, level: usize, skill: Skill) -> bool {
        if let Some(entry) = self.levels.get_mut(level.saturating_sub(1)) {
            let pos = entry.choices.iter().position(|c| {
                matches!(c, LevelChoice::ClassSkill(s) if *s == skill)
            });
            if let Some(i) = pos {
                entry.choices.remove(i);
                false
            } else {
                entry.choices.push(LevelChoice::ClassSkill(skill));
                true
            }
        } else {
            false
        }
    }

    /// Set (or clear with `roll = 0`) the HP roll for a specific 1-based level.
    pub fn set_hp_roll(&mut self, level: usize, roll: u8) {
        if let Some(entry) = self.levels.get_mut(level.saturating_sub(1)) {
            entry.choices.retain(|c| !matches!(c, LevelChoice::HpRoll(_)));
            if roll > 0 {
                entry.choices.push(LevelChoice::HpRoll(roll));
            }
        }
    }

    /// Return the 1-based character level at which ASI slot `slot` (0-indexed)
    /// opens for the primary class.  Returns `None` if the slot doesn't exist
    /// or the character isn't high enough level.
    pub fn asi_slot_character_level(&self, slot: usize) -> Option<usize> {
        let class_idx = self.primary_class_idx()?;
        let cls = dnd_data::class(class_idx)?;
        let char_level = *cls.asi_levels.get(slot)? as usize;
        if char_level <= self.levels.len() { Some(char_level) } else { None }
    }
}

// ─── Computed output ──────────────────────────────────────────────────────────

/// Every stat derived from a [`CharacterState`].  All integer fields are ready
/// to push directly into `i32` Q_PROPERTYs.
#[derive(Clone, Debug)]
pub struct ComputedStats {
    // ── Core ─────────────────────────────────────────────────────────────────
    pub total_level: i32,
    pub prof_bonus: i32,

    /// Final ability scores (base + race + all ASI/feat bonuses), clamped 1-30.
    pub final_scores: [i32; 6],
    pub ability_mods: [i32; 6],

    // ── Proficiencies ─────────────────────────────────────────────────────────
    pub save_profs: AbilitySet,
    pub skill_profs: SkillSet,

    // ── Bonuses ───────────────────────────────────────────────────────────────
    pub skill_bonuses: [i32; 18],
    pub save_bonuses: [i32; 6],
    pub initiative: i32,
    pub passive_perception: i32,
    pub passive_investigation: i32,

    // ── HP ────────────────────────────────────────────────────────────────────
    pub max_hp: i32,
    /// Hit die of the primary class (for display and short-rest recovery).
    pub hit_die: i32,

    // ── Spellcasting ──────────────────────────────────────────────────────────
    /// 0-5 (Ability index), -1 if no spellcasting.
    pub spellcasting_ability_idx: i32,
    pub spell_save_dc: i32,
    pub spell_attack_bonus: i32,

    // ── Selection display helpers ─────────────────────────────────────────────
    pub primary_subclass_idx: i32,
    /// Skills chosen as class skill profs at level 1 of the primary class.
    pub class_skill_choices: SkillSet,
    /// Slot values for the primary class's ASI/feat slots.
    /// -1 = unset, -2 = Scores chosen, ≥0 = feat index.
    pub feat_slot_values: [i32; 7],

    // ── Feat flags ────────────────────────────────────────────────────────────
    pub resilient_saves: AbilitySet,
    pub has_alert: bool,
    pub has_observant: bool,
    pub has_tough: bool,
    pub has_war_caster: bool,
    pub has_lucky: bool,
    pub has_sentinel: bool,
    pub has_polearm_master: bool,
    pub has_great_weapon_master: bool,
    pub has_sharpshooter: bool,
    pub has_mobile: bool,
    pub mobile_speed_bonus: i32,
}

impl Default for ComputedStats {
    fn default() -> Self {
        Self {
            total_level: 0,
            prof_bonus: 2,
            final_scores: [10; 6],
            ability_mods: [0; 6],
            save_profs: AbilitySet::new(),
            skill_profs: SkillSet::new(),
            skill_bonuses: [0; 18],
            save_bonuses: [0; 6],
            initiative: 0,
            passive_perception: 10,
            passive_investigation: 10,
            max_hp: 1,
            hit_die: 8,
            spellcasting_ability_idx: -1,
            spell_save_dc: 0,
            spell_attack_bonus: 0,
            primary_subclass_idx: -1,
            class_skill_choices: SkillSet::new(),
            feat_slot_values: [-1; 7],
            resilient_saves: AbilitySet::new(),
            has_alert: false,
            has_observant: false,
            has_tough: false,
            has_war_caster: false,
            has_lucky: false,
            has_sentinel: false,
            has_polearm_master: false,
            has_great_weapon_master: false,
            has_sharpshooter: false,
            has_mobile: false,
            mobile_speed_bonus: 0,
        }
    }
}

// ─── Computation ─────────────────────────────────────────────────────────────

/// Derive all stats from a `CharacterState`.  Pure function — no side-effects.
/// Call after every change to `CharacterState`.
pub fn compute(s: &CharacterState) -> ComputedStats {
    let total_level = s.levels.len() as i32;

    // No levels yet: return scored defaults (scores/mods are still valid).
    if total_level == 0 {
        let final_scores: [i32; 6] =
            core::array::from_fn(|i| (s.base_scores[i] + s.race_asi[i] as i32).clamp(1, 30));
        let ability_mods: [i32; 6] =
            core::array::from_fn(|i| (final_scores[i] - 10).div_euclid(2));
        return ComputedStats { final_scores, ability_mods, ..Default::default() };
    }

    let prof_bonus = (total_level - 1) / 4 + 2;

    // ── Scan all levels: collect subclasses, ASI bonuses, feat list, class skills
    let mut subclass_per_class = [None::<usize>; 12]; // indexed by class_idx (max 12)
    let mut asi_score_bonuses = [0i32; 6];
    let mut feat_indices: Vec<usize> = Vec::new();
    let mut class_skill_choices = SkillSet::new(); // primary-class skills only (for display)
    let primary_ci = s.levels[0].class_idx;

    for entry in &s.levels {
        let ci = entry.class_idx.min(11);
        for choice in &entry.choices {
            match choice {
                LevelChoice::Subclass(si) => {
                    subclass_per_class[ci].get_or_insert(*si);
                }
                LevelChoice::AsiSlot(AsiChoice::Scores { ability1, ability2 }) => {
                    asi_score_bonuses[*ability1 as usize] += if ability2.is_some() { 1 } else { 2 };
                    if let Some(ab2) = ability2 {
                        asi_score_bonuses[*ab2 as usize] += 1;
                    }
                }
                LevelChoice::AsiSlot(AsiChoice::Feat(fi)) => {
                    feat_indices.push(*fi);
                }
                LevelChoice::ClassSkill(sk) => {
                    if entry.class_idx == primary_ci {
                        class_skill_choices = SkillSet(class_skill_choices.0 | (1u32 << *sk as u32));
                    }
                }
                LevelChoice::HpRoll(_) => {}
                LevelChoice::FeaturePick { .. } => {}
            }
        }
    }

    // ── Process feats ─────────────────────────────────────────────────────────
    let mut feat_score_bonuses = [0i32; 6];
    let mut initiative_bonus = 0i32;
    let mut passive_bonus = 0i32;
    let mut resilient_saves = AbilitySet::new();
    let (mut has_alert, mut has_observant, mut has_tough, mut has_war_caster) =
        (false, false, false, false);
    let (mut has_lucky, mut has_sentinel, mut has_polearm_master) = (false, false, false);
    let (mut has_great_weapon_master, mut has_sharpshooter, mut has_mobile) =
        (false, false, false);

    for &fi in &feat_indices {
        if let Some(f) = dnd_data::feat(fi) {
            for (i, &b) in f.asi.iter().enumerate() {
                feat_score_bonuses[i] += b as i32;
            }
            match f.name {
                "Alert"               => { has_alert = true;  initiative_bonus += 5; }
                "Observant"           => { has_observant = true; passive_bonus += 5; }
                "Tough"               => has_tough = true,
                "Mobile"              => has_mobile = true,
                "War Caster"          => has_war_caster = true,
                "Lucky"               => has_lucky = true,
                "Sentinel"            => has_sentinel = true,
                "Polearm Master"      => has_polearm_master = true,
                "Great Weapon Master" => has_great_weapon_master = true,
                "Sharpshooter"        => has_sharpshooter = true,
                "Resilient (STR)"     => resilient_saves = resilient_saves.set(Ability::Str),
                "Resilient (DEX)"     => resilient_saves = resilient_saves.set(Ability::Dex),
                "Resilient (CON)"     => resilient_saves = resilient_saves.set(Ability::Con),
                "Resilient (INT)"     => resilient_saves = resilient_saves.set(Ability::Int),
                "Resilient (WIS)"     => resilient_saves = resilient_saves.set(Ability::Wis),
                "Resilient (CHA)"     => resilient_saves = resilient_saves.set(Ability::Cha),
                _                     => {}
            }
        }
    }

    // ── Final scores and mods ─────────────────────────────────────────────────
    let final_scores: [i32; 6] = core::array::from_fn(|i| {
        (s.base_scores[i]
            + s.race_asi[i] as i32
            + asi_score_bonuses[i]
            + feat_score_bonuses[i])
            .clamp(1, 30)
    });
    let ability_mods: [i32; 6] =
        core::array::from_fn(|i| (final_scores[i] - 10).div_euclid(2));

    // ── Save proficiencies ────────────────────────────────────────────────────
    // PHB multiclass rule: save profs from the FIRST class taken only.
    let primary_class_saves = dnd_data::class(primary_ci)
        .map(|c| c.save_profs)
        .unwrap_or(AbilitySet::new());
    let save_profs = AbilitySet(
        primary_class_saves.0 | resilient_saves.0 | s.extra_save_profs.0,
    );

    let save_bonuses: [i32; 6] = core::array::from_fn(|i| {
        let ab = Ability::from_index(i as u8).unwrap_or(Ability::Str);
        ability_mods[i] + if save_profs.contains(ab) { prof_bonus } else { 0 }
    });

    // ── Skill proficiencies ───────────────────────────────────────────────────
    // Collect class skills from ALL levels (multiclass support).
    let mut all_class_skills = SkillSet::new();
    for entry in &s.levels {
        for c in &entry.choices {
            if let LevelChoice::ClassSkill(sk) = c {
                all_class_skills = SkillSet(all_class_skills.0 | (1u32 << *sk as u32));
            }
        }
    }
    let skill_profs = SkillSet(
        all_class_skills.0 | s.background_skill_profs.0 | s.extra_skill_profs.0,
    );

    let skill_bonuses: [i32; 18] = core::array::from_fn(|i| {
        let sk = Skill::from_index(i as u8).unwrap_or(Skill::Acrobatics);
        ability_mods[sk.ability() as usize]
            + if skill_profs.contains_index(i as u8) { prof_bonus } else { 0 }
    });

    // ── Initiative and passives ───────────────────────────────────────────────
    let initiative = ability_mods[Ability::Dex as usize] + initiative_bonus;

    let perc_prof = skill_profs.contains_index(Skill::Perception as u8) as i32;
    let passive_perception =
        10 + ability_mods[Ability::Wis as usize] + perc_prof * prof_bonus + passive_bonus;

    let inv_prof = skill_profs.contains_index(Skill::Investigation as u8) as i32;
    let passive_investigation =
        10 + ability_mods[Ability::Int as usize] + inv_prof * prof_bonus + passive_bonus;

    // ── Spellcasting ──────────────────────────────────────────────────────────
    let primary_class_name = dnd_data::class(primary_ci).map(|c| c.name);
    let primary_subclass_name: Option<&str> = subclass_per_class[primary_ci.min(11)]
        .and_then(|si| dnd_data::class(primary_ci)?.subclasses.get(si).copied());

    let spellcasting_ability_idx: i32 = match primary_class_name {
        Some("Bard") | Some("Paladin") | Some("Sorcerer") | Some("Warlock") => Ability::Cha as i32,
        Some("Cleric") | Some("Druid") | Some("Ranger") | Some("Monk") => Ability::Wis as i32,
        Some("Wizard") => Ability::Int as i32,
        Some("Fighter") => match primary_subclass_name {
            Some("Eldritch Knight") => Ability::Int as i32,
            _ => -1,
        },
        Some("Rogue") => match primary_subclass_name {
            Some("Arcane Trickster") => Ability::Int as i32,
            _ => -1,
        },
        _ => -1,
    };

    let (spell_save_dc, spell_attack_bonus) = if spellcasting_ability_idx >= 0 {
        let m = ability_mods[spellcasting_ability_idx as usize];
        (8 + prof_bonus + m, prof_bonus + m)
    } else {
        (0, 0)
    };

    // ── Max HP ────────────────────────────────────────────────────────────────
    // Level 1 of the character always gets the maximum hit die of that class.
    // Each subsequent level uses the stored roll or the average (hd/2+1).
    // CON modifier is added at each level.  Tough feat: +2 × total_level.
    let con_mod = ability_mods[Ability::Con as usize];
    let tough_bonus = if has_tough { 2 * total_level } else { 0 };

    let max_hp = (s
        .levels
        .iter()
        .enumerate()
        .map(|(idx, entry)| {
            let hd = dnd_data::class(entry.class_idx)
                .map(|c| c.hit_die as i32)
                .unwrap_or(8);
            let roll = if idx == 0 {
                hd // level 1: always max
            } else {
                entry.hp_roll().map(|r| r as i32).unwrap_or(hd / 2 + 1)
            };
            (roll + con_mod).max(1)
        })
        .sum::<i32>()
        + tough_bonus)
        .max(1);

    let hit_die = dnd_data::class(primary_ci)
        .map(|c| c.hit_die as i32)
        .unwrap_or(8);

    // ── Selection display helpers ─────────────────────────────────────────────
    let primary_subclass_idx = subclass_per_class[primary_ci.min(11)]
        .map(|si| si as i32)
        .unwrap_or(-1);

    // feat_slot_values: for each ASI slot of the primary class, find the
    // choice (if any) in the level entry at that character level.
    let feat_slot_values: [i32; 7] = {
        let mut vals = [-1i32; 7];
        if let Some(cls) = dnd_data::class(primary_ci) {
            for (slot, &asi_level) in cls.asi_levels.iter().enumerate().take(7) {
                let idx = (asi_level as usize).saturating_sub(1);
                if let Some(entry) = s.levels.get(idx) {
                    vals[slot] = entry
                        .asi_choice()
                        .map(asi_choice_to_slot_value)
                        .unwrap_or(-1);
                }
            }
        }
        vals
    };

    ComputedStats {
        total_level,
        prof_bonus,
        final_scores,
        ability_mods,
        save_profs,
        skill_profs,
        skill_bonuses,
        save_bonuses,
        initiative,
        passive_perception,
        passive_investigation,
        max_hp,
        hit_die,
        spellcasting_ability_idx,
        spell_save_dc,
        spell_attack_bonus,
        primary_subclass_idx,
        class_skill_choices,
        feat_slot_values,
        resilient_saves,
        has_alert,
        has_observant,
        has_tough,
        has_war_caster,
        has_lucky,
        has_sentinel,
        has_polearm_master,
        has_great_weapon_master,
        has_sharpshooter,
        has_mobile,
        mobile_speed_bonus: if has_mobile { 10 } else { 0 },
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dnd_data::{Ability, Skill, SkillSet};

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn state(levels: usize, class_idx: usize, scores: [i32; 6]) -> CharacterState {
        let mut s = CharacterState::default();
        s.base_scores = scores;
        for _ in 0..levels { s.add_level(class_idx); }
        s
    }

    fn base_state() -> CharacterState { state(1, 0, [10; 6]) } // Barbarian
    fn fighter_state(level: usize, scores: [i32; 6]) -> CharacterState {
        state(level, 4, scores) // Fighter = class 4
    }

    fn add_feat(s: &mut CharacterState, level_idx: usize, feat_idx: usize) {
        if let Some(e) = s.levels.get_mut(level_idx) {
            e.choices.push(LevelChoice::AsiSlot(AsiChoice::Feat(feat_idx)));
        }
    }
    fn add_asi(s: &mut CharacterState, level_idx: usize, ab1: Ability, ab2: Option<Ability>) {
        if let Some(e) = s.levels.get_mut(level_idx) {
            e.choices.push(LevelChoice::AsiSlot(AsiChoice::Scores { ability1: ab1, ability2: ab2 }));
        }
    }

    // ── Proficiency bonus ─────────────────────────────────────────────────────

    #[test]
    fn prof_bonus_all_levels() {
        let expected = [
            (1,2),(2,2),(3,2),(4,2),(5,3),(6,3),(7,3),(8,3),
            (9,4),(10,4),(11,4),(12,4),(13,5),(14,5),(15,5),(16,5),
            (17,6),(18,6),(19,6),(20,6),
        ];
        for (level, pb) in expected {
            assert_eq!(compute(&state(level, 0, [10;6])).prof_bonus, pb, "level {level}");
        }
    }

    // ── Ability modifiers ─────────────────────────────────────────────────────

    #[test]
    fn ability_mods_full_range() {
        let cases = [
            (1,-5),(2,-4),(3,-4),(4,-3),(5,-3),(6,-2),(7,-2),(8,-1),(9,-1),
            (10,0),(11,0),(12,1),(13,1),(14,2),(15,2),(16,3),(17,3),
            (18,4),(19,4),(20,5),(24,7),(30,10),
        ];
        for (score, expected) in cases {
            let mut s = base_state();
            s.base_scores = [score; 6];
            for m in compute(&s).ability_mods { assert_eq!(m, expected, "score={score}"); }
        }
    }

    #[test]
    fn final_scores_race_plus_asi() {
        let mut s = CharacterState::default();
        s.base_scores = [10; 6];
        s.race_asi = [2, 0, 1, 0, 0, 0]; // +2 STR, +1 CON
        for _ in 0..4 { s.add_level(4); } // Fighter 4
        add_asi(&mut s, 3, Ability::Str, None); // +2 STR at level 4
        let c = compute(&s);
        assert_eq!(c.final_scores[Ability::Str as usize], 14); // 10+2+2
        assert_eq!(c.final_scores[Ability::Con as usize], 11); // 10+1
    }

    // ── Skills ────────────────────────────────────────────────────────────────

    #[test]
    fn skill_no_prof() {
        let mut s = base_state();
        s.base_scores[0] = 16; // STR +3
        assert_eq!(compute(&s).skill_bonuses[Skill::Athletics as usize], 3);
    }

    #[test]
    fn skill_class_prof() {
        let mut s = base_state();
        s.base_scores[0] = 16;
        s.levels[0].choices.push(LevelChoice::ClassSkill(Skill::Athletics));
        assert_eq!(compute(&s).skill_bonuses[Skill::Athletics as usize], 5); // 3+2
    }

    #[test]
    fn skill_background_prof() {
        let mut s = base_state();
        s.base_scores[0] = 16;
        s.background_skill_profs = SkillSet::new().set(Skill::Athletics);
        assert_eq!(compute(&s).skill_bonuses[Skill::Athletics as usize], 5);
    }

    #[test]
    fn skill_high_level_prof() {
        let mut s = state(17, 0, [10;6]); // prof +6
        s.base_scores[Ability::Dex as usize] = 20; // +5
        s.background_skill_profs = SkillSet::new().set(Skill::Stealth);
        assert_eq!(compute(&s).skill_bonuses[Skill::Stealth as usize], 11); // 5+6
    }

    // ── Saves ─────────────────────────────────────────────────────────────────

    #[test]
    fn save_not_proficient() {
        // Fighter: STR+CON saves. WIS not proficient.
        let s = fighter_state(1, [10,10,14,10,14,10]);
        assert_eq!(compute(&s).save_bonuses[Ability::Wis as usize], 2); // mod only
    }

    #[test]
    fn save_class_prof() {
        // Fighter CON save, level 5 → prof +3, CON 14 → +2. Total = 5.
        let s = fighter_state(5, [10,10,14,10,10,10]);
        assert_eq!(compute(&s).save_bonuses[Ability::Con as usize], 5);
    }

    #[test]
    fn save_resilient_feat() {
        // Resilient(WIS) = feat 39. Fighter 5 doesn't have WIS prof normally.
        let mut s = fighter_state(5, [10,10,10,10,10,10]);
        add_feat(&mut s, 0, 39);
        let c = compute(&s);
        assert!(c.resilient_saves.contains(Ability::Wis));
        assert_eq!(c.save_bonuses[Ability::Wis as usize], 3); // 0+3
    }

    #[test]
    fn save_resilient_all_six() {
        let mut s = base_state();
        for fi in [35,36,37,38,39,40] { add_feat(&mut s, 0, fi); }
        let c = compute(&s);
        for ab in Ability::ALL {
            assert!(c.resilient_saves.contains(ab));
            assert_eq!(c.save_bonuses[ab as usize], 2);
        }
    }

    #[test]
    fn multiclass_save_profs_primary_only() {
        let mut s = CharacterState::default();
        s.base_scores = [10;6];
        s.add_level(4); // Fighter: STR+CON
        s.add_level(2); // Cleric: WIS+CHA  (should NOT be granted)
        let c = compute(&s);
        assert!(c.save_profs.contains(Ability::Str));
        assert!(c.save_profs.contains(Ability::Con));
        assert!(!c.save_profs.contains(Ability::Wis));
        assert!(!c.save_profs.contains(Ability::Cha));
    }

    // ── Initiative / passives ─────────────────────────────────────────────────

    #[test]
    fn initiative_base() {
        let mut s = base_state();
        s.base_scores[Ability::Dex as usize] = 14;
        assert_eq!(compute(&s).initiative, 2);
    }

    #[test]
    fn initiative_alert_feat() {
        let mut s = base_state();
        s.base_scores[Ability::Dex as usize] = 14;
        add_feat(&mut s, 0, 0); // Alert
        let c = compute(&s);
        assert_eq!(c.initiative, 7); // 2+5
        assert!(c.has_alert);
    }

    #[test]
    fn passive_perception_no_prof() {
        let mut s = base_state();
        s.base_scores[Ability::Wis as usize] = 12;
        assert_eq!(compute(&s).passive_perception, 11); // 10+1
    }

    #[test]
    fn passive_perception_with_prof() {
        let mut s = state(5, 0, [10;6]);
        s.base_scores[Ability::Wis as usize] = 14;
        s.background_skill_profs = SkillSet::new().set(Skill::Perception);
        assert_eq!(compute(&s).passive_perception, 15); // 10+2+3
    }

    #[test]
    fn passive_observant() {
        let mut s = base_state(); // Observant = feat 33
        add_feat(&mut s, 0, 33);
        let c = compute(&s);
        assert_eq!(c.passive_perception, 15);
        assert_eq!(c.passive_investigation, 15);
        assert!(c.has_observant);
    }

    // ── Spellcasting ──────────────────────────────────────────────────────────

    #[test]
    fn spell_no_levels() {
        assert_eq!(compute(&CharacterState::default()).spellcasting_ability_idx, -1);
    }

    #[test]
    fn spell_wizard_int() {
        let mut s = state(1, 11, [10,10,10,18,10,10]); // Wizard, INT 18
        let c = compute(&s);
        assert_eq!(c.spellcasting_ability_idx, Ability::Int as i32);
        assert_eq!(c.spell_save_dc, 14);     // 8+2+4
        assert_eq!(c.spell_attack_bonus, 6); // 2+4
    }

    #[test]
    fn spell_cha_classes() {
        for ci in [1, 6, 9, 10] { // Bard, Paladin, Sorcerer, Warlock
            assert_eq!(compute(&state(1, ci, [10;6])).spellcasting_ability_idx,
                Ability::Cha as i32, "class {ci}");
        }
    }

    #[test]
    fn spell_fighter_no_subclass() {
        assert_eq!(compute(&fighter_state(5, [10;6])).spellcasting_ability_idx, -1);
    }

    #[test]
    fn spell_eldritch_knight() {
        let mut s = fighter_state(5, [10,10,10,16,10,10]); // INT 16
        s.levels[0].choices.push(LevelChoice::Subclass(2)); // EK
        let c = compute(&s);
        assert_eq!(c.spellcasting_ability_idx, Ability::Int as i32);
        assert_eq!(c.spell_save_dc, 14); // 8+3+3
    }

    #[test]
    fn spell_arcane_trickster() {
        let mut s = state(5, 8, [10,10,10,16,10,10]); // Rogue, INT 16
        s.levels[0].choices.push(LevelChoice::Subclass(0)); // AT
        assert_eq!(compute(&s).spellcasting_ability_idx, Ability::Int as i32);
    }

    #[test]
    fn spell_thief_none() {
        let mut s = state(5, 8, [10;6]);
        s.levels[0].choices.push(LevelChoice::Subclass(2)); // Thief
        assert_eq!(compute(&s).spellcasting_ability_idx, -1);
    }

    // ── Max HP ────────────────────────────────────────────────────────────────

    #[test]
    fn hp_level_1_fighter() {
        // d10 + CON(+2) = 12
        let s = fighter_state(1, [15,13,14,10,11,12]);
        assert_eq!(compute(&s).max_hp, 12);
    }

    #[test]
    fn hp_level_5_fighter() {
        // Level1: 10+2=12. Levels2-5: 4×(5+1+2)=32. Total=44.
        let s = fighter_state(5, [15,13,14,10,11,12]);
        assert_eq!(compute(&s).max_hp, 44);
    }

    #[test]
    fn hp_custom_rolls() {
        let mut s = fighter_state(3, [10;6]); // CON mod 0
        s.set_hp_roll(2, 8);
        s.set_hp_roll(3, 2);
        assert_eq!(compute(&s).max_hp, 20); // 10+8+2
    }

    #[test]
    fn hp_minimum_per_level() {
        // d6 Wizard, CON 1 (-5), level 3. Each level min 1.
        let mut s = state(3, 11, [10;6]);
        s.base_scores[Ability::Con as usize] = 1;
        assert_eq!(compute(&s).max_hp, 3);
    }

    #[test]
    fn hp_tough_feat() {
        // Fighter5 without Tough = 44. Tough adds 2×5=10 → 54.
        let mut s = fighter_state(5, [15,13,14,10,11,12]);
        add_feat(&mut s, 0, 50); // Tough
        assert_eq!(compute(&s).max_hp, 54);
        assert!(compute(&s).has_tough);
    }

    #[test]
    fn hp_multiclass_uses_each_class_hd() {
        // Fighter d10 + Wizard d6. Level1: 10, Level2: 6/2+1=4. Total=14.
        let mut s = CharacterState::default();
        s.base_scores = [10;6];
        s.add_level(4);  // Fighter d10
        s.add_level(11); // Wizard d6
        assert_eq!(compute(&s).max_hp, 14);
    }

    // ── ASI bonuses ───────────────────────────────────────────────────────────

    #[test]
    fn asi_plus_two() {
        let mut s = fighter_state(4, [14,10,10,10,10,10]);
        add_asi(&mut s, 3, Ability::Str, None); // +2 STR
        assert_eq!(compute(&s).final_scores[Ability::Str as usize], 16);
        assert_eq!(compute(&s).ability_mods[Ability::Str as usize], 3);
    }

    #[test]
    fn asi_split_plus_one() {
        let mut s = fighter_state(4, [14,14,10,10,10,10]);
        add_asi(&mut s, 3, Ability::Str, Some(Ability::Dex));
        let c = compute(&s);
        assert_eq!(c.final_scores[Ability::Str as usize], 15);
        assert_eq!(c.final_scores[Ability::Dex as usize], 15);
    }

    #[test]
    fn feat_score_bonus_applied() {
        // Durable (feat 8): +1 CON
        let mut s = base_state();
        s.base_scores[Ability::Con as usize] = 14;
        add_feat(&mut s, 0, 8);
        assert_eq!(compute(&s).final_scores[Ability::Con as usize], 15); // 14+1
    }

    // ── feat_slot_values ──────────────────────────────────────────────────────

    #[test]
    fn feat_slot_values_mapping() {
        // Fighter asi_levels = [4,6,8,12,14,16,19].
        // Slot0=level4=idx3, Slot1=level6=idx5.
        let mut s = fighter_state(8, [10;6]);
        add_asi(&mut s, 3, Ability::Str, None); // slot 0
        add_feat(&mut s, 5, 50);                // slot 1: Tough
        let c = compute(&s);
        assert_eq!(c.feat_slot_values[0], -2); // Scores
        assert_eq!(c.feat_slot_values[1], 50); // Feat index
        assert_eq!(c.feat_slot_values[2], -1); // unset (level 8 = idx 7 exists, no choice)
    }

    // ── Integration ───────────────────────────────────────────────────────────

    #[test]
    fn integration_ek_fighter_level8() {
        let mut s = CharacterState::default();
        s.base_scores = [16,14,16,18,12,10];
        for _ in 0..8 { s.add_level(4); } // Fighter 8
        s.levels[0].choices.push(LevelChoice::Subclass(2)); // EK
        s.background_skill_profs = SkillSet::new()
            .set(Skill::Athletics).set(Skill::Perception);
        add_feat(&mut s, 3, 39); // Slot0 (lvl4): Resilient(WIS)
        add_feat(&mut s, 5, 51); // Slot1 (lvl6): War Caster
        let c = compute(&s);
        assert_eq!(c.prof_bonus, 3);
        assert_eq!(c.spellcasting_ability_idx, Ability::Int as i32);
        assert_eq!(c.spell_save_dc, 15);
        assert_eq!(c.spell_attack_bonus, 7);
        assert_eq!(c.save_bonuses[Ability::Str as usize], 6); // 3+3 Fighter prof
        assert_eq!(c.save_bonuses[Ability::Con as usize], 6);
        assert_eq!(c.save_bonuses[Ability::Wis as usize], 4); // 1+3 Resilient
        assert_eq!(c.save_bonuses[Ability::Dex as usize], 2); // 2 mod only
        assert_eq!(c.skill_bonuses[Skill::Athletics as usize], 6); // 3+3
        assert_eq!(c.skill_bonuses[Skill::Perception as usize], 4); // 1+3
        assert_eq!(c.initiative, 2);
        assert_eq!(c.passive_perception, 14); // 10+1+3
        assert_eq!(c.max_hp, 76); // 13 + 7×9
        assert!(c.has_war_caster);
        assert!(c.resilient_saves.contains(Ability::Wis));
    }
}
