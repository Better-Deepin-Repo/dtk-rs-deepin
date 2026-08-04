//! DTK6 (dtkwidget) 安全 Rust 绑定。
//!
//! 生命周期：所有 Qt 对象由 Qt parent-child 机制释放，Rust wrapper 只是非拥有裸指针。
//! 顶层窗口（无 parent）退出时随 QApplication 销毁。
//! ponytail: 线程安全不做（Qt GUI 本就单线程），wrapper 一律 !Send。

use dtk_sys::ffi;
use std::marker::PhantomData;

macro_rules! widget_wrapper {
    ($name:ident, $ffi:ty) => {
        object_wrapper!($name, $ffi);
        impl $name {
            /// 当 QWidget* 用（基类操作/塞进布局）
            pub fn as_widget(&self) -> QWidget {
                QWidget::from_raw(self.ptr.cast())
            }
            pub fn show(&self) {
                unsafe { ffi::widget_show(self.ptr.cast()) }
            }
            pub fn resize(&self, w: i32, h: i32) {
                unsafe { ffi::widget_resize(self.ptr.cast(), w, h) }
            }
            pub fn set_enabled(&self, on: bool) {
                unsafe { ffi::widget_set_enabled(self.ptr.cast(), on) }
            }
            pub fn set_window_title(&self, title: &str) {
                unsafe { ffi::widget_set_window_title(self.ptr.cast(), title) }
            }
        }
    };
}

macro_rules! object_wrapper {
    ($name:ident, $ffi:ty) => {
        pub struct $name {
            pub(crate) ptr: *mut $ffi,
            _not_send: PhantomData<*mut ()>,
        }
        impl $name {
            #[allow(dead_code)] // 生成器产物中并非每个类都会被构造
            pub(crate) fn from_raw(ptr: *mut $ffi) -> Self {
                assert!(!ptr.is_null());
                Self { ptr, _not_send: PhantomData }
            }
            /// 当 QObject* 用（信号）
            #[allow(dead_code)] // 部分类没接信号，留着给生成器用
            pub(crate) fn as_qobject(&self) -> *mut ffi::QObject {
                self.ptr.cast()
            }
        }
        impl Signal0 for $name {
            fn qobject_ptr(&self) -> *mut ffi::QObject {
                self.as_qobject()
            }
        }
        impl SignalI32 for $name {
            fn qobject_ptr(&self) -> *mut ffi::QObject {
                self.as_qobject()
            }
        }
    };
}

/// 通用 QWidget 句柄（只用作基类视图）
pub struct QWidget {
    pub(crate) ptr: *mut ffi::QWidget,
    _not_send: PhantomData<*mut ()>,
}

impl QWidget {
    pub(crate) fn from_raw(ptr: *mut ffi::QWidget) -> Self {
        Self { ptr, _not_send: PhantomData }
    }
    pub fn new(parent: Option<&QWidget>) -> Self {
        let p = parent.map_or(std::ptr::null_mut(), |p| p.ptr);
        Self::from_raw(unsafe { ffi::widget_new(p) })
    }
    pub fn show(&self) {
        unsafe { ffi::widget_show(self.ptr) }
    }
}

/// 给任意控件接一个无参信号（如 clicked、timeout）
pub trait Signal0 {
    fn qobject_ptr(&self) -> *mut ffi::QObject;
    /// signal 形如 "clicked()" / "windowRadiusChanged()"
    fn connect_signal(&self, signal: &str, f: impl FnMut() + 'static) {
        let id = dtk_sys::register_cb0(f);
        unsafe { ffi::relay_connect0(self.qobject_ptr(), signal, id) }
    }
}

/// 带一个 i32 参数的信号（如 currentRowChanged(int)）
pub trait SignalI32 {
    fn qobject_ptr(&self) -> *mut ffi::QObject;
    fn connect_signal_i32(&self, signal: &str, f: impl FnMut(i32) + 'static) {
        let id = dtk_sys::register_cb_i32(f);
        unsafe { ffi::relay_connect_i32(self.qobject_ptr(), signal, id) }
    }
}

// ---- DApplication ----

pub struct DApplication {
    ptr: *mut ffi::DApplication,
    _not_send: PhantomData<*mut ()>,
}

impl DApplication {
    pub fn new(name: &str) -> Self {
        let ptr = unsafe { ffi::application_new(name) };
        Self { ptr, _not_send: PhantomData }
    }
    pub fn exec(&self) -> i32 {
        unsafe { ffi::application_exec(self.ptr) }
    }
    pub fn quit() {
        unsafe { ffi::application_quit() }
    }
}

