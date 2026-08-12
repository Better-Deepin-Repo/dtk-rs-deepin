//! DTK6 (dtkwidget) FFI layer. See cpp/shim.cpp for the C++ shim.

use std::cell::RefCell;
use std::collections::HashMap;

pub mod gen_ffi;

#[cxx::bridge(namespace = "dtkrs")]
pub mod ffi {
    extern "C++" {
        include!("dtk_shim.h");

        type QObject;
        type QWidget;
        type QAbstractButton;
        type QLayout;
        type QVBoxLayout;
        type QHBoxLayout;
        type QTableWidget;
        type QTableWidgetItem;
        type QTimer;
        type QIcon;
        type QColor;
        type QFont;
        type QPalette;
        type QPixmap;
        type QMargins;
        type DDciIcon;
        type QPoint;
        type QRect;
        type QSize;
        type QPainter;
        type QModelIndex;
        type QSocketNotifier;
        type QStyledItemDelegate;

        type DApplication;
        type DMainWindow;
        type DTitlebar;
        type DLabel;
        type DSuggestButton;
        type DPushButton;
        type QMessageBox;
        type QMenu;

        // DApplication
        // args: '|'-separated real argv (incl. argv[0]); application_name set separately
        unsafe fn application_new(name: &str, args: &str) -> *mut DApplication;
        unsafe fn application_new_ex(
            name: &str,
            args: &str,
            quit_guard_id: usize,
        ) -> *mut DApplication;
        unsafe fn application_exec(app: *mut DApplication) -> i32;
        unsafe fn application_quit();
        unsafe fn application_set_quit_on_last_window_closed(quit: bool);
        unsafe fn application_set_application_display_name(name: &str);
        unsafe fn application_load_translator(app: *mut DApplication) -> bool;
        unsafe fn application_has_arg(arg: &str) -> bool;

        // QWidget common
        unsafe fn widget_show(w: *mut QWidget);
        unsafe fn widget_set_mouse_tracking(w: *mut QWidget, on: bool);
        unsafe fn widget_hide(w: *mut QWidget);
        unsafe fn widget_resize(w: *mut QWidget, w_px: i32, h_px: i32);
        unsafe fn widget_width(w: *mut QWidget) -> i32;
        unsafe fn widget_height(w: *mut QWidget) -> i32;
        unsafe fn widget_set_enabled(w: *mut QWidget, on: bool);
        unsafe fn widget_set_window_title(w: *mut QWidget, title: &str);
        unsafe fn widget_set_window_icon(w: *mut QWidget, icon: *mut QIcon);
        unsafe fn widget_set_fixed_size(w: *mut QWidget, w_px: i32, h_px: i32);
        unsafe fn widget_raise(w: *mut QWidget);
        unsafe fn widget_update(w: *mut QWidget);
        unsafe fn widget_set_focus(w: *mut QWidget);
        unsafe fn widget_move(w: *mut QWidget, x: i32, y: i32);
        unsafe fn widget_set_parent(child: *mut QWidget, parent: *mut QWidget);
        unsafe fn tabbar_install_style(tb: *mut QWidget);
        unsafe fn tabbar_unlatch_scroll_buttons(tb: *mut QWidget);
        unsafe fn tabbar_flush_layout(tb: *mut QWidget);
        unsafe fn scrollbar_new(parent: *mut QWidget) -> *mut QWidget;
        unsafe fn scrollbar_set_range(sb: *mut QWidget, minimum: i32, maximum: i32);
        unsafe fn scrollbar_maximum(sb: *mut QWidget) -> i32;
        unsafe fn scrollbar_set_value(sb: *mut QWidget, v: i32);
        unsafe fn scrollbar_value(sb: *mut QWidget) -> i32;
        unsafe fn scrollbar_set_page_step(sb: *mut QWidget, v: i32);
        unsafe fn paint_widget_set_ime_rect(w: *mut QWidget, x: i32, y: i32, width: i32, height: i32);
        unsafe fn main_window_titlebar_set_tabbar(w: *mut QWidget, tabbar: *mut QWidget);
        unsafe fn main_window_titlebar_add_widget(w: *mut QWidget, child: *mut QWidget);
        unsafe fn widget_set_titlebar_icon(w: *mut QWidget, icon: *mut QIcon);
        unsafe fn app_popup_active() -> bool;
        unsafe fn app_palette_window_rgb() -> u32;
        unsafe fn widget_activate_window(w: *mut QWidget);
        unsafe fn widget_close(w: *mut QWidget);
        unsafe fn widget_is_visible(w: *mut QWidget) -> bool;
        unsafe fn widget_set_focus_policy(w: *mut QWidget, policy: i32);

        // QProgressBar common
        unsafe fn progressbar_set_value(w: *mut QWidget, value: i32);
        /// QLineEdit base-class getter (DLineEdit etc.); cast is the caller's responsibility
        unsafe fn line_edit_text(w: *mut QWidget) -> String;
        unsafe fn progressbar_set_range(w: *mut QWidget, minimum: i32, maximum: i32);
        unsafe fn progressbar_value(w: *mut QWidget) -> i32;
        unsafe fn widget_set_font(w: *mut QWidget, font: *mut QFont);
        /// Qt::CursorShape (qt::cursor::*)
        unsafe fn widget_set_cursor(w: *mut QWidget, shape: i32);
        unsafe fn widget_unset_cursor(w: *mut QWidget);
        unsafe fn widget_palette(w: *mut QWidget) -> *mut QPalette;
        unsafe fn widget_set_palette(w: *mut QWidget, pal: *mut QPalette);
        unsafe fn object_delete_later(o: *mut QObject);

        // DMainWindow
        unsafe fn mainwindow_new() -> *mut DMainWindow;
        unsafe fn mainwindow_new_ex(show_cb_id: usize, close_cb_id: usize) -> *mut DMainWindow;
        unsafe fn mainwindow_titlebar(w: *mut DMainWindow) -> *mut DTitlebar;
        unsafe fn mainwindow_set_central_widget(w: *mut DMainWindow, central: *mut QWidget);
        unsafe fn mainwindow_take_central_widget(w: *mut DMainWindow) -> *mut QWidget;
        unsafe fn mainwindow_set_window_radius(w: *mut DMainWindow, radius: i32);
        unsafe fn mainwindow_set_enable_blur(w: *mut DMainWindow, enable: bool);

        // DTitlebar
        unsafe fn titlebar_set_title(tb: *mut DTitlebar, title: &str);
        unsafe fn titlebar_set_icon(tb: *mut DTitlebar, icon: &QIcon);

        // QIcon
        unsafe fn icon_from_theme(name: &str) -> *mut QIcon;
        unsafe fn icon_from_theme_fallback(name: &str, fallback: *mut QIcon) -> *mut QIcon;
        unsafe fn icon_from_file(path: &str) -> *mut QIcon;

        // DLabel
        unsafe fn label_new(text: &str) -> *mut DLabel;
        unsafe fn label_set_text(l: *mut DLabel, text: &str);
        unsafe fn label_set_word_wrap(l: *mut DLabel, wrap: bool);
        unsafe fn label_set_alignment(l: *mut DLabel, alignment: i32);
        unsafe fn label_set_pixmap(l: *mut DLabel, pm: *mut QPixmap);

        // buttons
        unsafe fn suggest_button_new(text: &str) -> *mut DSuggestButton;
        unsafe fn push_button_new(text: &str) -> *mut DPushButton;
        unsafe fn button_set_text(b: *mut DPushButton, text: &str);
        unsafe fn button_click(b: *mut DPushButton);

        // QMessageBox (DMessageBox typedef)
        unsafe fn qmessagebox_new() -> *mut QMessageBox;
        unsafe fn qmessagebox_new_with(
            icon: i32,
            title: &str,
            text: &str,
            buttons: i32,
            parent: *mut QWidget,
        ) -> *mut QMessageBox;
        unsafe fn qmessagebox_set_text(mb: *mut QMessageBox, text: &str);
        unsafe fn qmessagebox_set_icon(mb: *mut QMessageBox, icon: i32);
        unsafe fn qmessagebox_set_standard_buttons(mb: *mut QMessageBox, buttons: i32);
        unsafe fn qmessagebox_set_informative_text(mb: *mut QMessageBox, text: &str);
        unsafe fn qmessagebox_set_detailed_text(mb: *mut QMessageBox, text: &str);
        unsafe fn qmessagebox_add_button_text(
            mb: *mut QMessageBox,
            text: &str,
            role: i32,
        ) -> *mut DPushButton;
        unsafe fn qmessagebox_add_button_standard(
            mb: *mut QMessageBox,
            button: i32,
        ) -> *mut DPushButton;
        unsafe fn qmessagebox_set_default_button(mb: *mut QMessageBox, button: i32);
        unsafe fn qmessagebox_exec(mb: *mut QMessageBox) -> i32;
        unsafe fn qmessagebox_clicked_button(mb: *mut QMessageBox) -> i32;
        unsafe fn qmessagebox_text(mb: *mut QMessageBox) -> String;
        // static helpers
        unsafe fn qmessagebox_information(
            parent: *mut QWidget,
            title: &str,
            text: &str,
            buttons: i32,
            default_button: i32,
        ) -> i32;
        unsafe fn qmessagebox_warning(
            parent: *mut QWidget,
            title: &str,
            text: &str,
            buttons: i32,
            default_button: i32,
        ) -> i32;
        unsafe fn qmessagebox_critical(
            parent: *mut QWidget,
            title: &str,
            text: &str,
            buttons: i32,
            default_button: i32,
        ) -> i32;
        unsafe fn qmessagebox_question(
            parent: *mut QWidget,
            title: &str,
            text: &str,
            buttons: i32,
            default_button: i32,
        ) -> i32;
        unsafe fn qmessagebox_about(parent: *mut QWidget, title: &str, text: &str);

        // layouts
        unsafe fn vbox_new(parent: *mut QWidget) -> *mut QVBoxLayout;
        unsafe fn hbox_new(parent: *mut QWidget) -> *mut QHBoxLayout;
        unsafe fn widget_new(parent: *mut QWidget) -> *mut QWidget;
        unsafe fn layout_add_widget(l: *mut QLayout, w: *mut QWidget);
        unsafe fn layout_add_widget_ex(
            l: *mut QLayout,
            w: *mut QWidget,
            stretch: i32,
            alignment: i32,
        );
        unsafe fn layout_add_stretch(l: *mut QLayout, stretch: i32);
        unsafe fn layout_add_layout(l: *mut QLayout, child: *mut QLayout);
        unsafe fn layout_set_spacing(l: *mut QLayout, spacing: i32);
        unsafe fn layout_set_contents_margins(l: *mut QLayout, l_: i32, t: i32, r: i32, b: i32);

        // QTableWidget
        unsafe fn table_new(rows: i32, cols: i32) -> *mut QTableWidget;
        unsafe fn table_set_column_count(t: *mut QTableWidget, cols: i32);
        unsafe fn table_set_row_count(t: *mut QTableWidget, rows: i32);
        unsafe fn table_set_horizontal_header_labels(t: *mut QTableWidget, joined: &str);
        unsafe fn table_set_cell_text(t: *mut QTableWidget, row: i32, col: i32, text: &str);
        unsafe fn table_set_cell_data(t: *mut QTableWidget, row: i32, col: i32, data: i64);
        unsafe fn table_cell_data(t: *mut QTableWidget, row: i32, col: i32) -> i64;
        unsafe fn table_cell_text(t: *mut QTableWidget, row: i32, col: i32) -> String;
        unsafe fn table_current_row(t: *mut QTableWidget) -> i32;
        unsafe fn table_row_count(t: *mut QTableWidget) -> i32;
        unsafe fn table_select_rows_readonly(t: *mut QTableWidget);
        unsafe fn table_header_stretch_last(t: *mut QTableWidget, stretch: bool);
        unsafe fn table_set_column_width(t: *mut QTableWidget, col: i32, width: i32);

        // QTableWidget extras
        unsafe fn table_item(t: *mut QTableWidget, row: i32, col: i32) -> *mut QTableWidgetItem;
        unsafe fn table_select_row(t: *mut QTableWidget, row: i32);
        unsafe fn table_hide_headers(t: *mut QTableWidget, horizontal: bool, vertical: bool);
        unsafe fn table_set_section_resize_mode(t: *mut QTableWidget, col: i32, mode: i32);
        unsafe fn table_set_vertical_header_default_section_size(t: *mut QTableWidget, size: i32);
        unsafe fn table_set_show_grid(t: *mut QTableWidget, show: bool);
        unsafe fn table_set_frame_shape(t: *mut QTableWidget, shape: i32);
        unsafe fn table_set_icon_size(t: *mut QTableWidget, w: i32, h: i32);
        unsafe fn table_set_delegate_for_column(
            t: *mut QTableWidget,
            col: i32,
            delegate: *mut QStyledItemDelegate,
        );

        // QTableWidgetItem
        unsafe fn item_set_icon(it: *mut QTableWidgetItem, icon: *mut QIcon);
        unsafe fn item_set_text_alignment(it: *mut QTableWidgetItem, alignment: i32);
        unsafe fn item_set_foreground(it: *mut QTableWidgetItem, color: *mut QColor);
        unsafe fn item_set_data_string(it: *mut QTableWidgetItem, role: i32, value: &str);
        unsafe fn item_data_string(it: *mut QTableWidgetItem, role: i32) -> String;
        unsafe fn item_set_data_bool(it: *mut QTableWidgetItem, role: i32, value: bool);
        unsafe fn item_data_bool(it: *mut QTableWidgetItem, role: i32) -> bool;

        // value types
        unsafe fn color_new_rgb(r: i32, g: i32, b: i32, a: i32) -> *mut QColor;
        unsafe fn color_rgba(c: *mut QColor) -> i32;
        unsafe fn color_delete(c: *mut QColor);
        unsafe fn font_new() -> *mut QFont;
        unsafe fn font_set_point_size(f: *mut QFont, size: i32);
        unsafe fn font_set_bold(f: *mut QFont, bold: bool);
        unsafe fn font_set_italic(f: *mut QFont, italic: bool);
        unsafe fn fontmetrics_height(f: *mut QFont) -> i32;
        unsafe fn fontmetrics_ascent(f: *mut QFont) -> i32;
        unsafe fn fontmetrics_max_width(f: *mut QFont) -> i32;
        unsafe fn fontmetrics_advance(f: *mut QFont, text: &str) -> i32;
        unsafe fn font_force_integer_metrics(f: *mut QFont);
        unsafe fn font_set_monospace(f: *mut QFont);
        unsafe fn font_set_family(f: *mut QFont, name: &str);
        unsafe fn font_delete(f: *mut QFont);
        unsafe fn palette_new() -> *mut QPalette;
        unsafe fn palette_set_color(pal: *mut QPalette, group: i32, role: i32, color: *mut QColor);
        unsafe fn palette_color(pal: *mut QPalette, group: i32, role: i32) -> *mut QColor;
        unsafe fn palette_delete(pal: *mut QPalette);
        unsafe fn pixmap_new(path: &str) -> *mut QPixmap;
        unsafe fn pixmap_delete(pm: *mut QPixmap);
        // QMargins
        unsafe fn q_margins_new(left: i32, top: i32, right: i32, bottom: i32) -> *mut QMargins;
        unsafe fn margins_delete(m: *mut QMargins);
        // DDciIcon (dtkgui)
        unsafe fn ddci_icon_new() -> *mut DDciIcon;
        unsafe fn ddci_icon_from_file(path: &str) -> *mut DDciIcon;
        unsafe fn ddci_icon_delete(i: *mut DDciIcon);
        unsafe fn standard_icon_pixmap(w: *mut QWidget, icon: i32, size: i32) -> *mut QPixmap;
        unsafe fn size_new(w: i32, h: i32) -> *mut QSize;
        unsafe fn size_delete(s: *mut QSize);
        unsafe fn point_new(x: i32, y: i32) -> *mut QPoint;
        unsafe fn point_delete(p: *mut QPoint);
        unsafe fn rect_new(x: i32, y: i32, w: i32, h: i32) -> *mut QRect;
        unsafe fn rect_delete(r: *mut QRect);
        unsafe fn rect_width(r: *mut QRect) -> i32;
        unsafe fn rect_height(r: *mut QRect) -> i32;
        unsafe fn rect_x(r: *mut QRect) -> i32;
        unsafe fn rect_y(r: *mut QRect) -> i32;
        unsafe fn icon_delete(icon: *mut QIcon);

        // QSocketNotifier
        unsafe fn socket_notifier_new(fd: i32) -> *mut QSocketNotifier;
        unsafe fn socket_notifier_set_enabled(n: *mut QSocketNotifier, on: bool);

        // generic paint delegate
        unsafe fn rust_delegate_new(
            paint_cb_id: usize,
            parent: *mut QObject,
        ) -> *mut QStyledItemDelegate;

        // QPainter primitives
        unsafe fn painter_draw_text_at(p: *mut QPainter, x: i32, y: i32, text: &str);
        unsafe fn painter_draw_line(p: *mut QPainter, x1: i32, y1: i32, x2: i32, y2: i32);
        unsafe fn painter_save(p: *mut QPainter);
        unsafe fn painter_restore(p: *mut QPainter);
        unsafe fn painter_set_pen_color(p: *mut QPainter, color: *mut QColor);
        unsafe fn painter_set_font(p: *mut QPainter, font: *mut QFont);
        unsafe fn painter_draw_text(
            p: *mut QPainter,
            x: i32,
            y: i32,
            w: i32,
            h: i32,
            flags: i32,
            text: &str,
        );
        unsafe fn painter_draw_pixmap(
            p: *mut QPainter,
            x: i32,
            y: i32,
            w: i32,
            h: i32,
            pm: *mut QPixmap,
        );
        unsafe fn painter_draw_icon(
            p: *mut QPainter,
            x: i32,
            y: i32,
            w: i32,
            h: i32,
            icon: *mut QIcon,
        );
        unsafe fn painter_fill_rect(
            p: *mut QPainter,
            x: i32,
            y: i32,
            w: i32,
            h: i32,
            color: *mut QColor,
        );
        unsafe fn painter_set_clip_rect(p: *mut QPainter, x: i32, y: i32, w: i32, h: i32);
        unsafe fn painter_elided_text(
            p: *mut QPainter,
            text: &str,
            mode: i32,
            width: i32,
        ) -> String;

        // QModelIndex data access
        unsafe fn index_data_string(idx: *mut QModelIndex, role: i32) -> String;
        unsafe fn index_data_bool(idx: *mut QModelIndex, role: i32) -> bool;
        unsafe fn index_data_i64(idx: *mut QModelIndex, role: i32) -> i64;

        // QTimer
        unsafe fn timer_new(parent: *mut QObject) -> *mut QTimer;
        unsafe fn timer_start(t: *mut QTimer, msec: i32);
        unsafe fn timer_stop(t: *mut QTimer);
        unsafe fn timer_single_shot(msec: i32, cb_id: usize);

        // QMenu (DMenu typedef in DTK6); actions fire cb_id on triggered
        unsafe fn menu_new(parent: *mut QWidget) -> *mut QMenu;
        unsafe fn menu_add_action_cb(m: *mut QMenu, text: &str, cb_id: usize);
        unsafe fn menu_add_separator(m: *mut QMenu);
        /// popup at (x, y) in ref widget coords; menu self-deletes on close (WA_DeleteOnClose)
        unsafe fn menu_popup(m: *mut QMenu, ref_: *mut QWidget, x: i32, y: i32);

        // signals; connect fns return false on failure (caller must roll back registration)
        unsafe fn relay_connect0(sender: *mut QObject, signal: &str, cb_id: usize) -> bool;
        unsafe fn relay_connect_i32(sender: *mut QObject, signal: &str, cb_id: usize) -> bool;
        unsafe fn relay_connect_bool(sender: *mut QObject, signal: &str, cb_id: usize) -> bool;
        unsafe fn relay_connect_i32_i32(sender: *mut QObject, signal: &str, cb_id: usize) -> bool;
        /// disconnect + schedule deletion of the relay for cb_id (no-op if unknown)
        unsafe fn relay_disconnect(cb_id: usize);

        // user-drawn widget + clipboard + shortcuts
        unsafe fn paint_widget_new(cb_id: usize, parent: *mut QWidget) -> *mut QWidget;
        unsafe fn paint_widget_inject_key(w: *mut QWidget, key: i32, mods: i32, text: &str);
        unsafe fn clipboard_set_text(text: &str, mode: i32);
        unsafe fn clipboard_text(mode: i32) -> String;
        unsafe fn shortcut_new(parent: *mut QWidget, key: &str, cb_id: usize);
    }

