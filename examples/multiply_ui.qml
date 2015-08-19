import QtQuick 2.2
import QtQuick.Controls 1.2
import QtQuick.Layouts 1.0

ApplicationWindow {
    visible: true
    title: "Multiply"

    property int margin: 11
    width: mainLayout.implicitWidth + 2 * margin
    height: mainLayout.implicitHeight + 2 * margin
    minimumWidth: mainLayout.Layout.minimumWidth + 2 * margin
    minimumHeight: mainLayout.Layout.minimumHeight + 2 * margin

    ColumnLayout {
        id: mainLayout
        anchors.fill: parent
        anchors.margins: margin

        RowLayout {
            TextField {
                id: number1Field
                Layout.fillWidth: true

                placeholderText: "Enter number"
                focus: true

                onAccepted: doCalculate()
            }

            Label {
                text: "*"
            }

            TextField {
                id: number2Field
                Layout.fillWidth: true
                placeholderText: "Enter number"

                onAccepted: doCalculate()
            }

            Button {
                text: "Calculate"

                onClicked: doCalculate()
            }
        }

        TextArea {
            id: resultArea
            Layout.fillWidth: true
            Layout.fillHeight: true
        }
    }

    function doCalculate() {
        var num1 = parseInt(number1Field.text);
        var num2 = parseInt(number2Field.text);
        resultArea.text = multiply.calculate(num1, num2);
    }

}
