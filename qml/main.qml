import QtQuick
import QtQuick.Window
import com.remarkable.calc
import InkTools 1.0

Window {
    id: root
    width: Screen.width
    height: Screen.height
    minimumWidth: 320
    minimumHeight: 540
    visible: true
    color: "white"
    title: "Calculator"

    property bool inkMode: false
    property real lastInkX: 0
    property real lastInkY: 0
    property bool lastInkValid: false

    Calculator {
        id: calc
    }

    PenInput {
        id: penInput
    }

    Rectangle {
        anchors.fill: parent
        color: "white"

        Column {
            anchors.fill: parent
            anchors.margins: 16
            spacing: 16

            Row {
                id: inkControls
                width: parent.width
                height: 44
                spacing: 10

                Rectangle {
                    width: (parent.width - parent.spacing) / 2
                    height: parent.height
                    radius: 8
                    color: "white"
                    border.color: "black"
                    border.width: 1

                    Text {
                        anchors.centerIn: parent
                        color: "black"
                        renderType: Text.NativeRendering
                        antialiasing: false
                        font.pixelSize: 20
                        text: inkMode ? "Ink: On" : "Ink: Off"
                    }

                    MouseArea {
                        anchors.fill: parent
                        onClicked: inkMode = !inkMode
                    }
                }

                Rectangle {
                    width: (parent.width - parent.spacing) / 2
                    height: parent.height
                    radius: 8
                    color: "white"
                    border.color: "black"
                    border.width: 1

                    Text {
                        anchors.centerIn: parent
                        color: "black"
                        renderType: Text.NativeRendering
                        antialiasing: false
                        font.pixelSize: 20
                        text: "Clear Ink"
                    }

                    MouseArea {
                        anchors.fill: parent
                        onClicked: inkCanvas.clear()
                    }
                }
            }

            Rectangle {
                width: parent.width
                height: parent.height * 0.22
                radius: 14
                color: "white"
                border.color: "black"
                border.width: 1

                Text {
                    anchors.fill: parent
                    anchors.margins: 18
                    horizontalAlignment: Text.AlignRight
                    verticalAlignment: Text.AlignVCenter
                    wrapMode: Text.NoWrap
                    elide: Text.ElideLeft
                    renderType: Text.NativeRendering
                    antialiasing: false
                    color: "black"
                    font.pixelSize: Math.max(28, Math.min(48, root.width / 8))
                    text: calc.displayText
                }
            }

            Grid {
                id: keypad
                width: parent.width
                height: parent.height - 44 - (parent.height * 0.22) - (parent.spacing * 2)
                columns: 4
                rows: 5
                columnSpacing: 10
                rowSpacing: 10

                Repeater {
                    model: [
                        { label: "C", type: "action" },
                        { label: "<-", type: "action" },
                        { label: "(", type: "op" },
                        { label: ")", type: "op" },

                        { label: "7", type: "num" },
                        { label: "8", type: "num" },
                        { label: "9", type: "num" },
                        { label: "/", type: "op" },

                        { label: "4", type: "num" },
                        { label: "5", type: "num" },
                        { label: "6", type: "num" },
                        { label: "*", type: "op" },

                        { label: "1", type: "num" },
                        { label: "2", type: "num" },
                        { label: "3", type: "num" },
                        { label: "-", type: "op" },

                        { label: "0", type: "num" },
                        { label: ".", type: "num" },
                        { label: "=", type: "eq" },
                        { label: "+", type: "op" }
                    ]

                    delegate: Rectangle {
                        required property var modelData
                        width: (keypad.width - (keypad.columns - 1) * keypad.columnSpacing) / keypad.columns
                        height: (keypad.height - (keypad.rows - 1) * keypad.rowSpacing) / keypad.rows
                        radius: 12
                        color: "white"
                        border.color: "black"
                        border.width: 1

                        Text {
                            anchors.centerIn: parent
                            color: "black"
                            renderType: Text.NativeRendering
                            antialiasing: false
                            font.pixelSize: 28
                            font.bold: modelData.type !== "num"
                            text: modelData.label
                        }

                        MouseArea {
                            anchors.fill: parent
                            onClicked: {
                                if (modelData.label === "C") {
                                    calc.resetAll();
                                } else if (modelData.label === "<-") {
                                    calc.backspace();
                                } else if (modelData.label === "=") {
                                    calc.evaluate();
                                } else {
                                    calc.appendValue(modelData.label);
                                }
                            }
                        }
                    }
                }
            }
        }

        InkCanvas {
            id: inkCanvas
            objectName: "inkArea"
            anchors.fill: parent
            z: 10
            penWidth: 3
            color: "black"
        }

        Connections {
            target: penInput

            function onPenChanged() {
                if (!inkMode || !penInput.penDown) {
                    lastInkValid = false;
                    return;
                }

                if (!lastInkValid) {
                    // Pen just touched down: start a new stroke here.
                    inkCanvas.moveTo(penInput.penX, penInput.penY);
                    lastInkValid = true;
                    return;
                }

                // Extend the stroke; only a small region around the pen refreshes.
                inkCanvas.lineTo(penInput.penX, penInput.penY);
            }
        }
    }

    Component.onCompleted: {
        penInput.screenWidth = root.width;
        penInput.screenHeight = root.height;
        penInput.open();
    }
}
