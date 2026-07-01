//! D&D 5e character sheet QObject implemented in Rust.
//!
//! All character state is stored here as Q_PROPERTYs. QML reads the properties
//! for display and calls the invokable methods to mutate them. Derived values
//! (ability modifier, proficiency bonus, skill/save bonuses) are computed in
//! QML JavaScript to stay reactive without extra Rust invocables.
//!
//! Selection invocables (selectClass, selectRace, selectBackground, selectFeat,
//! setSlotAsi) automatically calculate dependent stats so the UI always reflects
//! the correct PHB values.
#![allow(non_snake_case)]

#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        // ── Identity ──────────────────────────────────────────────────────
        #[qproperty(QString, name)]
        #[qproperty(QString, charClass)]
        #[qproperty(QString, race)]
        #[qproperty(QString, background)]
        #[qproperty(QString, alignment)]
        #[qproperty(i32, level)]
        #[qproperty(i32, xp)]
        // ── Ability scores ────────────────────────────────────────────────
        #[qproperty(i32, strScore)]
        #[qproperty(i32, dexScore)]
        #[qproperty(i32, conScore)]
        #[qproperty(i32, intScore)]
        #[qproperty(i32, wisScore)]
        #[qproperty(i32, chaScore)]
        // ── Hit points ────────────────────────────────────────────────────
        #[qproperty(i32, maxHp)]
        #[qproperty(i32, currentHp)]
        #[qproperty(i32, tempHp)]
        // ── Combat ────────────────────────────────────────────────────────
        #[qproperty(i32, armorClass)]
        #[qproperty(i32, speed)]
        // ── Misc ──────────────────────────────────────────────────────────
        #[qproperty(bool, inspiration)]
        #[qproperty(i32, deathSuccess)]
        #[qproperty(i32, deathFail)]
        // ── Proficiencies (bitmasks) ───────────────────────────────────────
        //  saveProfs:  bit i = saving-throw prof for ability i (0=STR…5=CHA)
        //  skillProfs: bit i = skill prof (PHB alphabetical order, 0..17)
        #[qproperty(i32, saveProfs)]
        #[qproperty(i32, skillProfs)]
        // ── Hit dice ──────────────────────────────────────────────────────
        #[qproperty(i32, hitDie)]
        #[qproperty(i32, hitDiceUsed)]
        // ── Selection indices (-1 = none selected) ────────────────────────
        #[qproperty(i32, classIdx)]
        #[qproperty(i32, raceIdx)]
        #[qproperty(i32, backgroundIdx)]
        #[qproperty(i32, subclassIdx)]
        // ── Class skill choices (bitmask of which class-pool skills were picked)
        #[qproperty(i32, classSkillChoices)]
        // ── ASI / Feat slots (-1=unset, -2=ASI chosen, >=0=feat index) ────
        // Fighter has 7 ASI slots; all other classes have 5.
        // Unused slots always report -1.
        #[qproperty(i32, featSlot0)]
        #[qproperty(i32, featSlot1)]
        #[qproperty(i32, featSlot2)]
        #[qproperty(i32, featSlot3)]
        #[qproperty(i32, featSlot4)]
        #[qproperty(i32, featSlot5)]
        #[qproperty(i32, featSlot6)]
        // ── Auto HP recalc ────────────────────────────────────────────────
        // When true, mutation methods automatically recalculate maxHp.
        #[qproperty(bool, autoHp)]
        // ── Computed / derived stats (updated by recalcDerived) ────────────
        #[qproperty(i32, profBonus)]
        #[qproperty(i32, strMod)]
        #[qproperty(i32, dexMod)]
        #[qproperty(i32, conMod)]
        #[qproperty(i32, intMod)]
        #[qproperty(i32, wisMod)]
        #[qproperty(i32, chaMod)]
        #[qproperty(i32, initiative)]
        #[qproperty(i32, passivePerception)]
        #[qproperty(i32, passiveInvestigation)]
        /// 0-5 (Ability index), -1 if no spellcasting for this class/subclass.
        #[qproperty(i32, spellcastingAbilityIdx)]
        #[qproperty(i32, spellSaveDc)]
        #[qproperty(i32, spellAttackBonus)]
        /// Increments on every choice/selection mutation so QML bindings that call
        /// invocables can depend on this to get re-evaluated.
        #[qproperty(i32, selectionVersion)]
        type CharacterSheet = super::CharacterSheetRust;

        #[qinvokable]
        #[cxx_name = "adjustHp"]
        fn adjust_hp(self: Pin<&mut Self>, delta: i32);

        #[qinvokable]
        #[cxx_name = "adjustMaxHp"]
        fn adjust_max_hp(self: Pin<&mut Self>, delta: i32);

        #[qinvokable]
        #[cxx_name = "adjustTempHp"]
        fn adjust_temp_hp(self: Pin<&mut Self>, delta: i32);

        #[qinvokable]
        #[cxx_name = "adjustScore"]
        fn adjust_score(self: Pin<&mut Self>, ability: i32, delta: i32);

        #[qinvokable]
        #[cxx_name = "adjustLevel"]
        fn adjust_level(self: Pin<&mut Self>, delta: i32);

        #[qinvokable]
        #[cxx_name = "adjustAc"]
        fn adjust_ac(self: Pin<&mut Self>, delta: i32);

        #[qinvokable]
        #[cxx_name = "adjustSpeed"]
        fn adjust_speed(self: Pin<&mut Self>, delta: i32);

        #[qinvokable]
        #[cxx_name = "adjustXp"]
        fn adjust_xp(self: Pin<&mut Self>, delta: i32);

        #[qinvokable]
        #[cxx_name = "toggleSaveProf"]
        fn toggle_save_prof(self: Pin<&mut Self>, ability: i32);

        #[qinvokable]
        #[cxx_name = "toggleSkillProf"]
        fn toggle_skill_prof(self: Pin<&mut Self>, skill: i32);

        #[qinvokable]
        #[cxx_name = "toggleInspiration"]
        fn toggle_inspiration(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "adjustDeathSuccess"]
        fn adjust_death_success(self: Pin<&mut Self>, delta: i32);

        #[qinvokable]
        #[cxx_name = "adjustDeathFail"]
        fn adjust_death_fail(self: Pin<&mut Self>, delta: i32);

        #[qinvokable]
        #[cxx_name = "cycleHitDie"]
        fn cycle_hit_die(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "adjustHitDiceUsed"]
        fn adjust_hit_dice_used(self: Pin<&mut Self>, delta: i32);

        /// Long rest: restore all HP, clear death saves, recover all hit dice.
        #[qinvokable]
        #[cxx_name = "longRest"]
        fn long_rest(self: Pin<&mut Self>);

        /// Short rest: recover half of total hit dice (rounded up, min 1).
        #[qinvokable]
        #[cxx_name = "shortRest"]
        fn short_rest(self: Pin<&mut Self>);

        // ── Selection / auto-calc invocables ──────────────────────────────

        /// Set class by index into dnd_data::CLASSES (-1 = clear).
        /// Updates hitDie and saveProfs. Calls recalcHp if autoHp.
        #[qinvokable]
        #[cxx_name = "selectClass"]
        fn select_class(self: Pin<&mut Self>, idx: i32);

        /// Set race by index into dnd_data::RACES (-1 = clear).
        /// Undoes the previous race's ASI delta, applies the new one.
        /// Updates speed.
        #[qinvokable]
        #[cxx_name = "selectRace"]
        fn select_race(self: Pin<&mut Self>, idx: i32);

        /// Set background by index into dnd_data::BACKGROUNDS (-1 = clear).
        /// ORs background skill_profs into skillProfs.
        /// Note: previous background's skills are not removed (user-controlled).
        #[qinvokable]
        #[cxx_name = "selectBackground"]
        fn select_background(self: Pin<&mut Self>, idx: i32);

        /// Set subclass index for the current class.
        #[qinvokable]
        #[cxx_name = "selectSubclass"]
        fn select_subclass(self: Pin<&mut Self>, idx: i32);

        /// Toggle a skill in classSkillChoices for the current class.
        /// Enforces pool membership and max-choices count.
        /// Recomputes skillProfs = classSkillChoices | (skillProfs & !classPool).
        #[qinvokable]
        #[cxx_name = "toggleClassSkill"]
        fn toggle_class_skill(self: Pin<&mut Self>, skill: i32);

        /// Set feat slot (0-6) to a feat by index into dnd_data::FEATS.
        /// Undoes previously applied ASI for this slot, applies new feat ASI.
        #[qinvokable]
        #[cxx_name = "selectFeat"]
        fn select_feat(self: Pin<&mut Self>, slot: i32, feat_idx: i32);

        /// Set feat slot to an ASI choice instead of a feat.
        /// ability1 receives +2 (if ability2 == -1) or +1 (if ability2 >= 0).
        /// ability2, when >= 0, also receives +1.
        /// Undoes any previously applied ASI for this slot.
        #[qinvokable]
        #[cxx_name = "setSlotAsi"]
        fn set_slot_asi(self: Pin<&mut Self>, slot: i32, ability1: i32, ability2: i32);

        /// Clear an ASI/feat slot, undoing any score bonus it applied.
        #[qinvokable]
        #[cxx_name = "clearFeatSlot"]
        fn clear_feat_slot(self: Pin<&mut Self>, slot: i32);

        /// Recalculate maxHp from class hit die, level, and CON modifier.
        /// Formula: (hitDie + CON_mod) + (hitDie/2 + 1 + CON_mod) × (level − 1).
        /// Clamps currentHp to new maxHp.
        #[qinvokable]
        #[cxx_name = "recalcHp"]
        fn recalc_hp(self: Pin<&mut Self>);

        // ── Query invocables (for QML lists / pickers) ────────────────────

        #[qinvokable]
        #[cxx_name = "numClasses"]
        fn num_classes(self: Pin<&mut Self>) -> i32;

        #[qinvokable]
        #[cxx_name = "className"]
        fn class_name(self: Pin<&mut Self>, idx: i32) -> QString;

        #[qinvokable]
        #[cxx_name = "numRaces"]
        fn num_races(self: Pin<&mut Self>) -> i32;

        #[qinvokable]
        #[cxx_name = "raceName"]
        fn race_name(self: Pin<&mut Self>, idx: i32) -> QString;

        #[qinvokable]
        #[cxx_name = "numBackgrounds"]
        fn num_backgrounds(self: Pin<&mut Self>) -> i32;

        #[qinvokable]
        #[cxx_name = "backgroundName"]
        fn background_name(self: Pin<&mut Self>, idx: i32) -> QString;

        #[qinvokable]
        #[cxx_name = "numFeats"]
        fn num_feats(self: Pin<&mut Self>) -> i32;

        #[qinvokable]
        #[cxx_name = "featName"]
        fn feat_name(self: Pin<&mut Self>, idx: i32) -> QString;

        #[qinvokable]
        #[cxx_name = "featDescription"]
        fn feat_description(self: Pin<&mut Self>, idx: i32) -> QString;

        /// Skill pool bitmask for the currently selected class (0 if no class).
        #[qinvokable]
        #[cxx_name = "classSkillPool"]
        fn class_skill_pool(self: Pin<&mut Self>) -> i32;

        /// Number of skills to choose at creation for the current class.
        #[qinvokable]
        #[cxx_name = "classSkillCount"]
        fn class_skill_count(self: Pin<&mut Self>) -> i32;

        /// Number of subclasses for the currently selected class.
        #[qinvokable]
        #[cxx_name = "numSubclasses"]
        fn num_subclasses(self: Pin<&mut Self>) -> i32;

        #[qinvokable]
        #[cxx_name = "subclassName"]
        fn subclass_name(self: Pin<&mut Self>, idx: i32) -> QString;

        /// Number of ASI/feat slots for the currently selected class (0 if none).
        #[qinvokable]
        #[cxx_name = "numAsiSlots"]
        fn num_asi_slots(self: Pin<&mut Self>) -> i32;

        /// Character level at which ASI slot `slot` opens (0 if out of range).
        #[qinvokable]
        #[cxx_name = "asiSlotLevel"]
        fn asi_slot_level(self: Pin<&mut Self>, slot: i32) -> i32;

        /// Skill bonus for skill index 0-17 (PHB alphabetical order).
        /// Accounts for proficiency, ability modifier, and relevant feats.
        #[qinvokable]
        #[cxx_name = "skillBonus"]
        fn skill_bonus(self: Pin<&mut Self>, skill: i32) -> i32;

        /// Saving throw bonus for ability index 0-5.
        /// Accounts for proficiency (class + Resilient feats) and ability modifier.
        #[qinvokable]
        #[cxx_name = "saveBonus"]
        fn save_bonus(self: Pin<&mut Self>, ability: i32) -> i32;

        /// Recompute all derived stats from current inputs and update Q_PROPERTYs.
        /// Also updates maxHp if autoHp is true.
        #[qinvokable]
        #[cxx_name = "recalcDerived"]
        fn recalc_derived(self: Pin<&mut Self>);

        // ── Level-based API ───────────────────────────────────────────────────

        /// Append a level in the given class index. No-op if already at level 20.
        #[qinvokable]
        #[cxx_name = "addLevel"]
        fn add_level(self: Pin<&mut Self>, class_idx: i32);

        /// Remove the last level. No-op if already at 1 level.
        #[qinvokable]
        #[cxx_name = "removeLastLevel"]
        fn remove_last_level(self: Pin<&mut Self>);

        /// Set base ability score directly (before race/ASI bonuses).
        #[qinvokable]
        #[cxx_name = "setBaseScore"]
        fn set_base_score(self: Pin<&mut Self>, ability: i32, value: i32);

        /// Set the ASI/feat choice at a specific 1-based character level.
        /// ability2 == -1 means +2 to ability1; otherwise +1+1.
        #[qinvokable]
        #[cxx_name = "setLevelAsi"]
        fn set_level_asi(self: Pin<&mut Self>, level: i32, ability1: i32, ability2: i32);

        /// Set a feat choice at a specific 1-based character level.
        #[qinvokable]
        #[cxx_name = "setLevelFeat"]
        fn set_level_feat(self: Pin<&mut Self>, level: i32, feat_idx: i32);

        /// Clear the ASI/feat choice at a specific 1-based character level.
        #[qinvokable]
        #[cxx_name = "clearLevelChoice"]
        fn clear_level_choice(self: Pin<&mut Self>, level: i32);

        /// Set the subclass for a specific 1-based character level.
        #[qinvokable]
        #[cxx_name = "setLevelSubclass"]
        fn set_level_subclass(self: Pin<&mut Self>, level: i32, subclass_idx: i32);

        /// Set a custom HP roll for a specific 1-based level (0 = use average).
        #[qinvokable]
        #[cxx_name = "setHpRoll"]
        fn set_hp_roll(self: Pin<&mut Self>, level: i32, roll: i32);

        /// Class name at a specific 1-based character level.
        #[qinvokable]
        #[cxx_name = "levelClassName"]
        fn level_class_name(self: Pin<&mut Self>, level: i32) -> QString;

        /// Subclass name at a specific 1-based character level (empty if none chosen).
        #[qinvokable]
        #[cxx_name = "levelSubclassName"]
        fn level_subclass_name(self: Pin<&mut Self>, level: i32) -> QString;

        // ── Per-level query invocables (used by the Character tab accordion) ──

        /// Class index at 1-based character level `level`, or -1 if out of range.
        #[qinvokable]
        #[cxx_name = "levelClassIdx"]
        fn level_class_idx(self: Pin<&mut Self>, level: i32) -> i32;

        /// Class-local level number at 1-based character level `level`
        /// (e.g. Fighter 3 when you have 2 Rogue + 3 Fighter levels at char level 5).
        /// Returns 0 if out of range.
        #[qinvokable]
        #[cxx_name = "levelClassLevel"]
        fn level_class_level(self: Pin<&mut Self>, level: i32) -> i32;

        /// Whether character level `level` is an ASI/feat level for its class.
        #[qinvokable]
        #[cxx_name = "levelHasAsi"]
        fn level_has_asi(self: Pin<&mut Self>, level: i32) -> bool;

        /// ASI/feat choice at character level `level`:
        ///   -1 = no ASI slot at this level (or slot unset),
        ///   -2 = ability score increase chosen,
        ///   ≥0 = feat index.
        #[qinvokable]
        #[cxx_name = "levelAsiChoice"]
        fn level_asi_choice(self: Pin<&mut Self>, level: i32) -> i32;

        /// First ability index (0-5) if an ASI +2/+1+1 was taken at this level,
        /// otherwise -1.
        #[qinvokable]
        #[cxx_name = "levelAsiAbility1"]
        fn level_asi_ability1(self: Pin<&mut Self>, level: i32) -> i32;

        /// Second ability index (0-5) if a +1+1 split was taken at this level,
        /// otherwise -1.
        #[qinvokable]
        #[cxx_name = "levelAsiAbility2"]
        fn level_asi_ability2(self: Pin<&mut Self>, level: i32) -> i32;

        /// Whether character level `level` is the subclass-choice level for its class.
        #[qinvokable]
        #[cxx_name = "levelHasSubclassChoice"]
        fn level_has_subclass_choice(self: Pin<&mut Self>, level: i32) -> bool;

        /// Number of PHB features granted at character level `level`.
        #[qinvokable]
        #[cxx_name = "levelNumFeatures"]
        fn level_num_features(self: Pin<&mut Self>, level: i32) -> i32;

        /// Name of PHB feature `feat` at character level `level`.
        #[qinvokable]
        #[cxx_name = "levelFeatureName"]
        fn level_feature_name(self: Pin<&mut Self>, level: i32, feat: i32) -> QString;

        /// Description of PHB feature `feat` at character level `level`.
        #[qinvokable]
        #[cxx_name = "levelFeatureDescription"]
        fn level_feature_description(self: Pin<&mut Self>, level: i32, feat: i32) -> QString;

        /// Number of subclasses for a given class index (not the selected class).
        #[qinvokable]
        #[cxx_name = "numSubclassesForClassIdx"]
        fn num_subclasses_for_class_idx(self: Pin<&mut Self>, class_idx: i32) -> i32;

        /// Subclass name by class index and subclass index.
        #[qinvokable]
        #[cxx_name = "subclassNameForClassIdx"]
        fn subclass_name_for_class_idx(self: Pin<&mut Self>, class_idx: i32, idx: i32) -> QString;

        // ── Class choice-feature API (generic picker) ─────────────────────────

        /// Number of ChoiceFeature sets available at character level `level`.
        /// A set covers one class level's worth of choices (e.g. Fighter L1 = Fighting Style).
        #[qinvokable]
        #[cxx_name = "levelNumChoiceSets"]
        fn level_num_choice_sets(self: Pin<&mut Self>, level: i32) -> i32;

        /// Name of ChoiceFeature `feat_idx` within choice-set `set_idx` at character level `level`.
        #[qinvokable]
        #[cxx_name = "levelChoiceFeatureName"]
        fn level_choice_feature_name(self: Pin<&mut Self>, level: i32, set_idx: i32, feat_idx: i32) -> QString;

        /// Description of ChoiceFeature `feat_idx` within choice-set `set_idx`.
        #[qinvokable]
        #[cxx_name = "levelChoiceFeatureDesc"]
        fn level_choice_feature_desc(self: Pin<&mut Self>, level: i32, set_idx: i32, feat_idx: i32) -> QString;

        /// Number of features in choice-set `set_idx` at character level `level`.
        #[qinvokable]
        #[cxx_name = "levelNumChoiceFeatures"]
        fn level_num_choice_features(self: Pin<&mut Self>, level: i32, set_idx: i32) -> i32;

        /// How many options the player must pick for this choice feature.
        #[qinvokable]
        #[cxx_name = "levelChoiceFeatureNumPicks"]
        fn level_choice_feature_num_picks(self: Pin<&mut Self>, level: i32, set_idx: i32, feat_idx: i32) -> i32;

        /// Total options available for this choice feature.
        #[qinvokable]
        #[cxx_name = "levelChoiceFeatureNumOptions"]
        fn level_choice_feature_num_options(self: Pin<&mut Self>, level: i32, set_idx: i32, feat_idx: i32) -> i32;

        /// Name of option `opt_idx`.
        #[qinvokable]
        #[cxx_name = "levelChoiceOptionName"]
        fn level_choice_option_name(self: Pin<&mut Self>, level: i32, set_idx: i32, feat_idx: i32, opt_idx: i32) -> QString;

        /// Description of option `opt_idx`.
        #[qinvokable]
        #[cxx_name = "levelChoiceOptionDesc"]
        fn level_choice_option_desc(self: Pin<&mut Self>, level: i32, set_idx: i32, feat_idx: i32, opt_idx: i32) -> QString;

        /// Whether option `opt_idx` is currently selected.
        #[qinvokable]
        #[cxx_name = "levelChoiceOptionSelected"]
        fn level_choice_option_selected(self: Pin<&mut Self>, level: i32, set_idx: i32, feat_idx: i32, opt_idx: i32) -> bool;

        /// Toggle selection of `opt_idx`. If already selected, deselects it.
        /// Respects `num_picks`: selecting a new option when already at limit
        /// replaces the oldest selection (first-in-first-out).
        #[qinvokable]
        #[cxx_name = "levelChoiceOptionToggle"]
        fn level_choice_option_toggle(self: Pin<&mut Self>, level: i32, set_idx: i32, feat_idx: i32, opt_idx: i32);

        /// Clear all picks for a choice feature.
        #[qinvokable]
        #[cxx_name = "levelChoiceClear"]
        fn level_choice_clear(self: Pin<&mut Self>, level: i32, set_idx: i32, feat_idx: i32);

        // ── Race trait / choice-feature API ──────────────────────────────────

        /// Number of racial traits for the currently selected race (-1 = none).
        #[qinvokable]
        #[cxx_name = "numRaceTraits"]
        fn num_race_traits(self: Pin<&mut Self>) -> i32;

        /// Name of racial trait `idx`.
        #[qinvokable]
        #[cxx_name = "raceTraitName"]
        fn race_trait_name(self: Pin<&mut Self>, idx: i32) -> QString;

        /// Description of racial trait `idx`.
        #[qinvokable]
        #[cxx_name = "raceTraitDesc"]
        fn race_trait_desc(self: Pin<&mut Self>, idx: i32) -> QString;

        /// How many options the player must pick for racial trait `idx` (0 = informational).
        #[qinvokable]
        #[cxx_name = "raceTraitNumPicks"]
        fn race_trait_num_picks(self: Pin<&mut Self>, idx: i32) -> i32;

        /// Total options for racial trait `idx`.
        #[qinvokable]
        #[cxx_name = "raceTraitNumOptions"]
        fn race_trait_num_options(self: Pin<&mut Self>, idx: i32) -> i32;

        /// Name of option `opt_idx` for racial trait `trait_idx`.
        #[qinvokable]
        #[cxx_name = "raceTraitOptionName"]
        fn race_trait_option_name(self: Pin<&mut Self>, trait_idx: i32, opt_idx: i32) -> QString;

        /// Description of option `opt_idx` for racial trait `trait_idx`.
        #[qinvokable]
        #[cxx_name = "raceTraitOptionDesc"]
        fn race_trait_option_desc(self: Pin<&mut Self>, trait_idx: i32, opt_idx: i32) -> QString;

        /// Whether option `opt_idx` is currently selected for racial trait `trait_idx`.
        #[qinvokable]
        #[cxx_name = "raceTraitOptionSelected"]
        fn race_trait_option_selected(self: Pin<&mut Self>, trait_idx: i32, opt_idx: i32) -> bool;

        /// Toggle selection of `opt_idx` for racial trait `trait_idx`.
        #[qinvokable]
        #[cxx_name = "raceTraitOptionToggle"]
        fn race_trait_option_toggle(self: Pin<&mut Self>, trait_idx: i32, opt_idx: i32);

        /// Clear all picks for racial trait `trait_idx`.
        #[qinvokable]
        #[cxx_name = "raceTraitClear"]
        fn race_trait_clear(self: Pin<&mut Self>, trait_idx: i32);
    }
}

