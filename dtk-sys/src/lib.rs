//! DTK6 (dtkwidget) FFI 层。C++ shim 见 cpp/shim.cpp。

use std::cell::RefCell;
use std::collections::HashMap;

pub mod gen_ffi;

#[cxx::bridge(namespace = "dtkrs")]
pub mod ffi {
    extern "C++" {
        include!("dtk_shim.h");

        type QObject;
        type QWidget;
        type QLayout;
        type QVBoxLayout;
        type QHBoxLayout;
        type QTableWidget;
        type QTimer;
        type QIcon;

        type DApplication;
        type DMainWindow;
        type DTitlebar;
        type DLabel;
        type DSuggestButton;
        type DPushButton;

        // DApplication
        unsafe fn application_new(name: &str) -> *mut DApplication;
        unsafe fn application_exec(app: *mut DApplication) -> i32;
        unsafe fn application_quit();

        // QWidget 通用
        unsafe fn widget_show(w: *mut QWidget);
        unsafe fn widget_resize(w: *mut QWidget, w_px: i32, h_px: i32);
        unsafe fn widget_set_enabled(w: *mut QWidget, on: bool);
        unsafe fn widget_set_window_title(w: *mut QWidget, title: &str);

        // DMainWindow
        unsafe fn mainwindow_new() -> *mut DMainWindow;
        unsafe fn mainwindow_titlebar(w: *mut DMainWindow) -> *mut DTitlebar;
        unsafe fn mainwindow_set_central_widget(w: *mut DMainWindow, central: *mut QWidget);
        unsafe fn mainwindow_set_window_radius(w: *mut DMainWindow, radius: i32);
        unsafe fn mainwindow_set_enable_blur(w: *mut DMainWindow, enable: bool);

        // DTitlebar
        unsafe fn titlebar_set_title(tb: *mut DTitlebar, title: &str);
        unsafe fn titlebar_set_icon(tb: *mut DTitlebar, icon: &QIcon);

        // QIcon
        unsafe fn icon_from_theme(name: &str) -> *mut QIcon;
        unsafe fn icon_from_file(path: &str) -> *mut QIcon;

        // DLabel
        unsafe fn label_new(text: &str) -> *mut DLabel;
        unsafe fn label_set_text(l: *mut DLabel, text: &str);

        // 按钮
        unsafe fn suggest_button_new(text: &str) -> *mut DSuggestButton;
        unsafe fn push_button_new(text: &str) -> *mut DPushButton;
        unsafe fn button_set_text(b: *mut DPushButton, text: &str);
        unsafe fn button_click(b: *mut DPushButton);

        // 布局
        unsafe fn vbox_new(parent: *mut QWidget) -> *mut QVBoxLayout;
        unsafe fn hbox_new(parent: *mut QWidget) -> *mut QHBoxLayout;
        unsafe fn widget_new(parent: *mut QWidget) -> *mut QWidget;
        unsafe fn layout_add_widget(l: *mut QLayout, w: *mut QWidget);
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

        // QTimer
        unsafe fn timer_new(parent: *mut QObject) -> *mut QTimer;
        unsafe fn timer_start(t: *mut QTimer, msec: i32);
        unsafe fn timer_stop(t: *mut QTimer);
        unsafe fn timer_single_shot(msec: i32, cb_id: usize);

        // 信号
        unsafe fn relay_connect0(sender: *mut QObject, signal: &str, cb_id: usize);
        unsafe fn relay_connect_i32(sender: *mut QObject, signal: &str, cb_id: usize);
    }

    extern "Rust" {
        fn dtk_cb0(id: usize);
        fn dtk_cb_i32(id: usize, v: i32);
    }
}

// ---- 回调注册表：Qt 信号 → Rust 闭包 ----
// ponytail: 回调只会被 Qt 主线程触发，thread_local 够用，不搞锁
enum Cb {
    C0(Box<dyn FnMut()>),
    I32(Box<dyn FnMut(i32)>),
}

thread_local! {
    static CALLBACKS: RefCell<HashMap<usize, Cb>> = RefCell::new(HashMap::new());
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

// 取出再调用，回调里可以安全地再注册新回调；调用完放回
fn dtk_cb0(id: usize) {
    let mut cb = CALLBACKS.with(|c| c.borrow_mut().remove(&id));
    if let Some(Cb::C0(f)) = &mut cb {
        f();
        CALLBACKS.with(|c| c.borrow_mut().insert(id, cb.take().unwrap()));
    }
}

fn dtk_cb_i32(id: usize, v: i32) {
    let mut cb = CALLBACKS.with(|c| c.borrow_mut().remove(&id));
    if let Some(Cb::I32(f)) = &mut cb {
        f(v);
        CALLBACKS.with(|c| c.borrow_mut().insert(id, cb.take().unwrap()));
    }
}
