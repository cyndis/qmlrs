#include "qrsdynamicobject.h"

QrsDynamicMetaObject::QrsDynamicMetaObject()
: _mo(NULL)
{
}

QrsDynamicMetaObject::~QrsDynamicMetaObject()
{
    if (_mo) {
        delete _mo->d.stringdata;
        delete _mo->d.data;
        delete _mo;
    }
}

void qrsStaticDynamicMetacall(QObject *qobj, QMetaObject::Call c, int id, void **a) {
    qobj->qt_metacall(c, id, a);
}

void QrsDynamicMetaObject::finalize()
{
    _mo = new QMetaObject;
    _mo->d.superdata = &QObject::staticMetaObject;
    _mo->d.extradata = NULL;
    _mo->d.relatedMetaObjects = NULL;
    _mo->d.static_metacall = qrsStaticDynamicMetacall;
    
    /* Build string data */
    
    struct ArrayData {
        int ref;
        int size;
        uint _1 : 31;
        uint _2 : 1;
        qptrdiff offset;
    };

    QVector<ArrayData> stringhdr;
    QByteArray stringdata;
    
#define ADD_STRING_HDR(len) { stringhdr.append(ArrayData { -1, len, 0, 0, \
        qptrdiff(stringdata.size() - stringhdr.size() * sizeof(ArrayData)) }); }
    
    QString classname = "DynamicObject";
    
    ADD_STRING_HDR(classname.size());
    stringdata.append(classname);
    stringdata.append('\0');
    for (int i = 0; i < _slots.size(); ++i) {
        ADD_STRING_HDR(_slots[i].name.size());
        stringdata.append(_slots[i].name);
        stringdata.append('\0');
        ADD_STRING_HDR(0);
        stringdata.append('\0'); // tag
        for (int j = 0; j < _slots[i].args; ++j) {
            QString arg = QString::fromUtf8("arg%1").arg(j);
            ADD_STRING_HDR(arg.size());
            stringdata.append(arg);
            stringdata.append('\0');
        }
    }

#undef ADD_STRING_HDR
    
    for (int i = 0; i < stringhdr.size(); ++i) {
        printf("hdr %d: len %d, offset %d\n", i, stringhdr[i].size, stringhdr[i].offset);
    }
    
    for (int i = 0; i < stringhdr.size(); ++i) {
        stringhdr[i].offset += stringhdr.size() * sizeof(ArrayData);
    }
    
    char *stringdata_buf = new char[stringhdr.size() * sizeof(ArrayData) + stringdata.size()];
    memcpy(stringdata_buf, stringhdr.constData(), stringhdr.size() * sizeof(ArrayData));
    memcpy(stringdata_buf + stringhdr.size() * sizeof(ArrayData), stringdata.data(), 
           stringdata.size());
    _mo->d.stringdata = (const QByteArrayData *)stringdata_buf;
    
    for (int i = 0; i < stringhdr.size(); ++i) {
        printf("hdr %d: len %d, offset %d\n", i, stringhdr[i].size, stringhdr[i].offset);
    }
    
    for (int i = 0; i < stringdata.size(); ++i) {
        if (!stringdata_buf[stringhdr.size() * sizeof(ArrayData) + i])
            printf(";");
        else
            printf("%c", stringdata_buf[stringhdr.size() * sizeof(ArrayData) + i]);
    }
    printf("\n");
    
    /* Build metadata */
    
    QVector<uint> metadata;
    metadata.append(7); /* revision */
    metadata.append(0); /* classname string id */
    metadata.append(0); metadata.append(0); /* class info */
    metadata.append(_slots.size()); metadata.append(0xbd); /* method number, list offset */
    metadata.append(0); metadata.append(0); /* properties */
    metadata.append(0); metadata.append(0); /* enums */
    metadata.append(0); metadata.append(0); /* constructors */
    metadata.append(0); /* flags */
    metadata.append(0); /* signals */
    
    metadata[5] = metadata.size(); /* fixup method list offset */
    int str_ptr = 1;
    QList<uint> fixup_offsets;
    for (int i = 0; i < _slots.size(); ++i) {
        metadata.append(str_ptr);
        metadata.append(_slots[i].args);
        fixup_offsets.append(metadata.size());
        metadata.append(0xbd); /* argument list offset */
        metadata.append(str_ptr+1); /* tag. unused */
        metadata.append(0x0a); /* public */
        str_ptr += 2 + _slots[i].args;
    }
    
    str_ptr = 3;
    for (int i = 0; i < _slots.size(); ++i) {
        metadata[fixup_offsets.takeFirst()] = metadata.size();
        metadata.append(QMetaType::QVariant);
        for (int j = 0; j < _slots[i].args; ++j) {
            metadata.append(QMetaType::QVariant);
        }
        for (int j = 0; j < _slots[i].args; ++j) {
            metadata.append(str_ptr+j);
        }
        str_ptr += 2 + _slots[i].args;
    }
    
    metadata.append(0);
    
    qDebug() << "metadata" << metadata;
    
    uint *metadata_buf = new uint[metadata.size()];
    memcpy(metadata_buf, metadata.constData(), metadata.size() * sizeof(uint));
    _mo->d.data = metadata_buf;
}

QObject* QrsDynamicMetaObject::create(QrsSlotFunction fun, void* data)
{
    if (!_mo)
        finalize();
    
    return new QrsDynamicObject(fun, data, _mo, _slots.size());
}

QrsDynamicObject::QrsDynamicObject(QrsSlotFunction* fun, void* data, QMetaObject* mo, int n_slots)
: QObject(), _fun(fun), _data(data), _mo(mo), _n_slots(n_slots)
{
}

const QMetaObject* QrsDynamicObject::metaObject() const
{
    if (!_mo)
        qFatal("QrsDynamicObject::metaObject() called without finalization");
    
    return _mo;
}

void* QrsDynamicObject::qt_metacast(const char* )
{
    if (!_mo)
        qFatal("QrsDynamicObject::qt_metacast() called without finalization");
    
    qDebug() << "qt_metacast called";

    return Q_NULLPTR;
}

int QrsDynamicObject::qt_metacall(QMetaObject::Call c, int id, void** a)
{
    if (!_mo)
        qFatal("QrsDynamicObject::qt_metacall() called without finalization");
    
    id = QObject::qt_metacall(c, id, a);
    
    qDebug() << "QrsDynamicObject::qt_metacall:" << c << id << a;
    
    if (c == QMetaObject::InvokeMetaMethod) {
        if (id < _n_slots) {
            invokeMetacall(id, a);
        }
        id -= 1;
    } else if (c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (id < _n_slots) {
            *reinterpret_cast<int*>(a[0]) = -1;
        }
        id -= 1;
    }
    return id;
}

void QrsDynamicObject::invokeMetacall(int id, void **args)
{
    if (_fun)
        _fun(_data, id, (QVariant **)args);
    else
        qWarning("QrsDynamicMetaObject: tried to invoke metacall but handler not set");
}
