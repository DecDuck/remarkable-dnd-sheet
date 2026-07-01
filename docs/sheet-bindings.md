# Sheet Bindings Reference

`sheet_bindings.json` maps PDF AcroForm field IDs → binding keys.
Place the file in the project root, then `cargo build --release` and deploy.

A binding key of `null` means the field is left as a handwriting canvas.

## Binding Key Reference

### Basic Stats
| Key | Description |
|---|---|
| `profBonus` | Proficiency bonus |
| `level` | Character level |
| `xp` | Experience points |

### Ability Scores
| Key | Description |
|---|---|
| `strScore` … `chaScore` | Raw score (e.g. `15`) |
| `strMod` … `chaMod` | Modifier as integer (e.g. `3`) |
| `strModFmt` … `chaModFmt` | Formatted modifier (e.g. `+3`, `-1`) |

### Hit Points
| Key | Description |
|---|---|
| `maxHp` | Maximum HP |
| `currentHp` | Current HP |
| `tempHp` | Temporary HP (empty string when 0) |
| `hpDisplay` | Combined: `"15/20 +3 temp"` |

### Combat
| Key | Description |
|---|---|
| `armorClass` | Armor class |
| `speed` | Speed in feet |
| `initiative` | Initiative modifier (integer) |
| `initiativeFmt` | Formatted: `+2`, `-1` |
| `passivePerception` | 10 + WIS mod + proficiency if trained |
| `passiveInvestigation` | 10 + INT mod + proficiency if trained |

### Spellcasting
| Key | Description |
|---|---|
| `spellSaveDc` | Spell save DC (blank if non-caster) |
| `spellAttackBonus` | Spell attack integer |
| `spellAttackBonusFmt` | Formatted spell attack bonus |

### Skill Bonuses
All 18 skills in PHB alphabetical order (index 0–17):

| Index | Skill | Named shortcut |
|---|---|---|
| 0 | Acrobatics | `skill:Acrobatics` |
| 1 | Animal Handling | `skill:AnimalHandling` |
| 2 | Arcana | `skill:Arcana` |
| 3 | Athletics | `skill:Athletics` |
| 4 | Deception | `skill:Deception` |
| 5 | History | `skill:History` |
| 6 | Insight | `skill:Insight` |
| 7 | Intimidation | `skill:Intimidation` |
| 8 | Investigation | `skill:Investigation` |
| 9 | Medicine | `skill:Medicine` |
| 10 | Nature | `skill:Nature` |
| 11 | Perception | `skill:Perception` |
| 12 | Performance | `skill:Performance` |
| 13 | Persuasion | `skill:Persuasion` |
| 14 | Religion | `skill:Religion` |
| 15 | Sleight of Hand | `skill:SleightOfHand` |
| 16 | Stealth | `skill:Stealth` |
| 17 | Survival | `skill:Survival` |

Use `skillBonusFmt:N` for the formatted version (e.g. `"+5"`), `skillBonus:N` for the integer.
Named shortcuts always return the formatted version.

### Saving Throw Bonuses
| Index | Save | Named shortcut |
|---|---|---|
| 0 | STR | `save:STR` |
| 1 | DEX | `save:DEX` |
| 2 | CON | `save:CON` |
| 3 | INT | `save:INT` |
| 4 | WIS | `save:WIS` |
| 5 | CHA | `save:CHA` |

Use `saveBonusFmt:N` for formatted, `saveBonus:N` for integer.

---

## Generating / Updating the Template

```bash
# List all field IDs from the extracted sheet:
python3 tools/dump_fields.py

# Write a ready-to-edit sheet_bindings.json to the project root:
python3 tools/dump_fields.py --write-template

# Point at a specific fields JSON:
python3 tools/dump_fields.py path/to/charsheet_fields.json --write-template

# Rebuild after editing:
cargo build --release
scp target/release/hello_remarkable root@10.61.28.49:/home/root/
```

The script auto-guesses bindings for the standard WotC D&D 5e fillable PDF.
Fields left as `null` remain handwriting canvases.
