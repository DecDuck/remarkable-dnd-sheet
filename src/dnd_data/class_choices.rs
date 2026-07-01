//! Player-facing choices for all 12 PHB base classes, grouped by class level.
//! Only features that require the player to pick one or more options are
//! represented here.  Subclass selection and ASI/feat slots are handled
//! separately.

use super::types::{ClassChoiceSet, ChoiceFeature, FeatureOption};

// ─── Shared option pools ──────────────────────────────────────────────────────

pub static FIGHTING_STYLES_FULL: &[FeatureOption] = &[
    FeatureOption { name: "Archery", description: "You gain a +2 bonus to attack rolls you make with ranged weapons." },
    FeatureOption { name: "Defense", description: "While you are wearing armor, you gain a +1 bonus to AC." },
    FeatureOption { name: "Dueling", description: "When you are wielding a melee weapon in one hand and no other weapons, you gain a +2 bonus to damage rolls with that weapon." },
    FeatureOption { name: "Great Weapon Fighting", description: "When you roll a 1 or 2 on a damage die for an attack you make with a melee weapon that you are wielding with two hands, you can reroll the die and must use the new roll, even if the new roll is a 1 or a 2." },
    FeatureOption { name: "Protection", description: "When a creature you can see attacks a target other than you that is within 5 feet of you, you can use your reaction to impose disadvantage on the attack roll. You must be wielding a shield." },
    FeatureOption { name: "Two-Weapon Fighting", description: "When you engage in two-weapon fighting, you can add your ability modifier to the damage of the second attack." },
];

pub static FIGHTING_STYLES_PALADIN: &[FeatureOption] = &[
    FeatureOption { name: "Defense", description: "While you are wearing armor, you gain a +1 bonus to AC." },
    FeatureOption { name: "Dueling", description: "When you are wielding a melee weapon in one hand and no other weapons, you gain a +2 bonus to damage rolls with that weapon." },
    FeatureOption { name: "Great Weapon Fighting", description: "When you roll a 1 or 2 on a damage die for an attack you make with a melee weapon that you are wielding with two hands, you can reroll the die and must use the new roll." },
    FeatureOption { name: "Protection", description: "When a creature you can see attacks a target other than you that is within 5 feet of you, you can use your reaction to impose disadvantage on the attack roll. You must be wielding a shield." },
];

pub static FIGHTING_STYLES_RANGER: &[FeatureOption] = &[
    FeatureOption { name: "Archery", description: "You gain a +2 bonus to attack rolls you make with ranged weapons." },
    FeatureOption { name: "Defense", description: "While you are wearing armor, you gain a +1 bonus to AC." },
    FeatureOption { name: "Dueling", description: "When you are wielding a melee weapon in one hand and no other weapons, you gain a +2 bonus to damage rolls with that weapon." },
    FeatureOption { name: "Two-Weapon Fighting", description: "When you engage in two-weapon fighting, you can add your ability modifier to the damage of the second attack." },
];

pub static FAVORED_ENEMY_OPTIONS: &[FeatureOption] = &[
    FeatureOption { name: "Aberrations", description: "Aberrations such as aboleths, beholders, mind flayers, and slaadi." },
    FeatureOption { name: "Beasts", description: "Natural beasts including animals of all kinds." },
    FeatureOption { name: "Celestials", description: "Celestials including angels, couatls, and pegasi." },
    FeatureOption { name: "Constructs", description: "Constructs such as animated objects, golems, and modrons." },
    FeatureOption { name: "Dragons", description: "True dragons and dragon-kin such as wyverns and pseudodragons." },
    FeatureOption { name: "Elementals", description: "Elementals including djinn, efreet, genies, and xorn." },
    FeatureOption { name: "Fey", description: "Fey creatures such as dryads, pixies, and satyrs." },
    FeatureOption { name: "Fiends", description: "Fiends including demons, devils, rakshasas, and yugoloths." },
    FeatureOption { name: "Giants", description: "Giants including cloud giants, fire giants, frost giants, hill giants, stone giants, and storm giants." },
    FeatureOption { name: "Monstrosities", description: "Monstrosities including basilisks, manticores, medusas, and owlbears." },
    FeatureOption { name: "Oozes", description: "Oozes such as black puddings and gelatinous cubes." },
    FeatureOption { name: "Plants", description: "Plants including blights, shambling mounds, and treants." },
    FeatureOption { name: "Undead", description: "Undead including ghosts, ghouls, liches, skeletons, vampires, and zombies." },
    FeatureOption { name: "Two Humanoid Races", description: "Instead of one creature type, choose two races of humanoid (e.g. orcs and gnolls). You can communicate with them and have advantage on checks to track and recall information about them." },
];

