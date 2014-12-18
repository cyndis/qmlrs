#ifndef libqmlrswrapper_H
#define libqmlrswrapper_H

#include <QtQuick>

typedef void *(QrsSlotFun)(const char *name, void *data, QVariant *result, QVariantList *args);

class QrsInterface;

class QrsApplicationEngine : public QQmlApplicationEngine {
    Q_OBJECT
    
public:
    QrsApplicationEngine();
    
public slots:
    QVariant invokeQmlSlot(QString name, QVariantList args);
};

#endif // libqmlrswrapper_H
