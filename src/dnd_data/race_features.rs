//! Structured racial traits for all 31 PHB/Volo's/Tasha's races.
//! Each entry lists both informational traits and choice-bearing traits.

use super::types::{RaceTrait, FeatureOption};

// ─── Shared option pools ──────────────────────────────────────────────────────

pub static DRACONIC_ANCESTRY_OPTIONS: &[FeatureOption] = &[
    FeatureOption { name: "Black (Acid)",       description: "Breath weapon: 5×30 ft. line (DEX save), acid damage. Resistance to acid." },
    FeatureOption { name: "Blue (Lightning)",   description: "Breath weapon: 5×30 ft. line (DEX save), lightning damage. Resistance to lightning." },
    FeatureOption { name: "Brass (Fire)",       description: "Breath weapon: 5×30 ft. line (DEX save), fire damage. Resistance to fire." },
    FeatureOption { name: "Bronze (Lightning)", description: "Breath weapon: 5×30 ft. line (DEX save), lightning damage. Resistance to lightning." },
    FeatureOption { name: "Copper (Acid)",      description: "Breath weapon: 5×30 ft. line (DEX save), acid damage. Resistance to acid." },
    FeatureOption { name: "Gold (Fire)",        description: "Breath weapon: 15 ft. cone (DEX save), fire damage. Resistance to fire." },
    FeatureOption { name: "Green (Poison)",     description: "Breath weapon: 15 ft. cone (CON save), poison damage. Resistance to poison." },
    FeatureOption { name: "Red (Fire)",         description: "Breath weapon: 15 ft. cone (DEX save), fire damage. Resistance to fire." },
    FeatureOption { name: "Silver (Cold)",      description: "Breath weapon: 15 ft. cone (CON save), cold damage. Resistance to cold." },
    FeatureOption { name: "White (Cold)",       description: "Breath weapon: 15 ft. cone (CON save), cold damage. Resistance to cold." },
];

pub static SKILL_CHOICES_2: &[FeatureOption] = &[
    FeatureOption { name: "Acrobatics",    description: "Gain proficiency in Acrobatics (DEX)." },
    FeatureOption { name: "Animal Handling", description: "Gain proficiency in Animal Handling (WIS)." },
    FeatureOption { name: "Arcana",        description: "Gain proficiency in Arcana (INT)." },
    FeatureOption { name: "Athletics",     description: "Gain proficiency in Athletics (STR)." },
    FeatureOption { name: "Deception",     description: "Gain proficiency in Deception (CHA)." },
    FeatureOption { name: "History",       description: "Gain proficiency in History (INT)." },
    FeatureOption { name: "Insight",       description: "Gain proficiency in Insight (WIS)." },
    FeatureOption { name: "Intimidation",  description: "Gain proficiency in Intimidation (CHA)." },
    FeatureOption { name: "Investigation", description: "Gain proficiency in Investigation (INT)." },
    FeatureOption { name: "Medicine",      description: "Gain proficiency in Medicine (WIS)." },
    FeatureOption { name: "Nature",        description: "Gain proficiency in Nature (INT)." },
    FeatureOption { name: "Perception",    description: "Gain proficiency in Perception (WIS)." },
    FeatureOption { name: "Performance",   description: "Gain proficiency in Performance (CHA)." },
    FeatureOption { name: "Persuasion",    description: "Gain proficiency in Persuasion (CHA)." },
    FeatureOption { name: "Religion",      description: "Gain proficiency in Religion (INT)." },
    FeatureOption { name: "Sleight of Hand", description: "Gain proficiency in Sleight of Hand (DEX)." },
    FeatureOption { name: "Stealth",       description: "Gain proficiency in Stealth (DEX)." },
    FeatureOption { name: "Survival",      description: "Gain proficiency in Survival (WIS)." },
];

pub static KENKU_SKILL_OPTIONS: &[FeatureOption] = &[
    FeatureOption { name: "Acrobatics",    description: "Gain proficiency in Acrobatics." },
    FeatureOption { name: "Deception",     description: "Gain proficiency in Deception." },
    FeatureOption { name: "Perception",    description: "Gain proficiency in Perception." },
    FeatureOption { name: "Persuasion",    description: "Gain proficiency in Persuasion." },
    FeatureOption { name: "Sleight of Hand", description: "Gain proficiency in Sleight of Hand." },
    FeatureOption { name: "Stealth",       description: "Gain proficiency in Stealth." },
];

pub static LIZARDFOLK_SKILL_OPTIONS: &[FeatureOption] = &[
    FeatureOption { name: "Animal Handling", description: "Gain proficiency in Animal Handling." },
    FeatureOption { name: "Nature",          description: "Gain proficiency in Nature." },
    FeatureOption { name: "Perception",      description: "Gain proficiency in Perception." },
    FeatureOption { name: "Stealth",         description: "Gain proficiency in Stealth." },
    FeatureOption { name: "Survival",        description: "Gain proficiency in Survival." },
];