pub static FAVORED_TERRAIN_OPTIONS: &[FeatureOption] = &[
    FeatureOption { name: "Arctic", description: "Frozen tundra, glaciers, and polar regions." },
    FeatureOption { name: "Coast", description: "Beaches, cliffs, and the tidal zone of the sea." },
    FeatureOption { name: "Desert", description: "Sandy and rocky wastelands, both hot and cold." },
    FeatureOption { name: "Forest", description: "Wooded terrain of all kinds, including jungles." },
    FeatureOption { name: "Grassland", description: "Open plains, savannahs, and agricultural land." },
    FeatureOption { name: "Mountain", description: "High peaks, rocky slopes, and alpine terrain." },
    FeatureOption { name: "Swamp", description: "Marshes, bogs, fens, and other wetlands." },
    FeatureOption { name: "Underdark", description: "Underground tunnels, caverns, and subterranean regions." },
];

pub static METAMAGIC_OPTIONS: &[FeatureOption] = &[
    FeatureOption { name: "Careful Spell", description: "When you cast a spell that forces other creatures to make a saving throw, you can protect some of those creatures from the spell's full force. To do so, you spend 1 sorcery point and choose a number of those creatures up to your Charisma modifier (minimum of one creature). A chosen creature automatically succeeds on its saving throw against the spell." },
    FeatureOption { name: "Distant Spell", description: "When you cast a spell that has a range of 5 feet or greater, you can spend 1 sorcery point to double the range of the spell. When you cast a spell that has a range of touch, you can spend 1 sorcery point to make the range of the spell 30 feet." },
    FeatureOption { name: "Empowered Spell", description: "When you roll damage for a spell, you can spend 1 sorcery point to reroll a number of the damage dice up to your Charisma modifier (minimum of one). You must use the new rolls. You can use Empowered Spell even if you have already used a different Metamagic option during the casting of the spell." },
    FeatureOption { name: "Extended Spell", description: "When you cast a spell that has a duration of 1 minute or longer, you can spend 1 sorcery point to double its duration, to a maximum duration of 24 hours." },
    FeatureOption { name: "Heightened Spell", description: "When you cast a spell that forces a creature to make a saving throw to resist its effects, you can spend 3 sorcery points to give one target of the spell disadvantage on its first saving throw made against the spell." },
    FeatureOption { name: "Quickened Spell", description: "When you cast a spell that has a casting time of 1 action, you can spend 2 sorcery points to change the casting time to 1 bonus action for this casting." },
    FeatureOption { name: "Subtle Spell", description: "When you cast a spell, you can spend 1 sorcery point to cast it without any somatic or verbal components." },
    FeatureOption { name: "Twinned Spell", description: "When you cast a spell that targets only one creature and doesn't have a range of self, you can spend a number of sorcery points equal to the spell's level to target a second creature in range with the same spell (1 sorcery point if the spell is a cantrip)." },
];

pub static PACT_BOON_OPTIONS: &[FeatureOption] = &[
    FeatureOption { name: "Pact of the Chain", description: "You learn the find familiar spell and can cast it as a ritual. The spell doesn't count against your number of spells known. When you cast the spell, you can choose one of the normal forms for your familiar or one of the following special forms: imp, pseudodragon, quasit, or sprite. Additionally, when you take the Attack action, you can forgo one of your own attacks to allow your familiar to make one attack of its own with its reaction." },
    FeatureOption { name: "Pact of the Blade", description: "You can use your action to create a pact weapon in your empty hand. You can choose the form that this melee weapon takes each time you create it. You are proficient with it while you wield it. This weapon counts as magical for the purpose of overcoming resistance and immunity to nonmagical attacks and damage. Your pact weapon disappears if it is more than 5 feet away from you for 1 minute or more. You can transform one magic weapon into your pact weapon by performing a special ritual while you hold the weapon." },
    FeatureOption { name: "Pact of the Tome", description: "Your patron gives you a grimoire called a Book of Shadows. When you gain this feature, choose three cantrips from any class's spell list (the three needn't be from the same list). While the book is on your person, you can cast those cantrips at will. They don't count against your number of cantrips known. If they don't appear on the warlock spell list, they are nonetheless warlock spells for you." },
];

