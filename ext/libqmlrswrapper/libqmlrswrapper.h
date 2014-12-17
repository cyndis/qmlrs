#ifndef libqmlrswrapper_H
#define libqmlrswrapper_H

#include <QtQuick>

typedef void *(QrsSlotFun)(const char *name, void *data, QVariant *result, QVariantList *args);

class QrsInterface;

class QrsApplicationEngine : public QQmlApplicationEngine {
    Q_OBJECT
    
public:
    QrsApplicationEngine();
    
    QrsSlotFun *slot_fun;
    void *slot_data;
    
public slots:
    QVariant invokeQmlSlot(QString name, QVariantList args);
    
private:
    QrsInterface *_interface;
};

class QrsInterface : public QObject {
    Q_OBJECT
    
public:
    QrsInterface(QrsApplicationEngine *engine) : _engine(engine)
    { }
    
public slots:
    QVariant invoke(QString event, QVariantList args);
    
private:
    QrsApplicationEngine *_engine;
};

#endif // libqmlrswrapper_H
