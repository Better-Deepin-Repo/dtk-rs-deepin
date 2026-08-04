// dtk-rs C++ shim：把 DTK6/Qt6 C++ API 拍平成自由函数，给 cxx::bridge 用。
// 所有对象由 Qt parent-child 机制管理生命周期，Rust 侧只拿裸指针。
#pragma once

#include <cstddef>
#include <cstdint>
#include <memory>
#include <rust/cxx.h>

#include <QObject>
#include <QWidget>
#include <QLayout>
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QTableWidget>
#include <QTimer>
#include <QIcon>
#include <QString>

#include <DApplication>
#include <DMainWindow>
#include <DTitlebar>
#include <DLabel>
#include <dsuggestbutton.h>
#include <dwidgetstype.h> // DPushButton = QPushButton typedef

namespace dtkrs {

// cxx 要求 opaque type 全在 bridge namespace 里，Qt 类也 alias 进来
using ::QObject;
using ::QWidget;
using ::QLayout;
using ::QVBoxLayout;
using ::QHBoxLayout;
using ::QTableWidget;
using ::QTimer;
using ::QIcon;

using DApplication = Dtk::Widget::DApplication;
using DMainWindow = Dtk::Widget::DMainWindow;
using DTitlebar = Dtk::Widget::DTitlebar;
using DLabel = Dtk::Widget::DLabel;
using DSuggestButton = Dtk::Widget::DSuggestButton;
using DPushButton = Dtk::Widget::DPushButton;

// ---- QString <-> rust string ----
rust::String to_rust_string(const QString &s);
QString from_rust_str(rust::Str s);

// ---- DApplication ----
DApplication *application_new(rust::Str name);
int32_t application_exec(DApplication *app);
void application_quit();

// ---- QWidget 通用（所有控件适用）----
void widget_show(QWidget *w);
void widget_resize(QWidget *w, int32_t w_px, int32_t h_px);
void widget_set_enabled(QWidget *w, bool on);
void widget_set_window_title(QWidget *w, rust::Str title);

// ---- DMainWindow ----
DMainWindow *mainwindow_new();
DTitlebar *mainwindow_titlebar(DMainWindow *w);
void mainwindow_set_central_widget(DMainWindow *w, QWidget *central);
void mainwindow_set_window_radius(DMainWindow *w, int32_t radius);
void mainwindow_set_enable_blur(DMainWindow *w, bool enable);

// ---- DTitlebar ----
void titlebar_set_title(DTitlebar *tb, rust::Str title);
void titlebar_set_icon(DTitlebar *tb, const QIcon &icon);

// ---- QIcon ----
QIcon *icon_from_theme(rust::Str name);
QIcon *icon_from_file(rust::Str path);

// ---- DLabel ----
DLabel *label_new(rust::Str text);
void label_set_text(DLabel *l, rust::Str text);

// ---- 按钮 ----
DSuggestButton *suggest_button_new(rust::Str text);
DPushButton *push_button_new(rust::Str text);
void button_set_text(DPushButton *b, rust::Str text);
void button_click(DPushButton *b); // 程序化点击，触发 clicked 信号

// ---- 布局 ----
QVBoxLayout *vbox_new(QWidget *parent);
QHBoxLayout *hbox_new(QWidget *parent);
QWidget *widget_new(QWidget *parent);
void layout_add_widget(QLayout *l, QWidget *w);
void layout_add_layout(QLayout *l, QLayout *child);
void layout_set_spacing(QLayout *l, int32_t spacing);
void layout_set_contents_margins(QLayout *l, int32_t l_, int32_t t, int32_t r, int32_t b);

// ---- QTableWidget ----
QTableWidget *table_new(int32_t rows, int32_t cols);
void table_set_column_count(QTableWidget *t, int32_t cols);
void table_set_row_count(QTableWidget *t, int32_t rows);
void table_set_horizontal_header_labels(QTableWidget *t, rust::Str joined); // '|' 分隔
void table_set_cell_text(QTableWidget *t, int32_t row, int32_t col, rust::Str text);
void table_set_cell_data(QTableWidget *t, int32_t row, int32_t col, int64_t data);
int64_t table_cell_data(QTableWidget *t, int32_t row, int32_t col);
rust::String table_cell_text(QTableWidget *t, int32_t row, int32_t col);
int32_t table_current_row(QTableWidget *t);
int32_t table_row_count(QTableWidget *t);
void table_select_rows_readonly(QTableWidget *t);
void table_header_stretch_last(QTableWidget *t, bool stretch);
void table_set_column_width(QTableWidget *t, int32_t col, int32_t width);

// ---- QTimer ----
QTimer *timer_new(QObject *parent);
void timer_start(QTimer *t, int32_t msec);
void timer_stop(QTimer *t);
void timer_single_shot(int32_t msec, size_t cb_id);

// ---- 信号回调 ----
// 通用：按信号名运行时 connect，忽略参数。signal 形如 "clicked()" 或 "clicked(bool)"
void relay_connect0(QObject *sender, rust::Str signal, size_t cb_id);
// 常用带参信号
void relay_connect_i32(QObject *sender, rust::Str signal, size_t cb_id);

} // namespace dtkrs