pub static ELDRITCH_INVOCATIONS: &[FeatureOption] = &[
    FeatureOption { name: "Agonizing Blast", description: "Prerequisite: eldritch blast cantrip. When you cast eldritch blast, add your Charisma modifier to the damage it deals on a hit." },
    FeatureOption { name: "Armor of Shadows", description: "You can cast mage armor on yourself at will, without expending a spell slot or material components." },
    FeatureOption { name: "Ascendant Step", description: "Prerequisite: 9th level. You can cast levitate on yourself at will, without expending a spell slot or material components." },
    FeatureOption { name: "Beast Speech", description: "You can cast speak with animals at will, without expending a spell slot." },
    FeatureOption { name: "Beguiling Influence", description: "You gain proficiency in the Deception and Persuasion skills." },
    FeatureOption { name: "Bewitching Whispers", description: "Prerequisite: 7th level. You can cast compulsion once using a warlock spell slot. You can't do so again until you finish a long rest." },
    FeatureOption { name: "Book of Ancient Secrets", description: "Prerequisite: Pact of the Tome. You can now inscribe magical rituals in your Book of Shadows. Choose two 1st-level spells that have the ritual tag from any class's spell list (the two needn't be from the same list). The spells appear in the book and don't count against the number of spells you know." },
    FeatureOption { name: "Chains of Carceri", description: "Prerequisite: 15th level, Pact of the Chain. You can cast hold monster at will — targeting a celestial, fiend, or elemental — without expending a spell slot or material components. You must finish a long rest before you can use this invocation on the same creature again." },
    FeatureOption { name: "Devil's Sight", description: "You can see normally in darkness, both magical and nonmagical, to a distance of 120 feet." },
    FeatureOption { name: "Dreadful Word", description: "Prerequisite: 7th level. You can cast confusion once using a warlock spell slot. You can't do so again until you finish a long rest." },
    FeatureOption { name: "Eldritch Sight", description: "You can cast detect magic at will, without expending a spell slot." },
    FeatureOption { name: "Eldritch Spear", description: "Prerequisite: eldritch blast cantrip. When you cast eldritch blast, its range is 300 feet." },
    FeatureOption { name: "Eyes of the Rune Keeper", description: "You can read all writing." },
    FeatureOption { name: "Fiendish Vigor", description: "You can cast false life on yourself at will as a 1st-level spell, without expending a spell slot or material components." },
    FeatureOption { name: "Gaze of Two Minds", description: "You can use your action to touch a willing humanoid and perceive through its senses until the end of your next turn." },
    FeatureOption { name: "Grasp of Hadar", description: "Prerequisite: eldritch blast cantrip. Once on each of your turns when you hit a creature with your eldritch blast, you can move that creature in a straight line 10 feet closer to you." },
    FeatureOption { name: "Investment of the Chain Master", description: "Prerequisite: Pact of the Chain. When you cast find familiar, you infuse the summoned familiar with a measure of your eldritch power, granting it various benefits." },
    FeatureOption { name: "Lance of Lethargy", description: "Prerequisite: eldritch blast cantrip. Once on each of your turns when you hit a creature with your eldritch blast, you can reduce that creature's speed by 10 feet until the end of your next turn." },
    FeatureOption { name: "Lifedrinker", description: "Prerequisite: 12th level, Pact of the Blade. When you hit a creature with your pact weapon, the creature takes extra necrotic damage equal to your Charisma modifier (minimum 1)." },
    FeatureOption { name: "Maddening Hex", description: "Prerequisite: 5th level, hex spell or a warlock feature that curses. As a bonus action, you cause a psychic disturbance around the target cursed by your hex spell or by a warlock feature of yours, such as Hexblade's Curse and Sign of Ill Omen." },
    FeatureOption { name: "Mask of Many Faces", description: "You can cast disguise self at will, without expending a spell slot." },
    FeatureOption { name: "Master of Myriad Forms", description: "Prerequisite: 15th level. You can cast alter self at will, without expending a spell slot." },
    FeatureOption { name: "Minions of Chaos", description: "Prerequisite: 9th level. You can cast conjure elemental once using a warlock spell slot. You can't do so again until you finish a long rest." },
    FeatureOption { name: "Mire the Mind", description: "Prerequisite: 5th level. You can cast slow once using a warlock spell slot. You can't do so again until you finish a long rest." },
    FeatureOption { name: "Misty Visions", description: "You can cast silent image at will, without expending a spell slot or material components." },
    FeatureOption { name: "One with Shadows", description: "Prerequisite: 5th level. When you are in an area of dim light or darkness, you can use your action to become invisible until you move or take an action or a reaction." },
    FeatureOption { name: "Otherworldly Leap", description: "Prerequisite: 9th level. You can cast jump on yourself at will, without expending a spell slot or material components." },
    FeatureOption { name: "Relentless Hex", description: "Prerequisite: 7th level, hex spell or a warlock feature that curses. Your curse creates a temporary bond between you and your target. As a bonus action, you can magically teleport up to 30 feet to an unoccupied space you can see within 5 feet of the target cursed by your hex spell." },
    FeatureOption { name: "Repelling Blast", description: "Prerequisite: eldritch blast cantrip. When you hit a creature with eldritch blast, you can push the creature up to 10 feet away from you in a straight line." },
    FeatureOption { name: "Sculptor of Flesh", description: "Prerequisite: 7th level. You can cast polymorph once using a warlock spell slot. You can't do so again until you finish a long rest." },
    FeatureOption { name: "Sign of Ill Omen", description: "Prerequisite: 5th level. You can cast bestow curse once using a warlock spell slot. You can't do so again until you finish a long rest." },
    FeatureOption { name: "Thief of Five Fates", description: "You can cast bane once using a warlock spell slot. You can't do so again until you finish a long rest." },
    FeatureOption { name: "Thirsting Blade", description: "Prerequisite: 5th level, Pact of the Blade. You can attack with your pact weapon twice, instead of once, whenever you take the Attack action on your turn." },
    FeatureOption { name: "Trickster's Escape", description: "Prerequisite: 7th level. You can cast freedom of movement once on yourself without expending a spell slot. You regain the ability to do so when you finish a long rest." },
    FeatureOption { name: "Undying Servitude", description: "Prerequisite: 5th level. You can cast animate dead without using a spell slot. Once you do so, you can't cast it in this way again until you finish a long rest." },
    FeatureOption { name: "Visions of Distant Realms", description: "Prerequisite: 15th level. You can cast arcane eye at will, without expending a spell slot." },
    FeatureOption { name: "Voice of the Chain Master", description: "Prerequisite: Pact of the Chain. You can communicate telepathically with your familiar and perceive through your familiar's senses as long as you are on the same plane of existence." },
    FeatureOption { name: "Whispers of the Grave", description: "Prerequisite: 9th level. You can cast speak with dead at will, without expending a spell slot." },
    FeatureOption { name: "Witch Sight", description: "Prerequisite: 15th level. You can see the true form of any shapechanger or creature concealed by illusion or transmutation magic while the creature is within 30 feet of you and within line of sight." },
];

