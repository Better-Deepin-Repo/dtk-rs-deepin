//! Safe Rust bindings for DTK6 (dtkwidget).
//!
//! Lifetime: all Qt objects are freed via the Qt parent-child mechanism; Rust wrappers
//! are non-owning raw pointers. Top-level windows (no parent) die with QApplication.
//! ponytail: no thread safety (Qt GUIs are single-threaded anyway); wrappers are !Send.

use dtk_sys::ffi;
use std::marker::PhantomData;

/// null for None (Qt parent pointer)
fn opt_ptr(w: Option<&QWidget>) -> *mut ffi::QWidget {
    w.map_or(std::ptr::null_mut(), |w| w.ptr)
}

/// `impl Default` for wrappers whose `new()` is the default ctor
macro_rules! impl_default {
    ($($name:ident),* $(,)?) => {
        $(impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        })*
    };
}
pub(crate) use impl_default;

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
            /// widget size in pixels (window: includes the DTK titlebar)
            pub fn width(&self) -> i32 {
                unsafe { ffi::widget_width(self.ptr.cast()) }
            }
            pub fn height(&self) -> i32 {
                unsafe { ffi::widget_height(self.ptr.cast()) }
            }
            pub fn move_to(&self, x: i32, y: i32) {
                unsafe { ffi::widget_move(self.ptr.cast(), x, y) }
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
            /// schedule a repaint
            pub fn update(&self) {
                unsafe { ffi::widget_update(self.ptr.cast()) }
            }
            /// grab keyboard focus
            pub fn set_focus(&self) {
                unsafe { ffi::widget_set_focus(self.ptr.cast()) }
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
            /// use qt::focus::NO_FOCUS etc. for policy
            pub fn set_focus_policy(&self, policy: i32) {
                unsafe { ffi::widget_set_focus_policy(self.ptr.cast(), policy) }
            }
            pub fn set_font(&self, font: &QFont) {
                unsafe { ffi::widget_set_font(self.ptr.cast(), font.ptr) }
            }
            /// qt::cursor::* shape; stays until unset_cursor
            pub fn set_cursor(&self, shape: i32) {
                unsafe { ffi::widget_set_cursor(self.ptr.cast(), shape) }
            }
            pub fn unset_cursor(&self) {
                unsafe { ffi::widget_unset_cursor(self.ptr.cast()) }
            }
            /// heap copy of the current palette
            pub fn palette(&self) -> QPalette {
                QPalette::from_raw(unsafe { ffi::widget_palette(self.ptr.cast()) })
            }
            pub fn set_palette(&self, pal: &QPalette) {
                unsafe { ffi::widget_set_palette(self.ptr.cast(), pal.ptr) }
            }
            /// use qt::standard_pixmap::* constants for icon
            pub fn standard_icon_pixmap(&self, icon: i32, size: i32) -> QPixmap {
                QPixmap::from_raw(unsafe { ffi::standard_icon_pixmap(self.ptr.cast(), icon, size) })
            }
            /// deferred delete (next event-loop turn)
            pub fn delete_later(&self) {
                unsafe { ffi::object_delete_later(self.ptr.cast()) }
            }
            /// leak the Rust handle; Qt parent-child still owns the object.
            /// kills `std::mem::forget` boilerplate for top-level widgets.
            pub fn leak(self) {
                // ManuallyDrop (not forget): handles are Copy, forget would be a no-op warning
                let _ = std::mem::ManuallyDrop::new(self);
            }
        }
    };
}

macro_rules! object_wrapper {
    ($name:ident, $ffi:ty) => {
        /// non-owning handle: Qt parent-child owns the object, Copy is just another view
        #[derive(Clone, Copy)]
        pub struct $name {
            pub(crate) ptr: *mut $ffi,
            _not_send: PhantomData<*mut ()>,
        }
        impl $name {
            #[allow(dead_code)] // not every generated class gets constructed
            pub(crate) fn from_raw(ptr: *mut $ffi) -> Self {
                assert!(!ptr.is_null());
                Self {
                    ptr,
                    _not_send: PhantomData,
                }
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
        impl SignalBool for $name {
            fn qobject_ptr(&self) -> *mut ffi::QObject {
                self.as_qobject()
            }
        }
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, concat!(stringify!($name), "({:p})"), self.ptr)
            }
        }
    };
}

/// generic QWidget handle (base-class view only); Copy: non-owning view, Qt owns the widget
#[derive(Clone, Copy)]
pub struct QWidget {
    pub(crate) ptr: *mut ffi::QWidget,
    _not_send: PhantomData<*mut ()>,
}

