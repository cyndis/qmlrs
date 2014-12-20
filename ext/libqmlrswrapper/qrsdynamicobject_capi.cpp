#include "qrsdynamicobject.h"

extern "C" QrsDynamicMetaObject *qmlrs_metaobject_create() {
    return new QrsDynamicMetaObject();
}

extern "C" void qmlrs_metaobject_destroy(QrsDynamicMetaObject *mo) {
    delete mo;
}

extern "C" void qmlrs_metaobject_add_slot(QrsDynamicMetaObject *mo, const char *name, uint name_len,
                                          uint argc)
{
    mo->addSlot(QrsDynamicMetaObject::Slot { QString::fromUtf8(name, name_len), argc });
}

extern "C" QObject *qmlrs_metaobject_instantiate(QrsDynamicMetaObject *mo, QrsSlotFunction fun, 
                                                 void *data)
{
    return mo->create(fun, data);
}

extern "C" void qmlrs_object_destroy(QrsDynamicObject *obj) {
    delete obj;
}