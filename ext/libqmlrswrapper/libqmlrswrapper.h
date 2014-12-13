#ifndef libqmlrswrapper_H
#define libqmlrswrapper_H

#include <QtQuick>

class QrsInterface;

class QrsView : public QQuickView {
    Q_OBJECT
    
public:
    QrsView();
    
    void (*slot_fun)(const char *, void *);
    void *slot_data;
    
public slots:
    void invokeQmlSlot(QString name);
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
