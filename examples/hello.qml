import QtQuick 2.2
import QtQuick.Controls 1.2
import QtQuick.Layouts 1.0

ApplicationWindow {
  id: root

  title: "QmlRs Hello World"
  visible: true

  width: 300
  height: 300

  property int times: 0

  ColumnLayout {
    anchors.fill: parent

    Label {
      id: t
      Layout.fillWidth: true

      text: if (root.times > 0) { return "Hello, QmlRs! (" + root.times + " times!)" }
            else { return "Hello, QmlRs!" }
    }

    Button {
      text: "Click me!"
      Layout.fillWidth: true

      onClicked: {
        var x = qmlrs.invoke("hello", [1,2,3]);
        console.log("QmlRS call returned " + x);
      }
    }
  }

  function hello(x) {
    times += 1;

    console.log("QML hello was called with " + x);

    return 123;
  }
}
