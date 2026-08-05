# dtk-rs architecture

How Rust bindings for DTK6 (dtkwidget) are implemented. For usage see [README.md](README.md);
for known gaps see [TODO.md](TODO.md).

## Big picture

Three layers plus a header-scanning generator:

```
┌────────────────────────────────────────────────────────────────┐
│ dtk/            safe layer (pure Rust, no unsafe in app code)  │
│   src/lib.rs      hand-written: core classes, macros, signals  │
│   src/widgets.rs  GENERATED: ~60 DTK class wrappers            │
│   ↑ raw-pointer structs, non-owning, !Send                     │
├────────────────────────────────────────────────────────────────┤
│ cxx::bridge #1  dtk-sys/src/lib.rs      hand-written core      │
│ cxx::bridge #2  dtk-sys/src/gen_ffi.rs  GENERATED (~100 types) │
│   ↑ cxx generates the Rust↔C++ ABI glue                        │
├────────────────────────────────────────────────────────────────┤
│ dtk-sys/cpp/    C++ shim (the unsafe FFI layer)                │
│   shim.cpp           hand-written: core + event-override       │
│                      subclasses + painter primitives           │
│   dtk_gen_shim.cpp   GENERATED: one free fn per DTK method     │
│   relay.cpp          DtkRelay (Q_OBJECT) signal forwarding     │
│   ↓ calls the real DTK6/Qt6 API                                │
├────────────────────────────────────────────────────────────────┤
│ dtk6widget / Qt6   (pkg-config link, moc for relay.h)          │
└────────────────────────────────────────────────────────────────┘

tools/gen.py  scans /usr/include/dtk6/DWidget/*.h and regenerates
              the four files marked GENERATED (+ GEN_REPORT.md)
```

## Why a C++ shim at all

cxx can only bridge types it knows: primitives, `rust::String`/`rust::Str`,
opaque C++ types behind pointers, and a few std containers. Qt types — `QString`,
`QList`, enums, overloaded methods — cannot cross. So every DTK method is
flattened into a **free function in namespace `dtkrs`** whose signature uses
only mappable types:

```cpp
// dtk_gen_shim.cpp — generated, one line per method
DComboBox *gen_d_combo_box_new() { return new DComboBox; }
void gen_d_combo_box_show_popup(DComboBox *self) { self->showPopup(); }
rust::String gen_d_about_dialog_product_name(DAboutDialog *self) { return to_rust_string(self->productName()); }
void gen_d_alert_control_set_message_alignment(DAlertControl *self, int32_t a) { self->setMessageAlignment(Qt::Alignment::fromInt(a)); }
QColor * gen_d_alert_control_alert_color(DAlertControl *self) { return new QColor(self->alertColor()); }
```

- `this` becomes the first parameter (`self`).
- Value-type returns are heap copies (`new QColor(...)`) — ownership passes to Rust.
- Enum/flags cross as `int32_t` with `static_cast` / `fromInt()`/`toInt()` at the boundary.
- `QString` crosses as `rust::Str`/`rust::String` via `from_rust_str`/`to_rust_string` helpers.

## End-to-end walkthrough: DComboBox

### 1. The DTK header

```cpp
// /usr/include/dtk6/DWidget/dcombobox.h
class DComboBox : public QComboBox, public DObject {
    Q_OBJECT
public:
    explicit DComboBox(QWidget *parent = nullptr);
    void showPopup() override;
    bool eventFilter(QObject *watched, QEvent *event) override;  // QEvent* unmapped → skipped
};
```

### 2. gen.py decides what to emit

Each public method's parameter/return types are checked against the type map:

| Method | Verdict |
|---|---|
| `DComboBox(QWidget *parent = nullptr)` | ctor with default args → emit `new()` |
| `void showPopup()` | clean → emit |
| `bool eventFilter(QObject*, QEvent*)` | `QEvent*` unmapped → skip, record in GEN_REPORT.md |

GEN_REPORT.md entry:

```
## DComboBox — 1 methods generated, 1 skipped
- `void addComboBox(const QString &text, const DComboBoxOptions &options);`
  ← unsupported param type: const DComboBoxOptions &
```

Skipped ≠ lost forever: the fix is usually adding a type mapping or declaring
another Qt class in both bridges (see TODO.md).

### 3. The four generated artifacts

```cpp
// dtk-sys/include/dtk_gen_shim.h — declaration
using Dtk::Widget::DComboBox;                 // DTK namespace flattened
DComboBox *gen_d_combo_box_new();
void gen_d_combo_box_show_popup(DComboBox *self);
```

```rust
// dtk-sys/src/gen_ffi.rs — cxx::bridge #2
#[cxx::bridge(namespace = "dtkrs")]
pub mod genffi {
    extern "C++" {
        include!("dtk_gen_shim.h");
        type DComboBox;                                      // opaque
        unsafe fn gen_d_combo_box_new() -> *mut DComboBox;
        unsafe fn gen_d_combo_box_show_popup(self_: *mut DComboBox);
    }
}
```

