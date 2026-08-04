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
#include <QTableWidgetItem>
#include <QTimer>
#include <QIcon>
#include <QString>
#include <QColor>
#include <QFont>
#include <QPalette>
#include <QPixmap>
#include <QPoint>
#include <QRect>
#include <QSize>
#include <QPainter>
#include <QStyledItemDelegate>
#include <QModelIndex>
#include <QSocketNotifier>
#include <QEvent>
#include <QCloseEvent>
#include <QShowEvent>

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
using ::QTableWidgetItem;
using ::QTimer;
using ::QIcon;
using ::QColor;
using ::QFont;
using ::QPalette;
using ::QPixmap;
using ::QPoint;
using ::QRect;
using ::QSize;
using ::QPainter;
using ::QModelIndex;
using ::QSocketNotifier;
using ::QEvent;
using ::QStyledItemDelegate;

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
DApplication *application_new_ex(rust::Str name, size_t quit_guard_id); // guard 返回 false 则吞掉 Quit 事件
int32_t application_exec(DApplication *app);
void application_quit();
void application_set_quit_on_last_window_closed(bool quit);
void application_set_application_display_name(rust::Str name);
bool application_load_translator(DApplication *app);
bool application_has_arg(rust::Str arg);

// ---- QWidget 通用（所有控件适用）----
void widget_show(QWidget *w);
void widget_resize(QWidget *w, int32_t w_px, int32_t h_px);
void widget_set_enabled(QWidget *w, bool on);
void widget_set_window_title(QWidget *w, rust::Str title);
void widget_set_fixed_size(QWidget *w, int32_t w_px, int32_t h_px);
void widget_raise(QWidget *w);
void widget_activate_window(QWidget *w);
void widget_close(QWidget *w);
bool widget_is_visible(QWidget *w);
void widget_set_focus_policy(QWidget *w, int32_t policy);
void widget_set_font(QWidget *w, QFont *font);
QPalette *widget_palette(QWidget *w); // 堆拷贝，调用方持有
void widget_set_palette(QWidget *w, QPalette *pal);
void object_delete_later(QObject *o);

// ---- DMainWindow ----
DMainWindow *mainwindow_new();
DMainWindow *mainwindow_new_ex(size_t show_cb_id, size_t close_cb_id); // close 回调返 false → ignore
DTitlebar *mainwindow_titlebar(DMainWindow *w);
void mainwindow_set_central_widget(DMainWindow *w, QWidget *central);
QWidget *mainwindow_take_central_widget(DMainWindow *w);
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
void label_set_word_wrap(DLabel *l, bool wrap);
void label_set_alignment(DLabel *l, int32_t alignment); // Qt::Alignment
void label_set_pixmap(DLabel *l, QPixmap *pm);

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

// ---- QTableWidget 扩展 ----
QTableWidgetItem *table_item(QTableWidget *t, int32_t row, int32_t col);
void table_select_row(QTableWidget *t, int32_t row);
void table_hide_headers(QTableWidget *t, bool horizontal, bool vertical);
void table_set_section_resize_mode(QTableWidget *t, int32_t col, int32_t mode); // QHeaderView::ResizeMode
void table_set_vertical_header_default_section_size(QTableWidget *t, int32_t size);
void table_set_show_grid(QTableWidget *t, bool show);
void table_set_frame_shape(QTableWidget *t, int32_t shape); // QFrame::Shape
void table_set_icon_size(QTableWidget *t, int32_t w, int32_t h);
void table_set_delegate_for_column(QTableWidget *t, int32_t col, QStyledItemDelegate *delegate);

// ---- QTableWidgetItem ----
void item_set_icon(QTableWidgetItem *it, QIcon *icon);
void item_set_text_alignment(QTableWidgetItem *it, int32_t alignment);
void item_set_foreground(QTableWidgetItem *it, QColor *color);
void item_set_data_string(QTableWidgetItem *it, int32_t role, rust::Str value);
rust::String item_data_string(QTableWidgetItem *it, int32_t role);
void item_set_data_bool(QTableWidgetItem *it, int32_t role, bool value);
bool item_data_bool(QTableWidgetItem *it, int32_t role);

// ---- 值类型 ----
QColor *color_new_rgb(int32_t r, int32_t g, int32_t b, int32_t a);
QFont *font_new();
void font_set_point_size(QFont *f, int32_t size);
void font_set_bold(QFont *f, bool bold);
QPalette *palette_new();
void palette_set_color(QPalette *pal, int32_t group, int32_t role, QColor *color);
QPixmap *pixmap_new(rust::Str path); // 文件或 qrc 路径
QPixmap *standard_icon_pixmap(QWidget *w, int32_t icon, int32_t size); // QStyle::StandardPixmap
QSize *size_new(int32_t w, int32_t h);
QPoint *point_new(int32_t x, int32_t y);
QRect *rect_new(int32_t x, int32_t y, int32_t w, int32_t h);

// ---- QSocketNotifier ----
QSocketNotifier *socket_notifier_new(int32_t fd); // Read 类型，activated 信号走 relay

// ---- 通用 paint delegate ----
QStyledItemDelegate *rust_delegate_new(size_t paint_cb_id, QObject *parent);

// ---- QPainter 原语 ----
void painter_save(QPainter *p);
void painter_restore(QPainter *p);
void painter_set_pen_color(QPainter *p, QColor *color);
void painter_set_font(QPainter *p, QFont *font);
void painter_draw_text(QPainter *p, int32_t x, int32_t y, int32_t w, int32_t h, int32_t flags, rust::Str text);
void painter_draw_pixmap(QPainter *p, int32_t x, int32_t y, int32_t w, int32_t h, QPixmap *pm);
void painter_draw_icon(QPainter *p, int32_t x, int32_t y, int32_t w, int32_t h, QIcon *icon);
void painter_fill_rect(QPainter *p, int32_t x, int32_t y, int32_t w, int32_t h, QColor *color);

// ---- QModelIndex 数据访问 ----
rust::String index_data_string(QModelIndex *idx, int32_t role);
bool index_data_bool(QModelIndex *idx, int32_t role);
int64_t index_data_i64(QModelIndex *idx, int32_t role);

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
