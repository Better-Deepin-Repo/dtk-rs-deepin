# dtk-rs

Rust bindings for DTK6 (dtkwidget). Goal: write deepin apps with DTK UIs in pure Rust.

## Layout

```
dtk-sys/   FFI layer: C++ shim (cpp/) + cxx::bridge + build.rs (locates Qt6/DTK6 via pkg-config, moc for the signal relay)
dtk/       safe wrappers: hand-written core classes + widgets.rs (generator output)
tools/gen.py  header-scanning generator; rescans /usr/include/dtk6/DWidget/*.h and regenerates bindings
GEN_REPORT.md coverage report: what was generated, what was skipped (with reasons)
TODO.md      known gaps and follow-ups
```

## Usage

```rust
use dtk::*;

let app = DApplication::new("my-app");
let win = DMainWindow::new();
win.titlebar().set_title("Hello");
let btn = DSuggestButton::new("Click me");
btn.on_clicked(|| println!("clicked"));
btn.show();
win.resize(400, 300);
win.show();
std::process::exit(app.exec());
```

Any signal can be connected (runtime connect by name):

```rust
widget.connect_signal("windowRadiusChanged()", || { ... });
widget.connect_signal_i32("currentRowChanged(int)", |row| { ... });
```

The 60 generator-covered classes live under `dtk::widgets` (DComboBox, DSpinner, DDialog...).
Run the demo: `cargo run --example demo`; headless smoke test: `QT_QPA_PLATFORM=offscreen ./target/debug/examples/demo --smoke`.

## Regenerating

```
python3 tools/gen.py && cargo build
```

## Design

- **Lifetime**: Qt parent-child owns everything; Rust wrappers are non-owning raw pointers (!Send, single GUI thread).
- **Signals**: `DtkRelay` (Q_OBJECT + SLOT) string-connects any signal -> Rust callback id -> closure in a thread_local registry.
- **Event overrides**: `DtkAppEx` (QEvent::Quit guard) / `DtkMainWindowEx` (showEvent/closeEvent) shim subclasses -> Rust callbacks. Entry points: `DApplication::new_with_quit_guard`, `DMainWindow::new_with_events`.
- **Custom painting**: `PaintDelegate` (QStyledItemDelegate subclass) forwards paint to Rust, with `Painter` primitives + `ModelIndex::data_*`.
- **Type mapping**: QString<->&str/String, numbers direct, enums/QFlags->i32 (`dtk::qt` constants module), QColor/QFont/QPalette/QPixmap/QSize/QPoint/QRect value types -> heap-allocated wrappers, QWidget* and DTK class pointers <-> wrappers.
- **QSocketNotifier**: `QSocketNotifier::new(fd)` + `on_activated`; pairs with signalfd/pipe/eventfd.
- **Generator**: regex-parses DTK headers (they are very regular). Only methods whose param/return types all map cleanly are generated; the rest land in GEN_REPORT.md with reasons.
