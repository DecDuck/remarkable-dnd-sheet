import QtQuick
import QtQuick.Window
import com.remarkable.calc
import InkTools 1.0
import "qrc:/charsheet/fields.js" as SheetFields

Window {
    id: root
    width: Screen.width
    height: Screen.height
    minimumWidth: 320
    minimumHeight: 540
    visible: true
    color: "white"
    title: "D&D Character Sheet"

    // ── Active tab (0=Sheet 1=Character 2=Notes) ─────────────────────────
    property int currentTab: 0
    property bool lastInkValid: false
    property var activeInkCanvas: null

    // PDF sheet background state
    property var sheetFields: []      // loaded from qrc:/charsheet/fields.json
    property bool showFieldHints: false

    // ── D&D 5e helper functions (reactive; depend on sheet.* properties) ──
    //
    // Proficiency bonus derived from level (5e formula: floor((level-1)/4)+2).
    function profBonus() {
        return Math.floor((sheet.level - 1) / 4) + 2
    }
    // Floor-division ability modifier: floor((score-10)/2).
    function abilityMod(score) {
        return Math.floor((score - 10) / 2)
    }
    function formatMod(v) {
        return v >= 0 ? "+" + v : "" + v
    }

    // PHB alphabetical skill list (18 entries).
    // Index maps: 0=STR 1=DEX 2=CON 3=INT 4=WIS 5=CHA
    readonly property var skillNames: [
        "Acrobatics","Animal Handling","Arcana","Athletics",
        "Deception","History","Insight","Intimidation","Investigation",
        "Medicine","Nature","Perception","Performance","Persuasion",
        "Religion","Sleight of Hand","Stealth","Survival"
    ]
    readonly property var skillAbilityIdx: [1,4,3,0,5,3,4,5,3,4,3,4,5,5,3,1,1,4]
    readonly property var abilityAbbrevs: ["STR","DEX","CON","INT","WIS","CHA"]

    function skillBonus(skillIdx) {
        var ab = root.skillAbilityIdx[skillIdx]
        var scores = [sheet.strScore, sheet.dexScore, sheet.conScore,
                      sheet.intScore, sheet.wisScore, sheet.chaScore]
        var mod = root.abilityMod(scores[ab])
        var prof = ((sheet.skillProfs >> skillIdx) & 1) === 1
        return prof ? mod + root.profBonus() : mod
    }
    function saveBonus(ab) {
        var scores = [sheet.strScore, sheet.dexScore, sheet.conScore,
                      sheet.intScore, sheet.wisScore, sheet.chaScore]
        var mod = root.abilityMod(scores[ab])
        var prof = ((sheet.saveProfs >> ab) & 1) === 1
        return prof ? mod + root.profBonus() : mod
    }

    /// Resolve a binding key to a display string.
    /// Called from the sheet field delegate for auto-calculated fields.
    /// All sheet.XXX accesses below create QML property bindings → reactive.
    function resolveSheetBinding(key) {
        if (!key) return ""

        // ── Colon-separated compound keys ──────────────────────────────────
        var colon = key.indexOf(":")
        if (colon !== -1) {
            var prefix = key.substring(0, colon)
            var idx = parseInt(key.substring(colon + 1))
            if (isNaN(idx)) return "?"
            if (prefix === "skillBonus")    return "" + root.skillBonus(idx)
            if (prefix === "skillBonusFmt") return root.formatMod(root.skillBonus(idx))
            if (prefix === "saveBonus")     return "" + root.saveBonus(idx)
            if (prefix === "saveBonusFmt")  return root.formatMod(root.saveBonus(idx))
            if (prefix === "skill") {
                var skillNames2 = {
                    "Acrobatics":0,"AnimalHandling":1,"Arcana":2,"Athletics":3,
                    "Deception":4,"History":5,"Insight":6,"Intimidation":7,
                    "Investigation":8,"Medicine":9,"Nature":10,"Perception":11,
                    "Performance":12,"Persuasion":13,"Religion":14,
                    "SleightOfHand":15,"Stealth":16,"Survival":17
                }
                var si = skillNames2[key.substring(colon + 1)]
                return si !== undefined ? root.formatMod(root.skillBonus(si)) : "?"
            }
            if (prefix === "save") {
                var saveMap = {"STR":0,"DEX":1,"CON":2,"INT":3,"WIS":4,"CHA":5}
                var ai = saveMap[key.substring(colon + 1)]
                return ai !== undefined ? root.formatMod(root.saveBonus(ai)) : "?"
            }
            if (prefix === "saveProf")  return ((sheet.saveProfs  >> idx) & 1) ? "●" : ""
            if (prefix === "skillProf") return ((sheet.skillProfs >> idx) & 1) ? "●" : ""
            if (prefix === "deathSuccess") return sheet.deathSuccess > idx ? "●" : ""
            if (prefix === "deathFail")    return sheet.deathFail    > idx ? "●" : ""
            return "?"
        }

        // ── Direct property keys ────────────────────────────────────────────
        // Touch every property we might reference so QML tracks them all.
        var s = sheet
        switch (key) {
            // Basic
            case "profBonus":          return "" + s.profBonus
            case "level":              return "" + s.level
            case "xp":                 return "" + s.xp
            // Ability scores
            case "strScore":           return "" + s.strScore
            case "dexScore":           return "" + s.dexScore
            case "conScore":           return "" + s.conScore
            case "intScore":           return "" + s.intScore
            case "wisScore":           return "" + s.wisScore
            case "chaScore":           return "" + s.chaScore
            // Ability modifiers (raw number)
            case "strMod":             return "" + s.strMod
            case "dexMod":             return "" + s.dexMod
            case "conMod":             return "" + s.conMod
            case "intMod":             return "" + s.intMod
            case "wisMod":             return "" + s.wisMod
            case "chaMod":             return "" + s.chaMod
            // Ability modifiers (formatted "+3" / "-1")
            case "strModFmt":          return root.formatMod(s.strMod)
            case "dexModFmt":          return root.formatMod(s.dexMod)
            case "conModFmt":          return root.formatMod(s.conMod)
            case "intModFmt":          return root.formatMod(s.intMod)
            case "wisModFmt":          return root.formatMod(s.wisMod)
            case "chaModFmt":          return root.formatMod(s.chaMod)
            // HP
            case "maxHp":              return "" + s.maxHp
            case "currentHp":          return "" + s.currentHp
            case "tempHp":             return s.tempHp > 0 ? "" + s.tempHp : ""
            case "hpDisplay":          return s.currentHp + "/" + s.maxHp + (s.tempHp > 0 ? " +" + s.tempHp : "")
            // Combat
            case "armorClass":         return "" + s.armorClass
            case "speed":              return "" + s.speed
            case "initiative":         return "" + s.initiative
            case "initiativeFmt":      return root.formatMod(s.initiative)
            // Passive
            case "passivePerception":  return "" + s.passivePerception
            case "passiveInvestigation": return "" + s.passiveInvestigation
            // Spellcasting
            case "spellSaveDc":        return s.spellSaveDc > 0 ? "" + s.spellSaveDc : ""
            case "spellAttackBonus":   return s.spellAttackBonus !== 0 ? "" + s.spellAttackBonus : ""
            case "spellAttackBonusFmt": return s.spellSaveDc > 0 ? root.formatMod(s.spellAttackBonus) : ""
            default:                   return "?"
        }
    }

    // ── Data objects ──────────────────────────────────────────────────────
    CharacterSheet  { id: sheet }
    PenInput        { id: penInput }
    ScreenRefresher { id: screenRefresher }

    // Full GC16 refresh whenever the active tab changes to clear e-ink ghosting.
    // contentItem is the root QQuickItem of the Window (Window itself is not a QQuickItem).
    onCurrentTabChanged: screenRefresher.refresh(root.contentItem)

    // ── Main container ────────────────────────────────────────────────────
    Rectangle {
        anchors.fill: parent
        color: "white"

        // ── Tab bar ───────────────────────────────────────────────────────
        Rectangle {
            id: tabBar
            anchors.top: parent.top
            anchors.left: parent.left
            anchors.right: parent.right
            height: 90
            color: "white"

            Row {
                anchors.fill: parent
                Repeater {
                    model: ["Sheet", "Character", "Notes"]
                    delegate: Rectangle {
                        required property int index
                        required property string modelData
                        width: tabBar.width / 3
                        height: tabBar.height
                        color: root.currentTab === index ? "black" : "white"
                        border.color: "black"
                        border.width: 1
                        Text {
                            anchors.centerIn: parent
                            text: modelData
                            color: root.currentTab === index ? "white" : "black"
                            font.pixelSize: 36
                            font.bold: root.currentTab === index
                            renderType: Text.NativeRendering
                            antialiasing: false
                        }
                        MouseArea {
                            anchors.fill: parent
                            onClicked: root.currentTab = index
                        }
                    }
                }
            }
        }

        // ── Content area ──────────────────────────────────────────────────
        Item {
            id: contentArea
            anchors.top: tabBar.bottom
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.bottom: parent.bottom

            // ── Tab 1: Character (level accordion + class builder) ────────────
            Flickable {
                id: charFlickable
                anchors.fill: parent
                visible: root.currentTab === 1
                contentWidth: width
                contentHeight: charCol.implicitHeight + 48
                clip: true
                flickableDirection: Flickable.VerticalFlick

                property int expandedLevel: -1
                property bool raceDropOpen: false

                Column {
                    id: charCol
                    x: 16; y: 16
                    width: parent.width - 32
                    spacing: 0

                    // ── Add Level ───────────────────────────────────────────────────
                    Text { text: "ADD LEVEL"; font.pixelSize: 28; font.bold: true; font.letterSpacing: 3; color: "black"; renderType: Text.NativeRendering; antialiasing: false }
                    Rectangle { width: parent.width; height: 2; color: "black" }
                    Item { width: parent.width; height: 8 }

                    Flow {
                        width: parent.width
                        spacing: 8
                        Repeater {
                            model: sheet.numClasses()
                            delegate: Rectangle {
                                required property int index
                                width: (charCol.width - 8) / 2; height: 64
                                radius: 8; color: "white"; border.color: "black"; border.width: 2
                                Text { anchors.centerIn: parent; text: sheet.className(index); font.pixelSize: 26; color: "black"; renderType: Text.NativeRendering; antialiasing: false }
                                MouseArea { anchors.fill: parent; onClicked: sheet.addLevel(index) }
                            }
                        }
                    }

                    Item { width: parent.width; height: 8 }
                    Rectangle {
                        width: parent.width; height: 64; radius: 8
                        color: "white"; border.color: "black"; border.width: 2
                        Text { anchors.centerIn: parent; text: "Remove Last Level"; font.pixelSize: 28; color: sheet.level > 0 ? "black" : "#aaa"; renderType: Text.NativeRendering; antialiasing: false }
                        MouseArea { anchors.fill: parent; onClicked: { if (sheet.level > 0) sheet.removeLastLevel() } }
                    }

                    Item { width: parent.width; height: 20 }

                    // ── Level accordion ────────────────────────────────────────────
                    Text { text: "LEVELS"; font.pixelSize: 28; font.bold: true; font.letterSpacing: 3; color: "black"; renderType: Text.NativeRendering; antialiasing: false }
                    Rectangle { width: parent.width; height: 2; color: "black" }

                    Repeater {
                        model: sheet.level
                        delegate: Column {
                            required property int index
                            readonly property int charLevel: index + 1
                            readonly property bool isExpanded: charFlickable.expandedLevel === charLevel
                            readonly property int clsIdx: sheet.levelClassIdx(charLevel)
                            readonly property int clsLocal: sheet.levelClassLevel(charLevel)
                            readonly property bool hasAsi: sheet.levelHasAsi(charLevel)
                            readonly property bool hasSubclass: sheet.levelHasSubclassChoice(charLevel)
                            readonly property int numFeatures: sheet.levelNumFeatures(charLevel)
                            readonly property int asiUiMode: {
                                var _v = sheet.selectionVersion
                                var c = sheet.levelAsiChoice(charLevel)
                                if (c === -2) return sheet.levelAsiAbility2(charLevel) >= 0 ? 2 : 1
                                if (c >= 0)  return 3
                                return 0
                            }
                            width: charCol.width

                            // Header row
                            Rectangle {
                                width: parent.width; height: 80
                                color: isExpanded ? "black" : "white"
                                border.color: "black"; border.width: 1
                                Row {
                                    anchors.fill: parent; anchors.leftMargin: 16; anchors.rightMargin: 16; spacing: 0
                                    Text {
                                        anchors.verticalCenter: parent.verticalCenter
                                        text: "Lv " + charLevel
                                        font.pixelSize: 32; font.bold: true; color: isExpanded ? "white" : "black"
                                        width: 100; renderType: Text.NativeRendering; antialiasing: false
                                    }
                                    Text {
                                        anchors.verticalCenter: parent.verticalCenter
                                        text: sheet.levelClassName(charLevel) + " " + clsLocal
                                        font.pixelSize: 30; color: isExpanded ? "white" : "#333"
                                        elide: Text.ElideRight; width: parent.width - 100 - 60
                                        renderType: Text.NativeRendering; antialiasing: false
                                    }
                                    Text {
                                        anchors.verticalCenter: parent.verticalCenter
                                        text: isExpanded ? "\u25b2" : "\u25bc"
                                        font.pixelSize: 28; color: isExpanded ? "white" : "#888"
                                        width: 60; horizontalAlignment: Text.AlignRight
                                        renderType: Text.NativeRendering; antialiasing: false
                                    }
                                }
                                MouseArea { anchors.fill: parent; onClicked: charFlickable.expandedLevel = isExpanded ? -1 : charLevel }
                            }

                            // Expanded body
                            Column {
                                visible: isExpanded
                                width: parent.width
                                spacing: 0

                                // ── Features ──────────────────────────────────────────
                                Column {
                                    width: parent.width
                                    visible: numFeatures > 0
                                    Repeater {
                                        model: numFeatures
                                        delegate: Column {
                                            required property int index
                                            property bool descOpen: false
                                            width: parent.width
                                            Rectangle {
                                                width: parent.width; height: 64; color: "white"
                                                Row {
                                                    anchors.fill: parent; anchors.leftMargin: 24; anchors.rightMargin: 16; spacing: 8
                                                    Text {
                                                        anchors.verticalCenter: parent.verticalCenter
                                                        text: sheet.levelFeatureName(charLevel, index)
                                                        font.pixelSize: 28; font.bold: true; color: "black"
                                                        elide: Text.ElideRight; width: parent.width - 56
                                                        renderType: Text.NativeRendering; antialiasing: false
                                                    }
                                                    Text {
                                                        anchors.verticalCenter: parent.verticalCenter
                                                        text: descOpen ? "\u25b2" : "\u25bc"
                                                        font.pixelSize: 24; color: "#888"
                                                        width: 48; horizontalAlignment: Text.AlignRight
                                                        renderType: Text.NativeRendering; antialiasing: false
                                                    }
                                                }
                                                MouseArea { anchors.fill: parent; onClicked: descOpen = !descOpen }
                                            }
                                            Rectangle {
                                                visible: descOpen; width: parent.width; color: "#f0f0f0"
                                                height: featureDesc.implicitHeight + 24
                                                Text {
                                                    id: featureDesc
                                                    x: 28; y: 12; width: parent.width - 56
                                                    text: sheet.levelFeatureDescription(charLevel, index)
                                                    font.pixelSize: 26; color: "#333"; wrapMode: Text.WordWrap
                                                    renderType: Text.NativeRendering; antialiasing: false
                                                }
                                            }
                                            Rectangle { width: parent.width; height: 1; color: "#ddd" }
                                        }
                                    }
                                }

                                // ── Subclass picker ──────────────────────────────────
                                Column {
                                    width: parent.width
                                    visible: hasSubclass
                                    Rectangle {
                                        width: parent.width; height: 52; color: "#e8e8e8"
                                        Text { x: 20; anchors.verticalCenter: parent.verticalCenter; text: "SUBCLASS"; font.pixelSize: 24; font.bold: true; font.letterSpacing: 2; color: "black"; renderType: Text.NativeRendering; antialiasing: false }
                                    }
                                    Repeater {
                                        model: clsIdx >= 0 ? sheet.numSubclassesForClassIdx(clsIdx) : 0
                                        delegate: Rectangle {
                                            required property int index
                                            readonly property string scName: clsIdx >= 0 ? sheet.subclassNameForClassIdx(clsIdx, index) : ""
                                            readonly property bool isChosen: {
                                                var _v = sheet.selectionVersion
                                                var cur = sheet.levelSubclassName(charLevel)
                                                return cur !== "" && cur === scName
                                            }
                                            width: parent.width; height: 64
                                            color: isChosen ? "black" : "white"; border.color: "black"; border.width: 1
                                            Text { x: 28; anchors.verticalCenter: parent.verticalCenter; text: scName; font.pixelSize: 28; color: isChosen ? "white" : "black"; renderType: Text.NativeRendering; antialiasing: false }
                                            MouseArea { anchors.fill: parent; onClicked: sheet.setLevelSubclass(charLevel, index) }
                                        }
                                    }
                                }

                                // ── ASI / Feat picker ────────────────────────────────
                                Column {
                                    width: parent.width
                                    visible: hasAsi
                                    Rectangle {
                                        width: parent.width; height: 52; color: "#e8e8e8"
                                        Text { x: 20; anchors.verticalCenter: parent.verticalCenter; text: "ABILITY SCORE / FEAT"; font.pixelSize: 24; font.bold: true; font.letterSpacing: 2; color: "black"; renderType: Text.NativeRendering; antialiasing: false }
                                    }
                                    // Mode selector row
                                    Row {
                                        width: parent.width; height: 68; spacing: 0
                                        Repeater {
                                            model: [{ lbl: "Ability +2", mode: 1 }, { lbl: "+1 / +1", mode: 2 }, { lbl: "Feat", mode: 3 }]
                                            delegate: Rectangle {
                                                required property int index
                                                required property var modelData
                                                width: parent.width / 3; height: parent.height
                                                color: asiUiMode === modelData.mode ? "black" : "white"
                                                border.color: "black"; border.width: 1
                                                Text { anchors.centerIn: parent; text: modelData.lbl; font.pixelSize: 26; color: asiUiMode === modelData.mode ? "white" : "black"; renderType: Text.NativeRendering; antialiasing: false }
                                                MouseArea {
                                                    anchors.fill: parent
                                                    onClicked: {
                                                        if (asiUiMode === modelData.mode) {
                                                            sheet.clearLevelChoice(charLevel)
                                                        } else {
                                                            sheet.clearLevelChoice(charLevel)
                                                            if (modelData.mode === 1) sheet.setLevelAsi(charLevel, 0, -1)
                                                            else if (modelData.mode === 2) sheet.setLevelAsi(charLevel, 0, 1)
                                                            // mode 3: user picks from list below
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    // Ability +2 picker
                                    Column {
                                        width: parent.width
                                        visible: asiUiMode === 1
                                        Text { x: 20; height: 44; verticalAlignment: Text.AlignVCenter; text: "+2 to one ability:"; font.pixelSize: 26; color: "#555"; renderType: Text.NativeRendering; antialiasing: false }
                                        Row {
                                            x: 12; width: parent.width - 24; height: 68; spacing: 8
                                            Repeater {
                                                model: 6
                                                delegate: Rectangle {
                                                    required property int index
                                                    readonly property bool chosen: sheet.selectionVersion >= 0 && sheet.levelAsiAbility1(charLevel) === index && sheet.levelAsiAbility2(charLevel) < 0
                                                    width: (parent.width - 5 * 8) / 6; height: 60; radius: 6
                                                    color: chosen ? "black" : "white"; border.color: "black"; border.width: 2
                                                    Text { anchors.centerIn: parent; text: ["STR","DEX","CON","INT","WIS","CHA"][index]; font.pixelSize: 22; font.bold: chosen; color: chosen ? "white" : "black"; renderType: Text.NativeRendering; antialiasing: false }
                                                    MouseArea { anchors.fill: parent; onClicked: sheet.setLevelAsi(charLevel, index, -1) }
                                                }
                                            }
                                        }
                                    }
                                    // Ability +1/+1 picker
                                    Column {
                                        width: parent.width
                                        visible: asiUiMode === 2
                                        Text { x: 20; height: 44; verticalAlignment: Text.AlignVCenter; text: "First ability (+1):"; font.pixelSize: 26; color: "#555"; renderType: Text.NativeRendering; antialiasing: false }
                                        Row {
                                            x: 12; width: parent.width - 24; height: 68; spacing: 8
                                            Repeater {
                                                model: 6
                                                delegate: Rectangle {
                                                    required property int index
                                                    readonly property bool chosen: sheet.selectionVersion >= 0 && sheet.levelAsiAbility1(charLevel) === index
                                                    width: (parent.width - 5 * 8) / 6; height: 60; radius: 6
                                                    color: chosen ? "black" : "white"; border.color: "black"; border.width: 2
                                                    Text { anchors.centerIn: parent; text: ["STR","DEX","CON","INT","WIS","CHA"][index]; font.pixelSize: 22; font.bold: chosen; color: chosen ? "white" : "black"; renderType: Text.NativeRendering; antialiasing: false }
                                                    MouseArea {
                                                        anchors.fill: parent
                                                        onClicked: {
                                                            var ab2 = sheet.levelAsiAbility2(charLevel)
                                                            sheet.setLevelAsi(charLevel, index, ab2 >= 0 ? ab2 : 1)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Text { x: 20; height: 44; verticalAlignment: Text.AlignVCenter; text: "Second ability (+1):"; font.pixelSize: 26; color: "#555"; renderType: Text.NativeRendering; antialiasing: false }
                                        Row {
                                            x: 12; width: parent.width - 24; height: 68; spacing: 8
                                            Repeater {
                                                model: 6
                                                delegate: Rectangle {
                                                    required property int index
                                                    readonly property bool chosen: sheet.selectionVersion >= 0 && sheet.levelAsiAbility2(charLevel) === index
                                                    width: (parent.width - 5 * 8) / 6; height: 60; radius: 6
                                                    color: chosen ? "black" : "white"; border.color: "black"; border.width: 2
                                                    Text { anchors.centerIn: parent; text: ["STR","DEX","CON","INT","WIS","CHA"][index]; font.pixelSize: 22; font.bold: chosen; color: chosen ? "white" : "black"; renderType: Text.NativeRendering; antialiasing: false }
                                                    MouseArea {
                                                        anchors.fill: parent
                                                        onClicked: {
                                                            var ab1 = sheet.levelAsiAbility1(charLevel)
                                                            sheet.setLevelAsi(charLevel, ab1 >= 0 ? ab1 : 0, index)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    // Feat picker
                                    Column {
                                        width: parent.width
                                        visible: asiUiMode === 3
                                        Column {
                                            id: featPickerCol
                                            width: parent.width
                                            Repeater {
                                                model: sheet.numFeats()
                                                delegate: Rectangle {
                                                    required property int index
                                                    readonly property bool chosen: sheet.selectionVersion >= 0 && sheet.levelAsiChoice(charLevel) === index
                                                    width: parent.width; height: 60
                                                    color: chosen ? "black" : "white"; border.color: "#ccc"; border.width: 1
                                                    Text { x: 20; anchors.verticalCenter: parent.verticalCenter; text: sheet.featName(index); font.pixelSize: 26; color: chosen ? "white" : "black"; renderType: Text.NativeRendering; antialiasing: false }
                                                    MouseArea { anchors.fill: parent; onClicked: sheet.setLevelFeat(charLevel, index) }
                                                }
                                            }
                                        }
                                    }
                                }

                                // ── Class choice-feature pickers ──────────────────────
                                // Shown for each choice set whose class_level == this class-local level
                                Repeater {
                                    model: sheet.levelNumChoiceSets(charLevel)
                                    delegate: Column {
                                        required property int index
                                        readonly property int setIdx: index
                                        width: charCol.width

                                        Repeater {
                                            model: sheet.levelNumChoiceFeatures(charLevel, setIdx)
                                            delegate: Column {
                                                required property int index
                                                readonly property int featIdx: index
                                                readonly property int numPicks: sheet.levelChoiceFeatureNumPicks(charLevel, setIdx, featIdx)
                                                readonly property int numOpts: sheet.levelChoiceFeatureNumOptions(charLevel, setIdx, featIdx)
                                                width: charCol.width

                                                // Section header
                                                Rectangle {
                                                    width: parent.width; height: 52; color: "#ddeeff"
                                                    Column {
                                                        anchors.left: parent.left; anchors.verticalCenter: parent.verticalCenter; anchors.leftMargin: 16
                                                        Text { text: sheet.levelChoiceFeatureName(charLevel, setIdx, featIdx); font.pixelSize: 26; font.bold: true; color: "#002244"; renderType: Text.NativeRendering; antialiasing: false }
                                                        Text { text: "Choose " + numPicks + " option" + (numPicks > 1 ? "s" : ""); font.pixelSize: 22; color: "#446688"; renderType: Text.NativeRendering; antialiasing: false }
                                                    }
                                                }

                                                // Options list
                                                Column {
                                                    width: parent.width
                                                    Column {
                                                        id: optPickCol
                                                        width: parent.width
                                                        Repeater {
                                                            model: numOpts
                                                            delegate: Rectangle {
                                                                required property int index
                                                                readonly property int optIdx: index
                                                                readonly property bool chosen: sheet.selectionVersion >= 0 && sheet.levelChoiceOptionSelected(charLevel, setIdx, featIdx, optIdx)
                                                                property bool descOpen: false
                                                                width: parent.width
                                                                height: chosen && descOpen ? optNameRow.height + optDescBox.height : optNameRow.height
                                                                color: chosen ? "#001840" : "white"
                                                                border.color: "#aac"; border.width: 1

                                                                Column {
                                                                    width: parent.width
                                                                    Item {
                                                                        id: optNameRow
                                                                        width: parent.width; height: 60
                                                                        Row {
                                                                            anchors.fill: parent; anchors.leftMargin: 20; anchors.rightMargin: 16; spacing: 8
                                                                            Rectangle {
                                                                                width: 28; height: 28; radius: 14; anchors.verticalCenter: parent.verticalCenter
                                                                                color: chosen ? "white" : "#e0e8ff"; border.color: chosen ? "white" : "#557"
                                                                            }
                                                                            Text {
                                                                                anchors.verticalCenter: parent.verticalCenter
                                                                                text: sheet.levelChoiceOptionName(charLevel, setIdx, featIdx, optIdx)
                                                                                font.pixelSize: 26; color: chosen ? "white" : "black"; elide: Text.ElideRight
                                                                                width: parent.width - 44 - 40
                                                                                renderType: Text.NativeRendering; antialiasing: false
                                                                            }
                                                                            Text {
                                                                                anchors.verticalCenter: parent.verticalCenter
                                                                                text: descOpen ? "\u25b2" : "\u25bc"; font.pixelSize: 22
                                                                                color: chosen ? "#cde" : "#888"; width: 40; horizontalAlignment: Text.AlignRight
                                                                                renderType: Text.NativeRendering; antialiasing: false
                                                                            }
                                                                        }
                                                                        MouseArea {
                                                                            anchors.fill: parent
                                                                            onClicked: {
                                                                                sheet.levelChoiceOptionToggle(charLevel, setIdx, featIdx, optIdx)
                                                                                descOpen = !descOpen
                                                                            }
                                                                        }
                                                                    }
                                                                    Rectangle {
                                                                        id: optDescBox
                                                                        visible: descOpen; width: parent.width; color: chosen ? "#002255" : "#f0f4ff"
                                                                        height: descOpen ? optDescText.implicitHeight + 20 : 0
                                                                        Text {
                                                                            id: optDescText
                                                                            x: 20; y: 10; width: parent.width - 40
                                                                            text: sheet.levelChoiceOptionDesc(charLevel, setIdx, featIdx, optIdx)
                                                                            font.pixelSize: 24; color: chosen ? "#cde" : "#334"; wrapMode: Text.WordWrap
                                                                            renderType: Text.NativeRendering; antialiasing: false
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                Rectangle { width: parent.width; height: 3; color: "#888" }
                            }
                        }
                    }

                    // ── Race Traits ────────────────────────────────────────────────────
                    Item { width: parent.width; height: 20 }
                    Text { text: "RACE TRAITS"; font.pixelSize: 28; font.bold: true; font.letterSpacing: 3; color: "black"; renderType: Text.NativeRendering; antialiasing: false }
                    Rectangle { width: parent.width; height: 2; color: "black" }

                    // Compact dropdown row ──────────────────────────────────────────
                    Rectangle {
                        width: parent.width; height: 80
                        radius: 8; color: "white"; border.color: "black"; border.width: 2
                        Row {
                            anchors.fill: parent; anchors.leftMargin: 20; anchors.rightMargin: 16; spacing: 0
                            Text {
                                anchors.verticalCenter: parent.verticalCenter
                                text: "Race: "
                                font.pixelSize: 30; color: "#555"; renderType: Text.NativeRendering; antialiasing: false
                            }
                            Text {
                                anchors.verticalCenter: parent.verticalCenter
                                text: sheet.raceIdx >= 0 ? sheet.raceName(sheet.raceIdx) : "None"
                                font.pixelSize: 30; font.bold: true; color: "black"
                                elide: Text.ElideRight; width: parent.width - 100
                                renderType: Text.NativeRendering; antialiasing: false
                            }
                            Text {
                                anchors.verticalCenter: parent.verticalCenter
                                text: charFlickable.raceDropOpen ? "\u25b2" : "\u25bc"
                                font.pixelSize: 28; color: "#888"; width: 48; horizontalAlignment: Text.AlignRight
                                renderType: Text.NativeRendering; antialiasing: false
                            }
                        }
                        MouseArea { anchors.fill: parent; onClicked: charFlickable.raceDropOpen = !charFlickable.raceDropOpen }
                    }

                    // Inline race list (shown when dropdown is open) ────────────────
                    Column {
                        visible: charFlickable.raceDropOpen
                        width: parent.width
                        Repeater {
                            model: sheet.numRaces()
                            delegate: Rectangle {
                                required property int index
                                readonly property bool chosen: sheet.raceIdx === index
                                width: parent.width; height: 64
                                color: chosen ? "black" : "white"; border.color: "#ccc"; border.width: 1
                                Text { x: 24; anchors.verticalCenter: parent.verticalCenter; text: sheet.raceName(index); font.pixelSize: 28; color: chosen ? "white" : "black"; renderType: Text.NativeRendering; antialiasing: false }
                                MouseArea { anchors.fill: parent; onClicked: { sheet.selectRace(chosen ? -1 : index); charFlickable.raceDropOpen = false } }
                            }
                        }
                    }

                    // Trait list for the selected race ─────────────────────────────
                    Repeater {
                        model: sheet.selectionVersion >= 0 ? sheet.numRaceTraits() : 0
                        delegate: Column {
                            id: traitDelegate
                            required property int index
                            property int traitIndex: index   // capture before shadowing
                            property bool expanded: false
                            width: charCol.width

                            // Trait header
                            Rectangle {
                                width: parent.width; height: 64; color: "white"
                                border.color: "#ccc"; border.width: 1
                                Row {
                                    anchors.fill: parent; anchors.leftMargin: 16; anchors.rightMargin: 16; spacing: 0
                                    Text {
                                        anchors.verticalCenter: parent.verticalCenter
                                        text: sheet.raceTraitName(traitDelegate.traitIndex)
                                        font.pixelSize: 28; font.bold: sheet.raceTraitNumPicks(traitDelegate.traitIndex) > 0; color: "black"
                                        elide: Text.ElideRight; width: parent.width - 56
                                        renderType: Text.NativeRendering; antialiasing: false
                                    }
                                    Text {
                                        anchors.verticalCenter: parent.verticalCenter
                                        text: traitDelegate.expanded ? "\u25b2" : "\u25bc"
                                        font.pixelSize: 24; color: "#888"
                                        width: 48; horizontalAlignment: Text.AlignRight
                                        renderType: Text.NativeRendering; antialiasing: false
                                    }
                                }
                                MouseArea { anchors.fill: parent; onClicked: traitDelegate.expanded = !traitDelegate.expanded }
                            }

                            // Expanded body
                            Column {
                                visible: traitDelegate.expanded
                                width: parent.width

                                // Description
                                Rectangle {
                                    width: parent.width; height: tdesc.implicitHeight + 24; color: "#f4f4f4"
                                    Text {
                                        id: tdesc
                                        x: 20; y: 12; width: parent.width - 40
                                        text: sheet.raceTraitDesc(traitDelegate.traitIndex)
                                        font.pixelSize: 26; color: "#333"; wrapMode: Text.WordWrap
                                        renderType: Text.NativeRendering; antialiasing: false
                                    }
                                }

                                // Option picker — only when num_picks > 0
                                // traitIndex is stored as a property on this Column so that
                                // the inner Repeater delegate can access it via parent.traitIndex
                                // (the visual parent of a Repeater's delegates is always the
                                //  Repeater's parent item, i.e. this Column).
                                Column {
                                    property int traitIndex: traitDelegate.traitIndex
                                    width: parent.width
                                    visible: sheet.raceTraitNumPicks(traitIndex) > 0
                                    Rectangle {
                                        width: parent.width; height: 48; color: "#e0e8ff"
                                        Text { x: 16; anchors.verticalCenter: parent.verticalCenter; text: "Choose " + sheet.raceTraitNumPicks(parent.traitIndex) + " option" + (sheet.raceTraitNumPicks(parent.traitIndex) > 1 ? "s" : "") + ":"; font.pixelSize: 24; font.bold: true; color: "#224"; renderType: Text.NativeRendering; antialiasing: false }
                                    }
                                    Repeater {
                                        model: sheet.raceTraitNumOptions(parent.traitIndex)
                                        delegate: Rectangle {
                                            required property int index
                                            readonly property bool chosen: sheet.selectionVersion >= 0 && sheet.raceTraitOptionSelected(parent.traitIndex, index)
                                            width: parent.width; height: 64
                                            color: chosen ? "black" : "white"; border.color: "#ccc"; border.width: 1
                                            Text { x: 24; anchors.verticalCenter: parent.verticalCenter; text: sheet.raceTraitOptionName(parent.traitIndex, index); font.pixelSize: 26; font.bold: true; color: chosen ? "white" : "black"; renderType: Text.NativeRendering; antialiasing: false }
                                            MouseArea { anchors.fill: parent; onClicked: sheet.raceTraitOptionToggle(parent.traitIndex, index) }
                                        }
                                    }
                                }
                            }
                            Rectangle { width: parent.width; height: 1; color: "#ddd" }
                        }
                    }
                } // Column charCol
            } // Flickable Character

            // ── Tab 2: Notes (full-screen ink canvas) ──────────────────────────
            Item {
                anchors.fill: parent
                visible: root.currentTab === 2
                InkCanvas {
                    id: notesCanvas
                    anchors.fill: parent
                    penWidth: 2; color: "black"
                }
                Rectangle {
                    anchors.right: parent.right; anchors.bottom: parent.bottom; anchors.margins: 24
                    width: 160; height: 66; radius: 10
                    color: "white"; border.color: "black"; border.width: 2
                    Text { anchors.centerIn: parent; text: "Clear"; font.pixelSize: 32; font.bold: true; color: "black"; renderType: Text.NativeRendering; antialiasing: false }
                    MouseArea { anchors.fill: parent; onClicked: notesCanvas.clear() }
                }
            } // Item Notes


            // ── Tab 0: Sheet (PDF background + full-page ink overlay) ─────────────
            Item {
                id: sheetTab
                anchors.fill: parent
                visible: root.currentTab === 0

                // PDF background rendered to PNG by tools/extract_fields.py
                Image {
                    id: sheetBg
                    anchors.fill: parent
                    source: "qrc:/charsheet/bg.png"
                    fillMode: Image.PreserveAspectFit
                    asynchronous: true
                    smooth: true
                }

                // Placeholder shown until the PNG is loaded or if missing
                Rectangle {
                    anchors.fill: parent
                    color: "#f8f8f8"
                    visible: sheetBg.status !== Image.Ready
                    Text {
                        anchors.centerIn: parent
                        width: parent.width * 0.8
                        wrapMode: Text.WordWrap
                        horizontalAlignment: Text.AlignHCenter
                        text: "No sheet loaded\n\nPlace sheet.pdf in the project root and rebuild:\n  cargo build --release"
                        font.pixelSize: 28; color: "#888"
                        renderType: Text.NativeRendering; antialiasing: false
                    }
                }

                // Ink overlay — positioned to match the painted image area.
                Item {
                    id: sheetOverlay
                    x: sheetBg.paintedWidth  > 0 ? (parent.width  - sheetBg.paintedWidth)  / 2 : 0
                    y: sheetBg.paintedHeight > 0 ? (parent.height - sheetBg.paintedHeight) / 2 : 0
                    width:  sheetBg.paintedWidth  > 0 ? sheetBg.paintedWidth  : parent.width
                    height: sheetBg.paintedHeight > 0 ? sheetBg.paintedHeight : parent.height
                    z: 1

                    // No fields loaded: single full-page canvas (freeform).
                    // Shown only when sheetFields is empty so the sheet is still
                    // usable as a blank notes surface before a PDF is provided.
                    InkCanvas {
                        id: sheetFullCanvas
                        anchors.fill: parent
                        visible: root.sheetFields.length === 0
                        penWidth: 2; color: "black"
                    }

                    // Fields loaded: one Item per PDF AcroForm field.
                    // Fields with a binding show a computed Text value.
                    // Fields without a binding show an InkCanvas for handwriting.
                    // The pen routing below uses the `fieldCanvas` alias.
                    Repeater {
                        id: fieldRepeater
                        model: root.sheetFields
                        delegate: Item {
                            id: fieldDelegate
                            required property var modelData

                            // Resolve the binding key from the embedded JS map.
                            readonly property string bindingKey:
                                (SheetFields.BINDINGS && SheetFields.BINDINGS[modelData.id])
                                    ? SheetFields.BINDINGS[modelData.id] : ""
                            readonly property bool hasBinding: bindingKey !== ""

                            // Expose the InkCanvas child to pen routing code.
                            // Null when this field is auto-calculated.
                            property var fieldCanvas: hasBinding ? null : inkField

                            x:      modelData.x * sheetOverlay.width
                            y:      modelData.y * sheetOverlay.height
                            width:  modelData.w * sheetOverlay.width
                            height: modelData.h * sheetOverlay.height

                            // Handwriting canvas — shown when no binding
                            InkCanvas {
                                id: inkField
                                anchors.fill: parent
                                visible: !fieldDelegate.hasBinding
                                penWidth: fieldDelegate.modelData.type === "checkbox" ? 1 : 2
                                color: "black"
                            }

                            // Auto-calculated value — shown when binding present
                            Text {
                                anchors.centerIn: parent
                                width: parent.width - 4
                                visible: fieldDelegate.hasBinding
                                text: fieldDelegate.hasBinding
                                      ? root.resolveSheetBinding(fieldDelegate.bindingKey) : ""
                                font.pixelSize: Math.min(Math.max(parent.height * 0.65, 12), 40)
                                font.bold: true
                                color: "black"
                                horizontalAlignment: Text.AlignHCenter
                                elide: Text.ElideRight
                                renderType: Text.NativeRendering
                                antialiasing: false
                            }
                        }
                    }
                }

                // ── HP quick-adjust buttons (overlaid on the sheet) ─────────────
                // Positioned relative to the HPCurrent and HPTemp PDF fields.
                // Two rows of ±1/±5/±10 buttons for current HP, ±1 for temp HP.
                Item {
                    id: hpOverlay
                    anchors.fill: parent
                    z: 3

                    // Current HP buttons — top of the HPCurrent field
                    Row {
                        x: 0.3777 * sheetOverlay.width
                        y: (0.2683 - 0.005) * sheetOverlay.height
                        width:  0.2435 * sheetOverlay.width
                        height: 0.013  * sheetOverlay.height
                        spacing: width * 0.02

                        Repeater {
                            model: [
                                { lbl: "\u22125", delta: -5 }, { lbl: "\u22121", delta: -1 },
                                { lbl: "+1", delta: 1 },       { lbl: "+5", delta: 5 }
                            ]
                            delegate: Rectangle {
                                required property var modelData
                                width:  (parent.width - 3 * parent.spacing) / 4
                                height: parent.height
                                radius: 3
                                color: "white"
                                border.color: "black"; border.width: 1
                                Text {
                                    anchors.centerIn: parent
                                    text: modelData.lbl
                                    font.pixelSize: Math.max(7, parent.height * 0.55)
                                    font.bold: true; color: "black"
                                    renderType: Text.NativeRendering; antialiasing: false
                                }
                                MouseArea { anchors.fill: parent; onClicked: sheet.adjustHp(modelData.delta) }
                            }
                        }
                    }

                    // Temp HP buttons — top of the HPTemp field
                    Row {
                        x: 0.3777 * sheetOverlay.width
                        y: (0.3338 - 0.013) * sheetOverlay.height
                        width:  0.2435 * sheetOverlay.width
                        height: 0.013  * sheetOverlay.height
                        spacing: width * 0.04

                        Repeater {
                            model: [{ lbl: "\u22121", delta: -1 }, { lbl: "+1", delta: 1 }]
                            delegate: Rectangle {
                                required property var modelData
                                width:  (parent.width - parent.spacing) / 2
                                height: parent.height
                                radius: 3
                                color: "white"
                                border.color: "black"; border.width: 1
                                Text {
                                    anchors.centerIn: parent
                                    text: modelData.lbl
                                    font.pixelSize: Math.max(7, parent.height * 0.55)
                                    font.bold: true; color: "black"
                                    renderType: Text.NativeRendering; antialiasing: false
                                }
                                MouseArea { anchors.fill: parent; onClicked: sheet.adjustTempHp(modelData.delta) }
                            }
                        }
                    }
                }

                // Optional field-hint rectangles (blue outlines over each PDF field)
                Repeater {
                    model: root.sheetFields
                    delegate: Rectangle {
                        required property var modelData
                        x: sheetOverlay.x + modelData.x * sheetOverlay.width
                        y: sheetOverlay.y + modelData.y * sheetOverlay.height
                        width:  modelData.w * sheetOverlay.width
                        height: modelData.h * sheetOverlay.height
                        z: 2
                        color: "transparent"
                        border.color: "#2255cc"
                        border.width: 2
                        opacity: 0.65
                        visible: root.showFieldHints
                        Text {
                            anchors.left: parent.left; anchors.top: parent.top; anchors.margins: 2
                            text: modelData.label
                            font.pixelSize: Math.max(10, Math.min(18, parent.height * 0.55))
                            color: "#2255cc"
                            renderType: Text.NativeRendering; antialiasing: false
                            visible: parent.visible
                        }
                    }
                }

                // Bottom-right controls
                Row {
                    anchors.right: parent.right
                    anchors.bottom: parent.bottom
                    anchors.margins: 24
                    spacing: 16; z: 3

                    Rectangle {
                        width: 170; height: 66; radius: 10
                        color: root.showFieldHints ? "black" : "white"
                        border.color: "black"; border.width: 2
                        Text { anchors.centerIn: parent; text: "Fields"; font.pixelSize: 32; font.bold: root.showFieldHints; color: root.showFieldHints ? "white" : "black"; renderType: Text.NativeRendering; antialiasing: false }
                        MouseArea { anchors.fill: parent; onClicked: root.showFieldHints = !root.showFieldHints }
                    }

                    Rectangle {
                        width: 160; height: 66; radius: 10; color: "white"; border.color: "black"; border.width: 2
                        Text { anchors.centerIn: parent; text: "Clear"; font.pixelSize: 32; font.bold: true; color: "black"; renderType: Text.NativeRendering; antialiasing: false }
                        MouseArea {
                            anchors.fill: parent
                            onClicked: {
                                if (root.sheetFields.length === 0) {
                                    sheetFullCanvas.clear()
                                } else {
                                    for (var i = 0; i < fieldRepeater.count; i++) {
                                        var fc = fieldRepeater.itemAt(i)
                                        if (fc && fc.fieldCanvas) fc.fieldCanvas.clear()
                                    }
                                }
                            }
                        }
                    }
                }
            }

        } // Item contentArea
    } // Rectangle main

    // ── Pen input → ink ───────────────────────────────────────────────────
    // penInput.penX/Y are screen-relative. mapFromItem(null, x, y) converts
    // them into the target InkCanvas's local space (handles all offsets).
    //
    // Eraser (back of marker, BTN_TOOL_RUBBER):
    //   - Erases on every penChanged while eraserDown is true.
    //   - Works across all canvases visible on the current tab.
    //   - Radius is fixed at 24 px (in canvas-local space).
    readonly property real eraserRadius: 24

    Connections {
        target: penInput
        function onPenChanged() {
            // ── Eraser path ───────────────────────────────────────────────
            if (penInput.eraserDown) {
                root.activeInkCanvas = null
                root.lastInkValid = false

                // Erase from every canvas on the current tab that is under
                // the pen tip.  We iterate all candidates rather than stopping
                // at the first hit so overlapping canvases are all erased.
                var eraseTargets = []
                if (root.currentTab === 0) {
                    if (root.sheetFields.length === 0) {
                        eraseTargets = [sheetFullCanvas]
                    } else {
                        for (var ei = 0; ei < fieldRepeater.count; ei++) {
                            var ec = fieldRepeater.itemAt(ei)
                            // Only erase fields that have an InkCanvas (skip auto-filled)
                            if (ec && ec.fieldCanvas) eraseTargets.push(ec.fieldCanvas)
                        }
                    }
                } else if (root.currentTab === 2) {
                    eraseTargets = [notesCanvas]
                }

                for (var ti = 0; ti < eraseTargets.length; ti++) {
                    var tc = eraseTargets[ti]
                    if (!tc) continue
                    var ep = tc.mapFromItem(null, penInput.penX, penInput.penY)
                    tc.eraseAt(ep.x, ep.y, root.eraserRadius)
                }
                return
            }

            // ── Draw path ─────────────────────────────────────────────────
            if (!penInput.penDown) {
                root.activeInkCanvas = null
                root.lastInkValid = false
                return
            }

            var canvas = null
            if (root.currentTab === 0) {
                if (root.activeInkCanvas === null) {
                    if (root.sheetFields.length === 0) {
                        root.activeInkCanvas = sheetFullCanvas
                    } else {
                        for (var fi = 0; fi < fieldRepeater.count; fi++) {
                            var fc = fieldRepeater.itemAt(fi)
                            // Skip auto-filled fields — no ink drawing there
                            if (!fc || !fc.fieldCanvas) continue
                            var flp = fc.mapFromItem(null, penInput.penX, penInput.penY)
                            if (flp.x >= 0 && flp.x < fc.width && flp.y >= 0 && flp.y < fc.height) {
                                root.activeInkCanvas = fc.fieldCanvas
                                break
                            }
                        }
                    }
                }
                canvas = root.activeInkCanvas
            } else if (root.currentTab === 2) {
                root.activeInkCanvas = notesCanvas
                canvas = notesCanvas
            }

            if (canvas === null) return
            var pt = canvas.mapFromItem(null, penInput.penX, penInput.penY)
            if (!root.lastInkValid) {
                canvas.moveTo(pt.x, pt.y)
                root.lastInkValid = true
            } else {
                canvas.lineTo(pt.x, pt.y)
            }
        }
    }

    Component.onCompleted: {
        penInput.screenWidth = root.width
        penInput.screenHeight = root.height
        penInput.open()

        // Fields are embedded directly in the binary via build.rs → rcc.
        // The JS module import above is resolved at load time; no XHR needed.
        if (Array.isArray(SheetFields.FIELDS) && SheetFields.FIELDS.length > 0) {
            root.sheetFields = SheetFields.FIELDS
        }
    }
}
