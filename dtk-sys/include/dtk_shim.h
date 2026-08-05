// dtk-rs C++ shim: flattens the DTK6/Qt6 C++ API into free functions for cxx::bridge.
// All objects are lifetime-managed by Qt parent-child; the Rust side only holds raw pointers.
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
#include <QMargins>
#include <DDciIcon>
#include <QPoint>
#include <QRect>
#include <QSize>
#include <QPainter>
#include <QStyledItemDelegate>
#include <QModelIndex>
#include <QSocketNotifier>
#include <QMessageBox>
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

// cxx requires opaque types to live in the bridge namespace; alias the Qt classes in too
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
using ::QMargins;
using Dtk::Gui::DDciIcon;
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
using QMessageBox = ::QMessageBox;

// ---- QString <-> rust string ----
rust::String to_rust_string(const QString &s);
QString from_rust_str(rust::Str s);

// ---- DApplication ----
// args: '|'-separated argv (incl. argv[0]); ponytail: '|' can't appear in normal flags like --hidden
DApplication *application_new(rust::Str name, rust::Str args);
DApplication *application_new_ex(rust::Str name, rust::Str args, size_t quit_guard_id); // guard returning false swallows the Quit event
int32_t application_exec(DApplication *app);
void application_quit();
void application_set_quit_on_last_window_closed(bool quit);
void application_set_application_display_name(rust::Str name);
bool application_load_translator(DApplication *app);
bool application_has_arg(rust::Str arg);

// ---- QWidget common (applies to all widgets) ----
void widget_show(QWidget *w);
void widget_resize(QWidget *w, int32_t w_px, int32_t h_px);
void widget_set_enabled(QWidget *w, bool on);
void widget_set_window_title(QWidget *w, rust::Str title);
void widget_set_window_icon(QWidget *w, QIcon *icon);
void widget_set_fixed_size(QWidget *w, int32_t w_px, int32_t h_px);
void widget_raise(QWidget *w);
void widget_activate_window(QWidget *w);
void widget_close(QWidget *w);
bool widget_is_visible(QWidget *w);
void widget_set_focus_policy(QWidget *w, int32_t policy);
void widget_set_font(QWidget *w, QFont *font);
QPalette *widget_palette(QWidget *w); // heap copy, owned by the caller
void widget_set_palette(QWidget *w, QPalette *pal);
void object_delete_later(QObject *o);

// ---- DMainWindow ----
DMainWindow *mainwindow_new();
DMainWindow *mainwindow_new_ex(size_t show_cb_id, size_t close_cb_id); // close callback returning false -> ignore
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
QIcon *icon_from_theme_fallback(rust::Str name, QIcon *fallback);
QIcon *icon_from_file(rust::Str path);

// ---- DLabel ----
DLabel *label_new(rust::Str text);
void label_set_text(DLabel *l, rust::Str text);
void label_set_word_wrap(DLabel *l, bool wrap);
void label_set_alignment(DLabel *l, int32_t alignment); // Qt::Alignment
void label_set_pixmap(DLabel *l, QPixmap *pm);

// ---- buttons ----
DSuggestButton *suggest_button_new(rust::Str text);
DPushButton *push_button_new(rust::Str text);
void button_set_text(DPushButton *b, rust::Str text);
void button_click(DPushButton *b);

// ---- QMessageBox (DMessageBox typedef = QMessageBox) ----
QMessageBox *qmessagebox_new();
QMessageBox *qmessagebox_new_with(int32_t icon, rust::Str title, rust::Str text, int32_t buttons, QWidget *parent);
void qmessagebox_set_text(QMessageBox *mb, rust::Str text);
void qmessagebox_set_icon(QMessageBox *mb, int32_t icon);
void qmessagebox_set_standard_buttons(QMessageBox *mb, int32_t buttons);
void qmessagebox_set_informative_text(QMessageBox *mb, rust::Str text);
void qmessagebox_set_detailed_text(QMessageBox *mb, rust::Str text);
DPushButton *qmessagebox_add_button_text(QMessageBox *mb, rust::Str text, int32_t role);
DPushButton *qmessagebox_add_button_standard(QMessageBox *mb, int32_t button);
void qmessagebox_set_default_button(QMessageBox *mb, int32_t button);
int32_t qmessagebox_exec(QMessageBox *mb);
int32_t qmessagebox_clicked_button(QMessageBox *mb);
rust::String qmessagebox_text(QMessageBox *mb);
// static helpers (return clicked StandardButton as i32)
int32_t qmessagebox_information(QWidget *parent, rust::Str title, rust::Str text, int32_t buttons, int32_t default_button);
int32_t qmessagebox_warning(QWidget *parent, rust::Str title, rust::Str text, int32_t buttons, int32_t default_button);
int32_t qmessagebox_critical(QWidget *parent, rust::Str title, rust::Str text, int32_t buttons, int32_t default_button);
int32_t qmessagebox_question(QWidget *parent, rust::Str title, rust::Str text, int32_t buttons, int32_t default_button);
void qmessagebox_about(QWidget *parent, rust::Str title, rust::Str text);