use core::pin::Pin;
use crate::dnd_data;
use crate::dnd_data::{ClassChoiceSet, RaceTrait};
use crate::player::{AsiChoice, CharacterState, LevelChoice};
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;

pub struct CharacterSheetRust {
    name: QString,
    charClass: QString,
    race: QString,
    background: QString,
    alignment: QString,
    level: i32,
    xp: i32,
    strScore: i32,
    dexScore: i32,
    conScore: i32,
    intScore: i32,
    wisScore: i32,
    chaScore: i32,
    maxHp: i32,
    currentHp: i32,
    tempHp: i32,
    armorClass: i32,
    speed: i32,
    inspiration: bool,
    deathSuccess: i32,
    deathFail: i32,
    saveProfs: i32,
    skillProfs: i32,
    hitDie: i32,
    hitDiceUsed: i32,
    // ── Selection state ───────────────────────────────────────────────────
    classIdx: i32,
    raceIdx: i32,
    backgroundIdx: i32,
    subclassIdx: i32,
    classSkillChoices: i32,
    featSlot0: i32,
    featSlot1: i32,
    featSlot2: i32,
    featSlot3: i32,
    featSlot4: i32,
    featSlot5: i32,
    featSlot6: i32,
    autoHp: bool,
    // ── Authoritative character model ─────────────────────────────────────
    /// All levels, choices, base scores, race/background data live here.
    /// `recalc_all()` calls `player::compute(&state)` and pushes every
    /// derived value into the Q_PROPERTY backing fields above.
    state: CharacterState,
    // ── Computed stats cache ────────────────────────────────────────────
    computed: crate::player::ComputedStats,
    // ── Computed Q_PROPERTY backing fields ───────────────────────────
    profBonus: i32,
    strMod: i32,
    dexMod: i32,
    conMod: i32,
    intMod: i32,
    wisMod: i32,
    chaMod: i32,
    initiative: i32,
    passivePerception: i32,
    passiveInvestigation: i32,
    spellcastingAbilityIdx: i32,
    spellSaveDc: i32,
    spellAttackBonus: i32,
    selectionVersion: i32,
}

