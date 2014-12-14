import QtQuick 2.0

Item {
  width: 300
  height: 300

  Text {
    id: t

    anchors.centerIn: parent

    text: if (parent.times > 0) { return "Hello, QmlRs! (" + parent.times + " times!)" }
          else { return "Hello, QmlRs! (Click me!)" }
  }

  property int times: 0

  function hello(x) {
    times += 1;

    console.log("QML hello was called with " + x);

    return 123;
  }

  MouseArea {
    anchors.fill: parent

    onClicked: {
      var x = qmlrs.invoke("hello");
      console.log("QmlRS call returned " + x);
    }
  }
}