pub static WIZARD_CANTRIPS: &[FeatureOption] = &[
    FeatureOption { name: "Acid Splash",      description: "Hurl a bubble of acid dealing 1d6 acid damage." },
    FeatureOption { name: "Blade Ward",       description: "Resistance to bludgeoning, piercing, slashing from weapon attacks until end of next turn." },
    FeatureOption { name: "Chill Touch",      description: "Spectral hand prevents healing and deals necrotic damage." },
    FeatureOption { name: "Control Flames",   description: "Instantaneous manipulation of nonmagical flames." },
    FeatureOption { name: "Create Bonfire",   description: "Summon a bonfire that damages creatures that enter it." },
    FeatureOption { name: "Dancing Lights",   description: "Create up to four torch-sized lights that you can move around." },
    FeatureOption { name: "Fire Bolt",        description: "Ranged spell attack dealing 1d10 fire damage." },
    FeatureOption { name: "Friends",          description: "Advantage on CHA checks against one nonhostile creature." },
    FeatureOption { name: "Frostbite",        description: "Cause numbing frost; target has disadvantage on next weapon attack roll." },
    FeatureOption { name: "Green-Flame Blade", description: "Weapon attack with fire leaping to an adjacent creature." },
    FeatureOption { name: "Gust",             description: "Push a creature or object with a burst of wind." },
    FeatureOption { name: "Infestation",      description: "Summon insects to bite a creature, dealing 1d6 poison damage." },
    FeatureOption { name: "Light",            description: "Imbue an object with light radiating a 20-foot radius." },
    FeatureOption { name: "Lightning Lure",   description: "Pull a creature toward you with a lash of lightning." },
    FeatureOption { name: "Mage Hand",        description: "Spectral floating hand can manipulate objects within 30 feet." },
    FeatureOption { name: "Mending",          description: "Repair a single break or tear in an object." },
    FeatureOption { name: "Message",          description: "Whisper a message to a creature; it can reply in a whisper." },
    FeatureOption { name: "Minor Illusion",   description: "Create a sound or image of an object within 30 feet." },
    FeatureOption { name: "Mold Earth",       description: "Move, shape, or color loose earth or stone." },
    FeatureOption { name: "Poison Spray",     description: "Puff of poison gas forces a CON save or 1d12 poison damage." },
    FeatureOption { name: "Prestidigitation", description: "Minor magical tricks for practicing magicians." },
    FeatureOption { name: "Ray of Frost",     description: "Icy ray slows target and deals 1d8 cold damage." },
    FeatureOption { name: "Shape Water",      description: "Move, shape, freeze, or animate water." },
    FeatureOption { name: "Shocking Grasp",   description: "Melee spell attack dealing 1d8 lightning; target can't take reactions." },
    FeatureOption { name: "Sword Burst",      description: "Force all adjacent creatures to make a DEX save or take force damage." },
    FeatureOption { name: "Thunderclap",      description: "Audible clap of thunder; creatures within 5 ft make CON save." },
    FeatureOption { name: "Toll the Dead",    description: "Toll a dolorous bell; target makes WIS save or takes necrotic damage." },
    FeatureOption { name: "True Strike",      description: "Gain advantage on your next attack roll against the target." },
];

pub static TIEFLING_VARIANT_CANTRIPS: &[FeatureOption] = &[
    FeatureOption { name: "Fire Bolt",       description: "Ranged spell attack for 1d10 fire damage." },
    FeatureOption { name: "Vicious Mockery", description: "Psychic taunt dealing 1d4 damage and imposing disadvantage." },
    FeatureOption { name: "Chill Touch",     description: "Prevents healing and deals 1d8 necrotic damage." },
    FeatureOption { name: "Minor Illusion",  description: "Create a sound or image of an object within 30 feet." },
    FeatureOption { name: "Poison Spray",    description: "Puff of poison gas, CON save or 1d12 poison damage." },
    FeatureOption { name: "Ray of Frost",    description: "Slows target and deals 1d8 cold damage." },
    FeatureOption { name: "Shocking Grasp",  description: "Melee spell attack, 1d8 lightning; target can't take reactions." },
];

pub static ABILITY_CHOICES: &[FeatureOption] = &[
    FeatureOption { name: "Strength",     description: "+1 to Strength (STR)." },
    FeatureOption { name: "Dexterity",    description: "+1 to Dexterity (DEX)." },
    FeatureOption { name: "Constitution", description: "+1 to Constitution (CON)." },
    FeatureOption { name: "Intelligence", description: "+1 to Intelligence (INT)." },
    FeatureOption { name: "Wisdom",       description: "+1 to Wisdom (WIS)." },
    FeatureOption { name: "Charisma",     description: "+1 to Charisma (CHA). (Half-Elf already gets +2 CHA; this is the free +1 to another ability.)" },
];