    extern "Rust" {
        fn dtk_cb0(id: usize);
        fn dtk_cb_i32(id: usize, v: i32);
        fn dtk_cb_bool(id: usize, v: bool);
        fn dtk_cb_i32_i32(id: usize, a: i32, b: i32);
        fn dtk_cb_guard(id: usize) -> bool;
        // DtkPaintWidget event callbacks
        unsafe fn dtk_cb_pw_paint(id: usize, painter: *mut QPainter, w: i32, h: i32);
        fn dtk_cb_pw_key(
            id: usize,
            key: i32,
            mods: i32,
            text: String,
            press: bool,
            autorepeat: bool,
        );
        fn dtk_cb_pw_mouse(id: usize, kind: i32, button: i32, x: i32, y: i32, mods: i32);
        fn dtk_cb_pw_wheel(id: usize, dy: i32, x: i32, y: i32, mods: i32);
        fn dtk_cb_pw_ime(id: usize, commit: String, preedit: String);
        fn dtk_cb_pw_resize(id: usize, w: i32, h: i32);
        fn dtk_cb_pw_focus(id: usize, focus_in: bool);
        unsafe fn dtk_cb_paint(
            id: usize,
            painter: *mut QPainter,
            index: *mut QModelIndex,
            x: i32,
            y: i32,
            w: i32,
            h: i32,
            state: i32,
        );
    }
}

