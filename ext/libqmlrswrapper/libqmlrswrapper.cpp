#include "libqmlrswrapper.h"

#include <QtQuick>
#include <QDebug>

#define rust_fun extern "C"

rust_fun QrsApplicationEngine *qmlrs_create_engine() {
    if (!QGuiApplication::instance()) {
        char *arg = (char *)malloc(13);
        strcpy(arg, "qmlrswrapper");
        char **argp = (char **)malloc(sizeof(char *));
        *argp = arg;
        
        int *argc = (int *)malloc(sizeof(int));
        *argc = 1;
        
        new QGuiApplication(*argc, argp);
    }
    
    return new QrsApplicationEngine();
}

rust_fun void qmlrs_destroy_engine(QrsApplicationEngine *engine) {
    delete engine;
}

rust_fun void qmlrs_engine_load_url(QrsApplicationEngine *engine, const char *path, unsigned int len) {
    engine->load(QUrl(QString::fromUtf8(path, len)));
}

rust_fun void qmlrs_engine_invoke(QrsApplicationEngine *engine, const char *method, 
                                  QVariant *result, const QVariantList *args)
{
    if (args->size() > 10) {
        qFatal("Cannot invoke method with more than 10 arguments");
    }
    
    QVariant returned;
    QMetaObject::invokeMethod(engine, "invokeQmlSlot", Q_RETURN_ARG(QVariant, returned), 
                              Q_ARG(QString, QString::fromUtf8(method)),
                              Q_ARG(QVariantList, *args));
    *result = returned;
}

rust_fun void qmlrs_engine_set_slot_function(QrsApplicationEngine *engine, QrsSlotFun fun, 
                                             void *data)
{
    engine->slot_fun = fun;
    engine->slot_data = data;
}

rust_fun QVariantList *qmlrs_varlist_create() {
    return new QVariantList();
}

rust_fun void qmlrs_varlist_destroy(QVariantList *list) {
    delete list;
}

rust_fun QVariant *qmlrs_varlist_push(QVariantList *list) {
    list->append(QVariant());
    return (QVariant *)&list->last();
}

rust_fun unsigned int qmlrs_varlist_length(const QVariantList *list) {
    return list->size();
}

rust_fun QVariant *qmlrs_varlist_get(const QVariantList *list, unsigned int i) {
    return (QVariant *)&(*list)[i];
}

rust_fun void qmlrs_app_exec() {
    QGuiApplication::exec();
}

rust_fun void qmlrs_variant_set_int(QVariant *v, int x) {
    *v = QVariant(x);
}

rust_fun void qmlrs_variant_set_invalid(QVariant *v) {
    *v = QVariant();
}

rust_fun void qmlrs_variant_set_string(QVariant *v, unsigned int len, const char *data) {
    *v = QVariant(QString::fromUtf8(data, len));
}

rust_fun QVariant *qmlrs_variant_create() {
    return new QVariant();
}

rust_fun void qmlrs_variant_destroy(QVariant *v) {
    delete v;
}

enum QrsVariantType {
    Invalid = 0, Int, String
};

rust_fun QrsVariantType qmlrs_variant_get_type(const QVariant *v) {
    if (!v->isValid())
        return Invalid;
    
    if (v->canConvert(QMetaType::Int))
        return Int;
    
    if (v->canConvert(QMetaType::QString))
        return String;
    
    /* Unknown type, not supported on Rust side */
    return Invalid;
}

rust_fun void qmlrs_variant_get_int(const QVariant *v, int *x) {
    *x = v->toInt();
}

rust_fun void qmlrs_variant_get_string_length(const QVariant *v, unsigned int *len) {
    *len = v->toString().toUtf8().size();
}

rust_fun void qmlrs_variant_get_string_data(const QVariant *v, char *data) {
    QByteArray ba = v->toString().toUtf8();
    memcpy(data, ba.data(), ba.size());
}

QrsApplicationEngine::QrsApplicationEngine()
: slot_fun(NULL), slot_data(NULL)
{
    rootContext()->setContextProperty("qmlrs", new QrsInterface(this));
}

QVariant QrsApplicationEngine::invokeQmlSlot(QString name, QVariantList args) {
    QObject *root = rootObjects().first();
    
    QVariant returned;
    
    QGenericArgument a0, a1, a2, a3, a4, a5, a6, a7, a8, a9;
    if (args.size() > 9) a9 = Q_ARG(QVariant, args[9]);
    if (args.size() > 8) a8 = Q_ARG(QVariant, args[8]);
    if (args.size() > 7) a7 = Q_ARG(QVariant, args[7]);
    if (args.size() > 6) a6 = Q_ARG(QVariant, args[6]);
    if (args.size() > 5) a5 = Q_ARG(QVariant, args[5]);
    if (args.size() > 4) a4 = Q_ARG(QVariant, args[4]);
    if (args.size() > 3) a3 = Q_ARG(QVariant, args[3]);
    if (args.size() > 2) a2 = Q_ARG(QVariant, args[2]);
    if (args.size() > 1) a1 = Q_ARG(QVariant, args[1]);
    if (args.size() > 0) a0 = Q_ARG(QVariant, args[0]);
    
    QMetaObject::invokeMethod(root, name.toUtf8(), Q_RETURN_ARG(QVariant, returned),
                              a0, a1, a2, a3, a4, a5, a6, a7, a8, a9);
    
    return returned;
}

QVariant QrsInterface::invoke(QString event, QVariantList args)
{
    if (_engine->slot_fun) {
        QVariant v;
        _engine->slot_fun(event.toUtf8(), _engine->slot_data, &v, &args);
        return v;
    } else {
        qWarning("QML side slot called but Rust slot handler not registered");
    }
    
    return QVariant();
}
