//! PHB class features for all 12 base classes, indexed by class level (0 = level 1).
//!
//! Each entry is a slice of `LevelFeature` — everything granted at that level.
//! Subclass features are represented as a single named entry at the appropriate level.
//! ASI levels include an "Ability Score Improvement" entry.

use super::types::LevelFeature;

// ─── Barbarian ────────────────────────────────────────────────────────────────

pub static BARBARIAN_FEATURES: &[&[LevelFeature]] = &[
    // Level 1
    &[
        LevelFeature { name: "Rage", description: "In battle, you fight with primal ferocity. On your turn, you can enter a rage as a bonus action. While raging, you have advantage on Strength checks and Strength saving throws; you gain a bonus to melee weapon attack damage rolls using Strength (increases as you level up); and you have resistance to bludgeoning, piercing, and slashing damage. Your rage lasts for 1 minute. It ends early if you are knocked unconscious or if your turn ends and you haven't attacked a hostile creature since your last turn or taken damage since then. You can also end your rage on your turn as a bonus action. Number of rages per day and damage bonus increase at higher levels." },
        LevelFeature { name: "Unarmored Defense", description: "While you are not wearing any armor, your Armor Class equals 10 + your Dexterity modifier + your Constitution modifier. You can use a shield and still gain this benefit." },
    ],
    // Level 2
    &[
        LevelFeature { name: "Reckless Attack", description: "Starting at 2nd level, you can throw aside all concern for defense to attack with fierce desperation. When you make your first attack on your turn, you can decide to attack recklessly. Doing so gives you advantage on melee weapon attack rolls using Strength during this turn, but attack rolls against you have advantage until your next turn." },
        LevelFeature { name: "Danger Sense", description: "At 2nd level, you gain an uncanny sense of when things nearby aren't as they should be, giving you an edge when you dodge away from danger. You have advantage on Dexterity saving throws against effects that you can see, such as traps and spells. To gain this benefit, you can't be blinded, deafened, or incapacitated." },
    ],
    // Level 3
    &[
        LevelFeature { name: "Primal Path", description: "At 3rd level, you choose a path that shapes the nature of your rage. Your choice grants you features at 3rd level and again at 6th, 10th, and 14th levels." },
    ],
    // Level 4
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 4th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. As normal, you can't increase an ability score above 20 using this feature. Alternatively, you can forgo this increase to take a feat." },
    ],
    // Level 5
    &[
        LevelFeature { name: "Extra Attack", description: "Beginning at 5th level, you can attack twice, instead of once, whenever you take the Attack action on your turn." },
        LevelFeature { name: "Fast Movement", description: "Starting at 5th level, your speed increases by 10 feet while you aren't wearing heavy armor." },
    ],
    // Level 6
    &[
        LevelFeature { name: "Primal Path Feature", description: "At 6th level, you gain a feature granted by your chosen Primal Path." },
    ],
    // Level 7
    &[
        LevelFeature { name: "Feral Instinct", description: "By 7th level, your instincts are so honed that you have advantage on initiative rolls. Additionally, if you are surprised at the beginning of combat and aren't incapacitated, you can act normally on your first turn, but only if you enter your rage before doing anything else on that turn." },
    ],
    // Level 8
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 8th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. As normal, you can't increase an ability score above 20 using this feature. Alternatively, you can forgo this increase to take a feat." },
    ],
    // Level 9
    &[
        LevelFeature { name: "Brutal Critical", description: "Beginning at 9th level, you can roll one additional weapon damage die when determining the extra damage for a critical hit with a melee attack. This increases to two additional dice at 13th level and three additional dice at 17th level." },
    ],
    // Level 10
    &[
        LevelFeature { name: "Primal Path Feature", description: "At 10th level, you gain a feature granted by your chosen Primal Path." },
    ],
    // Level 11
    &[
        LevelFeature { name: "Relentless Rage", description: "Starting at 11th level, your rage can keep you fighting despite grievous wounds. If you drop to 0 hit points while you're raging and don't die outright, you can make a DC 10 Constitution saving throw. If you succeed, you drop to 1 hit point instead. Each time you use this feature after the first, the DC increases by 5. When you finish a short or long rest, the DC resets to 10." },
    ],
    // Level 12
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 12th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. As normal, you can't increase an ability score above 20 using this feature. Alternatively, you can forgo this increase to take a feat." },
    ],
    // Level 13
    &[
        LevelFeature { name: "Brutal Critical (2 dice)", description: "At 13th level, you can roll two additional weapon damage dice when determining the extra damage for a critical hit with a melee attack (up from one)." },
    ],
    // Level 14
    &[
        LevelFeature { name: "Primal Path Feature", description: "At 14th level, you gain a feature granted by your chosen Primal Path." },
    ],
    // Level 15
    &[
        LevelFeature { name: "Persistent Rage", description: "Beginning at 15th level, your rage is so fierce that it ends early only if you fall unconscious or if you choose to end it." },
    ],
    // Level 16
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 16th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. As normal, you can't increase an ability score above 20 using this feature. Alternatively, you can forgo this increase to take a feat." },
    ],
    // Level 17
    &[
        LevelFeature { name: "Brutal Critical (3 dice)", description: "At 17th level, you can roll three additional weapon damage dice when determining the extra damage for a critical hit with a melee attack (up from two)." },
    ],
    // Level 18
    &[
        LevelFeature { name: "Indomitable Might", description: "Beginning at 18th level, if your total for a Strength check is less than your Strength score, you can use that score in place of the total." },
    ],
    // Level 19
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. As normal, you can't increase an ability score above 20 using this feature. Alternatively, you can forgo this increase to take a feat." },
    ],
    // Level 20
    &[
        LevelFeature { name: "Primal Champion", description: "At 20th level, you embody the power of the wilds. Your Strength and Constitution scores increase by 4. Your maximum for those scores is now 24." },
    ],
];

// ─── Bard ─────────────────────────────────────────────────────────────────────