// ---- layouts ----
QVBoxLayout *vbox_new(QWidget *parent);
QHBoxLayout *hbox_new(QWidget *parent);
QWidget *widget_new(QWidget *parent);
void layout_add_widget(QLayout *l, QWidget *w);
void layout_add_widget_ex(QLayout *l, QWidget *w, int32_t stretch, int32_t alignment); // box layouts only
void layout_add_stretch(QLayout *l, int32_t stretch); // box layouts only
void layout_add_layout(QLayout *l, QLayout *child);
void layout_set_spacing(QLayout *l, int32_t spacing);
void layout_set_contents_margins(QLayout *l, int32_t l_, int32_t t, int32_t r, int32_t b);

// ---- QTableWidget ----
QTableWidget *table_new(int32_t rows, int32_t cols);
void table_set_column_count(QTableWidget *t, int32_t cols);
void table_set_row_count(QTableWidget *t, int32_t rows);
void table_set_horizontal_header_labels(QTableWidget *t, rust::Str joined); // '|'-separated
void table_set_cell_text(QTableWidget *t, int32_t row, int32_t col, rust::Str text);
void table_set_cell_data(QTableWidget *t, int32_t row, int32_t col, int64_t data);
int64_t table_cell_data(QTableWidget *t, int32_t row, int32_t col);
rust::String table_cell_text(QTableWidget *t, int32_t row, int32_t col);
int32_t table_current_row(QTableWidget *t);
int32_t table_row_count(QTableWidget *t);
void table_select_rows_readonly(QTableWidget *t);
void table_header_stretch_last(QTableWidget *t, bool stretch);
void table_set_column_width(QTableWidget *t, int32_t col, int32_t width);

// ---- QTableWidget extras ----
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

// ---- value types ----
QColor *color_new_rgb(int32_t r, int32_t g, int32_t b, int32_t a);
int32_t color_rgba(QColor *c); // packed 0xAARRGGBB
void color_delete(QColor *c);
QFont *font_new();
void font_set_point_size(QFont *f, int32_t size);
void font_set_bold(QFont *f, bool bold);
void font_delete(QFont *f);
QPalette *palette_new();
void palette_set_color(QPalette *pal, int32_t group, int32_t role, QColor *color);
QColor *palette_color(QPalette *pal, int32_t group, int32_t role); // heap copy, owned by the caller
void palette_delete(QPalette *pal);
QPixmap *pixmap_new(rust::Str path); // file or qrc path
void pixmap_delete(QPixmap *pm);
QPixmap *standard_icon_pixmap(QWidget *w, int32_t icon, int32_t size); // QStyle::StandardPixmap
QSize *size_new(int32_t w, int32_t h);
void size_delete(QSize *s);
QPoint *point_new(int32_t x, int32_t y);
void point_delete(QPoint *p);
QRect *rect_new(int32_t x, int32_t y, int32_t w, int32_t h);
void rect_delete(QRect *r);
void icon_delete(QIcon *icon);

// ---- QMargins ----
QMargins *q_margins_new(int32_t left, int32_t top, int32_t right, int32_t bottom);

// ---- DDciIcon (dtkgui) ----
DDciIcon *ddci_icon_new();
DDciIcon *ddci_icon_from_file(rust::Str path);

// ---- QSocketNotifier ----
QSocketNotifier *socket_notifier_new(int32_t fd); // Read type; activated goes through the relay

// ---- generic paint delegate ----
QStyledItemDelegate *rust_delegate_new(size_t paint_cb_id, QObject *parent);

// ---- QPainter primitives ----
void painter_save(QPainter *p);
void painter_restore(QPainter *p);
void painter_set_pen_color(QPainter *p, QColor *color);
void painter_set_font(QPainter *p, QFont *font);
void painter_draw_text(QPainter *p, int32_t x, int32_t y, int32_t w, int32_t h, int32_t flags, rust::Str text);
void painter_draw_pixmap(QPainter *p, int32_t x, int32_t y, int32_t w, int32_t h, QPixmap *pm);
void painter_draw_icon(QPainter *p, int32_t x, int32_t y, int32_t w, int32_t h, QIcon *icon);
void painter_fill_rect(QPainter *p, int32_t x, int32_t y, int32_t w, int32_t h, QColor *color);
void painter_set_clip_rect(QPainter *p, int32_t x, int32_t y, int32_t w, int32_t h);
rust::String painter_elided_text(QPainter *p, rust::Str text, int32_t mode, int32_t width); // Qt::TextElideMode

// ---- QModelIndex data access ----
rust::String index_data_string(QModelIndex *idx, int32_t role);
bool index_data_bool(QModelIndex *idx, int32_t role);
int64_t index_data_i64(QModelIndex *idx, int32_t role);

// ---- QTimer ----
QTimer *timer_new(QObject *parent);
void timer_start(QTimer *t, int32_t msec);
void timer_stop(QTimer *t);
void timer_single_shot(int32_t msec, size_t cb_id);

// ---- signal callbacks ----
// generic: runtime-connect by signal name, ignoring args. signal looks like "clicked()" or "clicked(bool)"
void relay_connect0(QObject *sender, rust::Str signal, size_t cb_id);
// common signals with args
void relay_connect_i32(QObject *sender, rust::Str signal, size_t cb_id);

} // namespace dtkrs