impl Default for CharacterSheetRust {
    fn default() -> Self {
        // Build the initial character: Level 1 Fighter with standard array scores.
        let mut state = CharacterState::default();
        state.base_scores = [15, 13, 14, 10, 11, 12]; // STR DEX CON INT WIS CHA
        state.add_level(4); // Fighter
        let computed = crate::player::compute(&state);

        Self {
            name: QString::from("Adventurer"),
            charClass: QString::from("Fighter"),
            race: QString::from("Human"),
            background: QString::from("Soldier"),
            alignment: QString::from("Neutral Good"),
            level: computed.total_level,
            xp: 0,
            strScore: computed.final_scores[0],
            dexScore: computed.final_scores[1],
            conScore: computed.final_scores[2],
            intScore: computed.final_scores[3],
            wisScore: computed.final_scores[4],
            chaScore: computed.final_scores[5],
            maxHp: computed.max_hp,
            currentHp: computed.max_hp,
            tempHp: 0,
            armorClass: 16,
            speed: 30,
            inspiration: false,
            deathSuccess: 0,
            deathFail: 0,
            saveProfs: computed.save_profs.bits() as i32,
            skillProfs: computed.skill_profs.bits() as i32,
            hitDie: computed.hit_die,
            hitDiceUsed: 0,
            classIdx: 4, // Fighter
            raceIdx: -1,
            backgroundIdx: -1,
            subclassIdx: computed.primary_subclass_idx,
            classSkillChoices: computed.class_skill_choices.bits() as i32,
            featSlot0: computed.feat_slot_values[0],
            featSlot1: computed.feat_slot_values[1],
            featSlot2: computed.feat_slot_values[2],
            featSlot3: computed.feat_slot_values[3],
            featSlot4: computed.feat_slot_values[4],
            featSlot5: computed.feat_slot_values[5],
            featSlot6: computed.feat_slot_values[6],
            autoHp: true, // auto-HP on by default
            state,
            computed: computed.clone(),
            profBonus: computed.prof_bonus,
            strMod: computed.ability_mods[0],
            dexMod: computed.ability_mods[1],
            conMod: computed.ability_mods[2],
            intMod: computed.ability_mods[3],
            wisMod: computed.ability_mods[4],
            chaMod: computed.ability_mods[5],
            initiative: computed.initiative,
            passivePerception: computed.passive_perception,
            passiveInvestigation: computed.passive_investigation,
            spellcastingAbilityIdx: computed.spellcasting_ability_idx,
            spellSaveDc: computed.spell_save_dc,
            spellAttackBonus: computed.spell_attack_bonus,
            selectionVersion: 0,
        }
    }
}