```rust
// dtk/src/widgets.rs — safe wrapper
widget_wrapper!(DComboBox, genffi::DComboBox);
impl DComboBox {
    pub fn new() -> Self {
        Self::from_raw(unsafe { genffi::gen_d_combo_box_new() })
    }
    pub fn show_popup(&self) {
        unsafe { genffi::gen_d_combo_box_show_popup(self.ptr) }
    }
}
impl Default for DComboBox { fn default() -> Self { Self::new() } }
```

`widget_wrapper!` (in `dtk/src/lib.rs`) expands to the struct plus all common
QWidget operations:

```rust
pub struct DComboBox {
    pub(crate) ptr: *mut genffi::DComboBox,   // non-owning raw pointer
    _not_send: PhantomData<*mut ()>,           // !Send/!Sync — GUI thread only
}
impl DComboBox {
    pub(crate) fn from_raw(ptr: *mut genffi::DComboBox) -> Self { ... }
    pub fn as_widget(&self) -> QWidget { ... }   // base-class view, for layouts
    pub fn show(&self)        { unsafe { ffi::widget_show(self.ptr.cast()) } }
    pub fn resize(&self, ...) { ... }
    // ... every common QWidget op, from hand-written bridge #1
}
impl Signal0 for DComboBox { ... }   // connect any arg-less signal
impl SignalI32 for DComboBox { ... } // connect any (int) signal
```

Note: **no `Drop` on widgets** — see Lifetime below.

### 4. Call chain at runtime

```rust
let combo = DComboBox::new();
// → genffi::gen_d_combo_box_new()     cxx ABI glue
//   → dtkrs::gen_d_combo_box_new()   C++ shim
//     → new DComboBox                real DTK object

combo.show();
// → ffi::widget_show(self.ptr.cast())   hand bridge #1 (base-class op)
//   → w->show()

combo.show_popup();
// → genffi::gen_d_combo_box_show_popup(self.ptr)   generated bridge #2
//   → self->showPopup()
```

Base-class Qt methods (e.g. `QProgressBar::setValue`) are not in DTK headers,
so the generator never sees them — they're added by hand to shim.cpp /
bridge #1 when needed (e.g. `impl widgets::DProgressBar` in dtk/src/lib.rs).

## Type mapping

| C++ | crosses the bridge as | Rust safe side |
|---|---|---|
| `QString` | `rust::Str` (in) / `rust::String` (out) | `&str` / `String` |
| `int`/`qint32`, `bool`, `qreal`… | `int32_t`, `bool`, `double`… | `i32`, `bool`, `f64`… |
| enums | `int32_t` + `static_cast` | `i32`; constants in `dtk::qt` |
| `QFlags` (e.g. `Qt::Alignment`) | `int32_t` + `fromInt`/`toInt` | `i32` |
| value types (`QColor`, `QFont`, `QPixmap`, `QIcon`, `QPalette`, `QSize`, `QPoint`, `QRect`, `QMargins`, `DDciIcon`) | owning heap pointer | wrapper with `Drop` → shim `*_delete` |
| `QWidget*` / DTK class pointers | opaque pointer | non-owning wrapper struct |
| `QList<T>`, `QVariant`, `QAction*`, … | — | **unmapped → skipped** (GEN_REPORT.md, TODO.md) |

Two bridges, one namespace: bridge #1 (hand, core) and #2 (generated, bulk)
both declare opaque types in `namespace dtkrs`. Only `QWidget*` and a fixed
list of value types cross between the two bridges, via a `as _` pointer cast.
Sound because cxx opaque types are zero-sized Rust structs that are never
dereferenced or moved in Rust, and both bridges name the same C++ type.
The audit is recorded in tools/gen.py comments.

## Signals: runtime connect by name

No per-signal glue code. Any signal on any QObject can be connected at runtime:

```rust
btn.connect_signal("clicked()", || { ... });
table.connect_signal_i32("currentRowChanged(int)", |row| { ... });
```

Mechanism:

```
Rust:  connect_signal("clicked()", f)
         1. register_cb0(f) → id      closure boxed into thread_local HashMap<usize, Cb>
         2. relay_connect0(obj, "clicked()", id)
C++:   DtkRelay (Q_OBJECT, SLOTs fire0()/fireI32(int))
         3. string-based connect: "2" + QMetaObject::normalizedSignature(signal)
            QObject::connect(sender, sig, relay, SLOT(fire0()))
         4. on signal: fire0() → extern "Rust" dtk_cb0(id)
Rust:  5. dtk_cb0(id) looks the id up and calls the closure
```

Details that matter:

- The string-based `connect(sender, SIGNAL, relay, SLOT)` form is used because
  Qt6 dropped the unregistered-`QMetaMethod` + lambda connect overloads.
- The relay object is parented to the sender → destroyed with it automatically.
- `relay.h` contains `Q_OBJECT`, so `build.rs` runs **moc** on it.
- The registry is `thread_local` — no locks; callbacks only ever fire on the Qt
  main thread.