pub static EXPERTISE_SKILLS: &[FeatureOption] = &[
    FeatureOption { name: "Acrobatics", description: "Double proficiency bonus on Acrobatics checks." },
    FeatureOption { name: "Animal Handling", description: "Double proficiency bonus on Animal Handling checks." },
    FeatureOption { name: "Arcana", description: "Double proficiency bonus on Arcana checks." },
    FeatureOption { name: "Athletics", description: "Double proficiency bonus on Athletics checks." },
    FeatureOption { name: "Deception", description: "Double proficiency bonus on Deception checks." },
    FeatureOption { name: "History", description: "Double proficiency bonus on History checks." },
    FeatureOption { name: "Insight", description: "Double proficiency bonus on Insight checks." },
    FeatureOption { name: "Intimidation", description: "Double proficiency bonus on Intimidation checks." },
    FeatureOption { name: "Investigation", description: "Double proficiency bonus on Investigation checks." },
    FeatureOption { name: "Medicine", description: "Double proficiency bonus on Medicine checks." },
    FeatureOption { name: "Nature", description: "Double proficiency bonus on Nature checks." },
    FeatureOption { name: "Perception", description: "Double proficiency bonus on Perception checks." },
    FeatureOption { name: "Performance", description: "Double proficiency bonus on Performance checks." },
    FeatureOption { name: "Persuasion", description: "Double proficiency bonus on Persuasion checks." },
    FeatureOption { name: "Religion", description: "Double proficiency bonus on Religion checks." },
    FeatureOption { name: "Sleight of Hand", description: "Double proficiency bonus on Sleight of Hand checks." },
    FeatureOption { name: "Stealth", description: "Double proficiency bonus on Stealth checks." },
    FeatureOption { name: "Survival", description: "Double proficiency bonus on Survival checks." },
    FeatureOption { name: "Thieves' Tools", description: "Double proficiency bonus on Thieves' Tools checks." },
];