pub static BARD_FEATURES: &[&[LevelFeature]] = &[
    // Level 1
    &[
        LevelFeature { name: "Spellcasting", description: "You have learned to untangle and reshape the fabric of reality in harmony with your wishes and music. Your spells are part of your vast repertoire, magic that you can tune to different situations. You know two cantrips of your choice from the bard spell list. You know four 1st-level spells of your choice." },
        LevelFeature { name: "Bardic Inspiration", description: "You can inspire others through stirring words or music. To do so, you use a bonus action on your turn to choose one creature other than yourself within 60 feet of you who can hear you. That creature gains one Bardic Inspiration die, a d6. Once within the next 10 minutes, the creature can roll the die and add the number rolled to one ability check, attack roll, or saving throw it makes. The die changes when you reach certain levels in this class: d8 at 5th level, d10 at 10th level, and d12 at 15th level." },
    ],
    // Level 2
    &[
        LevelFeature { name: "Jack of All Trades", description: "Starting at 2nd level, you can add half your proficiency bonus, rounded down, to any ability check you make that doesn't already include your proficiency bonus." },
        LevelFeature { name: "Song of Rest", description: "Beginning at 2nd level, you can use soothing music or oration to help revitalize your wounded allies during a short rest. If you or any friendly creatures who can hear your performance regain hit points at the end of the short rest by spending one or more Hit Dice, each of those creatures regains an extra 1d6 hit points. The extra hit points increase when you reach certain levels in this class: 1d8 at 9th level, 1d10 at 13th level, and 1d12 at 17th level." },
    ],
    // Level 3
    &[
        LevelFeature { name: "Bard College", description: "At 3rd level, you delve into the advanced techniques of a bard college of your choice. Your choice grants you features at 3rd level and again at 6th and 14th level." },
        LevelFeature { name: "Expertise", description: "At 3rd level, choose two of your skill proficiencies. Your proficiency bonus is doubled for any ability check you make that uses either of the chosen proficiencies. At 10th level, you can choose another two skill proficiencies to gain this benefit." },
    ],
    // Level 4
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 4th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 5
    &[
        LevelFeature { name: "Bardic Inspiration (d8)", description: "At 5th level, your Bardic Inspiration die changes to a d8. Also, you regain all your expended uses of Bardic Inspiration when you finish a short or long rest." },
        LevelFeature { name: "Font of Inspiration", description: "Beginning when you reach 5th level, you regain all your expended uses of Bardic Inspiration when you finish a short or long rest." },
    ],
    // Level 6
    &[
        LevelFeature { name: "Countercharm", description: "At 6th level, you gain the ability to use musical notes or words of power to disrupt mind-influencing effects. As an action, you can start a performance that lasts until the end of your next turn. During that time, you and any friendly creatures within 30 feet of you have advantage on saving throws against being frightened or charmed." },
        LevelFeature { name: "Bard College Feature", description: "At 6th level, you gain a feature granted by your Bard College." },
    ],
    // Level 7
    &[],
    // Level 8
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 8th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 9
    &[
        LevelFeature { name: "Song of Rest (d8)", description: "At 9th level, your Song of Rest die changes to a d8." },
    ],
    // Level 10
    &[
        LevelFeature { name: "Bardic Inspiration (d10)", description: "At 10th level, your Bardic Inspiration die changes to a d10." },
        LevelFeature { name: "Expertise (2 more)", description: "At 10th level, choose two more skill proficiencies. Your proficiency bonus is doubled for any ability check you make that uses either of the chosen proficiencies." },
        LevelFeature { name: "Magical Secrets", description: "By 10th level, you have plundered magical knowledge from a wide spectrum of disciplines. Choose two spells from any classes, including this one. The spells must be of a level for which you have spell slots. The chosen spells count as bard spells for you and are included in the number in the Spells Known column of the Bard table. You learn two additional spells from any classes at 14th level and again at 18th level." },
    ],
    // Level 11
    &[],
    // Level 12
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 12th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 13
    &[
        LevelFeature { name: "Song of Rest (d10)", description: "At 13th level, your Song of Rest die changes to a d10." },
    ],
    // Level 14
    &[
        LevelFeature { name: "Magical Secrets (2 more)", description: "At 14th level, you learn two additional spells from any classes." },
        LevelFeature { name: "Bard College Feature", description: "At 14th level, you gain a feature granted by your Bard College." },
    ],
    // Level 15
    &[
        LevelFeature { name: "Bardic Inspiration (d12)", description: "At 15th level, your Bardic Inspiration die changes to a d12." },
    ],
    // Level 16
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 16th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 17
    &[
        LevelFeature { name: "Song of Rest (d12)", description: "At 17th level, your Song of Rest die changes to a d12." },
    ],
    // Level 18
    &[
        LevelFeature { name: "Magical Secrets (2 more)", description: "At 18th level, you learn two additional spells from any classes." },
    ],
    // Level 19
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 20
    &[
        LevelFeature { name: "Superior Inspiration", description: "At 20th level, when you roll initiative and have no uses of Bardic Inspiration left, you regain one use." },
    ],
];

// ─── Cleric ───────────────────────────────────────────────────────────────────

pub static CLERIC_FEATURES: &[&[LevelFeature]] = &[
    // Level 1
    &[
        LevelFeature { name: "Divine Domain", description: "At 1st level, you choose a domain shaped by your deity. Your choice grants you domain spells and other features when you choose it at 1st level. It also grants you additional ways to use Channel Divinity when you gain that feature at 2nd level, and additional benefits at 6th, 8th, and 17th levels." },
        LevelFeature { name: "Spellcasting", description: "As a conduit for divine power, you can cast cleric spells. You know three cantrips from the cleric spell list. You prepare the list of cleric spells that are available for you to cast, choosing from the cleric spell list. When you do so, choose a number of cleric spells equal to your Wisdom modifier + your cleric level (minimum of one spell)." },
    ],
    // Level 2
    &[
        LevelFeature { name: "Channel Divinity (1/rest)", description: "At 2nd level, you gain the ability to channel divine energy directly from your deity, using that energy to fuel magical effects. You start with two such effects: Turn Undead and an effect determined by your domain. Some domains grant you additional effects as you advance in levels. When you use your Channel Divinity, you choose which effect to create. You must then finish a short or long rest to use your Channel Divinity again." },
        LevelFeature { name: "Channel Divinity: Turn Undead", description: "As an action, you present your holy symbol and speak a prayer censuring the undead. Each undead that can see or hear you within 30 feet of you must make a Wisdom saving throw. If the creature fails its saving throw, it is turned for 1 minute or until it takes any damage." },
        LevelFeature { name: "Divine Domain Feature", description: "At 2nd level, you gain an additional Channel Divinity option from your chosen Divine Domain." },
    ],
    // Level 3
    &[],
    // Level 4
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 4th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 5
    &[
        LevelFeature { name: "Destroy Undead (CR 1/2)", description: "Starting at 5th level, when an undead fails its saving throw against your Turn Undead feature, the creature is instantly destroyed if its challenge rating is at or below 1/2." },
    ],
    // Level 6
    &[
        LevelFeature { name: "Channel Divinity (2/rest)", description: "Beginning at 6th level, you can use your Channel Divinity twice between rests." },
        LevelFeature { name: "Divine Domain Feature", description: "At 6th level, you gain a feature granted by your Divine Domain." },
    ],
    // Level 7
    &[],
    // Level 8
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 8th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
        LevelFeature { name: "Destroy Undead (CR 1)", description: "At 8th level, your Turn Undead destroys undead of CR 1 or lower." },
        LevelFeature { name: "Divine Domain Feature", description: "At 8th level, you gain a feature granted by your Divine Domain." },
    ],
    // Level 9
    &[],
    // Level 10
    &[
        LevelFeature { name: "Divine Intervention", description: "Beginning at 10th level, you can call on your deity to intervene on your behalf when your need is great. Imploring your deity's aid requires you to use your action. Describe the assistance you seek, and roll percentile dice. If you roll a number equal to or lower than your cleric level, your deity intervenes. The DM chooses the nature of the intervention; the effect of any cleric spell or cleric domain spell would be appropriate. If your deity intervenes, you can't use this feature again for 7 days. Otherwise, you can use it again after you finish a long rest. At 20th level, your call for intervention succeeds automatically, no roll required." },
    ],
    // Level 11
    &[
        LevelFeature { name: "Destroy Undead (CR 2)", description: "At 11th level, your Turn Undead destroys undead of CR 2 or lower." },
    ],
    // Level 12
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 12th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 13
    &[],
    // Level 14
    &[
        LevelFeature { name: "Destroy Undead (CR 3)", description: "At 14th level, your Turn Undead destroys undead of CR 3 or lower." },
    ],
    // Level 15
    &[],
    // Level 16
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 16th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 17
    &[
        LevelFeature { name: "Destroy Undead (CR 4)", description: "At 17th level, your Turn Undead destroys undead of CR 4 or lower." },
        LevelFeature { name: "Divine Domain Feature", description: "At 17th level, you gain a feature granted by your Divine Domain." },
    ],
    // Level 18
    &[
        LevelFeature { name: "Channel Divinity (3/rest)", description: "Beginning at 18th level, you can use your Channel Divinity three times between rests." },
    ],
    // Level 19
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 20
    &[
        LevelFeature { name: "Divine Intervention (auto)", description: "At 20th level, your call for Divine Intervention succeeds automatically, no roll required. You can't use this feature again for 7 days." },
    ],
];

