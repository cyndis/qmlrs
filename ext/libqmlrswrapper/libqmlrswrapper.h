#ifndef libqmlrswrapper_H
#define libqmlrswrapper_H

#include <QtQuick>

class QrsInterface;

class QrsApplicationEngine : public QQmlApplicationEngine {
    Q_OBJECT
    
public:
    QrsApplicationEngine();
    
    void (*slot_fun)(const char *, void *, QVariant *);
    void *slot_data;
    
public slots:
    QVariant invokeQmlSlot(QString name, QVariantList args);
};

class QrsInterface : public QObject {
    Q_OBJECT
    
public:
    QrsInterface(QrsApplicationEngine *engine) : _engine(engine)
    { }
    
public slots:
    QVariant invoke(QString event);
    
private:
    QrsApplicationEngine *_engine;
};

#endif // libqmlrswrapper_H