pub static ALL_SKILLS: &[FeatureOption] = &[
    FeatureOption { name: "Acrobatics", description: "Dexterity-based skill covering tumbling and balance." },
    FeatureOption { name: "Animal Handling", description: "Wisdom-based skill for calming and controlling animals." },
    FeatureOption { name: "Arcana", description: "Intelligence-based skill for recalling magical lore." },
    FeatureOption { name: "Athletics", description: "Strength-based skill for climbing, jumping, and swimming." },
    FeatureOption { name: "Deception", description: "Charisma-based skill for convincingly hiding the truth." },
    FeatureOption { name: "History", description: "Intelligence-based skill for recalling historical events." },
    FeatureOption { name: "Insight", description: "Wisdom-based skill for discerning intentions." },
    FeatureOption { name: "Intimidation", description: "Charisma-based skill for influencing through threat." },
    FeatureOption { name: "Investigation", description: "Intelligence-based skill for searching for clues." },
    FeatureOption { name: "Medicine", description: "Wisdom-based skill for stabilizing the dying." },
    FeatureOption { name: "Nature", description: "Intelligence-based skill for recalling natural lore." },
    FeatureOption { name: "Perception", description: "Wisdom-based skill for noticing things in your environment." },
    FeatureOption { name: "Performance", description: "Charisma-based skill for delighting an audience." },
    FeatureOption { name: "Persuasion", description: "Charisma-based skill for influencing through social grace." },
    FeatureOption { name: "Religion", description: "Intelligence-based skill for recalling religious lore." },
    FeatureOption { name: "Sleight of Hand", description: "Dexterity-based skill for legerdemain and pickpocketing." },
    FeatureOption { name: "Stealth", description: "Dexterity-based skill for moving silently and hiding." },
    FeatureOption { name: "Survival", description: "Wisdom-based skill for tracking and wilderness survival." },
];

// ─── Barbarian ────────────────────────────────────────────────────────────────
pub static BARBARIAN_CHOICES: &[ClassChoiceSet] = &[];

// ─── Bard ─────────────────────────────────────────────────────────────────────
pub static BARD_CHOICES: &[ClassChoiceSet] = &[
    ClassChoiceSet {
        class_level: 3,
        features: &[ChoiceFeature {
            name: "Expertise",
            description: "Choose two of your skill proficiencies. Your proficiency bonus is doubled for ability checks using those skills.",
            options: EXPERTISE_SKILLS,
            num_picks: 2,
        }],
    },
    ClassChoiceSet {
        class_level: 10,
        features: &[ChoiceFeature {
            name: "Expertise (2 more)",
            description: "Choose two more skill proficiencies to gain the Expertise benefit.",
            options: EXPERTISE_SKILLS,
            num_picks: 2,
        }],
    },
];

