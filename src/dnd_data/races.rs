//! PHB, Volo's Guide, and Tasha's Cauldron races with typed ASI arrays.
use super::types::{AsiBonus, RaceData};
use super::race_features::*;

pub static RACES: &[RaceData] = &[
    // ── PHB ──────────────────────────────────────────────────────────────────
    // 0 — Human (standard: +1 to all six)
    RaceData {
        name: "Human",
        asi: AsiBonus([1, 1, 1, 1, 1, 1]),
        speed: 30,
        traits: "+1 to all ability scores. Extra language.",
        race_traits: HUMAN_TRAITS,
    },
    // 1 — Dwarf (Hill)
    RaceData {
        name: "Dwarf (Hill)",
        asi: AsiBonus([0, 0, 2, 0, 1, 0]), // +2 CON, +1 WIS
        speed: 25,
        traits: "Darkvision. Dwarven Resilience. Dwarven Combat Training. Dwarven Toughness (+1 HP/level).",
        race_traits: DWARF_HILL_TRAITS,
    },
    // 2 — Dwarf (Mountain)
    RaceData {
        name: "Dwarf (Mountain)",
        asi: AsiBonus([2, 0, 2, 0, 0, 0]), // +2 STR, +2 CON
        speed: 25,
        traits: "Darkvision. Dwarven Resilience. Dwarven Combat Training. Dwarven Armor Training.",
        race_traits: DWARF_MOUNTAIN_TRAITS,
    },
    // 3 — Elf (High)
    RaceData {
        name: "Elf (High)",
        asi: AsiBonus([0, 2, 0, 1, 0, 0]), // +2 DEX, +1 INT
        speed: 30,
        traits: "Darkvision. Fey Ancestry. Trance. Keen Senses (Perception). Elf Weapon Training. Cantrip (INT).",
        race_traits: ELF_HIGH_TRAITS,
    },
    // 4 — Elf (Wood)
    RaceData {
        name: "Elf (Wood)",
        asi: AsiBonus([0, 2, 0, 0, 1, 0]), // +2 DEX, +1 WIS
        speed: 35,
        traits: "Darkvision. Fey Ancestry. Trance. Keen Senses. Elf Weapon Training. Fleet of Foot. Mask of the Wild.",
        race_traits: ELF_WOOD_TRAITS,
    },
    // 5 — Elf (Drow)
    RaceData {
        name: "Elf (Drow)",
        asi: AsiBonus([0, 2, 0, 0, 0, 1]), // +2 DEX, +1 CHA
        speed: 30,
        traits: "Superior Darkvision. Fey Ancestry. Trance. Drow Magic (Faerie Fire, Darkness). Sunlight Sensitivity.",
        race_traits: ELF_DROW_TRAITS,
    },
    // 6 — Halfling (Lightfoot)
    RaceData {
        name: "Halfling (Lightfoot)",
        asi: AsiBonus([0, 2, 0, 0, 0, 1]), // +2 DEX, +1 CHA
        speed: 25,
        traits: "Lucky. Brave. Halfling Nimbleness. Naturally Stealthy.",
        race_traits: HALFLING_LIGHTFOOT_TRAITS,
    },
    // 7 — Halfling (Stout)
    RaceData {
        name: "Halfling (Stout)",
        asi: AsiBonus([0, 2, 1, 0, 0, 0]), // +2 DEX, +1 CON
        speed: 25,
        traits: "Lucky. Brave. Halfling Nimbleness. Stout Resilience (advantage vs poison).",
        race_traits: HALFLING_STOUT_TRAITS,
    },
    // 8 — Human (Variant): +1 to any two — stored as zero; user applies via Stats tab
    RaceData {
        name: "Human (Variant)",
        asi: AsiBonus::ZERO,
        speed: 30,
        traits: "+1 to two ability scores of your choice (apply manually). One skill proficiency. One feat.",
        race_traits: HUMAN_VARIANT_TRAITS,
    },
    // 9 — Dragonborn
    RaceData {
        name: "Dragonborn",
        asi: AsiBonus([2, 0, 0, 0, 0, 1]), // +2 STR, +1 CHA
        speed: 30,
        traits: "Draconic Ancestry. Breath Weapon. Damage Resistance.",
        race_traits: DRAGONBORN_TRAITS,
    },
    // 10 — Gnome (Forest)
    RaceData {
        name: "Gnome (Forest)",
        asi: AsiBonus([0, 1, 0, 2, 0, 0]), // +2 INT, +1 DEX
        speed: 25,
        traits: "Darkvision. Gnome Cunning. Natural Illusionist (Minor Illusion). Speak with Small Beasts.",
        race_traits: GNOME_FOREST_TRAITS,
    },
    // 11 — Gnome (Rock)
    RaceData {
        name: "Gnome (Rock)",
        asi: AsiBonus([0, 0, 1, 2, 0, 0]), // +2 INT, +1 CON
        speed: 25,
        traits: "Darkvision. Gnome Cunning. Artificer's Lore. Tinker.",
        race_traits: GNOME_ROCK_TRAITS,
    },
    // 12 — Half-Elf: +2 CHA + any two +1s (player-chosen +1+1 applied manually)
    RaceData {
        name: "Half-Elf",
        asi: AsiBonus([0, 0, 0, 0, 0, 2]), // +2 CHA; +1+1 to two others applied manually
        speed: 30,
        traits: "Darkvision. Fey Ancestry. Skill Versatility (two skill profs). +1 to two ability scores of your choice (apply manually).",
        race_traits: HALF_ELF_TRAITS,
    },
    // 13 — Half-Orc
    RaceData {
        name: "Half-Orc",
        asi: AsiBonus([2, 0, 1, 0, 0, 0]), // +2 STR, +1 CON
        speed: 30,
        traits: "Darkvision. Menacing (Intimidation proficiency). Relentless Endurance. Savage Attacks.",
        race_traits: HALF_ORC_TRAITS,
    },
    // 14 — Tiefling
    RaceData {
        name: "Tiefling",
        asi: AsiBonus([0, 0, 0, 1, 0, 2]), // +1 INT, +2 CHA
        speed: 30,
        traits: "Darkvision. Hellish Resistance (fire). Infernal Legacy (Thaumaturgy, Hellish Rebuke, Darkness).",
        race_traits: TIEFLING_TRAITS,
    },

    // ── Volo's Guide / Xanathar's ────────────────────────────────────────────
    // 15 — Aasimar (Protector)
    RaceData {
        name: "Aasimar (Protector)",
        asi: AsiBonus([0, 0, 0, 0, 1, 2]), // +1 WIS, +2 CHA
        speed: 30,
        traits: "Darkvision. Celestial Resistance. Healing Hands. Light Bearer. Radiant Soul (wings, radiant damage).",
        race_traits: AASIMAR_PROTECTOR_TRAITS,
    },
    // 16 — Aasimar (Scourge)
    RaceData {
        name: "Aasimar (Scourge)",
        asi: AsiBonus([0, 0, 1, 0, 0, 2]), // +1 CON, +2 CHA
        speed: 30,
        traits: "Darkvision. Celestial Resistance. Healing Hands. Light Bearer. Radiant Consumption.",
        race_traits: AASIMAR_SCOURGE_TRAITS,
    },
    // 17 — Aasimar (Fallen)
    RaceData {
        name: "Aasimar (Fallen)",
        asi: AsiBonus([1, 0, 0, 0, 0, 2]), // +1 STR, +2 CHA
        speed: 30,
        traits: "Darkvision. Celestial Resistance. Healing Hands. Light Bearer. Necrotic Shroud.",
        race_traits: AASIMAR_FALLEN_TRAITS,
    },
    // 18 — Firbolg
    RaceData {
        name: "Firbolg",
        asi: AsiBonus([1, 0, 0, 0, 2, 0]), // +1 STR, +2 WIS
        speed: 30,
        traits: "Firbolg Magic (Detect Magic, Disguise Self). Hidden Step. Powerful Build. Speech of Beast and Leaf.",
        race_traits: FIRBOLG_TRAITS,
    },
    // 19 — Goliath
    RaceData {
        name: "Goliath",
        asi: AsiBonus([2, 0, 1, 0, 0, 0]), // +2 STR, +1 CON
        speed: 30,
        traits: "Natural Athlete (Athletics). Stone's Endurance. Powerful Build. Mountain Born.",
        race_traits: GOLIATH_TRAITS,
    },
    // 20 — Kenku
    RaceData {
        name: "Kenku",
        asi: AsiBonus([0, 2, 0, 0, 1, 0]), // +2 DEX, +1 WIS
        speed: 30,
        traits: "+2 DEX, +1 WIS. Expert Forgery. Kenku Training (two skills from pool). Mimicry.",
        race_traits: KENKU_TRAITS,
    },
    // 21 — Lizardfolk
    RaceData {
        name: "Lizardfolk",
        asi: AsiBonus([0, 0, 2, 0, 1, 0]), // +2 CON, +1 WIS
        speed: 30,
        traits: "Bite attack. Cunning Artisan. Hold Breath. Hunter's Lore (two skills). Natural Armor (AC 13+DEX). Hungry Jaws.",
        race_traits: LIZARDFOLK_TRAITS,
    },
    // 22 — Tabaxi
    RaceData {
        name: "Tabaxi",
        asi: AsiBonus([0, 2, 0, 0, 0, 1]), // +2 DEX, +1 CHA
        speed: 30,
        traits: "Darkvision. Feline Agility (double speed once/rest). Cat's Claws (climb, claw attack). Cat's Talents (Perception, Stealth).",
        race_traits: TABAXI_TRAITS,
    },
    // 23 — Triton
    RaceData {
        name: "Triton",
        asi: AsiBonus([1, 0, 1, 0, 0, 1]), // +1 STR, +1 CON, +1 CHA
        speed: 30,
        traits: "Amphibious. Control Air and Water. Darkvision. Emissary of the Sea. Guardians of the Depths (cold resistance).",
        race_traits: TRITON_TRAITS,
    },
    // 24 — Bugbear
    RaceData {
        name: "Bugbear",
        asi: AsiBonus([2, 0, 0, 0, 0, 1]), // +2 STR, +1 CHA
        speed: 30,
        traits: "Darkvision. Long-Limbed (+5 ft melee reach). Powerful Build. Sneaky (Stealth). Surprise Attack.",
        race_traits: BUGBEAR_TRAITS,
    },
    // 25 — Goblin
    RaceData {
        name: "Goblin",
        asi: AsiBonus([0, 2, 1, 0, 0, 0]), // +2 DEX, +1 CON
        speed: 30,
        traits: "Darkvision. Fury of the Small. Nimble Escape.",
        race_traits: GOBLIN_TRAITS,
    },
    // 26 — Hobgoblin
    RaceData {
        name: "Hobgoblin",
        asi: AsiBonus([0, 0, 2, 1, 0, 0]), // +2 CON, +1 INT
        speed: 30,
        traits: "Darkvision. Martial Training (light/medium armor, two martial weapons). Saving Face.",
        race_traits: HOBGOBLIN_TRAITS,
    },
    // 27 — Kobold
    RaceData {
        name: "Kobold",
        asi: AsiBonus([-2, 2, 0, 0, 0, 0]), // -2 STR, +2 DEX
        speed: 30,
        traits: "-2 STR, +2 DEX. Darkvision. Grovel Cower and Beg. Pack Tactics. Sunlight Sensitivity.",
        race_traits: KOBOLD_TRAITS,
    },
    // 28 — Orc (Volo's)
    RaceData {
        name: "Orc",
        asi: AsiBonus([2, 0, 1, -2, 0, 0]), // +2 STR, +1 CON, -2 INT
        speed: 30,
        traits: "Darkvision. Aggressive (bonus action to move toward enemy). Menacing (Intimidation). Powerful Build.",
        race_traits: ORC_TRAITS,
    },
    // 29 — Yuan-ti Pureblood
    RaceData {
        name: "Yuan-ti Pureblood",
        asi: AsiBonus([0, 0, 0, 1, 0, 2]), // +1 INT, +2 CHA
        speed: 30,
        traits: "Darkvision. Innate Spellcasting (Animal Friendship, Poison Spray, Suggestion). Magic Resistance. Poison Immunity.",
        race_traits: YUAN_TI_TRAITS,
    },
    // 30 — Tasha's Custom Lineage: +2 to any one (user applies manually)
    RaceData {
        name: "Custom Lineage",
        asi: AsiBonus::ZERO,
        speed: 30,
        traits: "+2 to one ability score of your choice (apply manually). Darkvision OR one skill proficiency. One feat.",
        race_traits: CUSTOM_LINEAGE_TRAITS,
    },
];