pub static TOOL_PROFICIENCY_OPTIONS: &[FeatureOption] = &[
    FeatureOption { name: "Smith's Tools",    description: "Proficiency with smith's tools, used to craft and repair metal items." },
    FeatureOption { name: "Brewer's Supplies", description: "Proficiency with brewer's supplies, used to craft ales, meads, and other fermented beverages." },
    FeatureOption { name: "Mason's Tools",    description: "Proficiency with mason's tools, used to craft and work with stone." },
];

// ─── Race trait tables ────────────────────────────────────────────────────────

pub static HUMAN_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Ability Score Increase", description: "+1 to all six ability scores.", options: &[], num_picks: 0 },
    RaceTrait { name: "Extra Language", description: "You can speak, read, and write one extra language of your choice.", options: &[], num_picks: 0 },
    RaceTrait { name: "Skills", description: "Humans acquire many proficiencies throughout their lives. You gain proficiency in one skill of your choice (included in your class selection).", options: &[], num_picks: 0 },
];

pub static DWARF_HILL_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Darkvision", description: "Accustomed to life underground, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light.", options: &[], num_picks: 0 },
    RaceTrait { name: "Dwarven Resilience", description: "You have advantage on saving throws against poison, and you have resistance against poison damage.", options: &[], num_picks: 0 },
    RaceTrait { name: "Dwarven Combat Training", description: "You have proficiency with the battleaxe, handaxe, light hammer, and warhammer.", options: &[], num_picks: 0 },
    RaceTrait { name: "Tool Proficiency", description: "You gain proficiency with the artisan's tools of your choice: smith's tools, brewer's supplies, or mason's tools.", options: TOOL_PROFICIENCY_OPTIONS, num_picks: 1 },
    RaceTrait { name: "Stonecunning", description: "Whenever you make an Intelligence (History) check related to the origin of stonework, you are considered proficient in the History skill and add double your proficiency bonus to the check.", options: &[], num_picks: 0 },
    RaceTrait { name: "Dwarven Toughness", description: "Your hit point maximum increases by 1, and it increases by 1 every time you gain a level.", options: &[], num_picks: 0 },
];

pub static DWARF_MOUNTAIN_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Darkvision", description: "You can see in dim light within 60 feet as if it were bright light, and in darkness as if it were dim light.", options: &[], num_picks: 0 },
    RaceTrait { name: "Dwarven Resilience", description: "You have advantage on saving throws against poison, and resistance against poison damage.", options: &[], num_picks: 0 },
    RaceTrait { name: "Dwarven Combat Training", description: "You have proficiency with the battleaxe, handaxe, light hammer, and warhammer.", options: &[], num_picks: 0 },
    RaceTrait { name: "Tool Proficiency", description: "You gain proficiency with the artisan's tools of your choice: smith's tools, brewer's supplies, or mason's tools.", options: TOOL_PROFICIENCY_OPTIONS, num_picks: 1 },
    RaceTrait { name: "Stonecunning", description: "History checks related to stonework use double proficiency bonus.", options: &[], num_picks: 0 },
    RaceTrait { name: "Dwarven Armor Training", description: "You have proficiency with light and medium armor.", options: &[], num_picks: 0 },
];

pub static ELF_HIGH_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Darkvision", description: "You can see in dim light within 60 feet as if it were bright light, and in darkness as if it were dim light.", options: &[], num_picks: 0 },
    RaceTrait { name: "Keen Senses", description: "You have proficiency in the Perception skill.", options: &[], num_picks: 0 },
    RaceTrait { name: "Fey Ancestry", description: "You have advantage on saving throws against being charmed, and magic can't put you to sleep.", options: &[], num_picks: 0 },
    RaceTrait { name: "Trance", description: "Elves don't need to sleep. Instead, they meditate deeply, remaining semiconscious, for 4 hours a day.", options: &[], num_picks: 0 },
    RaceTrait { name: "Elf Weapon Training", description: "You have proficiency with the longsword, shortsword, shortbow, and longbow.", options: &[], num_picks: 0 },
    RaceTrait { name: "Cantrip", description: "You know one cantrip of your choice from the wizard spell list. Intelligence is your spellcasting ability for it.", options: WIZARD_CANTRIPS, num_picks: 1 },
    RaceTrait { name: "Extra Language", description: "You can speak, read, and write one extra language of your choice.", options: &[], num_picks: 0 },
];