// ---- DMainWindow / DTitlebar ----

widget_wrapper!(DMainWindow, ffi::DMainWindow);
widget_wrapper!(DTitlebar, ffi::DTitlebar);

impl DMainWindow {
    pub fn new() -> Self {
        Self::from_raw(unsafe { ffi::mainwindow_new() })
    }
    pub fn titlebar(&self) -> DTitlebar {
        DTitlebar::from_raw(unsafe { ffi::mainwindow_titlebar(self.ptr) })
    }
    pub fn set_central_widget(&self, w: &QWidget) {
        unsafe { ffi::mainwindow_set_central_widget(self.ptr, w.ptr) }
    }
    pub fn set_window_radius(&self, radius: i32) {
        unsafe { ffi::mainwindow_set_window_radius(self.ptr, radius) }
    }
    pub fn set_enable_blur_window(&self, enable: bool) {
        unsafe { ffi::mainwindow_set_enable_blur(self.ptr, enable) }
    }
}

impl Default for DMainWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl DTitlebar {
    pub fn set_title(&self, title: &str) {
        unsafe { ffi::titlebar_set_title(self.ptr, title) }
    }
    pub fn set_icon(&self, icon: &QIcon) {
        unsafe { ffi::titlebar_set_icon(self.ptr, &*icon.ptr) }
    }
}

// ---- QIcon ----

pub struct QIcon {
    ptr: *mut ffi::QIcon,
    _not_send: PhantomData<*mut ()>,
}

impl QIcon {
    pub fn from_theme(name: &str) -> Self {
        Self { ptr: unsafe { ffi::icon_from_theme(name) }, _not_send: PhantomData }
    }
    pub fn from_file(path: &str) -> Self {
        Self { ptr: unsafe { ffi::icon_from_file(path) }, _not_send: PhantomData }
    }
}

// ---- DLabel ----

widget_wrapper!(DLabel, ffi::DLabel);

impl DLabel {
    pub fn new(text: &str) -> Self {
        Self::from_raw(unsafe { ffi::label_new(text) })
    }
    pub fn set_text(&self, text: &str) {
        unsafe { ffi::label_set_text(self.ptr, text) }
    }
}

// ---- 按钮 ----

widget_wrapper!(DSuggestButton, ffi::DSuggestButton);
widget_wrapper!(DPushButton, ffi::DPushButton);

impl DSuggestButton {
    pub fn new(text: &str) -> Self {
        Self::from_raw(unsafe { ffi::suggest_button_new(text) })
    }
    pub fn on_clicked(&self, f: impl FnMut() + 'static) {
        self.connect_signal("clicked(bool)", f);
    }
    /// 程序化点击（触发 clicked 信号，可用来测试）
    pub fn click(&self) {
        unsafe { ffi::button_click(self.ptr.cast()) }
    }
}

impl DPushButton {
    pub fn new(text: &str) -> Self {
        Self::from_raw(unsafe { ffi::push_button_new(text) })
    }
    pub fn set_text(&self, text: &str) {
        unsafe { ffi::button_set_text(self.ptr, text) }
    }
    pub fn click(&self) {
        unsafe { ffi::button_click(self.ptr) }
    }
    pub fn on_clicked(&self, f: impl FnMut() + 'static) {
        self.connect_signal("clicked(bool)", f);
    }
}

// ---- 布局 ----

macro_rules! layout_wrapper {
    ($name:ident, $ffi:ty, $new:ident) => {
        pub struct $name {
            pub(crate) ptr: *mut $ffi,
            _not_send: PhantomData<*mut ()>,
        }
        impl $name {
            /// parent 为 Some 时布局直接装到 parent widget 上
            pub fn new(parent: Option<&QWidget>) -> Self {
                let p = parent.map_or(std::ptr::null_mut(), |p| p.ptr);
                Self { ptr: unsafe { ffi::$new(p) }, _not_send: PhantomData }
            }
            pub fn add_widget(&self, w: &QWidget) {
                unsafe { ffi::layout_add_widget(self.ptr.cast(), w.ptr) }
            }
            pub fn add_layout<L: AsLayout>(&self, child: &L) {
                unsafe { ffi::layout_add_layout(self.ptr.cast(), child.layout_ptr()) }
            }
            pub fn set_spacing(&self, spacing: i32) {
                unsafe { ffi::layout_set_spacing(self.ptr.cast(), spacing) }
            }
            pub fn set_contents_margins(&self, l: i32, t: i32, r: i32, b: i32) {
                unsafe { ffi::layout_set_contents_margins(self.ptr.cast(), l, t, r, b) }
            }
        }
    };
}

