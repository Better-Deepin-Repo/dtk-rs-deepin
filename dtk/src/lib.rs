//! Safe Rust bindings for DTK6 (dtkwidget).
//!
//! Lifetime: all Qt objects are freed via the Qt parent-child mechanism; Rust wrappers
//! are non-owning raw pointers. Top-level windows (no parent) die with QApplication.
//! ponytail: no thread safety (Qt GUIs are single-threaded anyway); wrappers are !Send.

use dtk_sys::ffi;
use std::marker::PhantomData;

macro_rules! widget_wrapper {
    ($name:ident, $ffi:ty) => {
        object_wrapper!($name, $ffi);
        impl $name {
            /// use as QWidget* (base-class ops / adding to layouts)
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
            pub fn set_window_icon(&self, icon: &QIcon) {
                unsafe { ffi::widget_set_window_icon(self.ptr.cast(), icon.ptr) }
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
            /// use qt::NO_FOCUS etc. for policy
            pub fn set_focus_policy(&self, policy: i32) {
                unsafe { ffi::widget_set_focus_policy(self.ptr.cast(), policy) }
            }
            pub fn set_font(&self, font: &QFont) {
                unsafe { ffi::widget_set_font(self.ptr.cast(), font.ptr) }
            }
            /// heap copy of the current palette
            pub fn palette(&self) -> QPalette {
                QPalette::from_raw(unsafe { ffi::widget_palette(self.ptr.cast()) })
            }
            pub fn set_palette(&self, pal: &QPalette) {
                unsafe { ffi::widget_set_palette(self.ptr.cast(), pal.ptr) }
            }
            /// use qt::SP_* constants for icon
            pub fn standard_icon_pixmap(&self, icon: i32, size: i32) -> QPixmap {
                QPixmap::from_raw(unsafe { ffi::standard_icon_pixmap(self.ptr.cast(), icon, size) })
            }
            /// deferred delete (next event-loop turn)
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
            #[allow(dead_code)] // not every generated class gets constructed
            pub(crate) fn from_raw(ptr: *mut $ffi) -> Self {
                assert!(!ptr.is_null());
                Self { ptr, _not_send: PhantomData }
            }
            /// use as QObject* (signals)
            #[allow(dead_code)] // some classes connect no signals; kept for the generator
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

/// generic QWidget handle (base-class view only)
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

/// connect an arg-less signal on any widget (e.g. clicked, timeout)
pub trait Signal0 {
    fn qobject_ptr(&self) -> *mut ffi::QObject;
    /// signal looks like "clicked()" / "windowRadiusChanged()"
    fn connect_signal(&self, signal: &str, f: impl FnMut() + 'static) {
        let id = dtk_sys::register_cb0(f);
        unsafe { ffi::relay_connect0(self.qobject_ptr(), signal, id) }
    }
}

/// signal with one i32 arg (e.g. currentRowChanged(int))
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

/// real process argv for QApplication; ponytail: '|' separator, can't appear in normal flags
fn env_args_joined() -> String {
    std::env::args().collect::<Vec<_>>().join("|")
}

impl DApplication {
    pub fn new(name: &str) -> Self {
        let args = env_args_joined();
        let ptr = unsafe { ffi::application_new(name, &args) };
        Self { ptr, _not_send: PhantomData }
    }
    /// with quit guard: QEvent::Quit asks the guard; false swallows the event (Rust retries itself)
    pub fn new_with_quit_guard(name: &str, guard: impl FnMut() -> bool + 'static) -> Self {
        let id = dtk_sys::register_cb_guard(guard);
        let args = env_args_joined();
        let ptr = unsafe { ffi::application_new_ex(name, &args, id) };
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
    /// DTK translations (zh_CN etc.); false on failure
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
    /// with event callbacks: on_close returning false keeps the window open (event ignore)
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

/// value-type wrapper: heap-allocated, owned by Rust (small leak acceptable)
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
    /// packed 0xAARRGGBB
    pub fn rgba_u32(&self) -> u32 {
        unsafe { ffi::color_rgba(self.ptr) as u32 }
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
    /// use qt module constants for group/role
    pub fn set_color(&self, group: i32, role: i32, color: &QColor) {
        unsafe { ffi::palette_set_color(self.ptr, group, role, color.ptr) }
    }
    /// read a color (e.g. copy Active Highlight into the Inactive group)
    pub fn color(&self, group: i32, role: i32) -> QColor {
        QColor::from_raw(unsafe { ffi::palette_color(self.ptr, group, role) })
    }
}

impl Default for QPalette {
    fn default() -> Self {
        Self::new()
    }
}

impl QPixmap {
    /// file or qrc path
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

/// Qt enum/QFlags constants (for params mapped to i32)
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
    // Qt::TextElideMode
    pub const ELIDE_LEFT: i32 = 0;
    pub const ELIDE_RIGHT: i32 = 1;
    pub const ELIDE_MIDDLE: i32 = 2;
    pub const ELIDE_NONE: i32 = 3;
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
    /// use qt::ALIGN_* constants for alignment
    pub fn set_alignment(&self, alignment: i32) {
        unsafe { ffi::label_set_alignment(self.ptr, alignment) }
    }
    pub fn set_pixmap(&self, pm: &QPixmap) {
        unsafe { ffi::label_set_pixmap(self.ptr, pm.ptr) }
    }
}

// ---- buttons ----

widget_wrapper!(DSuggestButton, ffi::DSuggestButton);
widget_wrapper!(DPushButton, ffi::DPushButton);

impl DSuggestButton {
    pub fn new(text: &str) -> Self {
        Self::from_raw(unsafe { ffi::suggest_button_new(text) })
    }
    pub fn on_clicked(&self, f: impl FnMut() + 'static) {
        self.connect_signal("clicked(bool)", f);
    }
    /// programmatic click (emits clicked; useful for tests)
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

// ---- layouts ----

macro_rules! layout_wrapper {
    ($name:ident, $ffi:ty, $new:ident) => {
        pub struct $name {
            pub(crate) ptr: *mut $ffi,
            _not_send: PhantomData<*mut ()>,
        }
        impl $name {
            /// when parent is Some, the layout installs directly on the parent widget
            pub fn new(parent: Option<&QWidget>) -> Self {
                let p = parent.map_or(std::ptr::null_mut(), |p| p.ptr);
                Self { ptr: unsafe { ffi::$new(p) }, _not_send: PhantomData }
            }
            pub fn add_widget(&self, w: &QWidget) {
                unsafe { ffi::layout_add_widget(self.ptr.cast(), w.ptr) }
            }
            /// box layouts only: stretch factor + qt::ALIGN_* alignment
            pub fn add_widget_ex(&self, w: &QWidget, stretch: i32, alignment: i32) {
                unsafe { ffi::layout_add_widget_ex(self.ptr.cast(), w.ptr, stretch, alignment) }
            }
            /// box layouts only: insert stretchable space (e.g. push buttons right)
            pub fn add_stretch(&self, stretch: i32) {
                unsafe { ffi::layout_add_stretch(self.ptr.cast(), stretch) }
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
    /// labels are '|'-separated, e.g. "App|Memory"
    pub fn set_horizontal_header_labels(&self, joined: &str) {
        unsafe { ffi::table_set_horizontal_header_labels(self.ptr, joined) }
    }
    pub fn set_cell_text(&self, row: i32, col: i32, text: &str) {
        unsafe { ffi::table_set_cell_text(self.ptr, row, col, text) }
    }
    /// store i64 user data (e.g. a pid) on a cell
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
    /// row selection, read-only, single-select
    pub fn select_rows_readonly(&self) {
        unsafe { ffi::table_select_rows_readonly(self.ptr) }
    }
    pub fn header_stretch_last(&self, stretch: bool) {
        unsafe { ffi::table_header_stretch_last(self.ptr, stretch) }
    }
    pub fn set_column_width(&self, col: i32, width: i32) {
        unsafe { ffi::table_set_column_width(self.ptr, col, width) }
    }
    /// get cell item (None if absent)
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
    /// use qt::HEADER_* constants for mode
    pub fn set_section_resize_mode(&self, col: i32, mode: i32) {
        unsafe { ffi::table_set_section_resize_mode(self.ptr, col, mode) }
    }
    pub fn set_vertical_header_default_section_size(&self, size: i32) {
        unsafe { ffi::table_set_vertical_header_default_section_size(self.ptr, size) }
    }
    pub fn set_show_grid(&self, show: bool) {
        unsafe { ffi::table_set_show_grid(self.ptr, show) }
    }
    /// use qt::FRAME_* constants for shape
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
    /// one-shot timer; the callback entry is removed from the registry after firing
    pub fn single_shot(msec: i32, f: impl FnMut() + 'static) {
        use std::cell::Cell;
        use std::rc::Rc;
        let slot = Rc::new(Cell::new(0usize));
        let slot2 = slot.clone();
        let mut f = Some(f);
        let id = dtk_sys::register_cb0(move || {
            if let Some(mut f) = f.take() {
                f();
            }
            dtk_sys::unregister_cb(slot2.get()); // self-clean: one-shot entries must not accumulate
        });
        slot.set(id);
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
    /// use qt::ALIGN_* constants for alignment
    pub fn set_text_alignment(&self, alignment: i32) {
        unsafe { ffi::item_set_text_alignment(self.ptr, alignment) }
    }
    pub fn set_foreground(&self, color: &QColor) {
        unsafe { ffi::item_set_foreground(self.ptr, color.ptr) }
    }
    /// use qt::USER_ROLE + n for role
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

// ---- PaintDelegate: custom cell painting ----

pub struct PaintDelegate {
    pub(crate) ptr: *mut ffi::QStyledItemDelegate,
    _not_send: PhantomData<*mut ()>,
}

/// painter handed to the paint callback (valid only inside it)
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
    /// combine qt::ALIGN_* for flags
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
    pub fn set_clip_rect(&self, x: i32, y: i32, w: i32, h: i32) {
        unsafe { ffi::painter_set_clip_rect(self.ptr, x, y, w, h) }
    }
    /// elide text to width using the painter's current font; use qt::ELIDE_* for mode
    pub fn elided_text(&self, text: &str, mode: i32, width: i32) -> String {
        unsafe { ffi::painter_elided_text(self.ptr, text, mode, width) }
    }
}

/// model index handed to the paint callback (valid only inside it)
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
    /// f: (painter, index, x, y, w, h, state). test state against qt::STATE_*
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
    /// watch an fd for readability (Read type), e.g. signalfd
    pub fn new(fd: i32) -> Self {
        Self { ptr: unsafe { ffi::socket_notifier_new(fd) }, _not_send: PhantomData }
    }
    pub fn on_activated(&self, f: impl FnMut() + 'static) {
        let id = dtk_sys::register_cb0(f);
        unsafe { ffi::relay_connect0(self.ptr.cast(), "activated(QSocketDescriptor)", id) }
    }
}

// generator output: bindings for the rest of dtkwidget
pub mod widgets;