impl qobject::CharacterSheet {
    pub fn adjust_hp(mut self: Pin<&mut Self>, delta: i32) {
        let max = *self.as_ref().maxHp();
        let v = (*self.as_ref().currentHp() + delta).clamp(0, max);
        self.as_mut().set_currentHp(v);
    }

    pub fn adjust_max_hp(mut self: Pin<&mut Self>, delta: i32) {
        let new_max = (*self.as_ref().maxHp() + delta).max(1);
        self.as_mut().set_maxHp(new_max);
        let cur = *self.as_ref().currentHp();
        if cur > new_max {
            self.as_mut().set_currentHp(new_max);
        }
    }

    pub fn adjust_temp_hp(mut self: Pin<&mut Self>, delta: i32) {
        let v = (*self.as_ref().tempHp() + delta).max(0);
        self.as_mut().set_tempHp(v);
    }

    pub fn adjust_score(mut self: Pin<&mut Self>, ability: i32, delta: i32) {
        if ability < 0 || ability >= 6 { return; }
        let ab = ability as usize;
        let cur = self.as_ref().rust().state.base_scores[ab];
        // Adjust base score (final score = base + race_asi + ASI bonuses)
        let new_base = (cur + delta).clamp(1, 30);
        self.as_mut().rust_mut().state.base_scores[ab] = new_base;
        self.as_mut().recalc_derived();
    }

    pub fn adjust_level(mut self: Pin<&mut Self>, delta: i32) {
        if delta == 0 { return; }
        let class_idx = (*self.as_ref().classIdx()).max(0) as usize;
        if delta > 0 {
            let target = (*self.as_ref().level() + delta).min(20) as usize;
            let current = self.as_ref().rust().state.levels.len();
            for _ in current..target {
                self.as_mut().rust_mut().state.add_level(class_idx);
            }
        } else {
            // Clamp removal to minimum 1 level
            let target = (*self.as_ref().level() + delta).max(1) as usize;
            let current = self.as_ref().rust().state.levels.len();
            for _ in target..current {
                self.as_mut().rust_mut().state.remove_last_level();
            }
        }
        let used = *self.as_ref().hitDiceUsed();
        let new_level = self.as_ref().rust().state.levels.len() as i32;
        if used > new_level {
            self.as_mut().set_hitDiceUsed(new_level);
        }
        self.as_mut().recalc_derived();
    }

    pub fn adjust_ac(mut self: Pin<&mut Self>, delta: i32) {
        let v = (*self.as_ref().armorClass() + delta).clamp(0, 30);
        self.as_mut().set_armorClass(v);
    }

    pub fn adjust_speed(mut self: Pin<&mut Self>, delta: i32) {
        let v = (*self.as_ref().speed() + delta).max(0);
        self.as_mut().set_speed(v);
    }

    pub fn adjust_xp(mut self: Pin<&mut Self>, delta: i32) {
        let v = (*self.as_ref().xp() + delta).max(0);
        self.as_mut().set_xp(v);
    }

    pub fn toggle_save_prof(mut self: Pin<&mut Self>, ability: i32) {
        if ability < 0 || ability > 5 {
            return;
        }
        let v = *self.as_ref().saveProfs() ^ (1 << ability);
        self.as_mut().set_saveProfs(v);
        self.as_mut().recalc_derived();
    }

    pub fn toggle_skill_prof(mut self: Pin<&mut Self>, skill: i32) {
        if skill < 0 || skill > 17 {
            return;
        }
        let v = *self.as_ref().skillProfs() ^ (1 << skill);
        self.as_mut().set_skillProfs(v);
        self.as_mut().recalc_derived();
    }

    pub fn toggle_inspiration(mut self: Pin<&mut Self>) {
        let v = !*self.as_ref().inspiration();
        self.as_mut().set_inspiration(v);
    }

    pub fn adjust_death_success(mut self: Pin<&mut Self>, delta: i32) {
        let v = (*self.as_ref().deathSuccess() + delta).clamp(0, 3);
        self.as_mut().set_deathSuccess(v);
    }

    pub fn adjust_death_fail(mut self: Pin<&mut Self>, delta: i32) {
        let v = (*self.as_ref().deathFail() + delta).clamp(0, 3);
        self.as_mut().set_deathFail(v);
    }

    pub fn cycle_hit_die(mut self: Pin<&mut Self>) {
        let next = match *self.as_ref().hitDie() {
            6 => 8,
            8 => 10,
            10 => 12,
            _ => 6,
        };
        self.as_mut().set_hitDie(next);
    }

    pub fn adjust_hit_dice_used(mut self: Pin<&mut Self>, delta: i32) {
        let max = *self.as_ref().level();
        let v = (*self.as_ref().hitDiceUsed() + delta).clamp(0, max);
        self.as_mut().set_hitDiceUsed(v);
    }

    pub fn long_rest(mut self: Pin<&mut Self>) {
        let max_hp = *self.as_ref().maxHp();
        self.as_mut().set_currentHp(max_hp);
        self.as_mut().set_tempHp(0);
        self.as_mut().set_deathSuccess(0);
        self.as_mut().set_deathFail(0);
        self.as_mut().set_hitDiceUsed(0);
    }

    pub fn short_rest(mut self: Pin<&mut Self>) {
        // Recover half of total hit dice (rounded up, minimum 1)
        let level = *self.as_ref().level();
        let used = *self.as_ref().hitDiceUsed();
        let recover = ((level + 1) / 2).max(1);
        let v = (used - recover).max(0);
        self.as_mut().set_hitDiceUsed(v);
    }

    // ── Internal helpers ──────────────────────────────────────────────────────

    /// Set a feat slot Q_PROPERTY backing field by slot index (0-6).
    fn set_feat_slot(mut self: Pin<&mut Self>, slot: usize, v: i32) {
        match slot {
            0 => self.as_mut().set_featSlot0(v),
            1 => self.as_mut().set_featSlot1(v),
            2 => self.as_mut().set_featSlot2(v),
            3 => self.as_mut().set_featSlot3(v),
            4 => self.as_mut().set_featSlot4(v),
            5 => self.as_mut().set_featSlot5(v),
            6 => self.as_mut().set_featSlot6(v),
            _ => {}
        }
    }

    // ── Selection invocables ──────────────────────────────────────────────────

    pub fn select_class(mut self: Pin<&mut Self>, idx: i32) {
        self.as_mut().set_classIdx(idx);
        self.as_mut().set_subclassIdx(-1);
        self.as_mut().set_classSkillChoices(0);

        if idx < 0 {
            self.as_mut().recalc_derived();
            return;
        }
        let ci = idx as usize;

        // Change all existing levels of the old primary class to the new one.
        // This handles the common "I picked the wrong class" correction for
        // single-class characters without destroying multiclass info.
        if let Some(primary_ci) = self.as_ref().rust().state.primary_class_idx() {
            let n = self.as_ref().rust().state.levels.len();
            for i in 0..n {
                if self.as_ref().rust().state.levels[i].class_idx == primary_ci {
                    self.as_mut().rust_mut().state.levels[i].class_idx = ci;
                    // Clear class skill choices from these levels (pool changed)
                    self.as_mut().rust_mut().state.levels[i]
                        .choices
                        .retain(|c| !matches!(c, LevelChoice::ClassSkill(_)));
                }
            }
        } else {
            // No levels yet — add the first one.
            self.as_mut().rust_mut().state.add_level(ci);
        }

        if let Some(cls) = dnd_data::class(ci) {
            self.as_mut().set_charClass(QString::from(cls.name));
        }
        self.as_mut().recalc_derived();
    }

    pub fn select_race(mut self: Pin<&mut Self>, idx: i32) {
        self.as_mut().set_raceIdx(idx);
        if idx >= 0 {
            if let Some(r) = dnd_data::race(idx as usize) {
                self.as_mut().rust_mut().state.race_asi = r.asi.0;
                self.as_mut().set_speed(r.speed as i32);
                self.as_mut().set_race(QString::from(r.name));
            }
        } else {
            self.as_mut().rust_mut().state.race_asi = [0; 6];
        }
        self.as_mut().recalc_derived();
    }

