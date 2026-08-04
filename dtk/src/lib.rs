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
            pub fn set_fixed_size(&self, w: i32, h: i32) {
                unsafe { ffi::widget_set_fixed_size(self.ptr.cast(), w, h) }
            }
            pub fn raise(&self) {
                unsafe { ffi::widget_raise(self.ptr.cast()) }
            }
            pub fn activate_window(&self) {
                unsafe { ffi::widget_activate_window(self.ptr.cast()) }
            }
            pub fn close(&self) {
                unsafe { ffi::widget_close(self.ptr.cast()) }
            }
            pub fn is_visible(&self) -> bool {
                unsafe { ffi::widget_is_visible(self.ptr.cast()) }
            }
            /// policy 用 qt::NO_FOCUS 等常量
            pub fn set_focus_policy(&self, policy: i32) {
                unsafe { ffi::widget_set_focus_policy(self.ptr.cast(), policy) }
            }
            pub fn set_font(&self, font: &QFont) {
                unsafe { ffi::widget_set_font(self.ptr.cast(), font.ptr) }
            }
            /// 当前 palette 的堆拷贝
            pub fn palette(&self) -> QPalette {
                QPalette::from_raw(unsafe { ffi::widget_palette(self.ptr.cast()) })
            }
            pub fn set_palette(&self, pal: &QPalette) {
                unsafe { ffi::widget_set_palette(self.ptr.cast(), pal.ptr) }
            }
            /// icon 用 qt::SP_* 常量
            pub fn standard_icon_pixmap(&self, icon: i32, size: i32) -> QPixmap {
                QPixmap::from_raw(unsafe { ffi::standard_icon_pixmap(self.ptr.cast(), icon, size) })
            }
            /// 延迟删除（事件循环下一轮）
            pub fn delete_later(&self) {
                unsafe { ffi::object_delete_later(self.ptr.cast()) }
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
    /// 带退出守卫：QEvent::Quit 时问 guard，返 false 吞掉事件（Rust 侧自排重试）
    pub fn new_with_quit_guard(name: &str, guard: impl FnMut() -> bool + 'static) -> Self {
        let id = dtk_sys::register_cb_guard(guard);
        let ptr = unsafe { ffi::application_new_ex(name, id) };
        Self { ptr, _not_send: PhantomData }
    }
    pub fn exec(&self) -> i32 {
        unsafe { ffi::application_exec(self.ptr) }
    }
    pub fn quit() {
        unsafe { ffi::application_quit() }
    }
    pub fn set_quit_on_last_window_closed(quit: bool) {
        unsafe { ffi::application_set_quit_on_last_window_closed(quit) }
    }
    pub fn set_application_display_name(name: &str) {
        unsafe { ffi::application_set_application_display_name(name) }
    }
    /// DTK 翻译（zh_CN 等），失败返回 false
    pub fn load_translator(&self) -> bool {
        unsafe { ffi::application_load_translator(self.ptr) }
    }
    pub fn has_arg(arg: &str) -> bool {
        unsafe { ffi::application_has_arg(arg) }
    }
}

// ---- DMainWindow / DTitlebar ----

widget_wrapper!(DMainWindow, ffi::DMainWindow);
widget_wrapper!(DTitlebar, ffi::DTitlebar);