- Dispatch is **remove-then-call-then-reinsert**, so a callback may safely
  register new callbacks (no RefCell double-borrow).
- `dtk::unregister_callback(id)` removes the closure; the Qt-side connection
  stays but becomes a harmless no-op.
- Signals with other argument types (e.g. `checkedChanged(bool)`) can still be
  connected via the arg-less relay — the argument is dropped; read the state
  from the widget inside the closure.

## Event overrides and custom painting

Signals can't intercept events. For those, shim.cpp subclasses DTK/Qt classes
and forwards to Rust callback ids (same registry):

| C++ subclass | Overrides | Rust entry point |
|---|---|---|
| `DtkAppEx : DApplication` | `event()` — `QEvent::Quit` asks a guard; `false` swallows the quit | `DApplication::new_with_quit_guard(guard)` |
| `DtkMainWindowEx : DMainWindow` | `showEvent` / `closeEvent` (close can be vetoed) | `DMainWindow::new_with_events(...)` |
| `RustDelegate : QStyledItemDelegate` | `paint` — paints the default DTK background first, then calls Rust for the overlay (icon/text) | `PaintDelegate::new(...)` |

`dtk_cb_paint` hands Rust the raw `QPainter*`/`QModelIndex*`; the safe layer
wraps them as `Painter` (draw_text/draw_pixmap/fill_rect/…) and `ModelIndex`
(`data_string`/`data_i64`/…).

## Lifetime & memory

Two wrapper kinds, deliberately different:

1. **Widget/object wrappers** (`DComboBox`, `DLabel`, layouts, …): non-owning
   raw pointers, **no `Drop`**. Qt's parent-child tree owns the objects:
   children die with their parent, top-level windows die with `QApplication`.
   Use `delete_later()` for explicit deferred destruction.
2. **Value-type wrappers** (`QColor`, `QFont`, `QPixmap`, `QIcon`, `QPalette`,
   `QSize`, `QPoint`, `QRect`, `QMargins`, `DDciIcon`): own a heap copy created
   by the shim (`new QColor(...)`), freed in `Drop` via shim `*_delete`.

All wrappers carry `PhantomData<*mut ()>` → `!Send`/`!Sync`. Qt widgets may
only be touched from the GUI thread; the compiler enforces it. Callbacks are
`'static` — share state into them with `Rc`/`RefCell`/`Cell` (never
`Arc<Mutex>`: everything runs on one thread anyway).

## Threading: worker → GUI

Since widgets are `!Send`, background work reaches the UI through plain `Send`
data + a wakeup fd, the same trick as Qt's queued `invokeMethod`:

```
 worker thread                    GUI thread
 tx.send(msg) ── mpsc ──────────▶ receiver drained with try_recv()
 write(eventfd) ── 8 bytes ─────▶ QSocketNotifier::on_activated(...)
                                   → callback runs on GUI thread
```

Always drain the eventfd counter in the callback (level-triggered notifier
refires forever otherwise). Full working example with a headless `--smoke`
check: `dtk/examples/mpsc_eventfd.rs`. Same pattern works for signalfd/pipes.

## Build

`dtk-sys/build.rs`:

1. locates Qt6 **moc** (`/usr/lib/qt6/libexec/moc` → fallbacks), runs it on
   `include/relay.h` (the only header with `Q_OBJECT`);
2. `cxx_build::bridges(["src/lib.rs", "src/gen_ffi.rs"])` — both bridges;
3. compiles `shim.cpp` + `relay.cpp` + `dtk_gen_shim.cpp` + moc output with
   `-std=c++17 -fPIC` (Qt6 requires PIC);
4. `pkg-config` probes `Qt6Widgets/Gui/Core` + `dtk6widget/gui/core` for
   include paths, defines, and link flags;
5. `links = "dtk6widget"` in dtk-sys/Cargo.toml.

## Which files are hand-written vs generated

| File | Owner |
|---|---|
| `dtk-sys/src/lib.rs` (bridge #1, callback registry) | hand |
| `dtk-sys/cpp/shim.cpp`, `include/dtk_shim.h` | hand |
| `dtk-sys/cpp/relay.cpp`, `include/relay.h` | hand |
| `dtk-sys/build.rs` | hand |
| `dtk/src/lib.rs` (core classes, macros, signals, qt constants) | hand |
| `dtk-sys/src/gen_ffi.rs` | **generated** |
| `dtk-sys/cpp/dtk_gen_shim.cpp`, `include/dtk_gen_shim.h` | **generated** |
| `dtk/src/widgets.rs` | **generated** |
| `GEN_REPORT.md` | generated report |

Regenerate after DTK upgrades or when adding coverage:

```sh
python3 tools/gen.py && cargo build
```

Adding a new DTK method almost never needs hand work — extend the generator's
type map if the types are mappable, regenerate, done. Hand-written additions
(new Qt base-class methods, new value types, new event overrides) go in
shim.cpp + bridge #1.