    pub fn select_background(mut self: Pin<&mut Self>, idx: i32) {
        self.as_mut().set_backgroundIdx(idx);
        if idx >= 0 {
            if let Some(bg) = dnd_data::background(idx as usize) {
                self.as_mut().rust_mut().state.background_skill_profs = bg.skill_profs;
                self.as_mut().set_background(QString::from(bg.name));
            }
        } else {
            self.as_mut().rust_mut().state.background_skill_profs = dnd_data::SkillSet::new();
        }
        self.as_mut().recalc_derived();
    }

    pub fn select_subclass(mut self: Pin<&mut Self>, idx: i32) {
        self.as_mut().set_subclassIdx(idx);
        // Record in the appropriate level (level 3 for most classes, else level 1).
        let level_target = if self.as_ref().rust().state.levels.len() >= 3 { 3 } else { 1 };
        let ci = (*self.as_ref().classIdx()).max(0) as usize;
        // Remove existing subclass choices for primary class entries.
        let n = self.as_ref().rust().state.levels.len();
        for i in 0..n {
            if self.as_ref().rust().state.levels[i].class_idx == ci {
                self.as_mut().rust_mut().state.levels[i]
                    .choices
                    .retain(|c| !matches!(c, LevelChoice::Subclass(_)));
            }
        }
        if idx >= 0 {
            self.as_mut().rust_mut().state.set_subclass(level_target, idx as usize);
        }
        self.as_mut().recalc_derived();
    }

    pub fn toggle_class_skill(mut self: Pin<&mut Self>, skill: i32) {
        let class_idx = *self.as_ref().classIdx();
        if class_idx < 0 || skill < 0 || skill >= 18 { return; }
        let ci = class_idx as usize;
        let cls = match dnd_data::class(ci) { Some(c) => c, None => return };
        let sk = match dnd_data::Skill::from_index(skill as u8) { Some(s) => s, None => return };
        if !cls.skill_pool.contains(sk) { return; }

        // Find the first level entry for this class.
        let level_idx = self.as_ref().rust().state.levels.iter()
            .position(|l| l.class_idx == ci)
            .unwrap_or(0);
        let chosen_count = self.as_ref().rust().state.levels[level_idx]
            .class_skills().count();
        let is_chosen = self.as_ref().rust().state.levels[level_idx]
            .choices.iter()
            .any(|c| matches!(c, LevelChoice::ClassSkill(s) if *s == sk));

        if is_chosen {
            self.as_mut().rust_mut().state.levels[level_idx]
                .choices
                .retain(|c| !matches!(c, LevelChoice::ClassSkill(s) if *s == sk));
        } else if chosen_count < cls.skill_choices as usize {
            self.as_mut().rust_mut().state.levels[level_idx]
                .choices
                .push(LevelChoice::ClassSkill(sk));
        }
        self.as_mut().recalc_derived();
    }

    pub fn select_feat(mut self: Pin<&mut Self>, slot: i32, feat_idx: i32) {
        if slot < 0 || slot > 6 { return; }
        let char_level = match self.as_ref().rust().state.asi_slot_character_level(slot as usize) {
            Some(l) => l,
            None => return, // character not high enough level for this slot
        };
        let choice = if feat_idx >= 0 {
            Some(AsiChoice::Feat(feat_idx as usize))
        } else {
            None
        };
        self.as_mut().rust_mut().state.set_asi_choice(char_level, choice);
        self.as_mut().recalc_derived();
    }

    pub fn set_slot_asi(mut self: Pin<&mut Self>, slot: i32, ability1: i32, ability2: i32) {
        if slot < 0 || slot > 6 { return; }
        let char_level = match self.as_ref().rust().state.asi_slot_character_level(slot as usize) {
            Some(l) => l,
            None => return,
        };
        if ability1 < 0 || ability1 >= 6 { return; }
        let ab1 = dnd_data::Ability::from_index(ability1 as u8).unwrap_or(dnd_data::Ability::Str);
        let ab2 = if ability2 >= 0 && ability2 < 6 {
            dnd_data::Ability::from_index(ability2 as u8)
        } else {
            None
        };
        self.as_mut().rust_mut().state.set_asi_choice(
            char_level,
            Some(AsiChoice::Scores { ability1: ab1, ability2: ab2 }),
        );
        self.as_mut().recalc_derived();
    }

    pub fn clear_feat_slot(mut self: Pin<&mut Self>, slot: i32) {
        if slot < 0 || slot > 6 { return; }
        let char_level = match self.as_ref().rust().state.asi_slot_character_level(slot as usize) {
            Some(l) => l,
            None => return,
        };
        self.as_mut().rust_mut().state.set_asi_choice(char_level, None);
        self.as_mut().recalc_derived();
    }

    pub fn recalc_hp(mut self: Pin<&mut Self>) {
        self.as_mut().recalc_all();
        let new_max = self.as_ref().rust().computed.max_hp;
        self.as_mut().set_maxHp(new_max);
        let cur = *self.as_ref().currentHp();
        if cur > new_max {
            self.as_mut().set_currentHp(new_max);
        }
    }

    // ── Query invocables ──────────────────────────────────────────────────────

    pub fn num_classes(self: Pin<&mut Self>) -> i32 {
        dnd_data::num_classes() as i32
    }

    pub fn class_name(self: Pin<&mut Self>, idx: i32) -> QString {
        if idx < 0 {
            return QString::from("");
        }
        match dnd_data::class(idx as usize) {
            Some(c) => QString::from(c.name),
            None => QString::from(""),
        }
    }

    pub fn num_races(self: Pin<&mut Self>) -> i32 {
        dnd_data::num_races() as i32
    }

    pub fn race_name(self: Pin<&mut Self>, idx: i32) -> QString {
        if idx < 0 {
            return QString::from("");
        }
        match dnd_data::race(idx as usize) {
            Some(r) => QString::from(r.name),
            None => QString::from(""),
        }
    }

    pub fn num_backgrounds(self: Pin<&mut Self>) -> i32 {
        dnd_data::num_backgrounds() as i32
    }

    pub fn background_name(self: Pin<&mut Self>, idx: i32) -> QString {
        if idx < 0 {
            return QString::from("");
        }
        match dnd_data::background(idx as usize) {
            Some(b) => QString::from(b.name),
            None => QString::from(""),
        }
    }

    pub fn num_feats(self: Pin<&mut Self>) -> i32 {
        dnd_data::num_feats() as i32
    }

    pub fn feat_name(self: Pin<&mut Self>, idx: i32) -> QString {
        if idx < 0 {
            return QString::from("");
        }
        match dnd_data::feat(idx as usize) {
            Some(f) => QString::from(f.name),
            None => QString::from(""),
        }
    }

    pub fn feat_description(self: Pin<&mut Self>, idx: i32) -> QString {
        if idx < 0 {
            return QString::from("");
        }
        match dnd_data::feat(idx as usize) {
            Some(f) => QString::from(f.description),
            None => QString::from(""),
        }
    }

    pub fn class_skill_pool(self: Pin<&mut Self>) -> i32 {
        let idx = *self.as_ref().classIdx();
        if idx < 0 {
            return 0;
        }
        match dnd_data::class(idx as usize) {
            Some(c) => c.skill_pool.bits() as i32,
            None => 0,
        }
    }

    pub fn class_skill_count(self: Pin<&mut Self>) -> i32 {
        let idx = *self.as_ref().classIdx();
        if idx < 0 {
            return 0;
        }
        match dnd_data::class(idx as usize) {
            Some(c) => c.skill_choices as i32,
            None => 0,
        }
    }

    pub fn num_subclasses(self: Pin<&mut Self>) -> i32 {
        let idx = *self.as_ref().classIdx();
        if idx < 0 {
            return 0;
        }
        match dnd_data::class(idx as usize) {
            Some(c) => c.subclasses.len() as i32,
            None => 0,
        }
    }

    pub fn subclass_name(self: Pin<&mut Self>, idx: i32) -> QString {
        let class_idx = *self.as_ref().classIdx();
        if class_idx < 0 || idx < 0 {
            return QString::from("");
        }
        match dnd_data::class(class_idx as usize) {
            Some(c) => match c.subclasses.get(idx as usize) {
                Some(&name) => QString::from(name),
                None => QString::from(""),
            },
            None => QString::from(""),
        }
    }

    pub fn num_asi_slots(self: Pin<&mut Self>) -> i32 {
        let idx = *self.as_ref().classIdx();
        if idx < 0 {
            return 0;
        }
        match dnd_data::class(idx as usize) {
            Some(c) => c.asi_levels.len() as i32,
            None => 0,
        }
    }

    pub fn asi_slot_level(self: Pin<&mut Self>, slot: i32) -> i32 {
        let class_idx = *self.as_ref().classIdx();
        if class_idx < 0 || slot < 0 {
            return 0;
        }
        match dnd_data::class(class_idx as usize) {
            Some(c) => match c.asi_levels.get(slot as usize) {
                Some(&lvl) => lvl as i32,
                None => 0,
            },
            None => 0,
        }
    }

    // ── Player computation ────────────────────────────────────────────────────