pub static ELF_WOOD_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Darkvision", description: "You can see in dim light within 60 feet as if it were bright light, and in darkness as if it were dim light.", options: &[], num_picks: 0 },
    RaceTrait { name: "Keen Senses", description: "You have proficiency in the Perception skill.", options: &[], num_picks: 0 },
    RaceTrait { name: "Fey Ancestry", description: "You have advantage on saving throws against being charmed, and magic can't put you to sleep.", options: &[], num_picks: 0 },
    RaceTrait { name: "Trance", description: "Elves meditate for 4 hours instead of sleeping.", options: &[], num_picks: 0 },
    RaceTrait { name: "Elf Weapon Training", description: "Proficiency with longsword, shortsword, shortbow, and longbow.", options: &[], num_picks: 0 },
    RaceTrait { name: "Fleet of Foot", description: "Your base walking speed increases to 35 feet.", options: &[], num_picks: 0 },
    RaceTrait { name: "Mask of the Wild", description: "You can attempt to hide even when you are only lightly obscured by foliage, heavy rain, falling snow, mist, and other natural phenomena.", options: &[], num_picks: 0 },
];

pub static ELF_DROW_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Superior Darkvision", description: "Accustomed to the Underdark, your darkvision has a range of 120 feet.", options: &[], num_picks: 0 },
    RaceTrait { name: "Sunlight Sensitivity", description: "You have disadvantage on attack rolls and on Wisdom (Perception) checks that rely on sight when you, the target of your attack, or whatever you are trying to perceive is in direct sunlight.", options: &[], num_picks: 0 },
    RaceTrait { name: "Keen Senses", description: "You have proficiency in the Perception skill.", options: &[], num_picks: 0 },
    RaceTrait { name: "Fey Ancestry", description: "Advantage on saves vs. charm; immune to magical sleep.", options: &[], num_picks: 0 },
    RaceTrait { name: "Trance", description: "Meditate 4 hours instead of sleeping.", options: &[], num_picks: 0 },
    RaceTrait { name: "Drow Magic", description: "You know the dancing lights cantrip. At 3rd level, you can cast faerie fire once per day. At 5th level, you can cast darkness once per day. Charisma is your spellcasting ability for these spells.", options: &[], num_picks: 0 },
    RaceTrait { name: "Drow Weapon Training", description: "You have proficiency with rapiers, shortswords, and hand crossbows.", options: &[], num_picks: 0 },
];

pub static HALFLING_LIGHTFOOT_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Lucky", description: "When you roll a 1 on the d20 for an attack roll, ability check, or saving throw, you can reroll the die and must use the new roll.", options: &[], num_picks: 0 },
    RaceTrait { name: "Brave", description: "You have advantage on saving throws against being frightened.", options: &[], num_picks: 0 },
    RaceTrait { name: "Halfling Nimbleness", description: "You can move through the space of any creature that is of a size larger than yours.", options: &[], num_picks: 0 },
    RaceTrait { name: "Naturally Stealthy", description: "You can attempt to hide even when you are obscured only by a creature that is at least one size larger than you.", options: &[], num_picks: 0 },
];

pub static HALFLING_STOUT_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Lucky", description: "When you roll a 1 on an attack roll, ability check, or saving throw, you can reroll and must use the new roll.", options: &[], num_picks: 0 },
    RaceTrait { name: "Brave", description: "Advantage on saving throws against being frightened.", options: &[], num_picks: 0 },
    RaceTrait { name: "Halfling Nimbleness", description: "You can move through the space of any larger creature.", options: &[], num_picks: 0 },
    RaceTrait { name: "Stout Resilience", description: "You have advantage on saving throws against poison, and you have resistance against poison damage.", options: &[], num_picks: 0 },
];

pub static HUMAN_VARIANT_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Ability Score Increase", description: "+1 to two ability scores of your choice (apply manually via the Stats tab).", options: &[], num_picks: 0 },
    RaceTrait { name: "Skills", description: "You gain proficiency in one skill of your choice.", options: SKILL_CHOICES_2, num_picks: 1 },
    RaceTrait { name: "Feat", description: "You gain one feat of your choice (select via the ASI/Feat slot).", options: &[], num_picks: 0 },
];

pub static DRAGONBORN_TRAITS: &[RaceTrait] = &[
    RaceTrait {
        name: "Draconic Ancestry",
        description: "You have draconic ancestry of a particular dragon type. Choose a dragon type below. This determines your breath weapon damage type and your damage resistance.",
        options: DRACONIC_ANCESTRY_OPTIONS,
        num_picks: 1,
    },
    RaceTrait { name: "Breath Weapon", description: "You can use your action to exhale destructive energy. Your draconic ancestry determines the size, shape, and damage type of the exhalation. When you use your breath weapon, each creature in the area of the exhalation must make a saving throw (DC 8 + CON modifier + proficiency bonus). Damage: 2d6 at 1st level, 3d6 at 6th, 4d6 at 11th, 5d6 at 16th. Usable once per short or long rest.", options: &[], num_picks: 0 },
    RaceTrait { name: "Damage Resistance", description: "You have resistance to the damage type associated with your draconic ancestry.", options: &[], num_picks: 0 },
];

