// generic signal relay: any QObject signal -> Rust callback.
// uses string-based connect(sender, SIGNAL, relay, SLOT), working around Qt6 dropping QMetaMethod+lambda connect.
#pragma once

#include <QObject>
#include <cstddef>

namespace dtkrs {

class DtkRelay : public QObject {
    Q_OBJECT
public:
    explicit DtkRelay(size_t cb_id, QObject *parent);

    /// signal looks like "clicked()" / "timeout()"; failure logs qWarning
    static void connect0(QObject *sender, const char *signal, size_t cb_id);
    static void connectI32(QObject *sender, const char *signal, size_t cb_id);

public Q_SLOTS:
    void fire0();
    void fireI32(int v);

private:
    size_t m_cb_id;
};

} // namespace dtkrs
