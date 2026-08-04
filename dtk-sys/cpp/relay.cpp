#include "relay.h"
#include "dtk-sys/src/lib.rs.h" // dtk_cb0 / dtk_cb_i32

#include <QMetaObject>

namespace dtkrs {

DtkRelay::DtkRelay(size_t cb_id, QObject *parent) : QObject(parent), m_cb_id(cb_id) {}

void DtkRelay::connect0(QObject *sender, const char *signal, size_t cb_id) {
    auto *relay = new DtkRelay(cb_id, sender); // parented to sender, destroyed with it
    QByteArray sig = "2" + QMetaObject::normalizedSignature(signal); // '2' = signal prefix
    if (!QObject::connect(sender, sig.constData(), relay, SLOT(fire0()))) {
        qWarning("dtkrs: connect signal %s on %s failed", signal, sender->metaObject()->className());
        delete relay;
    }
}

void DtkRelay::connectI32(QObject *sender, const char *signal, size_t cb_id) {
    auto *relay = new DtkRelay(cb_id, sender);
    QByteArray sig = "2" + QMetaObject::normalizedSignature(signal);
    if (!QObject::connect(sender, sig.constData(), relay, SLOT(fireI32(int)))) {
        qWarning("dtkrs: connect signal %s on %s failed", signal, sender->metaObject()->className());
        delete relay;
    }
}

void DtkRelay::fire0() { dtk_cb0(m_cb_id); }
void DtkRelay::fireI32(int v) { dtk_cb_i32(m_cb_id, static_cast<int32_t>(v)); }

} // namespace dtkrs