pub static GNOME_FOREST_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Darkvision", description: "You can see in dim light within 60 feet as if bright light, and in darkness as if dim light.", options: &[], num_picks: 0 },
    RaceTrait { name: "Gnome Cunning", description: "You have advantage on all Intelligence, Wisdom, and Charisma saving throws against magic.", options: &[], num_picks: 0 },
    RaceTrait { name: "Natural Illusionist", description: "You know the minor illusion cantrip. Intelligence is your spellcasting ability for it.", options: &[], num_picks: 0 },
    RaceTrait { name: "Speak with Small Beasts", description: "Through sounds and gestures, you can communicate simple ideas with Small or smaller beasts. Forest gnomes love animals and often keep squirrels, badgers, rabbits, moles, woodpeckers, and other creatures as beloved pets.", options: &[], num_picks: 0 },
];

pub static GNOME_ROCK_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Darkvision", description: "You can see in dim light within 60 feet as if bright light, and in darkness as if dim light.", options: &[], num_picks: 0 },
    RaceTrait { name: "Gnome Cunning", description: "Advantage on all INT, WIS, and CHA saving throws against magic.", options: &[], num_picks: 0 },
    RaceTrait { name: "Artificer's Lore", description: "Whenever you make an Intelligence (History) check related to magic items, alchemical objects, or technological devices, you can add twice your proficiency bonus, instead of any proficiency bonus you normally apply.", options: &[], num_picks: 0 },
    RaceTrait { name: "Tinker", description: "You have proficiency with artisan's tools (tinker's tools). Using those tools, you can spend 1 hour and 10 gp worth of materials to construct a Tiny clockwork device (AC 5, 1 hp). The device ceases to function after 24 hours unless you spend 1 hour repairing it to keep it functioning.", options: &[], num_picks: 0 },
];

pub static HALF_ELF_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Darkvision", description: "You can see in dim light within 60 feet as if bright light, and in darkness as if dim light.", options: &[], num_picks: 0 },
    RaceTrait { name: "Fey Ancestry", description: "Advantage on saving throws against being charmed, and magic can't put you to sleep.", options: &[], num_picks: 0 },
    RaceTrait { name: "Ability Score Increase", description: "+1 to two ability scores of your choice (beyond the +2 CHA). Apply manually via the Stats tab.", options: ABILITY_CHOICES, num_picks: 2 },
    RaceTrait { name: "Skill Versatility", description: "You gain proficiency in two skills of your choice.", options: SKILL_CHOICES_2, num_picks: 2 },
    RaceTrait { name: "Extra Language", description: "You can speak, read, and write one extra language of your choice.", options: &[], num_picks: 0 },
];

pub static HALF_ORC_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Darkvision", description: "You can see in dim light within 60 feet as if bright light, and in darkness as if dim light.", options: &[], num_picks: 0 },
    RaceTrait { name: "Menacing", description: "You gain proficiency in the Intimidation skill.", options: &[], num_picks: 0 },
    RaceTrait { name: "Relentless Endurance", description: "When you are reduced to 0 hit points but not killed outright, you can drop to 1 hit point instead. You can't use this feature again until you finish a long rest.", options: &[], num_picks: 0 },
    RaceTrait { name: "Savage Attacks", description: "When you score a critical hit with a melee weapon attack, you can roll one of the weapon's damage dice one additional time and add it to the extra damage of the critical hit.", options: &[], num_picks: 0 },
];

pub static TIEFLING_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Darkvision", description: "You can see in dim light within 60 feet as if bright light, and in darkness as if dim light.", options: &[], num_picks: 0 },
    RaceTrait { name: "Hellish Resistance", description: "You have resistance to fire damage.", options: &[], num_picks: 0 },
    RaceTrait { name: "Infernal Legacy", description: "You know the thaumaturgy cantrip. At 3rd level, you can cast hellish rebuke as a 2nd-level spell once per long rest. At 5th level, you can cast darkness once per long rest. Charisma is your spellcasting ability for these spells.", options: &[], num_picks: 0 },
];