    /// Build a `CharacterState` snapshot from the current Q_PROPERTYs.
    /// Recompute all derived stats from `state`, cache in `computed`,
    /// and push every Q_PROPERTY backing field to its current value.
    fn recalc_all(mut self: Pin<&mut Self>) {
        let c = crate::player::compute(&self.as_ref().rust().state);

        // ── Derived scores ─────────────────────────────────────────────────────
        self.as_mut().set_level(c.total_level);
        self.as_mut().set_strScore(c.final_scores[0]);
        self.as_mut().set_dexScore(c.final_scores[1]);
        self.as_mut().set_conScore(c.final_scores[2]);
        self.as_mut().set_intScore(c.final_scores[3]);
        self.as_mut().set_wisScore(c.final_scores[4]);
        self.as_mut().set_chaScore(c.final_scores[5]);

        // ── Modifiers ──────────────────────────────────────────────────────────
        self.as_mut().set_profBonus(c.prof_bonus);
        self.as_mut().set_strMod(c.ability_mods[0]);
        self.as_mut().set_dexMod(c.ability_mods[1]);
        self.as_mut().set_conMod(c.ability_mods[2]);
        self.as_mut().set_intMod(c.ability_mods[3]);
        self.as_mut().set_wisMod(c.ability_mods[4]);
        self.as_mut().set_chaMod(c.ability_mods[5]);

        // ── Proficiencies ──────────────────────────────────────────────────────
        self.as_mut().set_saveProfs(c.save_profs.bits() as i32);
        self.as_mut().set_skillProfs(c.skill_profs.bits() as i32);
        self.as_mut().set_hitDie(c.hit_die);

        // ── Combat ─────────────────────────────────────────────────────────────
        self.as_mut().set_initiative(c.initiative);
        self.as_mut().set_passivePerception(c.passive_perception);
        self.as_mut().set_passiveInvestigation(c.passive_investigation);

        // ── Spellcasting ───────────────────────────────────────────────────────
        self.as_mut().set_spellcastingAbilityIdx(c.spellcasting_ability_idx);
        self.as_mut().set_spellSaveDc(c.spell_save_dc);
        self.as_mut().set_spellAttackBonus(c.spell_attack_bonus);

        // ── Selection display helpers ──────────────────────────────────────────
        self.as_mut().set_subclassIdx(c.primary_subclass_idx);
        self.as_mut().set_classSkillChoices(c.class_skill_choices.bits() as i32);
        for slot in 0..7 {
            self.as_mut().set_feat_slot(slot, c.feat_slot_values[slot]);
        }

        // Update charClass from primary class name
        if let Some(primary_ci) = self.as_ref().rust().state.primary_class_idx() {
            if let Some(cls) = dnd_data::class(primary_ci) {
                self.as_mut().set_classIdx(primary_ci as i32);
                self.as_mut().set_charClass(QString::from(cls.name));
            }
        }

        self.as_mut().rust_mut().computed = c;
    }

    pub fn recalc_derived(mut self: Pin<&mut Self>) {
        self.as_mut().recalc_all();
        if *self.as_ref().autoHp() {
            let new_max = self.as_ref().rust().computed.max_hp;
            self.as_mut().set_maxHp(new_max);
            let cur = *self.as_ref().currentHp();
            if cur > new_max {
                self.as_mut().set_currentHp(new_max);
            }
        }
        let v = *self.as_ref().selectionVersion() + 1;
        self.as_mut().set_selectionVersion(v);
    }

    pub fn skill_bonus(self: Pin<&mut Self>, skill: i32) -> i32 {
        if skill < 0 || skill >= 18 { return 0; }
        self.as_ref().rust().computed.skill_bonuses[skill as usize]
    }

    pub fn save_bonus(self: Pin<&mut Self>, ability: i32) -> i32 {
        if ability < 0 || ability >= 6 { return 0; }
        self.as_ref().rust().computed.save_bonuses[ability as usize]
    }

    // ── Level-based API ───────────────────────────────────────────────────────

    pub fn add_level(mut self: Pin<&mut Self>, class_idx: i32) {
        if class_idx < 0 { return; }
        self.as_mut().rust_mut().state.add_level(class_idx as usize);
        self.as_mut().recalc_derived();
    }

    pub fn remove_last_level(mut self: Pin<&mut Self>) {
        let current = self.as_ref().rust().state.levels.len();
        if current == 0 { return; }
        self.as_mut().rust_mut().state.remove_last_level();
        let used = *self.as_ref().hitDiceUsed();
        let new_level = self.as_ref().rust().state.levels.len() as i32;
        if used > new_level { self.as_mut().set_hitDiceUsed(new_level); }
        self.as_mut().recalc_derived();
    }

    pub fn set_base_score(mut self: Pin<&mut Self>, ability: i32, value: i32) {
        if ability < 0 || ability >= 6 { return; }
        self.as_mut().rust_mut().state.base_scores[ability as usize] = value.clamp(1, 30);
        self.as_mut().recalc_derived();
    }

    pub fn set_level_asi(mut self: Pin<&mut Self>, level: i32, ability1: i32, ability2: i32) {
        if level < 1 || ability1 < 0 || ability1 >= 6 { return; }
        let ab1 = dnd_data::Ability::from_index(ability1 as u8).unwrap_or(dnd_data::Ability::Str);
        let ab2 = if ability2 >= 0 && ability2 < 6 {
            dnd_data::Ability::from_index(ability2 as u8)
        } else {
            None
        };
        self.as_mut().rust_mut().state.set_asi_choice(
            level as usize,
            Some(AsiChoice::Scores { ability1: ab1, ability2: ab2 }),
        );
        self.as_mut().recalc_derived();
    }

    pub fn set_level_feat(mut self: Pin<&mut Self>, level: i32, feat_idx: i32) {
        if level < 1 || feat_idx < 0 { return; }
        self.as_mut().rust_mut().state.set_asi_choice(
            level as usize,
            Some(AsiChoice::Feat(feat_idx as usize)),
        );
        self.as_mut().recalc_derived();
    }

    pub fn clear_level_choice(mut self: Pin<&mut Self>, level: i32) {
        if level < 1 { return; }
        self.as_mut().rust_mut().state.set_asi_choice(level as usize, None);
        self.as_mut().recalc_derived();
    }

    pub fn set_level_subclass(mut self: Pin<&mut Self>, level: i32, subclass_idx: i32) {
        if level < 1 || subclass_idx < 0 { return; }
        self.as_mut().rust_mut().state.set_subclass(level as usize, subclass_idx as usize);
        self.as_mut().recalc_derived();
    }

    pub fn set_hp_roll(mut self: Pin<&mut Self>, level: i32, roll: i32) {
        if level < 1 { return; }
        self.as_mut().rust_mut().state.set_hp_roll(level as usize, roll.clamp(0, 255) as u8);
        self.as_mut().recalc_derived();
    }

    pub fn level_class_name(self: Pin<&mut Self>, level: i32) -> QString {
        if level < 1 { return QString::from(""); }
        match self.as_ref().rust().state.levels.get((level - 1) as usize) {
            Some(entry) => match dnd_data::class(entry.class_idx) {
                Some(cls) => QString::from(cls.name),
                None => QString::from(""),
            },
            None => QString::from(""),
        }
    }

    pub fn level_subclass_name(self: Pin<&mut Self>, level: i32) -> QString {
        if level < 1 { return QString::from(""); }
        let idx = (level - 1) as usize;
        // Copy primitive values out immediately so no ref crosses the statement boundary.
        let (class_idx, subclass_idx): (usize, Option<usize>) =
            match self.as_ref().rust().state.levels.get(idx) {
                Some(e) => (e.class_idx, e.subclass()),
                None => return QString::from(""),
            };
        let si = match subclass_idx {
            Some(s) => s,
            None => return QString::from(""),
        };
        match dnd_data::class(class_idx) {
            Some(cls) => match cls.subclasses.get(si) {
                Some(&name) => QString::from(name),
                None => QString::from(""),
            },
            None => QString::from(""),
        }
    }

    // ── Per-level query helpers ───────────────────────────────────────────────

    /// Helper: for character level `level` (1-based), return `(class_idx, class_local_level)`.
    /// `class_local_level` is how many times that class_idx appears up to and including `level`.
    fn level_info(state: &crate::player::CharacterState, level: usize) -> Option<(usize, usize)> {
        let entry = state.levels.get(level - 1)?;
        let ci = entry.class_idx;
        let class_local = state.levels[..level].iter()
            .filter(|l| l.class_idx == ci)
            .count();
        Some((ci, class_local))
    }

    pub fn level_class_idx(self: Pin<&mut Self>, level: i32) -> i32 {
        if level < 1 { return -1; }
        match Self::level_info(&self.as_ref().rust().state, level as usize) {
            Some((ci, _)) => ci as i32,
            None => -1,
        }
    }

    pub fn level_class_level(self: Pin<&mut Self>, level: i32) -> i32 {
        if level < 1 { return 0; }
        match Self::level_info(&self.as_ref().rust().state, level as usize) {
            Some((_, cl)) => cl as i32,
            None => 0,
        }
    }

    pub fn level_has_asi(self: Pin<&mut Self>, level: i32) -> bool {
        if level < 1 { return false; }
        let (ci, class_local) = match Self::level_info(&self.as_ref().rust().state, level as usize) {
            Some(x) => x,
            None => return false,
        };
        match dnd_data::class(ci) {
            Some(cls) => cls.asi_levels.contains(&(class_local as u8)),
            None => false,
        }
    }