// ─── Druid ────────────────────────────────────────────────────────────────────

pub static DRUID_FEATURES: &[&[LevelFeature]] = &[
    // Level 1
    &[
        LevelFeature { name: "Druidic", description: "You know Druidic, the secret language of druids. You can speak the language and use it to leave hidden messages. You and others who know this language automatically spot such a message. Others spot the message's presence with a successful DC 15 Wisdom (Perception) check but can't decipher it without magic." },
        LevelFeature { name: "Spellcasting", description: "Drawing on the divine essence of nature itself, you can cast spells to shape that essence to your will. You know two cantrips of your choice from the druid spell list. You prepare the list of druid spells that are available for you to cast, choosing from the druid spell list. When you do so, choose a number of druid spells equal to your Wisdom modifier + your druid level (minimum of one spell)." },
    ],
    // Level 2
    &[
        LevelFeature { name: "Wild Shape", description: "Starting at 2nd level, you can use your action to magically assume the shape of a beast that you have seen before. You can use this feature twice. You regain expended uses when you finish a short or long rest. At 2nd level, you can transform into a beast with a challenge rating as high as 1/4 (no flying or swimming speed). Starting at 4th level, you can transform into a beast with a challenge rating as high as 1/2. Starting at 8th level, you can transform into a beast with a challenge rating as high as 1." },
        LevelFeature { name: "Druid Circle", description: "At 2nd level, you choose to identify with a circle of druids. Your choice grants you features at 2nd level and again at 6th, 10th, and 14th level." },
    ],
    // Level 3
    &[],
    // Level 4
    &[
        LevelFeature { name: "Wild Shape (CR 1/2)", description: "Starting at 4th level, you can transform into a beast with a challenge rating as high as 1/2 (no flying speed)." },
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 4th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 5
    &[],
    // Level 6
    &[
        LevelFeature { name: "Druid Circle Feature", description: "At 6th level, you gain a feature granted by your Druid Circle." },
    ],
    // Level 7
    &[],
    // Level 8
    &[
        LevelFeature { name: "Wild Shape (CR 1)", description: "Starting at 8th level, you can transform into a beast with a challenge rating as high as 1." },
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 8th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 9
    &[],
    // Level 10
    &[
        LevelFeature { name: "Druid Circle Feature", description: "At 10th level, you gain a feature granted by your Druid Circle." },
    ],
    // Level 11
    &[],
    // Level 12
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 12th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 13
    &[],
    // Level 14
    &[
        LevelFeature { name: "Druid Circle Feature", description: "At 14th level, you gain a feature granted by your Druid Circle." },
    ],
    // Level 15
    &[],
    // Level 16
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 16th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 17
    &[],
    // Level 18
    &[
        LevelFeature { name: "Timeless Body", description: "Starting at 18th level, the primal magic that you wield causes you to age more slowly. For every 10 years that pass, your body ages only 1 year." },
        LevelFeature { name: "Beast Spells", description: "Beginning at 18th level, you can cast many of your druid spells in any shape you assume using Wild Shape. You can perform the somatic and verbal components of a druid spell while in a beast shape, but you aren't able to provide material components." },
    ],
    // Level 19
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 20
    &[
        LevelFeature { name: "Archdruid", description: "At 20th level, you can use your Wild Shape an unlimited number of times. Additionally, you can ignore the verbal and somatic components of your druid spells, as well as any material components that lack a cost and aren't consumed by a spell. You gain this benefit in both your normal shape and your beast shape from Wild Shape." },
    ],
];

// ─── Fighter ──────────────────────────────────────────────────────────────────

pub static FIGHTER_FEATURES: &[&[LevelFeature]] = &[
    // Level 1
    &[
        LevelFeature { name: "Fighting Style", description: "You adopt a particular style of fighting as your specialty. Choose one of the following options: Archery, Defense, Dueling, Great Weapon Fighting, Protection, or Two-Weapon Fighting. You can't take a Fighting Style option more than once, even if you later get to choose again." },
        LevelFeature { name: "Second Wind", description: "You have a limited well of stamina that you can draw on to protect yourself from harm. On your turn, you can use a bonus action to regain hit points equal to 1d10 + your fighter level. Once you use this feature, you must finish a short or long rest before you can use it again." },
    ],
    // Level 2
    &[
        LevelFeature { name: "Action Surge (1/rest)", description: "Starting at 2nd level, you can push yourself beyond your normal limits for a moment. On your turn, you can take one additional action on top of your regular action and a possible bonus action. Once you use this feature, you must finish a short or long rest before you can use it again. Starting at 17th level, you can use it twice before a rest, but only once on the same turn." },
    ],
    // Level 3
    &[
        LevelFeature { name: "Martial Archetype", description: "At 3rd level, you choose an archetype that you strive to emulate in your combat styles and techniques. The archetype you choose grants you features at 3rd level and again at 7th, 10th, 15th, and 18th level." },
    ],
    // Level 4
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 4th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 5
    &[
        LevelFeature { name: "Extra Attack (×2)", description: "Beginning at 5th level, you can attack twice, instead of once, whenever you take the Attack action on your turn." },
    ],
    // Level 6
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 6th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 7
    &[
        LevelFeature { name: "Martial Archetype Feature", description: "At 7th level, you gain a feature granted by your Martial Archetype." },
    ],
    // Level 8
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 8th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 9
    &[
        LevelFeature { name: "Indomitable (1/rest)", description: "Beginning at 9th level, you can reroll a saving throw that you fail. If you do so, you must use the new roll, and you can't use this feature again until you finish a long rest. You can use this feature twice between long rests starting at 13th level and three times between long rests starting at 17th level." },
    ],
    // Level 10
    &[
        LevelFeature { name: "Martial Archetype Feature", description: "At 10th level, you gain a feature granted by your Martial Archetype." },
    ],
    // Level 11
    &[
        LevelFeature { name: "Extra Attack (×3)", description: "At 11th level, you can attack three times whenever you take the Attack action on your turn." },
    ],
    // Level 12
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 12th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 13
    &[
        LevelFeature { name: "Indomitable (2/rest)", description: "At 13th level, you can use Indomitable twice between long rests." },
    ],
    // Level 14
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 14th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 15
    &[
        LevelFeature { name: "Martial Archetype Feature", description: "At 15th level, you gain a feature granted by your Martial Archetype." },
    ],
    // Level 16
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 16th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 17
    &[
        LevelFeature { name: "Action Surge (2/rest)", description: "At 17th level, you can use Action Surge twice before a rest, but only once on the same turn." },
        LevelFeature { name: "Indomitable (3/rest)", description: "At 17th level, you can use Indomitable three times between long rests." },
    ],
    // Level 18
    &[
        LevelFeature { name: "Martial Archetype Feature", description: "At 18th level, you gain a feature granted by your Martial Archetype." },
    ],
    // Level 19
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 20
    &[
        LevelFeature { name: "Extra Attack (×4)", description: "At 20th level, you can attack four times whenever you take the Attack action on your turn." },
    ],
];

// ─── Monk ─────────────────────────────────────────────────────────────────────

pub static MONK_FEATURES: &[&[LevelFeature]] = &[
    // Level 1
    &[
        LevelFeature { name: "Unarmored Defense", description: "Beginning at 1st level, while you are wearing no armor and not wielding a shield, your AC equals 10 + your Dexterity modifier + your Wisdom modifier." },
        LevelFeature { name: "Martial Arts", description: "At 1st level, your practice of martial arts gives you mastery of combat styles that use unarmed strikes and monk weapons. You gain the following benefits: you can use Dexterity instead of Strength for attack and damage rolls of your unarmed strikes and monk weapons; you can roll a d4 in place of the normal damage of your unarmed strike or monk weapon (this die changes as you gain monk levels); and when you use the Attack action with an unarmed strike or a monk weapon on your turn, you can make one unarmed strike as a bonus action." },
    ],
    // Level 2
    &[
        LevelFeature { name: "Ki", description: "Starting at 2nd level, your training allows you to harness the mystic energy of ki. Your access to this energy is represented by a number of ki points. You can spend ki points to fuel various ki features. You start knowing three such features: Flurry of Blows, Patient Defense, and Step of the Wind. You regain all expended ki points when you finish a short or long rest." },
        LevelFeature { name: "Unarmored Movement", description: "Starting at 2nd level, your speed increases by 10 feet while you are not wearing armor or wielding a shield. This bonus increases when you reach certain monk levels: 15 feet at 6th level, 20 feet at 10th level, 25 feet at 14th level, and 30 feet at 18th level." },
    ],
    // Level 3
    &[
        LevelFeature { name: "Monastic Tradition", description: "When you reach 3rd level, you commit yourself to a monastic tradition. Your tradition grants you features at 3rd level and again at 6th, 11th, and 17th level." },
        LevelFeature { name: "Deflect Missiles", description: "Starting at 3rd level, you can use your reaction to deflect or catch the missile when you are hit by a ranged weapon attack. When you do so, the damage you take from the attack is reduced by 1d10 + your Dexterity modifier + your monk level. If you reduce the damage to 0, you can catch the missile if it is small enough for you to hold in one hand and you have at least one hand free." },
    ],
    // Level 4
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 4th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
        LevelFeature { name: "Slow Fall", description: "Beginning at 4th level, you can use your reaction when you fall to reduce any falling damage you take by an amount equal to five times your monk level." },
    ],
    // Level 5
    &[
        LevelFeature { name: "Extra Attack", description: "Beginning at 5th level, you can attack twice, instead of once, whenever you take the Attack action on your turn." },
        LevelFeature { name: "Stunning Strike", description: "Starting at 5th level, you can interfere with the flow of ki in an opponent's body. When you hit another creature with a melee weapon attack, you can spend 1 ki point to attempt a stunning strike. The target must succeed on a Constitution saving throw or be stunned until the end of your next turn." },
    ],
    // Level 6
    &[
        LevelFeature { name: "Ki-Empowered Strikes", description: "Starting at 6th level, your unarmed strikes count as magical for the purpose of overcoming resistance and immunity to nonmagical attacks and damage." },
        LevelFeature { name: "Monastic Tradition Feature", description: "At 6th level, you gain a feature granted by your Monastic Tradition." },
        LevelFeature { name: "Unarmored Movement (+15 ft)", description: "At 6th level, your Unarmored Movement speed bonus increases to 15 feet." },
    ],
    // Level 7
    &[
        LevelFeature { name: "Evasion", description: "At 7th level, your instinctive agility lets you dodge out of the way of certain area effects, such as a blue dragon's lightning breath or a fireball spell. When you are subjected to an effect that allows you to make a Dexterity saving throw to take only half damage, you instead take no damage if you succeed on the saving throw, and only half damage if you fail." },
        LevelFeature { name: "Stillness of Mind", description: "Starting at 7th level, you can use your action to end one effect on yourself that is causing you to be charmed or frightened." },
    ],
    // Level 8
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 8th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 9
    &[
        LevelFeature { name: "Unarmored Movement (wall/water run)", description: "At 9th level, you gain the ability to move along vertical surfaces and across liquids on your turn without falling during the move." },
    ],
    // Level 10
    &[
        LevelFeature { name: "Purity of Body", description: "At 10th level, your mastery of the ki flowing through you makes you immune to disease and poison." },
        LevelFeature { name: "Unarmored Movement (+20 ft)", description: "At 10th level, your Unarmored Movement speed bonus increases to 20 feet." },
    ],
    // Level 11
    &[
        LevelFeature { name: "Monastic Tradition Feature", description: "At 11th level, you gain a feature granted by your Monastic Tradition." },
    ],
    // Level 12
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 12th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 13
    &[
        LevelFeature { name: "Tongue of the Sun and Moon", description: "Starting at 13th level, you learn to touch the ki of other minds so that you understand all spoken languages. Moreover, any creature that can understand a language can understand what you say." },
    ],
    // Level 14
    &[
        LevelFeature { name: "Diamond Soul", description: "Beginning at 14th level, your mastery of ki grants you proficiency in all saving throws. Additionally, whenever you make a saving throw and fail, you can spend 1 ki point to reroll it and take the second result." },
        LevelFeature { name: "Unarmored Movement (+25 ft)", description: "At 14th level, your Unarmored Movement speed bonus increases to 25 feet." },
    ],
    // Level 15
    &[
        LevelFeature { name: "Timeless Body", description: "At 15th level, your ki sustains you so that you suffer none of the frailty of old age, and you can't be aged magically. You can still die of old age, however. In addition, you no longer need food or water." },
    ],
    // Level 16
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 16th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 17
    &[
        LevelFeature { name: "Monastic Tradition Feature", description: "At 17th level, you gain a feature granted by your Monastic Tradition." },
    ],
    // Level 18
    &[
        LevelFeature { name: "Empty Body", description: "Beginning at 18th level, you can use your action to spend 4 ki points to become invisible for 1 minute. During that time, you also have resistance to all damage but force damage. Additionally, you can spend 8 ki points to cast the astral projection spell, without needing material components. When you do so, you can't take any other creatures with you." },
        LevelFeature { name: "Unarmored Movement (+30 ft)", description: "At 18th level, your Unarmored Movement speed bonus increases to 30 feet." },
    ],
    // Level 19
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 20
    &[
        LevelFeature { name: "Perfect Self", description: "At 20th level, when you roll for initiative and have no ki points remaining, you regain 4 ki points." },
    ],
];

// ─── Paladin ──────────────────────────────────────────────────────────────────

pub static PALADIN_FEATURES: &[&[LevelFeature]] = &[
    // Level 1
    &[
        LevelFeature { name: "Divine Sense", description: "The presence of strong evil registers on your senses like a noxious odor, and powerful good rings like heavenly music in your ears. As an action, you can open your awareness to detect such forces. Until the end of your next turn, you know the location of any celestial, fiend, or undead within 60 feet of you that is not behind total cover. You know the type of any being whose presence you sense, but not its identity. Within the same radius, you also detect the presence of any place or object that has been consecrated or desecrated. You can use this feature a number of times equal to 1 + your Charisma modifier. When you finish a long rest, you regain all expended uses." },
        LevelFeature { name: "Lay on Hands", description: "Your blessed touch can heal wounds. You have a pool of healing power that replenishes when you take a long rest. With that pool, you can restore a total number of hit points equal to your paladin level × 5. As an action, you can touch a creature and draw power from the pool to restore a number of hit points to that creature, up to the maximum amount remaining in your pool. Alternatively, you can expend 5 hit points from your pool of healing to cure the target of one disease or neutralize one poison affecting it." },
    ],
    // Level 2
    &[
        LevelFeature { name: "Fighting Style", description: "Starting at 2nd level, you adopt a style of fighting as your specialty. Choose one of the following options: Defense, Dueling, Great Weapon Fighting, or Protection." },
        LevelFeature { name: "Spellcasting", description: "By 2nd level, you have learned to draw on divine magic through meditation and prayer to cast spells as a cleric does. You prepare the list of paladin spells that are available for you to cast, choosing from the paladin spell list. When you do so, choose a number of paladin spells equal to your Charisma modifier + half your paladin level, rounded down (minimum of one spell)." },
        LevelFeature { name: "Divine Smite", description: "Starting at 2nd level, when you hit a creature with a melee weapon attack, you can expend one spell slot to deal radiant damage to the target, in addition to the weapon's damage. The extra damage is 2d8 for a 1st-level spell slot, plus 1d8 for each spell level higher than 1st, to a maximum of 5d8. The damage increases by 1d8 if the target is an undead or a fiend, to a maximum of 6d8." },
    ],
    // Level 3
    &[
        LevelFeature { name: "Divine Health", description: "By 3rd level, the divine magic flowing through you makes you immune to disease." },
        LevelFeature { name: "Sacred Oath", description: "When you reach 3rd level, you swear the oath that binds you as a paladin forever. Up to this time you have been in a preparatory stage, committed to the path but not yet sworn to it. Now you choose an oath. Your choice grants you features at 3rd level and again at 7th and 20th level." },
        LevelFeature { name: "Channel Divinity", description: "Your Sacred Oath allows you to channel divine energy to fuel magical effects. Each Channel Divinity option provided by your oath explains how to use it. When you use your Channel Divinity, you choose which option to use. You must then finish a short or long rest to use your Channel Divinity again. Some Channel Divinity effects require saving throws. When you use such an effect from this class, the DC equals your paladin spell save DC." },
    ],
    // Level 4
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 4th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 5
    &[
        LevelFeature { name: "Extra Attack", description: "Beginning at 5th level, you can attack twice, instead of once, whenever you take the Attack action on your turn." },
    ],
    // Level 6
    &[
        LevelFeature { name: "Aura of Protection", description: "Starting at 6th level, whenever you or a friendly creature within 10 feet of you must make a saving throw, the creature gains a bonus to the saving throw equal to your Charisma modifier (with a minimum bonus of +1). You must be conscious to grant this bonus. At 18th level, the range of this aura increases to 30 feet." },
    ],
    // Level 7
    &[
        LevelFeature { name: "Sacred Oath Feature", description: "At 7th level, you gain a feature granted by your Sacred Oath." },
    ],
    // Level 8
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 8th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 9
    &[],
    // Level 10
    &[
        LevelFeature { name: "Aura of Courage", description: "Starting at 10th level, you and friendly creatures within 10 feet of you can't be frightened while you are conscious. At 18th level, the range of this aura increases to 30 feet." },
    ],
    // Level 11
    &[
        LevelFeature { name: "Improved Divine Smite", description: "By 11th level, you are so suffused with righteous might that all your melee weapon strikes carry divine power with them. Whenever you hit a creature with a melee weapon, the creature takes an extra 1d8 radiant damage. If you also use your Divine Smite with an attack, you add this damage to the extra damage of your Divine Smite." },
    ],
    // Level 12
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 12th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 13
    &[],
    // Level 14
    &[
        LevelFeature { name: "Cleansing Touch", description: "Beginning at 14th level, you can use your action to end one spell on yourself or on one willing creature that you touch. You can use this feature a number of times equal to your Charisma modifier (a minimum of once). You regain expended uses when you finish a long rest." },
    ],
    // Level 15
    &[
        LevelFeature { name: "Sacred Oath Feature", description: "At 15th level, you gain a feature granted by your Sacred Oath." },
    ],
    // Level 16
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 16th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 17
    &[],
    // Level 18
    &[
        LevelFeature { name: "Aura improvements (30 ft)", description: "At 18th level, the ranges of your Aura of Protection and Aura of Courage increase to 30 feet." },
    ],
    // Level 19
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 20
    &[
        LevelFeature { name: "Sacred Oath Feature", description: "At 20th level, you gain a capstone feature granted by your Sacred Oath." },
    ],
];

// ─── Ranger ───────────────────────────────────────────────────────────────────

pub static RANGER_FEATURES: &[&[LevelFeature]] = &[
    // Level 1
    &[
        LevelFeature { name: "Favored Enemy", description: "Beginning at 1st level, you have significant experience studying, tracking, hunting, and even talking to a certain type of enemy. Choose a type of favored enemy: aberrations, beasts, celestials, constructs, dragons, elementals, fey, fiends, giants, monstrosities, oozes, plants, or undead. Alternatively, you can select two races of humanoid (such as gnolls and orcs) as favored enemies. You have advantage on Wisdom (Survival) checks to track your favored enemies, as well as on Intelligence checks to recall information about them." },
        LevelFeature { name: "Natural Explorer", description: "You are particularly familiar with one type of natural environment and are adept at traveling and surviving in such regions. Choose one type of favored terrain: arctic, coast, desert, forest, grassland, mountain, swamp, or the Underdark. When you make an Intelligence or Wisdom check related to your favored terrain, your proficiency bonus is doubled if you are using a skill that you're proficient in." },
    ],
    // Level 2
    &[
        LevelFeature { name: "Fighting Style", description: "At 2nd level, you adopt a particular style of fighting as your specialty. Choose one of the following options: Archery, Defense, Dueling, or Two-Weapon Fighting." },
        LevelFeature { name: "Spellcasting", description: "By the time you reach 2nd level, you have learned to use the magical essence of nature to cast spells, much as a druid does. You prepare the list of ranger spells that are available for you to cast, choosing from the ranger spell list. When you do so, choose a number of ranger spells equal to your Wisdom modifier + half your ranger level, rounded down (minimum of one spell)." },
    ],
    // Level 3
    &[
        LevelFeature { name: "Ranger Archetype", description: "At 3rd level, you choose an archetype that you strive to emulate. Your choice grants you features at 3rd level and again at 7th, 11th, and 15th level." },
        LevelFeature { name: "Primeval Awareness", description: "Beginning at 3rd level, you can use your action and expend one ranger spell slot to focus your awareness on the region around you. For 1 minute per level of the spell slot you expend, you can sense whether the following types of creatures are present within 1 mile of you (or within up to 6 miles if you are in your favored terrain): aberrations, celestials, dragons, elementals, fey, fiends, and undead. This feature doesn't reveal the creatures' location or number." },
    ],
    // Level 4
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 4th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 5
    &[
        LevelFeature { name: "Extra Attack", description: "Beginning at 5th level, you can attack twice, instead of once, whenever you take the Attack action on your turn." },
    ],
    // Level 6
    &[
        LevelFeature { name: "Favored Enemy and Terrain (2nd)", description: "At 6th level, you choose a second option for both Favored Enemy and Natural Explorer." },
    ],
    // Level 7
    &[
        LevelFeature { name: "Ranger Archetype Feature", description: "At 7th level, you gain a feature granted by your Ranger Archetype." },
    ],
    // Level 8
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 8th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
        LevelFeature { name: "Land's Stride", description: "Starting at 8th level, moving through nonmagical difficult terrain costs you no extra movement. You can also pass through nonmagical plants without being slowed by them and without taking damage from them if they have thorns, spines, or a similar hazard. In addition, you have advantage on saving throws against plants that are magically created or manipulated to impede movement." },
    ],
    // Level 9
    &[],
    // Level 10
    &[
        LevelFeature { name: "Natural Explorer (3rd terrain)", description: "At 10th level, you choose a third favored terrain." },
        LevelFeature { name: "Hide in Plain Sight", description: "Starting at 10th level, you can spend 1 minute creating camouflage for yourself. You must have access to fresh mud, dirt, plants, soot, and other naturally occurring materials with which to create your camouflage. Once you are camouflaged in this way, you can try to hide by pressing yourself up against a solid surface, such as a tree or wall, that is at least as tall and wide as you are." },
    ],
    // Level 11
    &[
        LevelFeature { name: "Ranger Archetype Feature", description: "At 11th level, you gain a feature granted by your Ranger Archetype." },
    ],
    // Level 12
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 12th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 13
    &[],
    // Level 14
    &[
        LevelFeature { name: "Favored Enemy (3rd)", description: "At 14th level, you choose a third favored enemy type, and you gain the language benefit for all favored enemies." },
        LevelFeature { name: "Vanish", description: "Starting at 14th level, you can use the Hide action as a bonus action on your turn. Also, you can't be tracked by nonmagical means, unless you choose to leave a trail." },
    ],
    // Level 15
    &[
        LevelFeature { name: "Ranger Archetype Feature", description: "At 15th level, you gain a feature granted by your Ranger Archetype." },
    ],
    // Level 16
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 16th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 17
    &[],
    // Level 18
    &[
        LevelFeature { name: "Feral Senses", description: "At 18th level, you gain preternatural senses that help you fight creatures you can't see. When you attack a creature you can't see, your inability to see it doesn't impose disadvantage on your attack rolls against it. You are also aware of the location of any invisible creature within 30 feet of you, provided that the creature isn't hidden from you and you aren't blinded or deafened." },
    ],
    // Level 19
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 20
    &[
        LevelFeature { name: "Foe Slayer", description: "At 20th level, you become an unparalleled hunter of your enemies. Once on each of your turns, you can add your Wisdom modifier to the attack roll or the damage roll of an attack you make against one of your favored enemies. You can choose to use this feature before or after the roll, but before any effects of the roll are applied." },
    ],
];

