//! All 12 PHB classes with PHB + Xanathar's + Tasha's subclasses.
use super::types::{Ability, Skill, AbilitySet, SkillSet, ClassData};
use super::class_features::*;
use super::class_choices::*;

pub static CLASSES: &[ClassData] = &[
    // 0 — Barbarian
    ClassData {
        name: "Barbarian",
        hit_die: 12,
        save_profs: AbilitySet::new().set(Ability::Str).set(Ability::Con),
        skill_pool: SkillSet::new()
            .set(Skill::AnimalHandling)
            .set(Skill::Athletics)
            .set(Skill::Intimidation)
            .set(Skill::Perception)
            .set(Skill::Stealth)
            .set(Skill::Survival),
        skill_choices: 2,
        subclasses: &[
            "Path of the Berserker",       // PHB
            "Path of the Totem Warrior",   // PHB
            "Path of the Ancestral Guardian", // Xanathar's
            "Path of the Storm Herald",    // Xanathar's
            "Path of the Zealot",          // Xanathar's
            "Path of the Beast",           // Tasha's
            "Path of Wild Magic",          // Tasha's
        ],
        asi_levels: &[4, 8, 12, 16, 19],
        subclass_level: 3,
        level_features: BARBARIAN_FEATURES,
        class_choices: BARBARIAN_CHOICES,
    },
    // 1 — Bard
    ClassData {
        name: "Bard",
        hit_die: 8,
        save_profs: AbilitySet::new().set(Ability::Dex).set(Ability::Cha),
        skill_pool: SkillSet::new() // any 3 skills
            .set(Skill::Acrobatics).set(Skill::AnimalHandling).set(Skill::Arcana)
            .set(Skill::Athletics).set(Skill::Deception).set(Skill::History)
            .set(Skill::Insight).set(Skill::Intimidation).set(Skill::Investigation)
            .set(Skill::Medicine).set(Skill::Nature).set(Skill::Perception)
            .set(Skill::Performance).set(Skill::Persuasion).set(Skill::Religion)
            .set(Skill::SleightOfHand).set(Skill::Stealth).set(Skill::Survival),
        skill_choices: 3,
        subclasses: &[
            "College of Lore",       // PHB
            "College of Valor",      // PHB
            "College of Glamour",    // Xanathar's
            "College of Swords",     // Xanathar's
            "College of Whispers",   // Xanathar's
            "College of Creation",   // Tasha's
            "College of Eloquence",  // Tasha's
        ],
        asi_levels: &[4, 8, 12, 16, 19],
        subclass_level: 3,
        level_features: BARD_FEATURES,
        class_choices: BARD_CHOICES,
    },
    // 2 — Cleric
    ClassData {
        name: "Cleric",
        hit_die: 8,
        save_profs: AbilitySet::new().set(Ability::Wis).set(Ability::Cha),
        skill_pool: SkillSet::new()
            .set(Skill::History)
            .set(Skill::Insight)
            .set(Skill::Medicine)
            .set(Skill::Perception)
            .set(Skill::Persuasion)
            .set(Skill::Religion),
        skill_choices: 2,
        subclasses: &[
            "Arcana Domain",     // PHB
            "Death Domain",      // PHB
            "Knowledge Domain",  // PHB
            "Life Domain",       // PHB
            "Light Domain",      // PHB
            "Nature Domain",     // PHB
            "Tempest Domain",    // PHB
            "Trickery Domain",   // PHB
            "War Domain",        // PHB
            "Forge Domain",      // Xanathar's
            "Grave Domain",      // Xanathar's
            "Order Domain",      // Tasha's
            "Peace Domain",      // Tasha's
            "Twilight Domain",   // Tasha's
        ],
        asi_levels: &[4, 8, 12, 16, 19],
        subclass_level: 1,
        level_features: CLERIC_FEATURES,
        class_choices: CLERIC_CHOICES,
    },
    // 3 — Druid
    ClassData {
        name: "Druid",
        hit_die: 8,
        save_profs: AbilitySet::new().set(Ability::Int).set(Ability::Wis),
        skill_pool: SkillSet::new()
            .set(Skill::AnimalHandling)
            .set(Skill::Arcana)
            .set(Skill::Insight)
            .set(Skill::Medicine)
            .set(Skill::Nature)
            .set(Skill::Perception)
            .set(Skill::Religion)
            .set(Skill::Survival),
        skill_choices: 2,
        subclasses: &[
            "Circle of the Land",      // PHB
            "Circle of the Moon",      // PHB
            "Circle of Dreams",        // Xanathar's
            "Circle of the Shepherd",  // Xanathar's
            "Circle of Spores",        // Tasha's
            "Circle of Stars",         // Tasha's
            "Circle of Wildfire",      // Tasha's
        ],
        asi_levels: &[4, 8, 12, 16, 19],
        subclass_level: 2,
        level_features: DRUID_FEATURES,
        class_choices: DRUID_CHOICES,
    },
    // 4 — Fighter
    ClassData {
        name: "Fighter",
        hit_die: 10,
        save_profs: AbilitySet::new().set(Ability::Str).set(Ability::Con),
        skill_pool: SkillSet::new()
            .set(Skill::AnimalHandling)
            .set(Skill::Athletics)
            .set(Skill::History)
            .set(Skill::Insight)
            .set(Skill::Intimidation)
            .set(Skill::Perception)
            .set(Skill::Persuasion)
            .set(Skill::Stealth),
        skill_choices: 2,
        subclasses: &[
            "Battle Master",    // PHB
            "Champion",         // PHB
            "Eldritch Knight",  // PHB
            "Arcane Archer",    // Xanathar's
            "Cavalier",         // Xanathar's
            "Samurai",          // Xanathar's
            "Psi Warrior",      // Tasha's
            "Rune Knight",      // Tasha's
        ],
        asi_levels: &[4, 6, 8, 12, 14, 16, 19], // 7 slots
        subclass_level: 3,
        level_features: FIGHTER_FEATURES,
        class_choices: FIGHTER_CHOICES,
    },
    // 5 — Monk
    ClassData {
        name: "Monk",
        hit_die: 8,
        save_profs: AbilitySet::new().set(Ability::Str).set(Ability::Wis),
        skill_pool: SkillSet::new()
            .set(Skill::AnimalHandling)
            .set(Skill::Athletics)
            .set(Skill::History)
            .set(Skill::Insight)
            .set(Skill::Perception)
            .set(Skill::Religion)
            .set(Skill::Stealth)
            .set(Skill::Survival),
        skill_choices: 2,
        subclasses: &[
            "Way of the Four Elements",  // PHB
            "Way of the Open Hand",      // PHB
            "Way of Shadow",             // PHB
            "Way of the Drunken Master", // Xanathar's
            "Way of the Kensei",         // Xanathar's
            "Way of the Sun Soul",       // Xanathar's
            "Way of Mercy",              // Tasha's
            "Way of the Astral Self",    // Tasha's
        ],
        asi_levels: &[4, 8, 12, 16, 19],
        subclass_level: 3,
        level_features: MONK_FEATURES,
        class_choices: MONK_CHOICES,
    },
    // 6 — Paladin
    ClassData {
        name: "Paladin",
        hit_die: 10,
        save_profs: AbilitySet::new().set(Ability::Wis).set(Ability::Cha),
        skill_pool: SkillSet::new()
            .set(Skill::Athletics)
            .set(Skill::Insight)
            .set(Skill::Intimidation)
            .set(Skill::Medicine)
            .set(Skill::Perception)
            .set(Skill::Persuasion)
            .set(Skill::Religion)
            .set(Skill::Survival),
        skill_choices: 2,
        subclasses: &[
            "Oath of Devotion",      // PHB
            "Oath of the Ancients",  // PHB
            "Oath of Vengeance",     // PHB
            "Oathbreaker",           // PHB (DM option)
            "Oath of Conquest",      // Xanathar's
            "Oath of Redemption",    // Xanathar's
            "Oath of Glory",         // Tasha's
            "Oath of the Watchers",  // Tasha's
        ],
        asi_levels: &[4, 8, 12, 16, 19],
        subclass_level: 3,
        level_features: PALADIN_FEATURES,
        class_choices: PALADIN_CHOICES,
    },
    // 7 — Ranger
    ClassData {
        name: "Ranger",
        hit_die: 10,
        save_profs: AbilitySet::new().set(Ability::Str).set(Ability::Dex),
        skill_pool: SkillSet::new()
            .set(Skill::AnimalHandling)
            .set(Skill::Athletics)
            .set(Skill::Insight)
            .set(Skill::Investigation)
            .set(Skill::Nature)       // PHB: Nature, not Religion
            .set(Skill::Perception)
            .set(Skill::Stealth)
            .set(Skill::Survival),
        skill_choices: 3,
        subclasses: &[
            "Beast Master",     // PHB
            "Hunter",           // PHB
            "Gloom Stalker",    // Xanathar's
            "Horizon Walker",   // Xanathar's
            "Monster Slayer",   // Xanathar's
            "Fey Wanderer",     // Tasha's
            "Swarmkeeper",      // Tasha's
        ],
        asi_levels: &[4, 8, 12, 16, 19],
        subclass_level: 3,
        level_features: RANGER_FEATURES,
        class_choices: RANGER_CHOICES,
    },
    // 8 — Rogue
    ClassData {
        name: "Rogue",
        hit_die: 8,
        save_profs: AbilitySet::new().set(Ability::Dex).set(Ability::Int),
        skill_pool: SkillSet::new()
            .set(Skill::Acrobatics)
            .set(Skill::Athletics)
            .set(Skill::Deception)
            .set(Skill::History)
            .set(Skill::Insight)
            .set(Skill::Intimidation)
            .set(Skill::Investigation)
            .set(Skill::Perception)
            .set(Skill::Performance)
            .set(Skill::Persuasion)
            .set(Skill::SleightOfHand)
            .set(Skill::Stealth),
        skill_choices: 4,
        subclasses: &[
            "Arcane Trickster",  // PHB
            "Assassin",          // PHB
            "Thief",             // PHB
            "Inquisitive",       // Xanathar's
            "Mastermind",        // Xanathar's
            "Scout",             // Xanathar's
            "Swashbuckler",      // Xanathar's
            "Phantom",           // Tasha's
            "Soulknife",         // Tasha's
        ],
        asi_levels: &[4, 8, 10, 16, 19], // extra slot at 10
        subclass_level: 3,
        level_features: ROGUE_FEATURES,
        class_choices: ROGUE_CHOICES,
    },
    // 9 — Sorcerer
    ClassData {
        name: "Sorcerer",
        hit_die: 6,
        save_profs: AbilitySet::new().set(Ability::Con).set(Ability::Cha),
        skill_pool: SkillSet::new()
            .set(Skill::Arcana)
            .set(Skill::Deception)
            .set(Skill::Insight)
            .set(Skill::Intimidation)
            .set(Skill::Persuasion)
            .set(Skill::Religion),
        skill_choices: 2,
        subclasses: &[
            "Draconic Bloodline",  // PHB
            "Wild Magic",          // PHB
            "Divine Soul",         // Xanathar's
            "Shadow Magic",        // Xanathar's
            "Storm Sorcery",       // Xanathar's
            "Aberrant Mind",       // Tasha's
            "Clockwork Soul",      // Tasha's
        ],
        asi_levels: &[4, 8, 12, 16, 19],
        subclass_level: 1,
        level_features: SORCERER_FEATURES,
        class_choices: SORCERER_CHOICES,
    },
    // 10 — Warlock
    ClassData {
        name: "Warlock",
        hit_die: 8,
        save_profs: AbilitySet::new().set(Ability::Wis).set(Ability::Cha),
        skill_pool: SkillSet::new()
            .set(Skill::Arcana)
            .set(Skill::Deception)
            .set(Skill::History)
            .set(Skill::Insight)
            .set(Skill::Intimidation)
            .set(Skill::Investigation)
            .set(Skill::Nature)
            .set(Skill::Persuasion)
            .set(Skill::Religion),
        skill_choices: 2,
        subclasses: &[
            "The Archfey",       // PHB
            "The Fiend",         // PHB
            "The Great Old One", // PHB
            "The Celestial",     // Xanathar's
            "The Hexblade",      // Xanathar's
            "The Fathomless",    // Tasha's
            "The Genie",         // Tasha's
        ],
        asi_levels: &[4, 8, 12, 16, 19],
        subclass_level: 1,
        level_features: WARLOCK_FEATURES,
        class_choices: WARLOCK_CHOICES,
    },
    // 11 — Wizard
    ClassData {
        name: "Wizard",
        hit_die: 6,
        save_profs: AbilitySet::new().set(Ability::Int).set(Ability::Wis),
        skill_pool: SkillSet::new()
            .set(Skill::Arcana)
            .set(Skill::History)
            .set(Skill::Insight)
            .set(Skill::Investigation)
            .set(Skill::Nature)
            .set(Skill::Religion),
        skill_choices: 2,
        subclasses: &[
            "School of Abjuration",    // PHB
            "School of Conjuration",   // PHB
            "School of Divination",    // PHB
            "School of Enchantment",   // PHB
            "School of Evocation",     // PHB
            "School of Illusion",      // PHB
            "School of Necromancy",    // PHB
            "School of Transmutation", // PHB
            "War Magic",               // Xanathar's
            "Bladesinging",            // Tasha's
            "Order of Scribes",        // Tasha's
        ],
        asi_levels: &[4, 8, 12, 16, 19],
        subclass_level: 2,
        level_features: WIZARD_FEATURES,
        class_choices: WIZARD_CHOICES,
    },
];