    pub fn level_asi_choice(mut self: Pin<&mut Self>, level: i32) -> i32 {
        if level < 1 { return -1; }
        if !self.as_mut().level_has_asi(level) { return -1; }
        match self.as_ref().rust().state.levels.get((level - 1) as usize) {
            Some(entry) => match entry.asi_choice() {
                Some(ch) => crate::player::asi_choice_to_slot_value(ch),
                None => -1,
            },
            None => -1,
        }
    }

    pub fn level_asi_ability1(self: Pin<&mut Self>, level: i32) -> i32 {
        if level < 1 { return -1; }
        match self.as_ref().rust().state.levels.get((level - 1) as usize) {
            Some(entry) => match entry.asi_choice() {
                Some(crate::player::AsiChoice::Scores { ability1, .. }) => *ability1 as i32,
                _ => -1,
            },
            None => -1,
        }
    }

    pub fn level_asi_ability2(self: Pin<&mut Self>, level: i32) -> i32 {
        if level < 1 { return -1; }
        match self.as_ref().rust().state.levels.get((level - 1) as usize) {
            Some(entry) => match entry.asi_choice() {
                Some(crate::player::AsiChoice::Scores { ability2: Some(ab2), .. }) => *ab2 as i32,
                _ => -1,
            },
            None => -1,
        }
    }

    pub fn level_has_subclass_choice(self: Pin<&mut Self>, level: i32) -> bool {
        if level < 1 { return false; }
        let (ci, class_local) = match Self::level_info(&self.as_ref().rust().state, level as usize) {
            Some(x) => x,
            None => return false,
        };
        match dnd_data::class(ci) {
            Some(cls) => cls.subclass_level as usize == class_local,
            None => false,
        }
    }

    pub fn level_num_features(self: Pin<&mut Self>, level: i32) -> i32 {
        if level < 1 { return 0; }
        let (ci, class_local) = match Self::level_info(&self.as_ref().rust().state, level as usize) {
            Some(x) => x,
            None => return 0,
        };
        match dnd_data::class(ci) {
            Some(cls) => cls.level_features
                .get(class_local - 1)
                .map_or(0, |f| f.len() as i32),
            None => 0,
        }
    }

    pub fn level_feature_name(self: Pin<&mut Self>, level: i32, feat: i32) -> QString {
        if level < 1 || feat < 0 { return QString::from(""); }
        let (ci, class_local) = match Self::level_info(&self.as_ref().rust().state, level as usize) {
            Some(x) => x,
            None => return QString::from(""),
        };
        match dnd_data::class(ci) {
            Some(cls) => cls.level_features
                .get(class_local - 1)
                .and_then(|fs| fs.get(feat as usize))
                .map_or(QString::from(""), |f| QString::from(f.name)),
            None => QString::from(""),
        }
    }

    pub fn level_feature_description(self: Pin<&mut Self>, level: i32, feat: i32) -> QString {
        if level < 1 || feat < 0 { return QString::from(""); }
        let (ci, class_local) = match Self::level_info(&self.as_ref().rust().state, level as usize) {
            Some(x) => x,
            None => return QString::from(""),
        };
        match dnd_data::class(ci) {
            Some(cls) => cls.level_features
                .get(class_local - 1)
                .and_then(|fs| fs.get(feat as usize))
                .map_or(QString::from(""), |f| QString::from(f.description)),
            None => QString::from(""),
        }
    }

    pub fn num_subclasses_for_class_idx(self: Pin<&mut Self>, class_idx: i32) -> i32 {
        if class_idx < 0 { return 0; }
        match dnd_data::class(class_idx as usize) {
            Some(cls) => cls.subclasses.len() as i32,
            None => 0,
        }
    }

    pub fn subclass_name_for_class_idx(self: Pin<&mut Self>, class_idx: i32, idx: i32) -> QString {
        if class_idx < 0 || idx < 0 { return QString::from(""); }
        match dnd_data::class(class_idx as usize) {
            Some(cls) => match cls.subclasses.get(idx as usize) {
                Some(&name) => QString::from(name),
                None => QString::from(""),
            },
            None => QString::from(""),
        }
    }

    // ── Class choice-feature implementation ───────────────────────────────────

    /// Given a class index and class-local level, find the nth matching ClassChoiceSet.
    fn choice_set_for(ci: usize, class_local: usize, set_idx: usize) -> Option<(&'static ClassChoiceSet, usize)> {
        let cls = dnd_data::class(ci)?;
        let mut nth = 0usize;
        for (gi, s) in cls.class_choices.iter().enumerate() {
            if s.class_level as usize == class_local {
                if nth == set_idx { return Some((s, gi)); }
                nth += 1;
            }
        }
        None
    }

    pub fn level_num_choice_sets(self: Pin<&mut Self>, level: i32) -> i32 {
        if level < 1 { return 0; }
        let (ci, class_local) = match Self::level_info(&self.as_ref().rust().state, level as usize) { Some(x) => x, None => return 0 };
        match dnd_data::class(ci) {
            Some(cls) => cls.class_choices.iter().filter(|s| s.class_level as usize == class_local).count() as i32,
            None => 0,
        }
    }

    pub fn level_num_choice_features(self: Pin<&mut Self>, level: i32, set_idx: i32) -> i32 {
        if level < 1 || set_idx < 0 { return 0; }
        let (ci, cl) = match Self::level_info(&self.as_ref().rust().state, level as usize) { Some(x) => x, None => return 0 };
        match Self::choice_set_for(ci, cl, set_idx as usize) { Some((s, _)) => s.features.len() as i32, None => 0 }
    }

    pub fn level_choice_feature_name(self: Pin<&mut Self>, level: i32, set_idx: i32, feat_idx: i32) -> QString {
        if level < 1 || set_idx < 0 || feat_idx < 0 { return QString::from(""); }
        let (ci, cl) = match Self::level_info(&self.as_ref().rust().state, level as usize) { Some(x) => x, None => return QString::from("") };
        match Self::choice_set_for(ci, cl, set_idx as usize) {
            Some((s, _)) => s.features.get(feat_idx as usize).map_or(QString::from(""), |f| QString::from(f.name)),
            None => QString::from(""),
        }
    }

    pub fn level_choice_feature_desc(self: Pin<&mut Self>, level: i32, set_idx: i32, feat_idx: i32) -> QString {
        if level < 1 || set_idx < 0 || feat_idx < 0 { return QString::from(""); }
        let (ci, cl) = match Self::level_info(&self.as_ref().rust().state, level as usize) { Some(x) => x, None => return QString::from("") };
        match Self::choice_set_for(ci, cl, set_idx as usize) {
            Some((s, _)) => s.features.get(feat_idx as usize).map_or(QString::from(""), |f| QString::from(f.description)),
            None => QString::from(""),
        }
    }

    pub fn level_choice_feature_num_picks(self: Pin<&mut Self>, level: i32, set_idx: i32, feat_idx: i32) -> i32 {
        if level < 1 || set_idx < 0 || feat_idx < 0 { return 0; }
        let (ci, cl) = match Self::level_info(&self.as_ref().rust().state, level as usize) { Some(x) => x, None => return 0 };
        match Self::choice_set_for(ci, cl, set_idx as usize) {
            Some((s, _)) => s.features.get(feat_idx as usize).map_or(0, |f| f.num_picks as i32),
            None => 0,
        }
    }

    pub fn level_choice_feature_num_options(self: Pin<&mut Self>, level: i32, set_idx: i32, feat_idx: i32) -> i32 {
        if level < 1 || set_idx < 0 || feat_idx < 0 { return 0; }
        let (ci, cl) = match Self::level_info(&self.as_ref().rust().state, level as usize) { Some(x) => x, None => return 0 };
        match Self::choice_set_for(ci, cl, set_idx as usize) {
            Some((s, _)) => s.features.get(feat_idx as usize).map_or(0, |f| f.options.len() as i32),
            None => 0,
        }
    }

    pub fn level_choice_option_name(self: Pin<&mut Self>, level: i32, set_idx: i32, feat_idx: i32, opt_idx: i32) -> QString {
        if level < 1 || set_idx < 0 || feat_idx < 0 || opt_idx < 0 { return QString::from(""); }
        let (ci, cl) = match Self::level_info(&self.as_ref().rust().state, level as usize) { Some(x) => x, None => return QString::from("") };
        match Self::choice_set_for(ci, cl, set_idx as usize) {
            Some((s, _)) => s.features.get(feat_idx as usize).and_then(|f| f.options.get(opt_idx as usize)).map_or(QString::from(""), |o| QString::from(o.name)),
            None => QString::from(""),
        }
    }

    pub fn level_choice_option_desc(self: Pin<&mut Self>, level: i32, set_idx: i32, feat_idx: i32, opt_idx: i32) -> QString {
        if level < 1 || set_idx < 0 || feat_idx < 0 || opt_idx < 0 { return QString::from(""); }
        let (ci, cl) = match Self::level_info(&self.as_ref().rust().state, level as usize) { Some(x) => x, None => return QString::from("") };
        match Self::choice_set_for(ci, cl, set_idx as usize) {
            Some((s, _)) => s.features.get(feat_idx as usize).and_then(|f| f.options.get(opt_idx as usize)).map_or(QString::from(""), |o| QString::from(o.description)),
            None => QString::from(""),
        }
    }