impl DMainWindow {
    pub fn new() -> Self {
        Self::from_raw(unsafe { ffi::mainwindow_new() })
    }
    /// 带事件回调：on_close 返 false → 窗口不关闭（event ignore）
    pub fn new_with_events(
        on_show: impl FnMut() + 'static,
        on_close: impl FnMut() -> bool + 'static,
    ) -> Self {
        let show_id = dtk_sys::register_cb0(on_show);
        let close_id = dtk_sys::register_cb_guard(on_close);
        Self::from_raw(unsafe { ffi::mainwindow_new_ex(show_id, close_id) })
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
    pub fn take_central_widget(&self) -> QWidget {
        QWidget::from_raw(unsafe { ffi::mainwindow_take_central_widget(self.ptr) })
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

/// 值类型 wrapper：堆分配对象，Rust 侧持有（泄漏可接受，量小）
macro_rules! value_wrapper {
    ($name:ident, $ffi:ty) => {
        pub struct $name {
            pub(crate) ptr: *mut $ffi,
            _not_send: PhantomData<*mut ()>,
        }
        impl $name {
            pub(crate) fn from_raw(ptr: *mut $ffi) -> Self {
                assert!(!ptr.is_null());
                Self { ptr, _not_send: PhantomData }
            }
        }
    };
}

value_wrapper!(QColor, ffi::QColor);
value_wrapper!(QFont, ffi::QFont);
value_wrapper!(QPalette, ffi::QPalette);
value_wrapper!(QPixmap, ffi::QPixmap);
value_wrapper!(QPoint, ffi::QPoint);
value_wrapper!(QRect, ffi::QRect);
value_wrapper!(QSize, ffi::QSize);

impl QColor {
    pub fn rgb(r: i32, g: i32, b: i32) -> Self {
        Self::from_raw(unsafe { ffi::color_new_rgb(r, g, b, 255) })
    }
    pub fn rgba(r: i32, g: i32, b: i32, a: i32) -> Self {
        Self::from_raw(unsafe { ffi::color_new_rgb(r, g, b, a) })
    }
}

impl QFont {
    pub fn new() -> Self {
        Self::from_raw(unsafe { ffi::font_new() })
    }
    pub fn set_point_size(&self, size: i32) {
        unsafe { ffi::font_set_point_size(self.ptr, size) }
    }
    pub fn set_bold(&self, bold: bool) {
        unsafe { ffi::font_set_bold(self.ptr, bold) }
    }
}

impl Default for QFont {
    fn default() -> Self {
        Self::new()
    }
}

impl QPalette {
    pub fn new() -> Self {
        Self::from_raw(unsafe { ffi::palette_new() })
    }
    /// group/role 用 qt 模块常量
    pub fn set_color(&self, group: i32, role: i32, color: &QColor) {
        unsafe { ffi::palette_set_color(self.ptr, group, role, color.ptr) }
    }
}

impl Default for QPalette {
    fn default() -> Self {
        Self::new()
    }
}

impl QPixmap {
    /// 文件或 qrc 路径
    pub fn new(path: &str) -> Self {
        Self::from_raw(unsafe { ffi::pixmap_new(path) })
    }
}

impl QPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self::from_raw(unsafe { ffi::point_new(x, y) })
    }
}

impl QRect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self::from_raw(unsafe { ffi::rect_new(x, y, w, h) })
    }
}

impl QSize {
    pub fn new(w: i32, h: i32) -> Self {
        Self::from_raw(unsafe { ffi::size_new(w, h) })
    }
}