// ─── Rogue ────────────────────────────────────────────────────────────────────

pub static ROGUE_FEATURES: &[&[LevelFeature]] = &[
    // Level 1
    &[
        LevelFeature { name: "Expertise", description: "At 1st level, choose two of your skill proficiencies, or one of your skill proficiencies and your proficiency with thieves' tools. Your proficiency bonus is doubled for any ability check you make that uses either of the chosen proficiencies. At 6th level, you can choose two more of your proficiencies (in skills or with thieves' tools) to gain this benefit." },
        LevelFeature { name: "Sneak Attack", description: "Beginning at 1st level, you know how to strike subtly and exploit a foe's distraction. Once per turn, you can deal an extra 1d6 damage to one creature you hit with an attack if you have advantage on the attack roll. The attack must use a finesse or a ranged weapon. You don't need advantage on the attack roll if another enemy of the target is within 5 feet of it, that enemy isn't incapacitated, and you don't have disadvantage on the attack roll. The amount of the extra damage increases as you gain levels in this class." },
        LevelFeature { name: "Thieves' Cant", description: "During your rogue training you learned thieves' cant, a secret mix of dialect, jargon, and code that allows you to hide messages in seemingly normal conversation. Only another creature that knows thieves' cant understands such messages." },
    ],
    // Level 2
    &[
        LevelFeature { name: "Cunning Action", description: "Starting at 2nd level, your quick thinking and agility allow you to move and act quickly. You can take a bonus action on each of your turns in combat. This action can be used only to take the Dash, Disengage, or Hide action." },
    ],
    // Level 3
    &[
        LevelFeature { name: "Roguish Archetype", description: "At 3rd level, you choose an archetype that you emulate in the exercise of your rogue abilities. Your archetype choice grants you features at 3rd level and then again at 9th, 13th, and 17th level." },
    ],
    // Level 4
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 4th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 5
    &[
        LevelFeature { name: "Uncanny Dodge", description: "Starting at 5th level, when an attacker that you can see hits you with an attack, you can use your reaction to halve the attack's damage against you." },
    ],
    // Level 6
    &[
        LevelFeature { name: "Expertise (2 more)", description: "At 6th level, you can choose two more of your proficiencies to gain the Expertise benefit." },
    ],
    // Level 7
    &[
        LevelFeature { name: "Evasion", description: "Beginning at 7th level, you can nimbly dodge out of the way of certain area effects, such as a red dragon's fiery breath or an ice storm spell. When you are subjected to an effect that allows you to make a Dexterity saving throw to take only half damage, you instead take no damage if you succeed on the saving throw, and only half damage if you fail." },
    ],
    // Level 8
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 8th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 9
    &[
        LevelFeature { name: "Roguish Archetype Feature", description: "At 9th level, you gain a feature granted by your Roguish Archetype." },
    ],
    // Level 10
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 10th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 11
    &[
        LevelFeature { name: "Reliable Talent", description: "By 11th level, you have refined your chosen skills until they approach perfection. Whenever you make an ability check that lets you add your proficiency bonus, you can treat a d20 roll of 9 or lower as a 10." },
    ],
    // Level 12
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 12th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 13
    &[
        LevelFeature { name: "Roguish Archetype Feature", description: "At 13th level, you gain a feature granted by your Roguish Archetype." },
    ],
    // Level 14
    &[
        LevelFeature { name: "Blindsense", description: "Starting at 14th level, if you are able to hear, you are aware of the location of any hidden or invisible creature within 10 feet of you." },
    ],
    // Level 15
    &[
        LevelFeature { name: "Slippery Mind", description: "By 15th level, you have acquired greater mental strength. You gain proficiency in Wisdom saving throws." },
    ],
    // Level 16
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 16th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 17
    &[
        LevelFeature { name: "Roguish Archetype Feature", description: "At 17th level, you gain a feature granted by your Roguish Archetype." },
    ],
    // Level 18
    &[
        LevelFeature { name: "Elusive", description: "Beginning at 18th level, you are so evasive that attackers rarely gain the upper hand against you. No attack roll has advantage against you while you aren't incapacitated." },
    ],
    // Level 19
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 20
    &[
        LevelFeature { name: "Stroke of Luck", description: "At 20th level, you have an uncanny knack for succeeding when you need to. If your attack misses a target within range, you can turn the miss into a hit. Alternatively, if you fail an ability check, you can treat the d20 roll as a 20. Once you use this feature, you can't use it again until you finish a short or long rest." },
    ],
];