// ─── Cleric ───────────────────────────────────────────────────────────────────
pub static CLERIC_CHOICES: &[ClassChoiceSet] = &[];

// ─── Druid ────────────────────────────────────────────────────────────────────
pub static DRUID_CHOICES: &[ClassChoiceSet] = &[];

// ─── Fighter ──────────────────────────────────────────────────────────────────
pub static FIGHTER_CHOICES: &[ClassChoiceSet] = &[
    ClassChoiceSet {
        class_level: 1,
        features: &[ChoiceFeature {
            name: "Fighting Style",
            description: "You adopt a particular style of fighting as your specialty. Choose one of the following options. You can't take a Fighting Style option more than once.",
            options: FIGHTING_STYLES_FULL,
            num_picks: 1,
        }],
    },
];

// ─── Monk ─────────────────────────────────────────────────────────────────────
pub static MONK_CHOICES: &[ClassChoiceSet] = &[];

// ─── Paladin ──────────────────────────────────────────────────────────────────
pub static PALADIN_CHOICES: &[ClassChoiceSet] = &[
    ClassChoiceSet {
        class_level: 2,
        features: &[ChoiceFeature {
            name: "Fighting Style",
            description: "Starting at 2nd level, you adopt a style of fighting as your specialty. Choose one of the following options.",
            options: FIGHTING_STYLES_PALADIN,
            num_picks: 1,
        }],
    },
];

// ─── Ranger ───────────────────────────────────────────────────────────────────
pub static RANGER_CHOICES: &[ClassChoiceSet] = &[
    ClassChoiceSet {
        class_level: 1,
        features: &[
            ChoiceFeature {
                name: "Favored Enemy",
                description: "Choose a type of favored enemy. You have advantage on Wisdom (Survival) checks to track your favored enemies, as well as on Intelligence checks to recall information about them.",
                options: FAVORED_ENEMY_OPTIONS,
                num_picks: 1,
            },
            ChoiceFeature {
                name: "Natural Explorer",
                description: "Choose a favored terrain type. You are particularly familiar with this terrain and gain several benefits while traveling or surviving there.",
                options: FAVORED_TERRAIN_OPTIONS,
                num_picks: 1,
            },
        ],
    },
    ClassChoiceSet {
        class_level: 2,
        features: &[ChoiceFeature {
            name: "Fighting Style",
            description: "At 2nd level, you adopt a particular style of fighting as your specialty.",
            options: FIGHTING_STYLES_RANGER,
            num_picks: 1,
        }],
    },
    ClassChoiceSet {
        class_level: 6,
        features: &[
            ChoiceFeature {
                name: "Favored Enemy (2nd)",
                description: "Choose a second favored enemy type.",
                options: FAVORED_ENEMY_OPTIONS,
                num_picks: 1,
            },
            ChoiceFeature {
                name: "Natural Explorer (2nd terrain)",
                description: "Choose a second favored terrain type.",
                options: FAVORED_TERRAIN_OPTIONS,
                num_picks: 1,
            },
        ],
    },
    ClassChoiceSet {
        class_level: 10,
        features: &[ChoiceFeature {
            name: "Natural Explorer (3rd terrain)",
            description: "Choose a third favored terrain type.",
            options: FAVORED_TERRAIN_OPTIONS,
            num_picks: 1,
        }],
    },
    ClassChoiceSet {
        class_level: 14,
        features: &[ChoiceFeature {
            name: "Favored Enemy (3rd)",
            description: "Choose a third favored enemy type. You also learn the languages of all your favored enemies.",
            options: FAVORED_ENEMY_OPTIONS,
            num_picks: 1,
        }],
    },
];

// ─── Rogue ────────────────────────────────────────────────────────────────────
pub static ROGUE_CHOICES: &[ClassChoiceSet] = &[
    ClassChoiceSet {
        class_level: 1,
        features: &[ChoiceFeature {
            name: "Expertise",
            description: "Choose two of your skill proficiencies, or one of your skill proficiencies and your proficiency with thieves' tools. Your proficiency bonus is doubled for any ability check you make that uses either of the chosen proficiencies.",
            options: EXPERTISE_SKILLS,
            num_picks: 2,
        }],
    },
    ClassChoiceSet {
        class_level: 6,
        features: &[ChoiceFeature {
            name: "Expertise (2 more)",
            description: "Choose two more of your proficiencies in skills or with thieves' tools to gain the Expertise benefit.",
            options: EXPERTISE_SKILLS,
            num_picks: 2,
        }],
    },
];

