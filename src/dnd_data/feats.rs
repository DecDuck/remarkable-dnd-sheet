//! PHB, Xanathar's, and Tasha's feats with typed ASI arrays.
use super::types::{AsiBonus, FeatData};

pub static FEATS: &[FeatData] = &[
    // ── PHB ──────────────────────────────────────────────────────────────────
    // 0 — Alert
    FeatData {
        name: "Alert",
        asi: AsiBonus::ZERO,
        description: "+5 to initiative. Can't be surprised while conscious. Hidden creatures gain no advantage on attacks against you.",
    },
    // 1 — Actor
    FeatData {
        name: "Actor",
        asi: AsiBonus([0, 0, 0, 0, 0, 1]), // +1 CHA
        description: "+1 CHA. Advantage on Deception/Performance when disguised. Mimic speech.",
    },
    // 2 — Athlete
    FeatData {
        name: "Athlete",
        asi: AsiBonus([1, 0, 0, 0, 0, 0]), // +1 STR or DEX (stored as STR; adjust manually)
        description: "+1 STR or DEX. Stand up using 5 ft of movement. Climbing costs no extra movement. Running long/high jump with 5 ft run-up.",
    },
    // 3 — Charger
    FeatData {
        name: "Charger",
        asi: AsiBonus::ZERO,
        description: "After Dash action, bonus-action attack (+5 damage) or shove up to 10 ft.",
    },
    // 4 — Crossbow Expert
    FeatData {
        name: "Crossbow Expert",
        asi: AsiBonus::ZERO,
        description: "Ignore loading. No disadvantage within 5 ft. Bonus-action hand crossbow attack.",
    },
    // 5 — Defensive Duelist
    FeatData {
        name: "Defensive Duelist",
        asi: AsiBonus::ZERO,
        description: "Prereq: DEX 13. Reaction to add proficiency bonus to AC vs a finesse weapon you're proficient with.",
    },
    // 6 — Dual Wielder
    FeatData {
        name: "Dual Wielder",
        asi: AsiBonus::ZERO,
        description: "+1 AC when wielding two melee weapons. TWF with non-light weapons. Draw/stow two weapons simultaneously.",
    },
    // 7 — Dungeon Delver
    FeatData {
        name: "Dungeon Delver",
        asi: AsiBonus::ZERO,
        description: "Advantage on Perception/Investigation for secret doors. Advantage on saves vs traps. Resistance to trap damage. Can search for traps at normal pace.",
    },
    // 8 — Durable
    FeatData {
        name: "Durable",
        asi: AsiBonus([0, 0, 1, 0, 0, 0]), // +1 CON
        description: "+1 CON. Minimum of 2×CON modifier (min 2) on hit die recovery.",
    },
    // 9 — Elemental Adept (Fire)
    FeatData {
        name: "Elemental Adept (Fire)",
        asi: AsiBonus::ZERO,
        description: "Prereq: spellcaster. Fire spells ignore resistance. Treat 1s as 2s on fire damage dice.",
    },
    // 10 — Elemental Adept (Acid)
    FeatData {
        name: "Elemental Adept (Acid)",
        asi: AsiBonus::ZERO,
        description: "Prereq: spellcaster. Acid spells ignore resistance. Treat 1s as 2s on acid damage dice.",
    },
    // 11 — Elemental Adept (Cold)
    FeatData {
        name: "Elemental Adept (Cold)",
        asi: AsiBonus::ZERO,
        description: "Prereq: spellcaster. Cold spells ignore resistance. Treat 1s as 2s on cold damage dice.",
    },
    // 12 — Elemental Adept (Lightning)
    FeatData {
        name: "Elemental Adept (Lightning)",
        asi: AsiBonus::ZERO,
        description: "Prereq: spellcaster. Lightning spells ignore resistance. Treat 1s as 2s on lightning damage dice.",
    },
    // 13 — Elemental Adept (Thunder)
    FeatData {
        name: "Elemental Adept (Thunder)",
        asi: AsiBonus::ZERO,
        description: "Prereq: spellcaster. Thunder spells ignore resistance. Treat 1s as 2s on thunder damage dice.",
    },
    // 14 — Grappler
    FeatData {
        name: "Grappler",
        asi: AsiBonus::ZERO,
        description: "Prereq: STR 13. Advantage on attacks vs a creature you're grappling. Can use action to pin (both restrained).",
    },
    // 15 — Great Weapon Master
    FeatData {
        name: "Great Weapon Master",
        asi: AsiBonus::ZERO,
        description: "Crit or kill → bonus-action attack. Can take -5 attack for +10 damage with heavy weapons.",
    },
    // 16 — Healer
    FeatData {
        name: "Healer",
        asi: AsiBonus::ZERO,
        description: "With healer's kit: stabilize as bonus action. Restore 1d6+4+max-HD HP (each creature once/short rest).",
    },
    // 17 — Heavily Armored
    FeatData {
        name: "Heavily Armored",
        asi: AsiBonus([1, 0, 0, 0, 0, 0]), // +1 STR
        description: "Prereq: medium armor proficiency. +1 STR. Gain heavy armor proficiency.",
    },
    // 18 — Heavy Armor Master
    FeatData {
        name: "Heavy Armor Master",
        asi: AsiBonus([1, 0, 0, 0, 0, 0]), // +1 STR
        description: "Prereq: heavy armor proficiency. +1 STR. While in heavy armor, reduce nonmagical B/P/S damage by 3.",
    },
    // 19 — Inspiring Leader
    FeatData {
        name: "Inspiring Leader",
        asi: AsiBonus::ZERO,
        description: "Prereq: CHA 13. 10-min speech grants temp HP = level + CHA mod to up to 6 allies (once/short rest).",
    },
    // 20 — Keen Mind
    FeatData {
        name: "Keen Mind",
        asi: AsiBonus([0, 0, 0, 1, 0, 0]), // +1 INT
        description: "+1 INT. Always know north/elapsed time. Can recall anything seen/heard within a month.",
    },
    // 21 — Lightly Armored
    FeatData {
        name: "Lightly Armored",
        asi: AsiBonus([1, 0, 0, 0, 0, 0]), // +1 STR or DEX
        description: "+1 STR or DEX. Gain light armor proficiency.",
    },
    // 22 — Linguist
    FeatData {
        name: "Linguist",
        asi: AsiBonus([0, 0, 0, 1, 0, 0]), // +1 INT
        description: "+1 INT. Learn 3 languages. Create ciphers (DC = INT + proficiency).",
    },
    // 23 — Lucky
    FeatData {
        name: "Lucky",
        asi: AsiBonus::ZERO,
        description: "3 luck points/long rest. Spend to roll extra d20 on attack/ability/save or force enemy reroll.",
    },
    // 24 — Mage Slayer
    FeatData {
        name: "Mage Slayer",
        asi: AsiBonus::ZERO,
        description: "Reaction attack when nearby creature casts spell. Concentration saves from your attacks at disadvantage. Advantage on saves vs adjacent casters.",
    },
    // 25 — Magic Initiate (Bard)
    FeatData {
        name: "Magic Initiate (Bard)",
        asi: AsiBonus::ZERO,
        description: "Learn 2 bard cantrips and one 1st-level bard spell (once/long rest without slot).",
    },
    // 26 — Magic Initiate (Cleric)
    FeatData {
        name: "Magic Initiate (Cleric)",
        asi: AsiBonus::ZERO,
        description: "Learn 2 cleric cantrips and one 1st-level cleric spell (once/long rest without slot).",
    },
    // 27 — Magic Initiate (Druid)
    FeatData {
        name: "Magic Initiate (Druid)",
        asi: AsiBonus::ZERO,
        description: "Learn 2 druid cantrips and one 1st-level druid spell (once/long rest without slot).",
    },
    // 28 — Martial Adept
    FeatData {
        name: "Martial Adept",
        asi: AsiBonus::ZERO,
        description: "Learn 2 Battle Master maneuvers. Gain 1 superiority die (d6).",
    },
    // 29 — Medium Armor Master
    FeatData {
        name: "Medium Armor Master",
        asi: AsiBonus::ZERO,
        description: "Prereq: medium armor proficiency. No Stealth disadvantage in medium armor. Max DEX bonus = +3.",
    },
    // 30 — Mobile
    FeatData {
        name: "Mobile",
        asi: AsiBonus::ZERO,
        description: "+10 ft speed. Dash ignores difficult terrain. No opportunity attacks from creatures you attack (hit or miss).",
    },
    // 31 — Moderately Armored
    FeatData {
        name: "Moderately Armored",
        asi: AsiBonus([1, 0, 0, 0, 0, 0]), // +1 STR or DEX
        description: "Prereq: light armor proficiency. +1 STR or DEX. Gain medium armor and shield proficiency.",
    },
    // 32 — Mounted Combatant
    FeatData {
        name: "Mounted Combatant",
        asi: AsiBonus::ZERO,
        description: "Advantage on attacks vs unmounted creatures smaller than mount. Force attacks on mount to target you. Mount saves vs no damage.",
    },
    // 33 — Observant
    FeatData {
        name: "Observant",
        asi: AsiBonus([0, 0, 0, 1, 0, 0]), // +1 INT or WIS (stored as INT)
        description: "+1 INT or WIS. Read lips. +5 to passive Perception and passive Investigation.",
    },
    // 34 — Polearm Master
    FeatData {
        name: "Polearm Master",
        asi: AsiBonus::ZERO,
        description: "Bonus-action butt attack with polearms (d4 bludgeoning). Opportunity attack when creature enters reach.",
    },
    // 35 — Resilient (STR)
    FeatData {
        name: "Resilient (STR)",
        asi: AsiBonus([1, 0, 0, 0, 0, 0]),
        description: "+1 STR. Gain proficiency in STR saving throws.",
    },
    // 36 — Resilient (DEX)
    FeatData {
        name: "Resilient (DEX)",
        asi: AsiBonus([0, 1, 0, 0, 0, 0]),
        description: "+1 DEX. Gain proficiency in DEX saving throws.",
    },
    // 37 — Resilient (CON)
    FeatData {
        name: "Resilient (CON)",
        asi: AsiBonus([0, 0, 1, 0, 0, 0]),
        description: "+1 CON. Gain proficiency in CON saving throws.",
    },
    // 38 — Resilient (INT)
    FeatData {
        name: "Resilient (INT)",
        asi: AsiBonus([0, 0, 0, 1, 0, 0]),
        description: "+1 INT. Gain proficiency in INT saving throws.",
    },
    // 39 — Resilient (WIS)
    FeatData {
        name: "Resilient (WIS)",
        asi: AsiBonus([0, 0, 0, 0, 1, 0]),
        description: "+1 WIS. Gain proficiency in WIS saving throws.",
    },
    // 40 — Resilient (CHA)
    FeatData {
        name: "Resilient (CHA)",
        asi: AsiBonus([0, 0, 0, 0, 0, 1]),
        description: "+1 CHA. Gain proficiency in CHA saving throws.",
    },
    // 41 — Ritual Caster
    FeatData {
        name: "Ritual Caster",
        asi: AsiBonus::ZERO,
        description: "Prereq: INT or WIS 13. Learn 2 ritual spells of chosen class. Can copy rituals into book.",
    },
    // 42 — Savage Attacker
    FeatData {
        name: "Savage Attacker",
        asi: AsiBonus::ZERO,
        description: "Once per turn, reroll melee damage dice and use either result.",
    },
    // 43 — Sentinel
    FeatData {
        name: "Sentinel",
        asi: AsiBonus::ZERO,
        description: "Opportunity attacks reduce speed to 0. Can make OA even if target Disengages. Reaction attack vs creature attacking your ally.",
    },
    // 44 — Sharpshooter
    FeatData {
        name: "Sharpshooter",
        asi: AsiBonus::ZERO,
        description: "Ignore long range penalty. Ignore half/three-quarters cover. -5 attack for +10 damage with ranged weapons.",
    },
    // 45 — Shield Master
    FeatData {
        name: "Shield Master",
        asi: AsiBonus::ZERO,
        description: "Bonus-action shove when you attack. Add shield bonus to solo-target DEX saves. No damage on successful DEX save if proficient in shield.",
    },
    // 46 — Skilled
    FeatData {
        name: "Skilled",
        asi: AsiBonus::ZERO,
        description: "Gain proficiency in any combination of 3 skills or tools.",
    },
    // 47 — Skulker
    FeatData {
        name: "Skulker",
        asi: AsiBonus::ZERO,
        description: "Prereq: DEX 13. Can hide when lightly obscured. Missing ranged attack doesn't reveal you. Dim light doesn't impose disadvantage on Perception.",
    },
    // 48 — Spell Sniper
    FeatData {
        name: "Spell Sniper",
        asi: AsiBonus::ZERO,
        description: "Prereq: spellcaster. Double range of attack-roll spells. Ignore half/three-quarters cover. Learn one attack-roll cantrip.",
    },
    // 49 — Tavern Brawler
    FeatData {
        name: "Tavern Brawler",
        asi: AsiBonus([1, 0, 0, 0, 0, 0]), // +1 STR or CON (stored as STR)
        description: "+1 STR or CON. Proficient with improvised weapons and unarmed strikes (d4). Grapple as bonus action after hit.",
    },
    // 50 — Tough
    FeatData {
        name: "Tough",
        asi: AsiBonus::ZERO,
        description: "Max HP increases by 2×level, and +2 HP per level thereafter.",
    },
    // 51 — War Caster
    FeatData {
        name: "War Caster",
        asi: AsiBonus::ZERO,
        description: "Prereq: spellcaster. Advantage on concentration checks. Can perform somatic components while holding weapons/shield. OA spells.",
    },
    // 52 — Weapon Master
    FeatData {
        name: "Weapon Master",
        asi: AsiBonus([1, 0, 0, 0, 0, 0]), // +1 STR or DEX
        description: "+1 STR or DEX. Gain proficiency with 4 weapons of your choice.",
    },

    // ── Xanathar's Guide ─────────────────────────────────────────────────────
    // 53 — Bountiful Luck (Halfling)
    FeatData {
        name: "Bountiful Luck",
        asi: AsiBonus::ZERO,
        description: "Prereq: halfling. When an ally within 30 ft rolls a 1, use your reaction to let them reroll.",
    },
    // 54 — Dragon Fear (Dragonborn)
    FeatData {
        name: "Dragon Fear",
        asi: AsiBonus::ZERO, // +1 STR, CON, or CHA — apply manually
        description: "Prereq: dragonborn. +1 STR, CON, or CHA (apply manually). Breath weapon can frighten instead of deal damage.",
    },
    // 55 — Dragon Hide (Dragonborn)
    FeatData {
        name: "Dragon Hide",
        asi: AsiBonus::ZERO, // +1 STR, CON, or CHA — apply manually
        description: "Prereq: dragonborn. +1 STR, CON, or CHA (apply manually). Natural armor AC = 13+DEX. Unarmed strike = d4 piercing.",
    },
    // 56 — Drow High Magic (Drow)
    FeatData {
        name: "Drow High Magic",
        asi: AsiBonus::ZERO,
        description: "Prereq: elf (drow). Detect Magic at will. Levitate and Dispel Magic each once/long rest (CHA-based).",
    },
    // 57 — Dwarven Fortitude (Dwarf)
    FeatData {
        name: "Dwarven Fortitude",
        asi: AsiBonus([0, 0, 1, 0, 0, 0]), // +1 CON
        description: "Prereq: dwarf. +1 CON. When you Dodge, spend a hit die to heal.",
    },
    // 58 — Elven Accuracy (Elf/Half-Elf)
    FeatData {
        name: "Elven Accuracy",
        asi: AsiBonus::ZERO, // +1 DEX, INT, WIS, or CHA — apply manually
        description: "Prereq: elf or half-elf. +1 DEX, INT, WIS, or CHA (apply manually). When you have advantage on an attack, roll 3 dice and take the highest.",
    },
    // 59 — Fade Away (Gnome)
    FeatData {
        name: "Fade Away",
        asi: AsiBonus::ZERO, // +1 DEX or INT — apply manually
        description: "Prereq: gnome. +1 DEX or INT (apply manually). Reaction to become invisible when you take damage (until end of next turn, once/short rest).",
    },
    // 60 — Fey Teleportation (High Elf)
    FeatData {
        name: "Fey Teleportation",
        asi: AsiBonus::ZERO, // +1 INT or CHA — apply manually
        description: "Prereq: high elf. +1 INT or CHA (apply manually). Learn Sylvan. Once/short rest, Misty Step as bonus action.",
    },
    // 61 — Flames of Phlegethos (Tiefling)
    FeatData {
        name: "Flames of Phlegethos",
        asi: AsiBonus::ZERO, // +1 INT or CHA — apply manually
        description: "Prereq: tiefling. +1 INT or CHA (apply manually). Reroll 1s on fire damage. Wreath in fire (dim light 10 ft, 1 fire to touching attackers).",
    },
    // 62 — Infernal Constitution (Tiefling)
    FeatData {
        name: "Infernal Constitution",
        asi: AsiBonus([0, 0, 1, 0, 0, 0]), // +1 CON
        description: "Prereq: tiefling. +1 CON. Resistance to cold and poison damage. Advantage vs poisoned condition.",
    },
    // 63 — Orcish Fury (Half-Orc)
    FeatData {
        name: "Orcish Fury",
        asi: AsiBonus::ZERO, // +1 STR or CON — apply manually
        description: "Prereq: half-orc. +1 STR or CON (apply manually). Extra damage die on crits with simple/martial weapon. Reaction attack when Relentless Endurance activates.",
    },
    // 64 — Prodigy (Half-Elf, Half-Orc, or Human)
    FeatData {
        name: "Prodigy",
        asi: AsiBonus::ZERO,
        description: "Prereq: half-elf, half-orc, or human. Gain one skill, one tool, one language, and expertise in one skill you're already proficient in.",
    },
    // 65 — Second Chance (Halfling)
    FeatData {
        name: "Second Chance",
        asi: AsiBonus::ZERO, // +1 DEX, CON, or CHA — apply manually
        description: "Prereq: halfling. +1 DEX, CON, or CHA (apply manually). Reaction to force attacker to reroll once/short rest.",
    },
    // 66 — Squat Nimbleness (Dwarf or Small race)
    FeatData {
        name: "Squat Nimbleness",
        asi: AsiBonus::ZERO, // +1 STR or DEX — apply manually
        description: "Prereq: dwarf or small race. +1 STR or DEX (apply manually). +5 ft speed. Proficiency in Athletics or Acrobatics. Advantage to escape grapple.",
    },
    // 67 — Wood Elf Magic (Wood Elf)
    FeatData {
        name: "Wood Elf Magic",
        asi: AsiBonus::ZERO,
        description: "Prereq: wood elf. Learn Longstrider and Pass Without Trace (once/short rest each), plus one druid cantrip.",
    },

    // ── Tasha's Cauldron ─────────────────────────────────────────────────────
    // 68 — Fey Touched
    FeatData {
        name: "Fey Touched",
        asi: AsiBonus::ZERO, // +1 INT, WIS, or CHA — apply manually
        description: "+1 INT, WIS, or CHA (apply manually). Learn Misty Step and one 1st-level divination/enchantment spell (once/long rest each without slot).",
    },
    // 69 — Fighting Initiate
    FeatData {
        name: "Fighting Initiate",
        asi: AsiBonus::ZERO,
        description: "Prereq: weapon proficiency. Learn one Fighting Style option from the Fighter list.",
    },
    // 70 — Gunner
    FeatData {
        name: "Gunner",
        asi: AsiBonus([0, 1, 0, 0, 0, 0]), // +1 DEX
        description: "+1 DEX. Proficiency with firearms. Ignore loading for firearms. No disadvantage in melee with ranged firearms.",
    },
    // 71 — Metamagic Adept
    FeatData {
        name: "Metamagic Adept",
        asi: AsiBonus::ZERO,
        description: "Prereq: spellcaster. Learn 2 Metamagic options. Gain 2 sorcery points (replenish on long rest).",
    },
    // 72 — Poisoner
    FeatData {
        name: "Poisoner",
        asi: AsiBonus::ZERO,
        description: "Proficiency (or expertise) with poisoner's kit. Ignore poison resistance with your poisons. Coat weapon as bonus action. Potent poison (d4 extra damage).",
    },
    // 73 — Shadow Touched
    FeatData {
        name: "Shadow Touched",
        asi: AsiBonus::ZERO, // +1 INT, WIS, or CHA — apply manually
        description: "+1 INT, WIS, or CHA (apply manually). Learn Invisibility and one 1st-level illusion/necromancy spell (once/long rest each without slot).",
    },
    // 74 — Skill Expert
    FeatData {
        name: "Skill Expert",
        asi: AsiBonus::ZERO, // +1 to any — apply manually
        description: "+1 to one ability score (apply manually). Gain proficiency in one skill. Expertise in one skill you're proficient in.",
    },
    // 75 — Telekinetic
    FeatData {
        name: "Telekinetic",
        asi: AsiBonus::ZERO, // +1 INT, WIS, or CHA — apply manually
        description: "+1 INT, WIS, or CHA (apply manually). Learn Mage Hand (invisible). Bonus action to push/pull a creature 5 ft (STR save vs 8+prof+mod).",
    },
    // 76 — Telepathic
    FeatData {
        name: "Telepathic",
        asi: AsiBonus::ZERO, // +1 INT, WIS, or CHA — apply manually
        description: "+1 INT, WIS, or CHA (apply manually). Telepathic speech (30 ft). Detect Thoughts once/long rest without slot.",
    },
];