pub trait AsLayout {
    fn layout_ptr(&self) -> *mut ffi::QLayout;
}

layout_wrapper!(QVBoxLayout, ffi::QVBoxLayout, vbox_new);
layout_wrapper!(QHBoxLayout, ffi::QHBoxLayout, hbox_new);

impl AsLayout for QVBoxLayout {
    fn layout_ptr(&self) -> *mut ffi::QLayout {
        self.ptr.cast()
    }
}
impl AsLayout for QHBoxLayout {
    fn layout_ptr(&self) -> *mut ffi::QLayout {
        self.ptr.cast()
    }
}

// ---- QTableWidget ----

widget_wrapper!(QTableWidget, ffi::QTableWidget);

impl QTableWidget {
    pub fn new(rows: i32, cols: i32) -> Self {
        Self::from_raw(unsafe { ffi::table_new(rows, cols) })
    }
    pub fn set_column_count(&self, cols: i32) {
        unsafe { ffi::table_set_column_count(self.ptr, cols) }
    }
    pub fn set_row_count(&self, rows: i32) {
        unsafe { ffi::table_set_row_count(self.ptr, rows) }
    }
    /// labels 用 '|' 分隔，如 "应用|内存"
    pub fn set_horizontal_header_labels(&self, joined: &str) {
        unsafe { ffi::table_set_horizontal_header_labels(self.ptr, joined) }
    }
    pub fn set_cell_text(&self, row: i32, col: i32, text: &str) {
        unsafe { ffi::table_set_cell_text(self.ptr, row, col, text) }
    }
    /// 给单元格存 i64 用户数据（如 pid）
    pub fn set_cell_data(&self, row: i32, col: i32, data: i64) {
        unsafe { ffi::table_set_cell_data(self.ptr, row, col, data) }
    }
    pub fn cell_data(&self, row: i32, col: i32) -> i64 {
        unsafe { ffi::table_cell_data(self.ptr, row, col) }
    }
    pub fn cell_text(&self, row: i32, col: i32) -> String {
        unsafe { ffi::table_cell_text(self.ptr, row, col) }
    }
    pub fn current_row(&self) -> i32 {
        unsafe { ffi::table_current_row(self.ptr) }
    }
    pub fn row_count(&self) -> i32 {
        unsafe { ffi::table_row_count(self.ptr) }
    }
    /// 整行选择、只读、单选
    pub fn select_rows_readonly(&self) {
        unsafe { ffi::table_select_rows_readonly(self.ptr) }
    }
    pub fn header_stretch_last(&self, stretch: bool) {
        unsafe { ffi::table_header_stretch_last(self.ptr, stretch) }
    }
    pub fn set_column_width(&self, col: i32, width: i32) {
        unsafe { ffi::table_set_column_width(self.ptr, col, width) }
    }
    pub fn on_selection_changed(&self, f: impl FnMut() + 'static) {
        self.connect_signal("itemSelectionChanged()", f);
    }
    pub fn on_current_row_changed(&self, f: impl FnMut(i32) + 'static) {
        self.connect_signal_i32("currentRowChanged(int)", f);
    }
}

// ---- QTimer ----

pub struct QTimer {
    ptr: *mut ffi::QTimer,
    _not_send: PhantomData<*mut ()>,
}

impl QTimer {
    pub fn new() -> Self {
        Self { ptr: unsafe { ffi::timer_new(std::ptr::null_mut()) }, _not_send: PhantomData }
    }
    pub fn start(&self, msec: i32) {
        unsafe { ffi::timer_start(self.ptr, msec) }
    }
    pub fn stop(&self) {
        unsafe { ffi::timer_stop(self.ptr) }
    }
    pub fn on_timeout(&self, f: impl FnMut() + 'static) {
        let id = dtk_sys::register_cb0(f);
        unsafe { ffi::relay_connect0(self.ptr.cast(), "timeout()", id) }
    }
    /// 一次性定时器
    pub fn single_shot(msec: i32, f: impl FnMut() + 'static) {
        let id = dtk_sys::register_cb0(f);
        unsafe { ffi::timer_single_shot(msec, id) }
    }
}

impl Default for QTimer {
    fn default() -> Self {
        Self::new()
    }
}

// 生成器产物：dtkwidget 其余类的绑定
pub mod widgets;