impl QWidget {
    pub(crate) fn from_raw(ptr: *mut ffi::QWidget) -> Self {
        Self {
            ptr,
            _not_send: PhantomData,
        }
    }
    pub fn new(parent: Option<&QWidget>) -> Self {
        let p = opt_ptr(parent);
        Self::from_raw(unsafe { ffi::widget_new(p) })
    }
    pub fn show(&self) {
        unsafe { ffi::widget_show(self.ptr) }
    }
    /// schedule a repaint
    pub fn update(&self) {
        unsafe { ffi::widget_update(self.ptr) }
    }
    /// grab keyboard focus
    pub fn set_focus(&self) {
        unsafe { ffi::widget_set_focus(self.ptr) }
    }
    /// qt::cursor::* shape; stays until unset_cursor
    pub fn set_cursor(&self, shape: i32) {
        unsafe { ffi::widget_set_cursor(self.ptr, shape) }
    }
    pub fn unset_cursor(&self) {
        unsafe { ffi::widget_unset_cursor(self.ptr) }
    }
    /// deepin-terminal style: install the tab bar as titlebar custom widget
    /// (zero-margin layout, vertically centered)
    pub fn titlebar_set_tabbar(&self, tabbar: &QWidget) {
        unsafe { ffi::main_window_titlebar_set_tabbar(self.ptr, tabbar.ptr) }
    }
    /// add a widget into the titlebar, left-aligned (DMainWindow only)
    pub fn titlebar_add_widget(&self, child: &QWidget) {
        unsafe { ffi::main_window_titlebar_add_widget(self.ptr, child.ptr) }
    }
    /// set the titlebar icon (DMainWindow only; no-op otherwise)
    pub fn set_titlebar_icon(&self, icon: &QIcon) {
        unsafe { ffi::widget_set_titlebar_icon(self.ptr, icon.ptr) }
    }
    /// IME candidate window anchor: cursor rect in widget coords (PaintWidget only)
    pub fn set_ime_cursor_rect(&self, x: i32, y: i32, w: i32, h: i32) {
        unsafe { ffi::paint_widget_set_ime_rect(self.ptr, x, y, w, h) }
    }
    pub fn resize(&self, w: i32, h: i32) {
        unsafe { ffi::widget_resize(self.ptr, w, h) }
    }
    pub fn move_to(&self, x: i32, y: i32) {
        unsafe { ffi::widget_move(self.ptr, x, y) }
    }
    /// deepin-terminal-style tab labels: centered, elided clear of the close button
    pub fn install_tab_label_style(&self) {
        unsafe { ffi::tabbar_install_style(self.ptr) }
    }
    /// force the tab bar's internal layout (and ancestors) to relayout now, not
    /// on the next event pass; call after add/remove/set_tab_text to avoid a
    /// 1-frame misaligned paint
    pub fn flush_layout(&self) {
        unsafe { ffi::tabbar_flush_layout(self.ptr) }
    }
    /// reparent (widget keeps geometry; shown with the new parent)
    pub fn set_parent(&self, parent: &QWidget) {
        unsafe { ffi::widget_set_parent(self.ptr, parent.ptr) }
    }
    /// deferred delete (next event-loop turn)
    pub fn delete_later(&self) {
        unsafe { ffi::object_delete_later(self.ptr.cast()) }
    }
    /// key sequence like "Ctrl+Shift+C"; fires while the widget lives.
    /// Returns the callback id for [`unregister_callback`].
    pub fn add_shortcut(&self, key: &str, f: impl FnMut() + 'static) -> usize {
        let id = dtk_sys::register_cb0(f);
        unsafe { ffi::shortcut_new(self.ptr, key, id) };
        id
    }
}

impl Signal0 for QWidget {
    fn qobject_ptr(&self) -> *mut ffi::QObject {
        self.ptr.cast()
    }
}
impl SignalI32 for QWidget {
    fn qobject_ptr(&self) -> *mut ffi::QObject {
        self.ptr.cast()
    }
}
impl SignalBool for QWidget {
    fn qobject_ptr(&self) -> *mut ffi::QObject {
        self.ptr.cast()
    }
}

impl std::fmt::Debug for QWidget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QWidget({:p})", self.ptr)
    }
}

// Qt button base handle (returned by e.g. DDialog button accessors)
widget_wrapper!(QAbstractButton, ffi::QAbstractButton);

// fully user-drawn widget: every paint/input event goes to the Rust handler
widget_wrapper!(PaintWidget, ffi::QWidget);

/// key event (key = qt::key::*, mods = qt::modifier::*)
#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub key: i32,
    pub mods: i32,
    pub text: String,
    pub press: bool,
    pub autorepeat: bool,
}

/// mouse event (kind = qt::mouse_kind::*, button = qt::mouse_button::*)
#[derive(Debug, Clone)]
pub struct MouseEvent {
    pub kind: i32,
    pub button: i32,
    pub x: i32,
    pub y: i32,
    pub mods: i32,
}