// ---- callback registry: Qt signals -> Rust closures ----
// ponytail: callbacks only fire on the Qt main thread; thread_local is enough, no locks
/// events delivered to a PaintWidget callback (QPainter valid only during Paint)
pub enum PwEvent {
    Paint(*mut ffi::QPainter, i32, i32),
    Key {
        key: i32,
        mods: i32,
        text: String,
        press: bool,
        autorepeat: bool,
    },
    Mouse {
        kind: i32,
        button: i32,
        x: i32,
        y: i32,
        mods: i32,
    },
    Wheel {
        dy: i32,
        x: i32,
        y: i32,
        mods: i32,
    },
    Ime {
        commit: String,
        preedit: String,
    },
    Resize {
        w: i32,
        h: i32,
    },
    Focus(bool),
}

enum Cb {
    C0(Box<dyn FnMut()>),
    I32(Box<dyn FnMut(i32)>),
    I32I32(Box<dyn FnMut(i32, i32)>),
    Bool(Box<dyn FnMut(bool)>),
    Guard(Box<dyn FnMut() -> bool>),
    Paint(Box<dyn FnMut(*mut ffi::QPainter, *mut ffi::QModelIndex, i32, i32, i32, i32, i32)>),
    Pw(Box<dyn FnMut(PwEvent)>),
}

