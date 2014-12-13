#include "libqmlrswrapper.h"

#include <QtQuick>
#include <QDebug>

#define rust_fun extern "C"

rust_fun int hello() {
    return 42;
}

rust_fun QrsView *qmlrs_create_view() {
    if (!QGuiApplication::instance()) {
        char *arg = (char *)malloc(13);
        strcpy(arg, "qmlrswrapper");
        char **argp = (char **)malloc(sizeof(char *));
        *argp = arg;
        
        int *argc = (int *)malloc(sizeof(int));
        *argc = 1;
        
        new QGuiApplication(*argc, argp);
    }
    
    return new QrsView();
}

rust_fun void qmlrs_destroy_view(QrsView *view) {
    delete view;
}

rust_fun void qmlrs_view_set_source(QrsView *view, const char *path, unsigned int len) {
    view->setSource(QString::fromUtf8(path, len));
}

rust_fun void qmlrs_view_show(QrsView *view) {
    view->show();
}

rust_fun void qmlrs_view_invoke(QrsView *view, const char *method) {
    QMetaObject::invokeMethod(view, "invokeQmlSlot", Q_ARG(QString, QString::fromUtf8(method)));
}

rust_fun void qmlrs_view_set_slot_function(QrsView *view, void (*fun)(const char *name, void *data),
                                           void *data)
{
    view->slot_fun = fun;
    view->slot_data = data;
}

rust_fun void qmlrs_app_exec() {
    QGuiApplication::exec();
}

QrsView::QrsView()
: slot_fun(NULL), slot_data(NULL)
{
    rootContext()->setContextProperty("qmlrs", new QrsInterface(this));
}

void QrsView::invokeQmlSlot(QString name) {
    QObject *root = rootObject();
    
    QVariant returned;
    QMetaObject::invokeMethod(root, name.toUtf8(), Q_RETURN_ARG(QVariant, returned));
}

QVariant QrsInterface::invoke(QString event)
{
    if (_view->slot_fun) {
        _view->slot_fun(event.toUtf8(), _view->slot_data);
    } else {
        qWarning("QML side slot called but Rust slot handler not registered");
    }
    
    return QVariant();
}
