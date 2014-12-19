import QtQuick 2.2

Item {
  Timer {
    interval: 0
    running: true
    onTriggered: Qt.quit();
  }
}