thread_local! {
    static CALLBACKS: RefCell<HashMap<usize, Cb>> = RefCell::new(HashMap::new());
    // ids unregistered from inside their own dispatch; dispatch must not resurrect them
    static TOMBSTONES: RefCell<std::collections::HashSet<usize>> =
        RefCell::new(std::collections::HashSet::new());
}

fn next_id() -> usize {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

pub fn register_cb0(f: impl FnMut() + 'static) -> usize {
    let id = next_id();
    CALLBACKS.with(|c| c.borrow_mut().insert(id, Cb::C0(Box::new(f))));
    id
}

pub fn register_cb_i32(f: impl FnMut(i32) + 'static) -> usize {
    let id = next_id();
    CALLBACKS.with(|c| c.borrow_mut().insert(id, Cb::I32(Box::new(f))));
    id
}

pub fn register_cb_bool(f: impl FnMut(bool) + 'static) -> usize {
    let id = next_id();
    CALLBACKS.with(|c| c.borrow_mut().insert(id, Cb::Bool(Box::new(f))));
    id
}

pub fn register_cb_i32_i32(f: impl FnMut(i32, i32) + 'static) -> usize {
    let id = next_id();
    CALLBACKS.with(|c| c.borrow_mut().insert(id, Cb::I32I32(Box::new(f))));
    id
}

pub fn register_cb_pw(f: impl FnMut(PwEvent) + 'static) -> usize {
    let id = next_id();
    CALLBACKS.with(|c| c.borrow_mut().insert(id, Cb::Pw(Box::new(f))));
    id
}

pub fn register_cb_guard(f: impl FnMut() -> bool + 'static) -> usize {
    let id = next_id();
    CALLBACKS.with(|c| c.borrow_mut().insert(id, Cb::Guard(Box::new(f))));
    id
}

pub fn register_cb_paint(
    f: impl FnMut(*mut ffi::QPainter, *mut ffi::QModelIndex, i32, i32, i32, i32, i32) + 'static,
) -> usize {
    let id = next_id();
    CALLBACKS.with(|c| c.borrow_mut().insert(id, Cb::Paint(Box::new(f))));
    id
}

/// Remove a callback from the registry and disconnect its Qt-side relay.
/// Returns false if the id is unknown. Safe to call from inside the callback itself.
pub fn unregister_cb(id: usize) -> bool {
    let removed = CALLBACKS.with(|c| c.borrow_mut().remove(&id)).is_some();
    if !removed {
        // may be mid-dispatch (callback unregisters itself): block resurrection
        TOMBSTONES.with(|t| t.borrow_mut().insert(id));
    }
    // no-op for ids without a relay (e.g. timer callbacks)
    unsafe { ffi::relay_disconnect(id) };
    removed
}

/// Reinsert after dispatch unless the callback unregistered itself mid-call.
fn finish_dispatch(id: usize, cb: Cb) {
    let tombstoned = TOMBSTONES.with(|t| t.borrow_mut().remove(&id));
    if !tombstoned {
        CALLBACKS.with(|c| c.borrow_mut().insert(id, cb));
    }
}

// remove-then-call so callbacks can safely (un)register callbacks; put back after.
// catch_unwind: a panicking Rust callback must never unwind across the FFI boundary (UB).
fn dtk_cb0(id: usize) {
    let mut cb = CALLBACKS.with(|c| c.borrow_mut().remove(&id));
    if let Some(Cb::C0(f)) = &mut cb {
        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(&mut *f)).is_err() {
            eprintln!("dtk-rs: callback {id} panicked, swallowed at FFI boundary");
        }
        finish_dispatch(id, cb.take().unwrap());
    }
}