/// Qt 枚举/QFlags 常量（传给映射成 i32 的参数）
pub mod qt {
    // Qt::Alignment
    pub const ALIGN_LEFT: i32 = 0x1;
    pub const ALIGN_RIGHT: i32 = 0x2;
    pub const ALIGN_HCENTER: i32 = 0x4;
    pub const ALIGN_TOP: i32 = 0x20;
    pub const ALIGN_BOTTOM: i32 = 0x40;
    pub const ALIGN_VCENTER: i32 = 0x80;
    pub const ALIGN_CENTER: i32 = 0x84;
    // Qt::FocusPolicy
    pub const NO_FOCUS: i32 = 0;
    // Qt::ItemDataRole
    pub const USER_ROLE: i32 = 0x0100;
    // Qt::Orientation
    pub const HORIZONTAL: i32 = 1;
    pub const VERTICAL: i32 = 2;
    // QFrame::Shape
    pub const FRAME_STYLED_PANEL: i32 = 0x6;
    // QStyle::StandardPixmap
    pub const SP_MESSAGE_BOX_WARNING: i32 = 10;
    // QHeaderView::ResizeMode
    pub const HEADER_INTERACTIVE: i32 = 0;
    pub const HEADER_STRETCH: i32 = 1;
    pub const HEADER_RESIZE_TO_CONTENTS: i32 = 3;
    // QPalette::ColorGroup
    pub const PALETTE_ACTIVE: i32 = 0;
    pub const PALETTE_DISABLED: i32 = 1;
    pub const PALETTE_INACTIVE: i32 = 2;
    pub const PALETTE_CURRENT: i32 = 3;
    pub const PALETTE_ALL: i32 = 5;
    // QPalette::ColorRole
    pub const ROLE_WINDOW_TEXT: i32 = 0;
    pub const ROLE_TEXT: i32 = 6;
    pub const ROLE_BASE: i32 = 9;
    pub const ROLE_WINDOW: i32 = 10;
    pub const ROLE_HIGHLIGHT: i32 = 12;
    pub const ROLE_HIGHLIGHTED_TEXT: i32 = 13;
    // QStyleOption::State
    pub const STATE_SELECTED: i32 = 0x1;
    pub const STATE_MOUSE_OVER: i32 = 0x2000;
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
    pub fn set_word_wrap(&self, wrap: bool) {
        unsafe { ffi::label_set_word_wrap(self.ptr, wrap) }
    }
    /// alignment 用 qt::ALIGN_* 常量
    pub fn set_alignment(&self, alignment: i32) {
        unsafe { ffi::label_set_alignment(self.ptr, alignment) }
    }
    pub fn set_pixmap(&self, pm: &QPixmap) {
        unsafe { ffi::label_set_pixmap(self.ptr, pm.ptr) }
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
    /// 取单元格 item（不存在返 None）
    pub fn item(&self, row: i32, col: i32) -> Option<QTableWidgetItem> {
        let p = unsafe { ffi::table_item(self.ptr, row, col) };
        if p.is_null() {
            None
        } else {
            Some(QTableWidgetItem { ptr: p, _not_send: PhantomData })
        }
    }
    pub fn select_row(&self, row: i32) {
        unsafe { ffi::table_select_row(self.ptr, row) }
    }
    pub fn hide_headers(&self, horizontal: bool, vertical: bool) {
        unsafe { ffi::table_hide_headers(self.ptr, horizontal, vertical) }
    }
    /// mode 用 qt::HEADER_* 常量
    pub fn set_section_resize_mode(&self, col: i32, mode: i32) {
        unsafe { ffi::table_set_section_resize_mode(self.ptr, col, mode) }
    }
    pub fn set_vertical_header_default_section_size(&self, size: i32) {
        unsafe { ffi::table_set_vertical_header_default_section_size(self.ptr, size) }
    }
    pub fn set_show_grid(&self, show: bool) {
        unsafe { ffi::table_set_show_grid(self.ptr, show) }
    }
    /// shape 用 qt::FRAME_* 常量
    pub fn set_frame_shape(&self, shape: i32) {
        unsafe { ffi::table_set_frame_shape(self.ptr, shape) }
    }
    pub fn set_icon_size(&self, w: i32, h: i32) {
        unsafe { ffi::table_set_icon_size(self.ptr, w, h) }
    }
    pub fn set_delegate_for_column(&self, col: i32, delegate: &PaintDelegate) {
        unsafe { ffi::table_set_delegate_for_column(self.ptr, col, delegate.ptr) }
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

// ---- QTableWidgetItem ----

pub struct QTableWidgetItem {
    pub(crate) ptr: *mut ffi::QTableWidgetItem,
    _not_send: PhantomData<*mut ()>,
}

impl QTableWidgetItem {
    pub fn set_icon(&self, icon: &QIcon) {
        unsafe { ffi::item_set_icon(self.ptr, icon.ptr) }
    }
    /// alignment 用 qt::ALIGN_* 常量
    pub fn set_text_alignment(&self, alignment: i32) {
        unsafe { ffi::item_set_text_alignment(self.ptr, alignment) }
    }
    pub fn set_foreground(&self, color: &QColor) {
        unsafe { ffi::item_set_foreground(self.ptr, color.ptr) }
    }
    /// role 用 qt::USER_ROLE + n
    pub fn set_data_string(&self, role: i32, value: &str) {
        unsafe { ffi::item_set_data_string(self.ptr, role, value) }
    }
    pub fn data_string(&self, role: i32) -> String {
        unsafe { ffi::item_data_string(self.ptr, role) }
    }
    pub fn set_data_bool(&self, role: i32, value: bool) {
        unsafe { ffi::item_set_data_bool(self.ptr, role, value) }
    }
    pub fn data_bool(&self, role: i32) -> bool {
        unsafe { ffi::item_data_bool(self.ptr, role) }
    }
}

// ---- PaintDelegate：表格单元格自定义绘制 ----

pub struct PaintDelegate {
    pub(crate) ptr: *mut ffi::QStyledItemDelegate,
    _not_send: PhantomData<*mut ()>,
}

/// paint 回调拿到的画家（只在回调内有效）
pub struct Painter {
    ptr: *mut ffi::QPainter,
}

impl Painter {
    pub fn save(&self) {
        unsafe { ffi::painter_save(self.ptr) }
    }
    pub fn restore(&self) {
        unsafe { ffi::painter_restore(self.ptr) }
    }
    pub fn set_pen_color(&self, color: &QColor) {
        unsafe { ffi::painter_set_pen_color(self.ptr, color.ptr) }
    }
    pub fn set_font(&self, font: &QFont) {
        unsafe { ffi::painter_set_font(self.ptr, font.ptr) }
    }
    /// flags 用 qt::ALIGN_* 组合
    pub fn draw_text(&self, x: i32, y: i32, w: i32, h: i32, flags: i32, text: &str) {
        unsafe { ffi::painter_draw_text(self.ptr, x, y, w, h, flags, text) }
    }
    pub fn draw_pixmap(&self, x: i32, y: i32, w: i32, h: i32, pm: &QPixmap) {
        unsafe { ffi::painter_draw_pixmap(self.ptr, x, y, w, h, pm.ptr) }
    }
    pub fn draw_icon(&self, x: i32, y: i32, w: i32, h: i32, icon: &QIcon) {
        unsafe { ffi::painter_draw_icon(self.ptr, x, y, w, h, icon.ptr) }
    }
    pub fn fill_rect(&self, x: i32, y: i32, w: i32, h: i32, color: &QColor) {
        unsafe { ffi::painter_fill_rect(self.ptr, x, y, w, h, color.ptr) }
    }
}

/// paint 回调拿到的模型索引（只在回调内有效）
pub struct ModelIndex {
    ptr: *mut ffi::QModelIndex,
}

impl ModelIndex {
    pub fn data_string(&self, role: i32) -> String {
        unsafe { ffi::index_data_string(self.ptr, role) }
    }
    pub fn data_bool(&self, role: i32) -> bool {
        unsafe { ffi::index_data_bool(self.ptr, role) }
    }
    pub fn data_i64(&self, role: i32) -> i64 {
        unsafe { ffi::index_data_i64(self.ptr, role) }
    }
}

impl PaintDelegate {
    /// f: (painter, index, x, y, w, h, state)。state 用 qt::STATE_* 判断
    pub fn new(f: impl FnMut(&Painter, &ModelIndex, i32, i32, i32, i32, i32) + 'static) -> Self {
        let mut f = f;
        let id = dtk_sys::register_cb_paint(move |p, idx, x, y, w, h, state| {
            f(&Painter { ptr: p }, &ModelIndex { ptr: idx }, x, y, w, h, state);
        });
        Self {
            ptr: unsafe { ffi::rust_delegate_new(id, std::ptr::null_mut()) },
            _not_send: PhantomData,
        }
    }
}

// ---- QSocketNotifier ----

pub struct QSocketNotifier {
    ptr: *mut ffi::QSocketNotifier,
    _not_send: PhantomData<*mut ()>,
}

impl QSocketNotifier {
    /// 监听 fd 可读（Read 类型），如 signalfd
    pub fn new(fd: i32) -> Self {
        Self { ptr: unsafe { ffi::socket_notifier_new(fd) }, _not_send: PhantomData }
    }
    pub fn on_activated(&self, f: impl FnMut() + 'static) {
        let id = dtk_sys::register_cb0(f);
        unsafe { ffi::relay_connect0(self.ptr.cast(), "activated(QSocketDescriptor)", id) }
    }
}

// 生成器产物：dtkwidget 其余类的绑定
pub mod widgets;
