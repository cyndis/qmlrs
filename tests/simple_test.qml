import QtQuick 2.2

Item {
  Timer {
    // 0ms (firing instantly) confuses travis CI, therefore 50ms
    interval: 50
    running: true
    onTriggered: {
      console.debug("Test timer triggered!");
      Qt.quit();
    }
  }
}
