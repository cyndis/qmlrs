#include "libqmlrswrapper.h"

#include "qrsdynamicobject.h"

#include <QtQuick>
#include <QDebug>

#define rust_fun extern "C"

rust_fun QrsApplicationEngine *qmlrs_create_engine_headless() {
    if (!QCoreApplication::instance()) {
        char *arg = (char *)malloc(13);
        strcpy(arg, "qmlrswrapper");
        char **argp = (char **)malloc(sizeof(char *));
        *argp = arg;

        int *argc = (int *)malloc(sizeof(int));
        *argc = 1;

        new QCoreApplication(*argc, argp);
    }

    return new QrsApplicationEngine();
}

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

rust_fun void qmlrs_engine_load_from_data(QrsApplicationEngine *engine, const char *data, unsigned int len) {
    engine->loadData(QByteArray::fromRawData(data, len), QUrl());
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

rust_fun void qmlrs_engine_set_property(QrsApplicationEngine *engine, const char *name, uint len,
                                        QObject *object) {
    engine->rootContext()->setContextProperty(QString::fromUtf8(name, len), object);
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

rust_fun void qmlrs_variant_set_int64(QVariant *v, int64_t x) {
    *v = QVariant((qlonglong)x);
}

rust_fun void qmlrs_variant_set_bool(QVariant *v, bool x) {
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
    Invalid = 0, Int64, Bool, String
};

rust_fun QrsVariantType qmlrs_variant_get_type(const QVariant *v) {
    if (!v->isValid())
        return Invalid;

    if (v->type() == (QVariant::Type)QMetaType::QString)
        return String;

    if (v->canConvert(QMetaType::LongLong))
        return Int64;

    if (v->canConvert(QMetaType::Bool))
        return Bool;

    /* Unknown type, not supported on Rust side */
    return Invalid;
}

rust_fun void qmlrs_variant_get_int64(const QVariant *v, int64_t *x) {
    *x = v->toLongLong();
}

rust_fun void qmlrs_variant_get_bool(const QVariant *v, bool *x) {
    *x = v->toBool();
}

rust_fun void qmlrs_variant_get_string_length(const QVariant *v, unsigned int *len) {
    *len = v->toString().toUtf8().size();
}

rust_fun void qmlrs_variant_get_string_data(const QVariant *v, char *data) {
    QByteArray ba = v->toString().toUtf8();
    memcpy(data, ba.data(), ba.size());
}

QrsApplicationEngine::QrsApplicationEngine()
{
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