pub static AASIMAR_PROTECTOR_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Darkvision", description: "You can see in dim light within 60 feet as if bright light, and in darkness as if dim light.", options: &[], num_picks: 0 },
    RaceTrait { name: "Celestial Resistance", description: "You have resistance to necrotic damage and radiant damage.", options: &[], num_picks: 0 },
    RaceTrait { name: "Healing Hands", description: "As an action, you can touch a creature and cause it to regain a number of hit points equal to your level. You can't use this trait again until you finish a long rest.", options: &[], num_picks: 0 },
    RaceTrait { name: "Light Bearer", description: "You know the light cantrip. Charisma is your spellcasting ability for it.", options: &[], num_picks: 0 },
    RaceTrait { name: "Radiant Soul", description: "Starting at 3rd level, you can use your action to unleash the divine energy within yourself, causing your eyes to glimmer and two luminous, incorporeal wings to sprout from your back. Your transformation lasts for 1 minute or until you end it as a bonus action. During it, you have a flying speed of 30 feet, and once on each of your turns, you can deal extra radiant damage equal to your level to one target when you deal damage to it with an attack or a spell.", options: &[], num_picks: 0 },
];

pub static AASIMAR_SCOURGE_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Darkvision", description: "60-foot darkvision.", options: &[], num_picks: 0 },
    RaceTrait { name: "Celestial Resistance", description: "Resistance to necrotic and radiant damage.", options: &[], num_picks: 0 },
    RaceTrait { name: "Healing Hands", description: "Touch a creature as an action to heal hit points equal to your level. Once per long rest.", options: &[], num_picks: 0 },
    RaceTrait { name: "Light Bearer", description: "You know the light cantrip (CHA).", options: &[], num_picks: 0 },
    RaceTrait { name: "Radiant Consumption", description: "At 3rd level, you can use your action to cause searing light to radiate from you. For 1 minute, you shed bright light in a 10-foot radius and dim light for an additional 10 feet. Creatures within the bright light radius take radiant damage equal to half your level at the start of each of their turns. At the end of each of your turns for the duration, you take radiant damage equal to half your level.", options: &[], num_picks: 0 },
];

pub static AASIMAR_FALLEN_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Darkvision", description: "60-foot darkvision.", options: &[], num_picks: 0 },
    RaceTrait { name: "Celestial Resistance", description: "Resistance to necrotic and radiant damage.", options: &[], num_picks: 0 },
    RaceTrait { name: "Healing Hands", description: "Heal (level) hit points with a touch. Once per long rest.", options: &[], num_picks: 0 },
    RaceTrait { name: "Light Bearer", description: "You know the light cantrip (CHA).", options: &[], num_picks: 0 },
    RaceTrait { name: "Necrotic Shroud", description: "At 3rd level, you can use your action to unleash the divine energy within yourself. Your eyes turn into pools of darkness and two skeletal, ghostly, flightless wings sprout from your back. Creatures other than your allies within 10 feet of you that can see you must succeed on a Charisma saving throw (DC 8 + your proficiency bonus + your Charisma modifier) or become frightened of you until the end of your next turn.", options: &[], num_picks: 0 },
];

pub static FIRBOLG_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Firbolg Magic", description: "You can cast detect magic and disguise self with this trait, using Wisdom as your spellcasting ability for them. Once you cast either spell, you can't cast it again with this trait until you finish a short or long rest. When you use this version of disguise self, you can seem up to 3 feet shorter than normal, allowing you to more easily blend in with humans and elves.", options: &[], num_picks: 0 },
    RaceTrait { name: "Hidden Step", description: "As a bonus action, you can magically turn invisible until the start of your next turn or until you attack, make a damage roll, or force someone to make a saving throw. Once you use this trait, you can't use it again until you finish a short or long rest.", options: &[], num_picks: 0 },
    RaceTrait { name: "Powerful Build", description: "You count as one size larger when determining your carrying capacity and the weight you can push, drag, or lift.", options: &[], num_picks: 0 },
    RaceTrait { name: "Speech of Beast and Leaf", description: "You have the ability to communicate in a limited manner with beasts and plants. They can understand the meaning of your words, though you have no special ability to understand them in return.", options: &[], num_picks: 0 },
];

pub static GOLIATH_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Natural Athlete", description: "You have proficiency in the Athletics skill.", options: &[], num_picks: 0 },
    RaceTrait { name: "Stone's Endurance", description: "You can focus yourself to occasionally shrug off injury. When you take damage, you can use your reaction to roll a d12. Add your Constitution modifier to the number rolled, and reduce the damage by that total. After you use this trait, you can't use it again until you finish a short or long rest.", options: &[], num_picks: 0 },
    RaceTrait { name: "Powerful Build", description: "You count as one size larger when determining carrying capacity and weight you can push, drag, or lift.", options: &[], num_picks: 0 },
    RaceTrait { name: "Mountain Born", description: "You're acclimated to high altitude, including elevations above 20,000 feet. You're also naturally adapted to cold climates.", options: &[], num_picks: 0 },
];