// ─── Sorcerer ─────────────────────────────────────────────────────────────────

pub static SORCERER_FEATURES: &[&[LevelFeature]] = &[
    // Level 1
    &[
        LevelFeature { name: "Spellcasting", description: "An event in your past, or in the life of a parent or ancestor, left an indelible mark on you, infusing you with arcane magic. This font of magic, whatever its origin, fuels your spells. You know four cantrips of your choice from the sorcerer spell list. You know two 1st-level spells of your choice from the sorcerer spell list." },
        LevelFeature { name: "Sorcerous Origin", description: "Choose a sorcerous origin, which describes the source of your innate magical power. Your choice grants you features when you choose it at 1st level and again at 6th, 14th, and 18th level." },
    ],
    // Level 2
    &[
        LevelFeature { name: "Font of Magic", description: "At 2nd level, you tap into a deep wellspring of magic within yourself. This wellspring is represented by sorcery points, which allow you to create a variety of magical effects. You have 2 sorcery points, and you gain one additional point every time you level up, to a maximum of 20 at level 20. You can never have more sorcery points than shown on the table for your level." },
    ],
    // Level 3
    &[
        LevelFeature { name: "Metamagic", description: "At 3rd level, you gain the ability to twist your spells to suit your needs. You gain two of the following Metamagic options of your choice: Careful Spell, Distant Spell, Empowered Spell, Extended Spell, Heightened Spell, Quickened Spell, Subtle Spell, or Twinned Spell. You gain another one at 10th and 17th level." },
    ],
    // Level 4
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 4th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 5
    &[],
    // Level 6
    &[
        LevelFeature { name: "Sorcerous Origin Feature", description: "At 6th level, you gain a feature granted by your Sorcerous Origin." },
    ],
    // Level 7
    &[],
    // Level 8
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 8th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 9
    &[],
    // Level 10
    &[
        LevelFeature { name: "Metamagic (3rd option)", description: "At 10th level, you learn one additional Metamagic option." },
    ],
    // Level 11
    &[],
    // Level 12
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 12th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 13
    &[],
    // Level 14
    &[
        LevelFeature { name: "Sorcerous Origin Feature", description: "At 14th level, you gain a feature granted by your Sorcerous Origin." },
    ],
    // Level 15
    &[],
    // Level 16
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 16th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 17
    &[
        LevelFeature { name: "Metamagic (4th option)", description: "At 17th level, you learn one additional Metamagic option." },
    ],
    // Level 18
    &[
        LevelFeature { name: "Sorcerous Origin Feature", description: "At 18th level, you gain a feature granted by your Sorcerous Origin." },
    ],
    // Level 19
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 20
    &[
        LevelFeature { name: "Sorcerous Restoration", description: "At 20th level, you regain 4 expended sorcery points whenever you finish a short rest." },
    ],
];

