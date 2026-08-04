// 通用信号转发器：任意 QObject 信号 → Rust 回调。
// 用字符串式 connect(sender, SIGNAL, relay, SLOT)，绕开 Qt6 砍掉的 QMetaMethod+lambda connect。
#pragma once

#include <QObject>
#include <cstddef>

namespace dtkrs {

class DtkRelay : public QObject {
    Q_OBJECT
public:
    explicit DtkRelay(size_t cb_id, QObject *parent);

    /// signal 形如 "clicked()" / "timeout()"，失败打 qWarning
    static void connect0(QObject *sender, const char *signal, size_t cb_id);
    static void connectI32(QObject *sender, const char *signal, size_t cb_id);

public Q_SLOTS:
    void fire0();
    void fireI32(int v);

private:
    size_t m_cb_id;
};

} // namespace dtkrs
