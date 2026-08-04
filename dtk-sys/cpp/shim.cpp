#include "dtk_shim.h"
#include "relay.h"
#include "dtk-sys/src/lib.rs.h" // Rust 回调 dtk_cb0/dtk_cb_i32 声明

#include <QCoreApplication>
#include <QHeaderView>
#include <QTableWidgetItem>
#include <cstdio>
#include <string>

namespace dtkrs {

rust::String to_rust_string(const QString &s) {
    QByteArray utf8 = s.toUtf8();
    return rust::String(utf8.constData(), utf8.size());
}

QString from_rust_str(rust::Str s) {
    return QString::fromUtf8(s.data(), static_cast<qsizetype>(s.size()));
}

// ---- DApplication ----
DApplication *application_new(rust::Str name) {
    // QApplication 要求 argv 生命周期覆盖 app，用 static 顶上
    static int argc = 1;
    static char arg0[256];
    std::snprintf(arg0, sizeof(arg0), "%.*s", static_cast<int>(name.size()), name.data());
    static char *argv[] = {arg0, nullptr};
    auto *app = new DApplication(argc, argv);
    app->setApplicationName(QString::fromUtf8(arg0));
    return app;
}

int32_t application_exec(DApplication *app) { return app->exec(); }
void application_quit() { QCoreApplication::quit(); }

// ---- QWidget 通用 ----
void widget_show(QWidget *w) { w->show(); }
void widget_resize(QWidget *w, int32_t w_px, int32_t h_px) { w->resize(w_px, h_px); }
void widget_set_enabled(QWidget *w, bool on) { w->setEnabled(on); }
void widget_set_window_title(QWidget *w, rust::Str title) { w->setWindowTitle(from_rust_str(title)); }

// ---- DMainWindow ----
DMainWindow *mainwindow_new() { return new DMainWindow; }
DTitlebar *mainwindow_titlebar(DMainWindow *w) { return w->titlebar(); }
void mainwindow_set_central_widget(DMainWindow *w, QWidget *central) { w->setCentralWidget(central); }
void mainwindow_set_window_radius(DMainWindow *w, int32_t radius) { w->setWindowRadius(radius); }
void mainwindow_set_enable_blur(DMainWindow *w, bool enable) { w->setEnableBlurWindow(enable); }

// ---- DTitlebar ----
void titlebar_set_title(DTitlebar *tb, rust::Str title) { tb->setTitle(from_rust_str(title)); }
void titlebar_set_icon(DTitlebar *tb, const QIcon &icon) { tb->setIcon(icon); }

// ---- QIcon ----
QIcon *icon_from_theme(rust::Str name) { return new QIcon(QIcon::fromTheme(from_rust_str(name))); }
QIcon *icon_from_file(rust::Str path) { return new QIcon(from_rust_str(path)); }

// ---- DLabel ----
DLabel *label_new(rust::Str text) { return new DLabel(from_rust_str(text)); }
void label_set_text(DLabel *l, rust::Str text) { l->setText(from_rust_str(text)); }

// ---- 按钮 ----
DSuggestButton *suggest_button_new(rust::Str text) {
    auto *b = new DSuggestButton;
    b->setText(from_rust_str(text));
    return b;
}
DPushButton *push_button_new(rust::Str text) { return new DPushButton(from_rust_str(text)); }
void button_set_text(DPushButton *b, rust::Str text) { b->setText(from_rust_str(text)); }
void button_click(DPushButton *b) { b->click(); }

// ---- 布局 ----
QVBoxLayout *vbox_new(QWidget *parent) { return new QVBoxLayout(parent); }
QHBoxLayout *hbox_new(QWidget *parent) { return new QHBoxLayout(parent); }
QWidget *widget_new(QWidget *parent) { return new QWidget(parent); }
void layout_add_widget(QLayout *l, QWidget *w) { l->addWidget(w); }
void layout_add_layout(QLayout *l, QLayout *child) { l->addItem(child); }
void layout_set_spacing(QLayout *l, int32_t spacing) { l->setSpacing(spacing); }
void layout_set_contents_margins(QLayout *l, int32_t l_, int32_t t, int32_t r, int32_t b) {
    l->setContentsMargins(l_, t, r, b);
}

// ---- QTableWidget ----
QTableWidget *table_new(int32_t rows, int32_t cols) { return new QTableWidget(rows, cols); }
void table_set_column_count(QTableWidget *t, int32_t cols) { t->setColumnCount(cols); }
void table_set_row_count(QTableWidget *t, int32_t rows) { t->setRowCount(rows); }
void table_set_horizontal_header_labels(QTableWidget *t, rust::Str joined) {
    t->setHorizontalHeaderLabels(from_rust_str(joined).split(QLatin1Char('|')));
}
void table_set_cell_text(QTableWidget *t, int32_t row, int32_t col, rust::Str text) {
    t->setItem(row, col, new QTableWidgetItem(from_rust_str(text)));
}
void table_set_cell_data(QTableWidget *t, int32_t row, int32_t col, int64_t data) {
    if (auto *item = t->item(row, col))
        item->setData(Qt::UserRole, QVariant::fromValue<qint64>(data));
}
int64_t table_cell_data(QTableWidget *t, int32_t row, int32_t col) {
    if (auto *item = t->item(row, col))
        return item->data(Qt::UserRole).toLongLong();
    return 0;
}
rust::String table_cell_text(QTableWidget *t, int32_t row, int32_t col) {
    if (auto *item = t->item(row, col))
        return to_rust_string(item->text());
    return rust::String();
}
int32_t table_current_row(QTableWidget *t) { return t->currentRow(); }
int32_t table_row_count(QTableWidget *t) { return t->rowCount(); }
void table_select_rows_readonly(QTableWidget *t) {
    t->setSelectionBehavior(QAbstractItemView::SelectRows);
    t->setEditTriggers(QAbstractItemView::NoEditTriggers);
    t->setSelectionMode(QAbstractItemView::SingleSelection);
}
void table_header_stretch_last(QTableWidget *t, bool stretch) {
    t->horizontalHeader()->setStretchLastSection(stretch);
}
void table_set_column_width(QTableWidget *t, int32_t col, int32_t width) {
    t->setColumnWidth(col, width);
}

// ---- QTimer ----
QTimer *timer_new(QObject *parent) { return new QTimer(parent); }
void timer_start(QTimer *t, int32_t msec) { t->start(msec); }
void timer_stop(QTimer *t) { t->stop(); }
void timer_single_shot(int32_t msec, size_t cb_id) {
    QTimer::singleShot(msec, QCoreApplication::instance(), [cb_id] { dtk_cb0(cb_id); });
}

// ---- 信号回调 ----
void relay_connect0(QObject *sender, rust::Str signal, size_t cb_id) {
    DtkRelay::connect0(sender, std::string(signal).c_str(), cb_id);
}

void relay_connect_i32(QObject *sender, rust::Str signal, size_t cb_id) {
    DtkRelay::connectI32(sender, std::string(signal).c_str(), cb_id);
}

} // namespace dtkrs
