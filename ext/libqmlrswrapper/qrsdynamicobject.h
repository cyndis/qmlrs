#ifndef QRSDYNAMICOBJECT_H
#define QRSDYNAMICOBJECT_H

#include <QtCore>

#if Q_MOC_OUTPUT_REVISION != 67
#error "Unsupport Qt version. Qt with Q_MOC_OUTPUT_REVISION == 67 is required."
#endif

typedef void *(QrsSlotFunction)(const char *name, void *data, QVariant *result, QVariantList *args);

class QrsDynamicMetaObject
{
public:
    QrsDynamicMetaObject();
    virtual ~QrsDynamicMetaObject();
    
    struct Slot {
        QString name;
        int args;
    };
    
    void addSlot(Slot s) {
        if (_mo)
            qFatal("Cannot add slot after object created");

        _slots.append(s);
    }
    
    QObject *create(QrsSlotFunction fun, void *data);

private:
    QList<Slot> _slots;
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

private:
    QrsSlotFunction *_fun;
    void *_data;
    int _n_slots;

    QMetaObject *_mo;
    
    void invokeMetacall(int id);
};

#endif // QRSDYNAMICOBJECT_H