// ─── Warlock ──────────────────────────────────────────────────────────────────

pub static WARLOCK_FEATURES: &[&[LevelFeature]] = &[
    // Level 1
    &[
        LevelFeature { name: "Otherworldly Patron", description: "At 1st level, you have struck a bargain with an otherworldly being of your choice: the Archfey, the Fiend, or the Great Old One, each of which is detailed at the end of the class description. Your choice grants you features at 1st level and again at 6th, 10th, and 14th level." },
        LevelFeature { name: "Pact Magic", description: "Your arcane research and the magic bestowed on you by your patron have given you facility with spells. You know two cantrips of your choice from the warlock spell list. The Eldritch Invocations feature lets you unlock your cantrip selection further. You regain all expended spell slots when you finish a short or long rest." },
    ],
    // Level 2
    &[
        LevelFeature { name: "Eldritch Invocations", description: "In your study of occult lore, you have unearthed eldritch invocations, fragments of forbidden knowledge that imbue you with an abiding magical ability. At 2nd level, you gain two eldritch invocations of your choice. Your invocation options are detailed at the end of the class description. When you gain certain warlock levels, you gain additional invocations of your choice. Additionally, when you gain a level in this class, you can choose one of the invocations you know and replace it with another invocation that you could learn at that level." },
    ],
    // Level 3
    &[
        LevelFeature { name: "Pact Boon", description: "At 3rd level, your otherworldly patron bestows a gift upon you for your loyal service. You gain one of the following features of your choice: Pact of the Chain, Pact of the Blade, or Pact of the Tome." },
    ],
    // Level 4
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 4th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 5
    &[],
    // Level 6
    &[
        LevelFeature { name: "Otherworldly Patron Feature", description: "At 6th level, you gain a feature granted by your Otherworldly Patron." },
    ],
    // Level 7
    &[],
    // Level 8
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 8th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 9
    &[],
    // Level 10
    &[
        LevelFeature { name: "Otherworldly Patron Feature", description: "At 10th level, you gain a feature granted by your Otherworldly Patron." },
    ],
    // Level 11
    &[
        LevelFeature { name: "Mystic Arcanum (6th level)", description: "At 11th level, your patron bestows upon you a magical secret called an arcanum. Choose one 6th-level spell from the warlock spell list as this arcanum. You can cast your arcanum spell once without expending a spell slot. You must finish a long rest before you can do so again." },
    ],
    // Level 12
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 12th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 13
    &[
        LevelFeature { name: "Mystic Arcanum (7th level)", description: "At 13th level, choose one 7th-level spell from the warlock spell list as your Mystic Arcanum." },
    ],
    // Level 14
    &[
        LevelFeature { name: "Otherworldly Patron Feature", description: "At 14th level, you gain a feature granted by your Otherworldly Patron." },
    ],
    // Level 15
    &[
        LevelFeature { name: "Mystic Arcanum (8th level)", description: "At 15th level, choose one 8th-level spell from the warlock spell list as your Mystic Arcanum." },
    ],
    // Level 16
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 16th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 17
    &[
        LevelFeature { name: "Mystic Arcanum (9th level)", description: "At 17th level, choose one 9th-level spell from the warlock spell list as your Mystic Arcanum." },
    ],
    // Level 18
    &[],
    // Level 19
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 20
    &[
        LevelFeature { name: "Eldritch Master", description: "At 20th level, you can draw on your inner reserve of mystical power while entreating your patron to regain expended spell slots. You can spend 1 minute entreating your patron for aid to regain all your expended spell slots from your Pact Magic feature. Once you regain spell slots with this feature, you must finish a long rest before you can do so again." },
    ],
];

