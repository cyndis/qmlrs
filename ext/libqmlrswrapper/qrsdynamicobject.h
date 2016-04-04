#ifndef QRSDYNAMICOBJECT_H
#define QRSDYNAMICOBJECT_H

#include <QtCore>

#if Q_MOC_OUTPUT_REVISION != 67
#error "Unsupport Qt version. Qt with Q_MOC_OUTPUT_REVISION == 67 is required."
#endif

extern "C" typedef void *(QrsSlotFunction)(void *data, int slot, QVariant **args);

class QrsDynamicMetaObject
{
public:
    QrsDynamicMetaObject();
    virtual ~QrsDynamicMetaObject();

    struct Method {
        QString name;
        uint args;
        uint flags;
    };

    void addSlot(QString name, uint args) {
        if (_mo)
            qFatal("Cannot add slot after object created");

        Method m = { name, args, 0x0a };

        _methods.append(m);
    }

    void addSignal(QString name, uint args) {
        if (_mo)
            qFatal("Cannot add signal after object created");

        Method m = { name, args, 0x06 };

        _methods.append(m);
    }

    QObject *create(QrsSlotFunction fun, void *data);

private:
    QList<Method> _methods;
    QMetaObject *_mo;

    void finalize();
};

class QrsDynamicObject : public QObject
{
public:
    QrsDynamicObject(QrsSlotFunction *fun, void *data, QMetaObject *mo, int n_slots);
    virtual const QMetaObject* metaObject() const;
    virtual void* qt_metacast(const char* );
    virtual int qt_metacall(QMetaObject::Call , int , void** );

    void emitSignal(int id);

private:
    QrsSlotFunction *_fun;
    void *_data;
    int _n_slots;

    QMetaObject *_mo;

    void invokeMetacall(int id, void** args);
};

#endif // QRSDYNAMICOBJECT_H