pub static KENKU_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Expert Forgery", description: "You can duplicate other creatures' handwriting and craftwork. You have advantage on all checks made to produce forgeries or duplicates of existing objects.", options: &[], num_picks: 0 },
    RaceTrait { name: "Kenku Training", description: "You are proficient in your choice of two of the following skills: Acrobatics, Deception, Perception, Persuasion, Sleight of Hand, and Stealth.", options: KENKU_SKILL_OPTIONS, num_picks: 2 },
    RaceTrait { name: "Mimicry", description: "You can mimic sounds you have heard, including voices. A creature that hears the sounds you make can tell they are imitations with a successful DC 14 Wisdom (Insight) check.", options: &[], num_picks: 0 },
];

pub static LIZARDFOLK_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Bite", description: "Your fanged maw is a natural weapon, which you can use to make unarmed strikes. If you hit with it, you deal piercing damage equal to 1d6 + your Strength modifier, instead of the bludgeoning damage normal for an unarmed strike.", options: &[], num_picks: 0 },
    RaceTrait { name: "Cunning Artisan", description: "As part of a short rest, you can harvest bone and hide from a slain beast, construct, dragon, monstrosity, or plant creature of size Small or larger to create one of the following items: a shield, a club, a javelin, or 1d4 darts or blowgun needles.", options: &[], num_picks: 0 },
    RaceTrait { name: "Hold Breath", description: "You can hold your breath for up to 15 minutes at a time.", options: &[], num_picks: 0 },
    RaceTrait { name: "Hunter's Lore", description: "You gain proficiency with two of the following skills of your choice: Animal Handling, Nature, Perception, Stealth, and Survival.", options: LIZARDFOLK_SKILL_OPTIONS, num_picks: 2 },
    RaceTrait { name: "Natural Armor", description: "You have tough, scaly skin. When you aren't wearing armor, your AC is 13 + your Dexterity modifier.", options: &[], num_picks: 0 },
    RaceTrait { name: "Hungry Jaws", description: "In battle, you can throw yourself into a vicious feeding frenzy. As a bonus action, you can make a special attack with your bite. If the attack hits, it deals its normal damage, and you gain temporary hit points (minimum of 1) equal to your Constitution modifier, and you can't use this trait again until you finish a short or long rest.", options: &[], num_picks: 0 },
];

pub static TABAXI_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Darkvision", description: "You can see in dim light within 60 feet as if it were bright light, and in darkness as if it were dim light.", options: &[], num_picks: 0 },
    RaceTrait { name: "Feline Agility", description: "Your reflexes and agility allow you to move with a burst of speed. When you move on your turn in combat, you can double your speed until the end of the turn. Once you use this trait, you can't use it again until you move 0 feet on one of your turns.", options: &[], num_picks: 0 },
    RaceTrait { name: "Cat's Claws", description: "Because of your claws, you have a climbing speed of 20 feet. In addition, your claws are natural weapons, which you can use to make unarmed strikes. If you hit with them, you deal slashing damage equal to 1d4 + your Strength modifier.", options: &[], num_picks: 0 },
    RaceTrait { name: "Cat's Talents", description: "You have proficiency in the Perception and Stealth skills.", options: &[], num_picks: 0 },
];

pub static TRITON_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Amphibious", description: "You can breathe air and water.", options: &[], num_picks: 0 },
    RaceTrait { name: "Control Air and Water", description: "A child of the sea, you can call on the magic of elemental air and water. You can cast fog cloud with this trait. Starting at 3rd level, you can cast gust of wind with it, and starting at 5th level, you can also cast wall of water with it. Once you cast a spell with this trait, you can't cast that spell with it again until you finish a long rest. Charisma is your spellcasting ability for these spells.", options: &[], num_picks: 0 },
    RaceTrait { name: "Darkvision", description: "You can see in dim light within 60 feet as if it were bright light, and in darkness as if it were dim light.", options: &[], num_picks: 0 },
    RaceTrait { name: "Emissary of the Sea", description: "Aquatic beasts can understand your speech, and you can decipher their noises and motions. Most fish, whales, sharks, and sea birds have limited vocabularies; communication is possible, but not always productive.", options: &[], num_picks: 0 },
    RaceTrait { name: "Guardians of the Depths", description: "Adapted to even the most extreme ocean depths, you have resistance to cold damage.", options: &[], num_picks: 0 },
];