// ─── Wizard ───────────────────────────────────────────────────────────────────

pub static WIZARD_FEATURES: &[&[LevelFeature]] = &[
    // Level 1
    &[
        LevelFeature { name: "Spellcasting", description: "As a student of arcane magic, you have a spellbook containing spells that show the first glimmerings of your true power. You know three cantrips of your choice from the wizard spell list. At 1st level, you have a spellbook containing six 1st-level wizard spells of your choice." },
        LevelFeature { name: "Arcane Recovery", description: "You have learned to regain some of your magical energy by studying your spellbook. Once per day when you finish a short rest, you can choose expended spell slots to recover. The spell slots can have a combined level that is equal to or less than half your wizard level (rounded up), and none of the slots can be 6th level or higher." },
    ],
    // Level 2
    &[
        LevelFeature { name: "Arcane Tradition", description: "When you reach 2nd level, you choose an arcane tradition, shaping your practice of magic through one of eight schools: Abjuration, Conjuration, Divination, Enchantment, Evocation, Illusion, Necromancy, or Transmutation. Your choice grants you features at 2nd level and again at 6th, 10th, and 14th level." },
    ],
    // Level 3
    &[],
    // Level 4
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 4th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 5
    &[],
    // Level 6
    &[
        LevelFeature { name: "Arcane Tradition Feature", description: "At 6th level, you gain a feature granted by your Arcane Tradition." },
    ],
    // Level 7
    &[],
    // Level 8
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 8th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 9
    &[],
    // Level 10
    &[
        LevelFeature { name: "Arcane Tradition Feature", description: "At 10th level, you gain a feature granted by your Arcane Tradition." },
    ],
    // Level 11
    &[],
    // Level 12
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 12th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 13
    &[],
    // Level 14
    &[
        LevelFeature { name: "Arcane Tradition Feature", description: "At 14th level, you gain a feature granted by your Arcane Tradition." },
    ],
    // Level 15
    &[],
    // Level 16
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 16th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 17
    &[],
    // Level 18
    &[
        LevelFeature { name: "Spell Mastery", description: "At 18th level, you have achieved such mastery over certain spells that you can cast them at will. Choose a 1st-level wizard spell and a 2nd-level wizard spell that are in your spellbook. You can cast those spells at their lowest level without expending a spell slot when you have them prepared. If you want to cast either spell at a higher level, you must expend a spell slot as normal." },
    ],
    // Level 19
    &[
        LevelFeature { name: "Ability Score Improvement", description: "When you reach 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1 each. Alternatively, you can forgo this to take a feat." },
    ],
    // Level 20
    &[
        LevelFeature { name: "Signature Spells", description: "When you reach 20th level, you gain mastery over two powerful spells and can cast them with little effort. Choose two 3rd-level wizard spells in your spellbook as your signature spells. You always have these spells prepared, they don't count against the number of spells you have prepared, and you can cast each of them once at 3rd level without expending a spell slot. When you do so, you can't do so again until you finish a short or long rest. If you want to cast either spell at a higher level, you must expend a spell slot as normal." },
    ],
];