    pub fn level_choice_option_selected(self: Pin<&mut Self>, level: i32, set_idx: i32, feat_idx: i32, opt_idx: i32) -> bool {
        if level < 1 || set_idx < 0 || feat_idx < 0 || opt_idx < 0 { return false; }
        let (ci, cl) = match Self::level_info(&self.as_ref().rust().state, level as usize) { Some(x) => x, None => return false };
        let gi = match Self::choice_set_for(ci, cl, set_idx as usize) { Some((_, g)) => g as u8, None => return false };
        let lv_idx = (level - 1) as usize;
        let fi = feat_idx as u8; let oi = opt_idx as u8;
        self.as_ref().rust().state.levels.get(lv_idx).map_or(false, |entry| {
            entry.choices.iter().any(|c| matches!(c, LevelChoice::FeaturePick { choice_set_idx, feature_idx, option_idx } if *choice_set_idx == gi && *feature_idx == fi && *option_idx == oi))
        })
    }

    pub fn level_choice_option_toggle(mut self: Pin<&mut Self>, level: i32, set_idx: i32, feat_idx: i32, opt_idx: i32) {
        if level < 1 || set_idx < 0 || feat_idx < 0 || opt_idx < 0 { return; }
        let (ci, cl) = match Self::level_info(&self.as_ref().rust().state, level as usize) { Some(x) => x, None => return };
        let (gi, num_picks) = match Self::choice_set_for(ci, cl, set_idx as usize) {
            Some((s, g)) => (g as u8, s.features.get(feat_idx as usize).map_or(1u8, |f| f.num_picks)),
            None => return,
        };
        let lv_idx = (level - 1) as usize;
        let fi = feat_idx as u8; let oi = opt_idx as u8;
        let already = self.as_ref().rust().state.levels.get(lv_idx).map_or(false, |entry| {
            entry.choices.iter().any(|c| matches!(c, LevelChoice::FeaturePick { choice_set_idx, feature_idx, option_idx } if *choice_set_idx == gi && *feature_idx == fi && *option_idx == oi))
        });
        if already {
            self.as_mut().rust_mut().state.levels[lv_idx].choices.retain(|c| !matches!(c, LevelChoice::FeaturePick { choice_set_idx, feature_idx, option_idx } if *choice_set_idx == gi && *feature_idx == fi && *option_idx == oi));
        } else {
            let current_picks: usize = self.as_ref().rust().state.levels[lv_idx].choices.iter()
                .filter(|c| matches!(c, LevelChoice::FeaturePick { choice_set_idx, feature_idx, .. } if *choice_set_idx == gi && *feature_idx == fi))
                .count();
            if current_picks >= num_picks as usize {
                let mut removed = false;
                self.as_mut().rust_mut().state.levels[lv_idx].choices.retain(|c| {
                    if !removed && matches!(c, LevelChoice::FeaturePick { choice_set_idx, feature_idx, .. } if *choice_set_idx == gi && *feature_idx == fi) {
                        removed = true; return false;
                    }
                    true
                });
            }
            self.as_mut().rust_mut().state.levels[lv_idx].choices.push(LevelChoice::FeaturePick { choice_set_idx: gi, feature_idx: fi, option_idx: oi });
        }
        let v = *self.as_ref().selectionVersion() + 1; self.as_mut().set_selectionVersion(v);
    }

    pub fn level_choice_clear(mut self: Pin<&mut Self>, level: i32, set_idx: i32, feat_idx: i32) {
        if level < 1 || set_idx < 0 || feat_idx < 0 { return; }
        let (ci, cl) = match Self::level_info(&self.as_ref().rust().state, level as usize) { Some(x) => x, None => return };
        let gi = match Self::choice_set_for(ci, cl, set_idx as usize) { Some((_, g)) => g as u8, None => return };
        let lv_idx = (level - 1) as usize; let fi = feat_idx as u8;
        self.as_mut().rust_mut().state.levels[lv_idx].choices.retain(|c| !matches!(c, LevelChoice::FeaturePick { choice_set_idx, feature_idx, .. } if *choice_set_idx == gi && *feature_idx == fi));
        let v = *self.as_ref().selectionVersion() + 1; self.as_mut().set_selectionVersion(v);
    }

    // ── Race trait / choice-feature implementation ────────────────────────────

    fn race_traits_from_idx(race_idx: i32) -> Option<&'static [dnd_data::RaceTrait]> {
        if race_idx < 0 { return None; }
        dnd_data::race(race_idx as usize).map(|r| r.race_traits)
    }

    pub fn num_race_traits(self: Pin<&mut Self>) -> i32 {
        match Self::race_traits_from_idx(*self.as_ref().raceIdx()) {
            Some(traits) => traits.len() as i32,
            None => 0,
        }
    }

    pub fn race_trait_name(self: Pin<&mut Self>, idx: i32) -> QString {
        if idx < 0 { return QString::from(""); }
        match Self::race_traits_from_idx(*self.as_ref().raceIdx()) {
            Some(traits) => traits.get(idx as usize).map_or(QString::from(""), |t| QString::from(t.name)),
            None => QString::from(""),
        }
    }

    pub fn race_trait_desc(self: Pin<&mut Self>, idx: i32) -> QString {
        if idx < 0 { return QString::from(""); }
        match Self::race_traits_from_idx(*self.as_ref().raceIdx()) {
            Some(traits) => traits.get(idx as usize).map_or(QString::from(""), |t| QString::from(t.description)),
            None => QString::from(""),
        }
    }

    pub fn race_trait_num_picks(self: Pin<&mut Self>, idx: i32) -> i32 {
        if idx < 0 { return 0; }
        match Self::race_traits_from_idx(*self.as_ref().raceIdx()) {
            Some(traits) => traits.get(idx as usize).map_or(0, |t| t.num_picks as i32),
            None => 0,
        }
    }

    pub fn race_trait_num_options(self: Pin<&mut Self>, idx: i32) -> i32 {
        if idx < 0 { return 0; }
        match Self::race_traits_from_idx(*self.as_ref().raceIdx()) {
            Some(traits) => traits.get(idx as usize).map_or(0, |t| t.options.len() as i32),
            None => 0,
        }
    }

    pub fn race_trait_option_name(self: Pin<&mut Self>, trait_idx: i32, opt_idx: i32) -> QString {
        if trait_idx < 0 || opt_idx < 0 { return QString::from(""); }
        match Self::race_traits_from_idx(*self.as_ref().raceIdx()) {
            Some(traits) => traits.get(trait_idx as usize)
                .and_then(|t| t.options.get(opt_idx as usize))
                .map_or(QString::from(""), |o| QString::from(o.name)),
            None => QString::from(""),
        }
    }

    pub fn race_trait_option_desc(self: Pin<&mut Self>, trait_idx: i32, opt_idx: i32) -> QString {
        if trait_idx < 0 || opt_idx < 0 { return QString::from(""); }
        match Self::race_traits_from_idx(*self.as_ref().raceIdx()) {
            Some(traits) => traits.get(trait_idx as usize)
                .and_then(|t| t.options.get(opt_idx as usize))
                .map_or(QString::from(""), |o| QString::from(o.description)),
            None => QString::from(""),
        }
    }

    pub fn race_trait_option_selected(self: Pin<&mut Self>, trait_idx: i32, opt_idx: i32) -> bool {
        if trait_idx < 0 || opt_idx < 0 { return false; }
        self.as_ref().rust().state.race_feature_picks.iter()
            .any(|&(ti, oi)| ti == trait_idx as u8 && oi == opt_idx as u8)
    }

    pub fn race_trait_option_toggle(mut self: Pin<&mut Self>, trait_idx: i32, opt_idx: i32) {
        if trait_idx < 0 || opt_idx < 0 { return; }
        let num_picks = match Self::race_traits_from_idx(*self.as_ref().raceIdx()) {
            Some(traits) => match traits.get(trait_idx as usize) {
                Some(t) if t.num_picks > 0 => t.num_picks,
                _ => return,
            },
            None => return,
        };
        let ti = trait_idx as u8;
        let oi = opt_idx as u8;
        let already = self.as_ref().rust().state.race_feature_picks.iter().any(|&(t, o)| t == ti && o == oi);
        if already {
            self.as_mut().rust_mut().state.race_feature_picks.retain(|&(t, o)| !(t == ti && o == oi));
        } else {
            let current = self.as_ref().rust().state.race_feature_picks.iter().filter(|&&(t, _)| t == ti).count();
            if current >= num_picks as usize {
                // FIFO: remove first pick for this trait
                let mut removed = false;
                self.as_mut().rust_mut().state.race_feature_picks.retain(|&(t, _)| {
                    if !removed && t == ti { removed = true; return false; }
                    true
                });
            }
            self.as_mut().rust_mut().state.race_feature_picks.push((ti, oi));
        }
        let v = *self.as_ref().selectionVersion() + 1; self.as_mut().set_selectionVersion(v);
    }

    pub fn race_trait_clear(mut self: Pin<&mut Self>, trait_idx: i32) {
        if trait_idx < 0 { return; }
        let ti = trait_idx as u8;
        self.as_mut().rust_mut().state.race_feature_picks.retain(|&(t, _)| t != ti);
        let v = *self.as_ref().selectionVersion() + 1; self.as_mut().set_selectionVersion(v);
    }
}