// ─── Sorcerer ─────────────────────────────────────────────────────────────────
pub static SORCERER_CHOICES: &[ClassChoiceSet] = &[
    ClassChoiceSet {
        class_level: 3,
        features: &[ChoiceFeature {
            name: "Metamagic",
            description: "You gain the ability to twist your spells to suit your needs. Choose two of the following Metamagic options.",
            options: METAMAGIC_OPTIONS,
            num_picks: 2,
        }],
    },
    ClassChoiceSet {
        class_level: 10,
        features: &[ChoiceFeature {
            name: "Metamagic (3rd option)",
            description: "You learn one additional Metamagic option.",
            options: METAMAGIC_OPTIONS,
            num_picks: 1,
        }],
    },
    ClassChoiceSet {
        class_level: 17,
        features: &[ChoiceFeature {
            name: "Metamagic (4th option)",
            description: "You learn one additional Metamagic option.",
            options: METAMAGIC_OPTIONS,
            num_picks: 1,
        }],
    },
];

// ─── Warlock ──────────────────────────────────────────────────────────────────
pub static WARLOCK_CHOICES: &[ClassChoiceSet] = &[
    ClassChoiceSet {
        class_level: 2,
        features: &[ChoiceFeature {
            name: "Eldritch Invocations",
            description: "In your study of occult lore, you have unearthed eldritch invocations — fragments of forbidden knowledge that imbue you with abiding magical ability. Choose two invocations.",
            options: ELDRITCH_INVOCATIONS,
            num_picks: 2,
        }],
    },
    ClassChoiceSet {
        class_level: 3,
        features: &[ChoiceFeature {
            name: "Pact Boon",
            description: "Your otherworldly patron bestows a gift upon you for your loyal service. You gain one of the following features of your choice.",
            options: PACT_BOON_OPTIONS,
            num_picks: 1,
        }],
    },
    ClassChoiceSet {
        class_level: 5,
        features: &[ChoiceFeature {
            name: "Eldritch Invocation (3rd)",
            description: "You gain one additional eldritch invocation of your choice.",
            options: ELDRITCH_INVOCATIONS,
            num_picks: 1,
        }],
    },
    ClassChoiceSet {
        class_level: 7,
        features: &[ChoiceFeature {
            name: "Eldritch Invocation (4th)",
            description: "You gain one additional eldritch invocation of your choice.",
            options: ELDRITCH_INVOCATIONS,
            num_picks: 1,
        }],
    },
    ClassChoiceSet {
        class_level: 9,
        features: &[ChoiceFeature {
            name: "Eldritch Invocation (5th)",
            description: "You gain one additional eldritch invocation of your choice.",
            options: ELDRITCH_INVOCATIONS,
            num_picks: 1,
        }],
    },
    ClassChoiceSet {
        class_level: 12,
        features: &[ChoiceFeature {
            name: "Eldritch Invocation (6th)",
            description: "You gain one additional eldritch invocation of your choice.",
            options: ELDRITCH_INVOCATIONS,
            num_picks: 1,
        }],
    },
    ClassChoiceSet {
        class_level: 15,
        features: &[ChoiceFeature {
            name: "Eldritch Invocation (7th)",
            description: "You gain one additional eldritch invocation of your choice.",
            options: ELDRITCH_INVOCATIONS,
            num_picks: 1,
        }],
    },
    ClassChoiceSet {
        class_level: 18,
        features: &[ChoiceFeature {
            name: "Eldritch Invocation (8th)",
            description: "You gain one additional eldritch invocation of your choice.",
            options: ELDRITCH_INVOCATIONS,
            num_picks: 1,
        }],
    },
];

// ─── Wizard ───────────────────────────────────────────────────────────────────
pub static WIZARD_CHOICES: &[ClassChoiceSet] = &[];