fn dtk_cb_i32(id: usize, v: i32) {
    let mut cb = CALLBACKS.with(|c| c.borrow_mut().remove(&id));
    if let Some(Cb::I32(f)) = &mut cb {
        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(v))).is_err() {
            eprintln!("dtk-rs: callback {id} panicked, swallowed at FFI boundary");
        }
        finish_dispatch(id, cb.take().unwrap());
    }
}

fn dispatch_pw(id: usize, ev: PwEvent) {
    let mut cb = CALLBACKS.with(|c| c.borrow_mut().remove(&id));
    if let Some(Cb::Pw(f)) = &mut cb {
        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(ev))).is_err() {
            eprintln!("dtk-rs: paint-widget callback {id} panicked, swallowed at FFI boundary");
        }
        finish_dispatch(id, cb.take().unwrap());
    }
}

unsafe fn dtk_cb_pw_paint(id: usize, painter: *mut ffi::QPainter, w: i32, h: i32) {
    dispatch_pw(id, PwEvent::Paint(painter, w, h));
}

fn dtk_cb_pw_key(id: usize, key: i32, mods: i32, text: String, press: bool, autorepeat: bool) {
    dispatch_pw(
        id,
        PwEvent::Key {
            key,
            mods,
            text,
            press,
            autorepeat,
        },
    );
}