/// events delivered to a [`PaintWidget`] handler
pub enum PaintWidgetEvent {
    /// painter valid only inside the callback; w/h = widget size
    Paint(Painter, i32, i32),
    Key(KeyEvent),
    Mouse(MouseEvent),
    Wheel {
        dy: i32,
        x: i32,
        y: i32,
        mods: i32,
    },
    /// input method: committed text + in-progress preedit
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

impl PaintWidget {
    /// handler receives every event; register nothing else
    pub fn new(parent: Option<&QWidget>, handler: impl FnMut(PaintWidgetEvent) + 'static) -> Self {
        let mut handler = handler;
        let id = dtk_sys::register_cb_pw(move |ev| {
            let ev = match ev {
                dtk_sys::PwEvent::Paint(p, w, h) => {
                    PaintWidgetEvent::Paint(Painter { ptr: p }, w, h)
                }
                dtk_sys::PwEvent::Key {
                    key,
                    mods,
                    text,
                    press,
                    autorepeat,
                } => PaintWidgetEvent::Key(KeyEvent {
                    key,
                    mods,
                    text,
                    press,
                    autorepeat,
                }),
                dtk_sys::PwEvent::Mouse {
                    kind,
                    button,
                    x,
                    y,
                    mods,
                } => PaintWidgetEvent::Mouse(MouseEvent {
                    kind,
                    button,
                    x,
                    y,
                    mods,
                }),
                dtk_sys::PwEvent::Wheel { dy, x, y, mods } => {
                    PaintWidgetEvent::Wheel { dy, x, y, mods }
                }
                dtk_sys::PwEvent::Ime { commit, preedit } => {
                    PaintWidgetEvent::Ime { commit, preedit }
                }
                dtk_sys::PwEvent::Resize { w, h } => PaintWidgetEvent::Resize { w, h },
                dtk_sys::PwEvent::Focus(gained) => PaintWidgetEvent::Focus(gained),
            };
            handler(ev);
        });
        Self::from_raw(unsafe { ffi::paint_widget_new(id, opt_ptr(parent)) })
    }
    /// test/helper: synchronously deliver a key-press (bypasses the OS event source)
    pub fn inject_key(&self, key: i32, mods: i32, text: &str) {
        unsafe { ffi::paint_widget_inject_key(self.ptr, key, mods, text) }
    }
    /// IME candidate window anchor: cursor rect in widget coords (call when it moves)
    pub fn set_ime_cursor_rect(&self, x: i32, y: i32, w: i32, h: i32) {
        unsafe { ffi::paint_widget_set_ime_rect(self.ptr, x, y, w, h) }
    }
}

/// vertical scrollbar (DScrollBar = QScrollBar in DTK6); child of a widget
#[derive(Clone, Copy)]
pub struct ScrollBar {
    w: QWidget,
}

impl ScrollBar {
    pub fn new(parent: &QWidget) -> Self {
        Self { w: QWidget::from_raw(unsafe { ffi::scrollbar_new(parent.ptr) }) }
    }
    pub fn as_widget(&self) -> QWidget {
        QWidget::from_raw(self.w.ptr)
    }
    pub fn set_range(&self, min: i32, max: i32) {
        unsafe { ffi::scrollbar_set_range(self.w.ptr, min, max) }
    }
    pub fn maximum(&self) -> i32 {
        unsafe { ffi::scrollbar_maximum(self.w.ptr) }
    }
    pub fn set_value(&self, v: i32) {
        unsafe { ffi::scrollbar_set_value(self.w.ptr, v) }
    }
    pub fn value(&self) -> i32 {
        unsafe { ffi::scrollbar_value(self.w.ptr) }
    }
    pub fn set_page_step(&self, v: i32) {
        unsafe { ffi::scrollbar_set_page_step(self.w.ptr, v) }
    }
    /// valueChanged(int); 0 on failure
    pub fn on_value_changed(&self, f: impl FnMut(i32) + 'static) -> usize {
        self.w.connect_signal_i32("valueChanged(int)", f)
    }
}

/// system clipboard (QGuiApplication::clipboard)
pub struct Clipboard;

impl Clipboard {
    pub fn set_text(text: &str) {
        unsafe { ffi::clipboard_set_text(text, 0) }
    }
    pub fn text() -> String {
        unsafe { ffi::clipboard_text(0) }
    }
    /// X11 primary selection (copy-on-select)
    pub fn set_selection(text: &str) {
        unsafe { ffi::clipboard_set_text(text, 1) }
    }
    pub fn selection() -> String {
        unsafe { ffi::clipboard_text(1) }
    }
}

/// connect an arg-less signal on any widget (e.g. clicked, timeout)
pub trait Signal0 {
    fn qobject_ptr(&self) -> *mut ffi::QObject;
    /// Full Qt signature, e.g. "clicked(bool)" / "timeout()"; signal args are ignored.
    /// Returns the callback id for [`unregister_callback`]; 0 = connect failed.
    fn connect_signal(&self, signal: &str, f: impl FnMut() + 'static) -> usize {
        let id = dtk_sys::register_cb0(f);
        if unsafe { ffi::relay_connect0(self.qobject_ptr(), signal, id) } {
            id
        } else {
            dtk_sys::unregister_cb(id); // roll back registration
            0
        }
    }
}

/// signal with one i32 arg (e.g. currentRowChanged(int))
pub trait SignalI32 {
    fn qobject_ptr(&self) -> *mut ffi::QObject;
    /// Returns the callback id for [`unregister_callback`]; 0 = connect failed.
    fn connect_signal_i32(&self, signal: &str, f: impl FnMut(i32) + 'static) -> usize {
        let id = dtk_sys::register_cb_i32(f);
        if unsafe { ffi::relay_connect_i32(self.qobject_ptr(), signal, id) } {
            id
        } else {
            dtk_sys::unregister_cb(id);
            0
        }
    }
}

/// signal with one bool arg (e.g. checkedChanged(bool))
pub trait SignalBool {
    fn qobject_ptr(&self) -> *mut ffi::QObject;
    /// Returns the callback id for [`unregister_callback`]; 0 = connect failed.
    fn connect_signal_bool(&self, signal: &str, f: impl FnMut(bool) + 'static) -> usize {
        let id = dtk_sys::register_cb_bool(f);
        if unsafe { ffi::relay_connect_bool(self.qobject_ptr(), signal, id) } {
            id
        } else {
            dtk_sys::unregister_cb(id);
            0
        }
    }
}

/// disconnect + unregister a connected signal callback (id from connect_signal*)
pub use dtk_sys::unregister_cb as unregister_callback;

// ---- DApplication ----

pub struct DApplication {
    ptr: *mut ffi::DApplication,
    _not_send: PhantomData<*mut ()>,
}

/// real process argv for QApplication; U+001F separator: cannot appear in a real argv entry
fn env_args_joined() -> String {
    std::env::args().collect::<Vec<_>>().join("\u{1f}")
}

impl DApplication {
    pub fn new(name: &str) -> Self {
        let args = env_args_joined();
        let ptr = unsafe { ffi::application_new(name, &args) };
        Self {
            ptr,
            _not_send: PhantomData,
        }
    }
    /// with quit guard: QEvent::Quit asks the guard; false swallows the event (Rust retries itself)
    pub fn new_with_quit_guard(name: &str, guard: impl FnMut() -> bool + 'static) -> Self {
        let id = dtk_sys::register_cb_guard(guard);
        let args = env_args_joined();
        let ptr = unsafe { ffi::application_new_ex(name, &args, id) };
        Self {
            ptr,
            _not_send: PhantomData,
        }
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
    /// true while a modal dialog or popup menu is active
    pub fn popup_active() -> bool {
        unsafe { ffi::app_popup_active() }
    }
    /// QPalette::Window as (r, g, b) — use to detect light vs dark DTK theme
    pub fn palette_window_rgb() -> (u8, u8, u8) {
        let rgb = unsafe { ffi::app_palette_window_rgb() };
        ((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8)
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

impl DTitlebar {
    pub fn set_title(&self, title: &str) {
        unsafe { ffi::titlebar_set_title(self.ptr, title) }
    }
    pub fn set_icon(&self, icon: &QIcon) {
        unsafe { ffi::titlebar_set_icon(self.ptr, &*icon.ptr) }
    }
}

// ---- value types ----

/// value-type wrapper: heap-allocated, owned by Rust (small leak acceptable)
macro_rules! value_wrapper {
    ($name:ident, $ffi:ty, $del:ident) => {
        pub struct $name {
            pub(crate) ptr: *mut $ffi,
            _not_send: PhantomData<*mut ()>,
        }
        impl $name {
            pub(crate) fn from_raw(ptr: *mut $ffi) -> Self {
                assert!(!ptr.is_null());
                Self {
                    ptr,
                    _not_send: PhantomData,
                }
            }
        }
        impl Drop for $name {
            fn drop(&mut self) {
                unsafe { ffi::$del(self.ptr) }
            }
        }
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, concat!(stringify!($name), "({:p})"), self.ptr)
            }
        }
    };
}

value_wrapper!(QColor, ffi::QColor, color_delete);
value_wrapper!(QFont, ffi::QFont, font_delete);
value_wrapper!(QPalette, ffi::QPalette, palette_delete);
value_wrapper!(QPixmap, ffi::QPixmap, pixmap_delete);
value_wrapper!(QPoint, ffi::QPoint, point_delete);
value_wrapper!(QRect, ffi::QRect, rect_delete);
value_wrapper!(QSize, ffi::QSize, size_delete);
value_wrapper!(QMargins, ffi::QMargins, margins_delete);
value_wrapper!(DDciIcon, ffi::DDciIcon, ddci_icon_delete);

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
    /// italic on/off
    pub fn set_italic(&self, italic: bool) {
        unsafe { ffi::font_set_italic(self.ptr, italic) }
    }
    /// generic monospace family (terminals, code)
    pub fn set_monospace(&self) {
        unsafe { ffi::font_set_monospace(self.ptr) }
    }
    /// set the font family by name (e.g. "Fira Code")
    pub fn set_family(&self, name: &str) {
        unsafe { ffi::font_set_family(self.ptr, name) }
    }
    /// QFont::ForceIntegerMetrics: terminals need integer per-cell advances,
    /// otherwise shaped runs drift off the cell grid (fractional 9.85 vs cell 10)
    pub fn force_integer_metrics(&self) {
        unsafe { ffi::font_force_integer_metrics(self.ptr) }
    }
    /// shaped advance of a text run (kerning/ligatures included)
    pub fn advance(&self, text: &str) -> i32 {
        unsafe { ffi::fontmetrics_advance(self.ptr, text) }
    }
    /// cell geometry for grid rendering: (max char width, line height, ascent)
    pub fn metrics(&self) -> (i32, i32, i32) {
        unsafe {
            (
                ffi::fontmetrics_max_width(self.ptr),
                ffi::fontmetrics_height(self.ptr),
                ffi::fontmetrics_ascent(self.ptr),
            )
        }
    }
}

impl QPalette {
    pub fn new() -> Self {
        Self::from_raw(unsafe { ffi::palette_new() })
    }
    /// use qt::palette_group / qt::palette_role constants
    pub fn set_color(&self, group: i32, role: i32, color: &QColor) {
        unsafe { ffi::palette_set_color(self.ptr, group, role, color.ptr) }
    }
    /// read a color (e.g. copy Active Highlight into the Inactive group)
    pub fn color(&self, group: i32, role: i32) -> QColor {
        QColor::from_raw(unsafe { ffi::palette_color(self.ptr, group, role) })
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

impl QMargins {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self::from_raw(unsafe { ffi::q_margins_new(left, top, right, bottom) })
    }
}

impl DDciIcon {
    pub fn new() -> Self {
        Self::from_raw(unsafe { ffi::ddci_icon_new() })
    }
    /// load from a .dci file path
    pub fn from_file(path: &str) -> Self {
        Self::from_raw(unsafe { ffi::ddci_icon_from_file(path) })
    }
}

/// Qt enum/QFlags constants (for params mapped to i32), grouped by enum
pub mod qt {
    /// Qt::Alignment
    pub mod alignment {
        pub const LEFT: i32 = 0x1;
        pub const RIGHT: i32 = 0x2;
        pub const HCENTER: i32 = 0x4;
        pub const TOP: i32 = 0x20;
        pub const BOTTOM: i32 = 0x40;
        pub const VCENTER: i32 = 0x80;
        pub const CENTER: i32 = 0x84;
    }
    /// Qt::FocusPolicy
    pub mod focus {
        pub const NO_FOCUS: i32 = 0;
    }
    /// Qt::ItemDataRole
    pub mod item_role {
        pub const USER_ROLE: i32 = 0x0100;
    }
    /// Qt::KeyboardModifier
    pub mod modifier {
        pub const SHIFT: i32 = 0x02000000;
        pub const CONTROL: i32 = 0x04000000;
        pub const ALT: i32 = 0x08000000;
        pub const META: i32 = 0x10000000;
        pub const KEYPAD: i32 = 0x20000000;
    }
    /// Qt::MouseButton
    pub mod mouse_button {
        pub const LEFT: i32 = 1;
        pub const RIGHT: i32 = 2;
        pub const MIDDLE: i32 = 4;
        pub const BACK: i32 = 8;
        pub const FORWARD: i32 = 16;
    }
    /// PaintWidget mouse event kinds
    pub mod mouse_kind {
        pub const PRESS: i32 = 0;
        pub const RELEASE: i32 = 1;
        pub const MOVE: i32 = 2;
        pub const DOUBLE_CLICK: i32 = 3;
    }
    /// Qt::CursorShape subset
    pub mod cursor {
        pub const ARROW: i32 = 0;
        pub const IBEAM: i32 = 4;
        pub const POINTING_HAND: i32 = 13;
    }
    /// Qt::Key (letters/digits are ASCII); common subset for terminal input
    pub mod key {
        pub const ESCAPE: i32 = 0x01000000;
        pub const TAB: i32 = 0x01000001;
        pub const BACKTAB: i32 = 0x01000002;
        pub const BACKSPACE: i32 = 0x01000003;
        pub const RETURN: i32 = 0x01000004;
        pub const ENTER: i32 = 0x01000005; // keypad
        pub const INSERT: i32 = 0x01000006;
        pub const DELETE: i32 = 0x01000007;
        pub const PAUSE: i32 = 0x01000008;
        pub const PRINT: i32 = 0x01000009;
        pub const CLEAR: i32 = 0x0100000b;
        pub const HOME: i32 = 0x01000010;
        pub const END: i32 = 0x01000011;
        pub const LEFT: i32 = 0x01000012;
        pub const UP: i32 = 0x01000013;
        pub const RIGHT: i32 = 0x01000014;
        pub const DOWN: i32 = 0x01000015;
        pub const PAGE_UP: i32 = 0x01000016;
        pub const PAGE_DOWN: i32 = 0x01000017;
        pub const SHIFT: i32 = 0x01000020;
        pub const CONTROL: i32 = 0x01000021;
        pub const META: i32 = 0x01000022;
        pub const ALT: i32 = 0x01000023;
        pub const CAPS_LOCK: i32 = 0x01000024;
        pub const NUM_LOCK: i32 = 0x01000025;
        pub const SCROLL_LOCK: i32 = 0x01000026;
        pub const F1: i32 = 0x01000030;
        pub const F2: i32 = 0x01000031;
        pub const F3: i32 = 0x01000032;
        pub const F4: i32 = 0x01000033;
        pub const F5: i32 = 0x01000034;
        pub const F6: i32 = 0x01000035;
        pub const F7: i32 = 0x01000036;
        pub const F8: i32 = 0x01000037;
        pub const F9: i32 = 0x01000038;
        pub const F10: i32 = 0x01000039;
        pub const F11: i32 = 0x0100003a;
        pub const F12: i32 = 0x0100003b;
    }
    /// Qt::Orientation
    pub mod orientation {
        pub const HORIZONTAL: i32 = 1;
        pub const VERTICAL: i32 = 2;
    }
    /// QFrame::Shape
    pub mod frame {
        pub const STYLED_PANEL: i32 = 0x6;
    }
    /// QStyle::StandardPixmap
    pub mod standard_pixmap {
        pub const MESSAGE_BOX_WARNING: i32 = 10;
    }
    /// QHeaderView::ResizeMode
    pub mod header_resize {
        pub const INTERACTIVE: i32 = 0;
        pub const STRETCH: i32 = 1;
        pub const RESIZE_TO_CONTENTS: i32 = 3;
    }
    /// QPalette::ColorGroup
    pub mod palette_group {
        pub const ACTIVE: i32 = 0;
        pub const DISABLED: i32 = 1;
        pub const INACTIVE: i32 = 2;
        pub const CURRENT: i32 = 3;
        pub const ALL: i32 = 5;
    }
    /// QPalette::ColorRole
    pub mod palette_role {
        pub const WINDOW_TEXT: i32 = 0;
        pub const TEXT: i32 = 6;
        pub const BASE: i32 = 9;
        pub const WINDOW: i32 = 10;
        pub const HIGHLIGHT: i32 = 12;
        pub const HIGHLIGHTED_TEXT: i32 = 13;
    }
    /// Qt::TextElideMode
    pub mod elide {
        pub const LEFT: i32 = 0;
        pub const RIGHT: i32 = 1;
        pub const MIDDLE: i32 = 2;
        pub const NONE: i32 = 3;
    }
    /// QStyleOption::State
    pub mod state {
        pub const SELECTED: i32 = 0x8000;
        pub const MOUSE_OVER: i32 = 0x2000;
    }
    /// QMessageBox::Icon
    pub mod msg_icon {
        pub const NO_ICON: i32 = 0;
        pub const INFORMATION: i32 = 1;
        pub const WARNING: i32 = 2;
        pub const CRITICAL: i32 = 3;
        pub const QUESTION: i32 = 4;
    }
    /// QMessageBox::ButtonRole
    pub mod msg_role {
        pub const INVALID: i32 = -1;
        pub const ACCEPT: i32 = 0;
        pub const REJECT: i32 = 1;
        pub const DESTRUCTIVE: i32 = 2;
        pub const ACTION: i32 = 3;
        pub const HELP: i32 = 4;
        pub const YES: i32 = 5;
        pub const NO: i32 = 6;
        pub const RESET: i32 = 7;
        pub const APPLY: i32 = 8;
    }
    /// QMessageBox::StandardButton (QFlags-compatible bitmask)
    pub mod msg_btn {
        pub const NO_BUTTON: i32 = 0x00000000;
        pub const OK: i32 = 0x00000400;
        pub const SAVE: i32 = 0x00000800;
        pub const SAVE_ALL: i32 = 0x00001000;
        pub const OPEN: i32 = 0x00002000;
        pub const YES: i32 = 0x00004000;
        pub const YES_TO_ALL: i32 = 0x00008000;
        pub const NO: i32 = 0x00010000;
        pub const NO_TO_ALL: i32 = 0x00020000;
        pub const ABORT: i32 = 0x00040000;
        pub const RETRY: i32 = 0x00080000;
        pub const IGNORE: i32 = 0x00100000;
        pub const CLOSE: i32 = 0x00200000;
        pub const CANCEL: i32 = 0x00400000;
        pub const DISCARD: i32 = 0x00800000;
        pub const HELP: i32 = 0x01000000;
        pub const APPLY: i32 = 0x02000000;
        pub const RESET: i32 = 0x04000000;
        pub const RESTORE_DEFAULTS: i32 = 0x08000000;
        /// convenience: standard Yes|No button set
        pub const YES_NO: i32 = 0x00014000;
        /// convenience: standard Ok|Cancel button set
        pub const OK_CANCEL: i32 = 0x00400400;
    }
}

// ---- QIcon ----

value_wrapper!(QIcon, ffi::QIcon, icon_delete);

impl QIcon {
    pub fn from_theme(name: &str) -> Self {
        Self::from_raw(unsafe { ffi::icon_from_theme(name) })
    }
    /// like from_theme, but falls back to `fallback` when `name` is not in the icon theme.
    pub fn from_theme_with_fallback(name: &str, fallback: &QIcon) -> Self {
        Self::from_raw(unsafe { ffi::icon_from_theme_fallback(name, fallback.ptr) })
    }
    pub fn from_file(path: &str) -> Self {
        Self::from_raw(unsafe { ffi::icon_from_file(path) })
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
    /// use qt::alignment::* constants for alignment
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
    pub fn set_text(&self, text: &str) {
        unsafe { ffi::button_set_text(self.ptr.cast(), text) }
    }
    pub fn on_clicked(&self, f: impl FnMut() + 'static) -> usize {
        self.connect_signal("clicked(bool)", f)
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
    pub fn on_clicked(&self, f: impl FnMut() + 'static) -> usize {
        self.connect_signal("clicked(bool)", f)
    }
}

// ---- DMessageBox (DMessageBox = typedef QMessageBox) ----

widget_wrapper!(DMessageBox, ffi::QMessageBox);

impl DMessageBox {
    /// create an empty dialog; set title/text/icon/buttons then exec()
    pub fn new() -> Self {
        Self::from_raw(unsafe { ffi::qmessagebox_new() })
    }
    /// create a pre-configured dialog; use qt::msg_icon::* and qt::msg_btn::* constants
    pub fn with(
        icon: i32,
        title: &str,
        text: &str,
        buttons: i32,
        parent: Option<&QWidget>,
    ) -> Self {
        let p = opt_ptr(parent);
        Self::from_raw(unsafe { ffi::qmessagebox_new_with(icon, title, text, buttons, p) })
    }
    pub fn set_text(&self, text: &str) {
        unsafe { ffi::qmessagebox_set_text(self.ptr, text) }
    }
    pub fn set_icon(&self, icon: i32) {
        unsafe { ffi::qmessagebox_set_icon(self.ptr, icon) }
    }
    pub fn set_informative_text(&self, text: &str) {
        unsafe { ffi::qmessagebox_set_informative_text(self.ptr, text) }
    }
    pub fn set_detailed_text(&self, text: &str) {
        unsafe { ffi::qmessagebox_set_detailed_text(self.ptr, text) }
    }
    pub fn set_standard_buttons(&self, buttons: i32) {
        unsafe { ffi::qmessagebox_set_standard_buttons(self.ptr, buttons) }
    }
    /// add a custom button with a text + ButtonRole; returns the button handle
    pub fn add_button(&self, text: &str, role: i32) -> DPushButton {
        DPushButton::from_raw(unsafe { ffi::qmessagebox_add_button_text(self.ptr, text, role) })
    }
    /// add a standard button (qt::msg_btn::*); returns the button handle
    pub fn add_standard_button(&self, button: i32) -> DPushButton {
        DPushButton::from_raw(unsafe { ffi::qmessagebox_add_button_standard(self.ptr, button) })
    }
    pub fn set_default_button(&self, button: i32) {
        unsafe { ffi::qmessagebox_set_default_button(self.ptr, button) }
    }
    /// exec the dialog (blocking); returns the clicked StandardButton (qt::msg_btn::*)
    pub fn exec(&self) -> i32 {
        unsafe { ffi::qmessagebox_exec(self.ptr) }
    }
    /// which standard button was clicked after exec returns
    pub fn clicked_button(&self) -> i32 {
        unsafe { ffi::qmessagebox_clicked_button(self.ptr) }
    }
    pub fn text(&self) -> String {
        unsafe { ffi::qmessagebox_text(self.ptr) }
    }

    // ---- static helpers ----
    /// information dialog; returns clicked StandardButton
    pub fn information(
        parent: Option<&QWidget>,
        title: &str,
        text: &str,
        buttons: i32,
        default_button: i32,
    ) -> i32 {
        let p = opt_ptr(parent);
        unsafe { ffi::qmessagebox_information(p, title, text, buttons, default_button) }
    }
    pub fn warning(
        parent: Option<&QWidget>,
        title: &str,
        text: &str,
        buttons: i32,
        default_button: i32,
    ) -> i32 {
        let p = opt_ptr(parent);
        unsafe { ffi::qmessagebox_warning(p, title, text, buttons, default_button) }
    }
    pub fn critical(
        parent: Option<&QWidget>,
        title: &str,
        text: &str,
        buttons: i32,
        default_button: i32,
    ) -> i32 {
        let p = opt_ptr(parent);
        unsafe { ffi::qmessagebox_critical(p, title, text, buttons, default_button) }
    }
    pub fn question(
        parent: Option<&QWidget>,
        title: &str,
        text: &str,
        buttons: i32,
        default_button: i32,
    ) -> i32 {
        let p = opt_ptr(parent);
        unsafe { ffi::qmessagebox_question(p, title, text, buttons, default_button) }
    }
    pub fn about(parent: Option<&QWidget>, title: &str, text: &str) {
        let p = opt_ptr(parent);
        unsafe { ffi::qmessagebox_about(p, title, text) }
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
                let p = opt_ptr(parent);
                Self {
                    ptr: unsafe { ffi::$new(p) },
                    _not_send: PhantomData,
                }
            }
            pub fn add_widget(&self, w: &QWidget) {
                unsafe { ffi::layout_add_widget(self.ptr.cast(), w.ptr) }
            }
            /// box layouts only: stretch factor + qt::alignment::* alignment
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
            /// leak the Rust handle; Qt parent-child still owns the layout
            pub fn leak(self) {
                // ManuallyDrop (not forget): handles are Copy, forget would be a no-op warning
                let _ = std::mem::ManuallyDrop::new(self);
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
            Some(QTableWidgetItem {
                ptr: p,
                _not_send: PhantomData,
            })
        }
    }
    pub fn select_row(&self, row: i32) {
        unsafe { ffi::table_select_row(self.ptr, row) }
    }
    pub fn hide_headers(&self, horizontal: bool, vertical: bool) {
        unsafe { ffi::table_hide_headers(self.ptr, horizontal, vertical) }
    }
    /// use qt::header_resize::* constants for mode
    pub fn set_section_resize_mode(&self, col: i32, mode: i32) {
        unsafe { ffi::table_set_section_resize_mode(self.ptr, col, mode) }
    }
    pub fn set_vertical_header_default_section_size(&self, size: i32) {
        unsafe { ffi::table_set_vertical_header_default_section_size(self.ptr, size) }
    }
    pub fn set_show_grid(&self, show: bool) {
        unsafe { ffi::table_set_show_grid(self.ptr, show) }
    }
    /// use qt::frame::* constants for shape
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
        Self {
            ptr: unsafe { ffi::timer_new(std::ptr::null_mut()) },
            _not_send: PhantomData,
        }
    }
    pub fn start(&self, msec: i32) {
        unsafe { ffi::timer_start(self.ptr, msec) }
    }
    pub fn stop(&self) {
        unsafe { ffi::timer_stop(self.ptr) }
    }
    /// Returns the callback id for [`unregister_callback`]; 0 = connect failed.
    pub fn on_timeout(&self, f: impl FnMut() + 'static) -> usize {
        let id = dtk_sys::register_cb0(f);
        if unsafe { ffi::relay_connect0(self.ptr.cast(), "timeout()", id) } {
            id
        } else {
            dtk_sys::unregister_cb(id);
            0
        }
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

// ---- QTableWidgetItem ----

pub struct QTableWidgetItem {
    pub(crate) ptr: *mut ffi::QTableWidgetItem,
    _not_send: PhantomData<*mut ()>,
}

impl QTableWidgetItem {
    pub fn set_icon(&self, icon: &QIcon) {
        unsafe { ffi::item_set_icon(self.ptr, icon.ptr) }
    }
    /// use qt::alignment::* constants for alignment
    pub fn set_text_alignment(&self, alignment: i32) {
        unsafe { ffi::item_set_text_alignment(self.ptr, alignment) }
    }
    pub fn set_foreground(&self, color: &QColor) {
        unsafe { ffi::item_set_foreground(self.ptr, color.ptr) }
    }
    /// use qt::item_role::USER_ROLE + n for role
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
    /// draw text with the baseline at (x, y) — for cell-grid rendering
    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        unsafe { ffi::painter_draw_line(self.ptr, x1, y1, x2, y2) }
    }
    pub fn draw_text_at(&self, x: i32, y: i32, text: &str) {
        unsafe { ffi::painter_draw_text_at(self.ptr, x, y, text) }
    }
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
    /// combine qt::alignment::* for flags
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
    /// elide text to width using the painter's current font; use qt::elide::* for mode
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
    /// f: (painter, index, x, y, w, h, state). test state against qt::state::*
    pub fn new(f: impl FnMut(&Painter, &ModelIndex, i32, i32, i32, i32, i32) + 'static) -> Self {
        let mut f = f;
        let id = dtk_sys::register_cb_paint(move |p, idx, x, y, w, h, state| {
            f(
                &Painter { ptr: p },
                &ModelIndex { ptr: idx },
                x,
                y,
                w,
                h,
                state,
            );
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
        Self {
            ptr: unsafe { ffi::socket_notifier_new(fd) },
            _not_send: PhantomData,
        }
    }
    /// stop firing (e.g. watched fd hit EOF and stays "readable" forever)
    pub fn set_enabled(&self, on: bool) {
        unsafe { ffi::socket_notifier_set_enabled(self.ptr, on) }
    }
    pub fn on_activated(&self, f: impl FnMut() + 'static) -> usize {
        let id = dtk_sys::register_cb0(f);
        if unsafe { ffi::relay_connect0(self.ptr.cast(), "activated(QSocketDescriptor)", id) } {
            id
        } else {
            dtk_sys::unregister_cb(id);
            0
        }
    }
}

// ---- DProgressBar: base-class QProgressBar ops (generator only scans DTK headers) ----

impl widgets::DProgressBar {
    pub fn set_value(&self, value: i32) {
        unsafe { ffi::progressbar_set_value(self.ptr.cast(), value) }
    }
    /// (0, 0) = busy/indeterminate mode
    pub fn set_range(&self, minimum: i32, maximum: i32) {
        unsafe { ffi::progressbar_set_range(self.ptr.cast(), minimum, maximum) }
    }
    pub fn value(&self) -> i32 {
        unsafe { ffi::progressbar_value(self.ptr.cast()) }
    }
}

impl_default!(DMainWindow, QFont, QPalette, DDciIcon, DMessageBox, QTimer);

/// leak() for the hand-rolled non-macro wrappers (same semantics as object_wrapper::leak)
macro_rules! impl_leak {
    ($($name:ident),* $(,)?) => {
        $(impl $name {
            /// leak the Rust handle; Qt parent-child still owns the object
            pub fn leak(self) {
                // ManuallyDrop (not forget): handles are Copy, forget would be a no-op warning
                let _ = std::mem::ManuallyDrop::new(self);
            }
        })*
    };
}
impl_leak!(QTimer, QSocketNotifier, PaintDelegate);

// ---- typed signal helpers + base-class getters for generated widgets ----

impl widgets::DLineEdit {
    /// Returns the callback id for [`unregister_callback`]; 0 = connect failed.
    pub fn on_return_pressed(&self, f: impl FnMut() + 'static) -> usize {
        self.connect_signal("returnPressed()", f)
    }
}

impl widgets::DSwitchButton {
    /// checkedChanged(bool) with the bool delivered. Returns id; 0 = connect failed.
    pub fn on_checked_changed(&self, f: impl FnMut(bool) + 'static) -> usize {
        self.connect_signal_bool("checkedChanged(bool)", f)
    }
}

impl widgets::DSearchEdit {
    /// QLineEdit base-class getter, via the DLineEdit ancestry
    pub fn text(&self) -> String {
        unsafe { ffi::line_edit_text(self.ptr.cast()) }
    }
}

impl widgets::DComboBox {
    /// currentIndexChanged(int). Returns id; 0 = connect failed.
    pub fn on_current_index_changed(&self, f: impl FnMut(i32) + 'static) -> usize {
        self.connect_signal_i32("currentIndexChanged(int)", f)
    }
}

// generator output: bindings for the rest of dtkwidget
pub mod widgets;
