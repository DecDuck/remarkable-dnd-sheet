//! PHB + Xanathar's backgrounds with typed skill-proficiency sets.
use super::types::{Skill, SkillSet, BackgroundData};

pub static BACKGROUNDS: &[BackgroundData] = &[
    // ── PHB ──────────────────────────────────────────────────────────────────
    // 0 — Acolyte
    BackgroundData {
        name: "Acolyte",
        skill_profs: SkillSet::new().set(Skill::Insight).set(Skill::Religion),
    },
    // 1 — Charlatan
    BackgroundData {
        name: "Charlatan",
        skill_profs: SkillSet::new().set(Skill::Deception).set(Skill::SleightOfHand),
    },
    // 2 — Criminal
    BackgroundData {
        name: "Criminal",
        skill_profs: SkillSet::new().set(Skill::Deception).set(Skill::Stealth),
    },
    // 3 — Entertainer
    BackgroundData {
        name: "Entertainer",
        skill_profs: SkillSet::new().set(Skill::Acrobatics).set(Skill::Performance),
    },
    // 4 — Folk Hero
    BackgroundData {
        name: "Folk Hero",
        skill_profs: SkillSet::new().set(Skill::AnimalHandling).set(Skill::Survival),
    },
    // 5 — Guild Artisan
    BackgroundData {
        name: "Guild Artisan",
        skill_profs: SkillSet::new().set(Skill::Insight).set(Skill::Persuasion),
    },
    // 6 — Hermit
    BackgroundData {
        name: "Hermit",
        skill_profs: SkillSet::new().set(Skill::Medicine).set(Skill::Religion),
    },
    // 7 — Noble
    BackgroundData {
        name: "Noble",
        skill_profs: SkillSet::new().set(Skill::History).set(Skill::Persuasion),
    },
    // 8 — Outlander
    BackgroundData {
        name: "Outlander",
        skill_profs: SkillSet::new().set(Skill::Athletics).set(Skill::Survival),
    },
    // 9 — Sage
    BackgroundData {
        name: "Sage",
        skill_profs: SkillSet::new().set(Skill::Arcana).set(Skill::History),
    },
    // 10 — Sailor
    BackgroundData {
        name: "Sailor",
        skill_profs: SkillSet::new().set(Skill::Athletics).set(Skill::Perception),
    },
    // 11 — Soldier
    BackgroundData {
        name: "Soldier",
        skill_profs: SkillSet::new().set(Skill::Athletics).set(Skill::Intimidation),
    },
    // 12 — Urchin
    BackgroundData {
        name: "Urchin",
        skill_profs: SkillSet::new().set(Skill::SleightOfHand).set(Skill::Stealth),
    },
    // ── Xanathar's Guide ─────────────────────────────────────────────────────
    // 13 — City Watch
    BackgroundData {
        name: "City Watch",
        skill_profs: SkillSet::new().set(Skill::Athletics).set(Skill::Insight),
    },
    // 14 — Clan Crafter
    BackgroundData {
        name: "Clan Crafter",
        skill_profs: SkillSet::new().set(Skill::History).set(Skill::Insight),
    },
    // 15 — Cloistered Scholar
    BackgroundData {
        name: "Cloistered Scholar",
        skill_profs: SkillSet::new().set(Skill::History).set(Skill::Arcana),
    },
    // 16 — Courtier
    BackgroundData {
        name: "Courtier",
        skill_profs: SkillSet::new().set(Skill::Insight).set(Skill::Persuasion),
    },
    // 17 — Far Traveler
    BackgroundData {
        name: "Far Traveler",
        skill_profs: SkillSet::new().set(Skill::Insight).set(Skill::Perception),
    },
    // 18 — Mercenary Veteran
    BackgroundData {
        name: "Mercenary Veteran",
        skill_profs: SkillSet::new().set(Skill::Athletics).set(Skill::Persuasion),
    },
    // 19 — Urban Bounty Hunter
    BackgroundData {
        name: "Urban Bounty Hunter",
        skill_profs: SkillSet::new().set(Skill::Deception).set(Skill::Intimidation),
    },
    // 20 — Waterdhavian Noble
    BackgroundData {
        name: "Waterdhavian Noble",
        skill_profs: SkillSet::new().set(Skill::History).set(Skill::Persuasion),
    },
];