fn dtk_cb_pw_mouse(id: usize, kind: i32, button: i32, x: i32, y: i32, mods: i32) {
    dispatch_pw(
        id,
        PwEvent::Mouse {
            kind,
            button,
            x,
            y,
            mods,
        },
    );
}

fn dtk_cb_pw_wheel(id: usize, dy: i32, x: i32, y: i32, mods: i32) {
    dispatch_pw(id, PwEvent::Wheel { dy, x, y, mods });
}

fn dtk_cb_pw_ime(id: usize, commit: String, preedit: String) {
    dispatch_pw(id, PwEvent::Ime { commit, preedit });
}

fn dtk_cb_pw_resize(id: usize, w: i32, h: i32) {
    dispatch_pw(id, PwEvent::Resize { w, h });
}

fn dtk_cb_pw_focus(id: usize, focus_in: bool) {
    dispatch_pw(id, PwEvent::Focus(focus_in));
}

fn dtk_cb_bool(id: usize, v: bool) {
    let mut cb = CALLBACKS.with(|c| c.borrow_mut().remove(&id));
    if let Some(Cb::Bool(f)) = &mut cb {
        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(v))).is_err() {
            eprintln!("dtk-rs: callback {id} panicked, swallowed at FFI boundary");
        }
        finish_dispatch(id, cb.take().unwrap());
    }
}

