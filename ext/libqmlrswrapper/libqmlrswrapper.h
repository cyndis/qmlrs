#ifndef libqmlrswrapper_H
#define libqmlrswrapper_H

#include <QtQuick>

class QrsInterface;

class QrsView : public QQuickView {
    Q_OBJECT
    
public:
    QrsView();
    
    void (*slot_fun)(const char *, void *, QVariant *);
    void *slot_data;
    
public slots:
    QVariant invokeQmlSlot(QString name, QVariantList args);
};

class QrsInterface : public QObject {
    Q_OBJECT
    
public:
    QrsInterface(QrsView *view) : _view(view)
    { }
    
public slots:
    QVariant invoke(QString event);
    
private:
    QrsView *_view;
};

#endif // libqmlrswrapper_H