pub static BUGBEAR_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Darkvision", description: "60-foot darkvision.", options: &[], num_picks: 0 },
    RaceTrait { name: "Long-Limbed", description: "When you make a melee attack on your turn, your reach for it is 5 feet greater than normal.", options: &[], num_picks: 0 },
    RaceTrait { name: "Powerful Build", description: "You count as one size larger when determining carrying capacity.", options: &[], num_picks: 0 },
    RaceTrait { name: "Sneaky", description: "You are proficient in the Stealth skill.", options: &[], num_picks: 0 },
    RaceTrait { name: "Surprise Attack", description: "If you surprise a creature and hit it with an attack on your first turn in combat, the attack deals an extra 2d6 damage to it. You can use this trait only once per combat.", options: &[], num_picks: 0 },
];

pub static GOBLIN_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Darkvision", description: "60-foot darkvision.", options: &[], num_picks: 0 },
    RaceTrait { name: "Fury of the Small", description: "When you damage a creature with an attack or a spell and the creature's size is larger than yours, you can cause the attack or spell to deal extra damage to the creature equal to your level. Once you use this trait, you can't use it again until you finish a short or long rest.", options: &[], num_picks: 0 },
    RaceTrait { name: "Nimble Escape", description: "You can take the Disengage or Hide action as a bonus action on each of your turns.", options: &[], num_picks: 0 },
];

pub static HOBGOBLIN_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Darkvision", description: "60-foot darkvision.", options: &[], num_picks: 0 },
    RaceTrait { name: "Martial Training", description: "You are proficient with two martial weapons of your choice and with light armor.", options: &[], num_picks: 0 },
    RaceTrait { name: "Saving Face", description: "Hobgoblins are careful not to show weakness in front of their allies, for fear of losing status. If you miss with an attack roll or fail an ability check or a saving throw, you can gain a bonus to the roll equal to the number of allies you can see within 30 feet of you (maximum bonus of +5). Once you use this trait, you can't use it again until you finish a short or long rest.", options: &[], num_picks: 0 },
];

pub static KOBOLD_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Darkvision", description: "60-foot darkvision.", options: &[], num_picks: 0 },
    RaceTrait { name: "Grovel, Cower, and Beg", description: "As an action on your turn, you can cower pathetically to distract nearby foes. Until the end of your next turn, your allies gain advantage on attack rolls against enemies within 10 feet of you that can see you. Once you use this trait, you can't use it again until you finish a short or long rest.", options: &[], num_picks: 0 },
    RaceTrait { name: "Pack Tactics", description: "You have advantage on an attack roll against a creature if at least one of your allies is adjacent to the creature and the ally isn't incapacitated.", options: &[], num_picks: 0 },
    RaceTrait { name: "Sunlight Sensitivity", description: "You have disadvantage on attack rolls and on Wisdom (Perception) checks that rely on sight when you, the target of your attack, or whatever you are trying to perceive is in direct sunlight.", options: &[], num_picks: 0 },
];

pub static ORC_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Darkvision", description: "60-foot darkvision.", options: &[], num_picks: 0 },
    RaceTrait { name: "Aggressive", description: "As a bonus action, you can move up to your speed toward an enemy of your choice that you can see or hear. You must end this move closer to the enemy than you started.", options: &[], num_picks: 0 },
    RaceTrait { name: "Menacing", description: "You are proficient in the Intimidation skill.", options: &[], num_picks: 0 },
    RaceTrait { name: "Powerful Build", description: "You count as one size larger when determining carrying capacity.", options: &[], num_picks: 0 },
];

pub static YUAN_TI_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Darkvision", description: "60-foot darkvision.", options: &[], num_picks: 0 },
    RaceTrait { name: "Innate Spellcasting", description: "You know the poison spray cantrip. You can cast animal friendship an unlimited number of times with this trait, but you can target only snakes with it. Starting at 3rd level, you can also cast suggestion with this trait. Once you cast it, you can't do so again until you finish a long rest. Charisma is your spellcasting ability for these spells.", options: &[], num_picks: 0 },
    RaceTrait { name: "Magic Resistance", description: "You have advantage on saving throws against spells and other magical effects.", options: &[], num_picks: 0 },
    RaceTrait { name: "Poison Immunity", description: "You are immune to poison damage and the poisoned condition.", options: &[], num_picks: 0 },
];

pub static CUSTOM_LINEAGE_TRAITS: &[RaceTrait] = &[
    RaceTrait { name: "Ability Score Increase", description: "+2 to one ability score of your choice. Apply manually via the Stats tab.", options: &[], num_picks: 0 },
    RaceTrait { name: "Variable Trait", description: "You gain darkvision with a range of 60 feet, OR you gain proficiency in one skill of your choice.", options: SKILL_CHOICES_2, num_picks: 1 },
    RaceTrait { name: "Feat", description: "You gain one feat of your choice (select via the ASI/Feat slot).", options: &[], num_picks: 0 },
    RaceTrait { name: "Language", description: "You can speak, read, and write Common and one other language that you and your DM agree is appropriate for the character.", options: &[], num_picks: 0 },
];