fn dtk_cb_i32_i32(id: usize, a: i32, b: i32) {
    let mut cb = CALLBACKS.with(|c| c.borrow_mut().remove(&id));
    if let Some(Cb::I32I32(f)) = &mut cb {
        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(a, b))).is_err() {
            eprintln!("dtk-rs: callback {id} panicked, swallowed at FFI boundary");
        }
        finish_dispatch(id, cb.take().unwrap());
    }
}

fn dtk_cb_guard(id: usize) -> bool {
    let mut cb = CALLBACKS.with(|c| c.borrow_mut().remove(&id));
    let Some(Cb::Guard(f)) = &mut cb else {
        eprintln!("dtk-rs: guard callback {id} missing, defaulting to allow");
        return true;
    };
    let result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(&mut *f)) {
        Ok(r) => r,
        Err(_) => {
            eprintln!("dtk-rs: guard callback {id} panicked, defaulting to allow");
            true
        }
    };
    finish_dispatch(id, cb.take().unwrap());
    result
}

unsafe fn dtk_cb_paint(
    id: usize,
    painter: *mut ffi::QPainter,
    index: *mut ffi::QModelIndex,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    state: i32,
) {
    let mut cb = CALLBACKS.with(|c| c.borrow_mut().remove(&id));
    if let Some(Cb::Paint(f)) = &mut cb {
        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            f(painter, index, x, y, w, h, state)
        }))
        .is_err()
        {
            eprintln!("dtk-rs: paint callback {id} panicked, swallowed at FFI boundary");
        }
        finish_dispatch(id, cb.take().unwrap());
    }
}
