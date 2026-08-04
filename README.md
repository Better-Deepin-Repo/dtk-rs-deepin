# dtk-rs

DTK6 (dtkwidget) 的 Rust binding。目标：deepin 应用可以纯 Rust 写 DTK 界面。

## 结构

```
dtk-sys/   FFI 层：C++ shim（cpp/）+ cxx::bridge + build.rs（pkg-config 定位 Qt6/DTK6，moc 处理信号转发器）
dtk/       安全 wrapper：手写核心类 + widgets.rs（生成器产物）
tools/gen.py  头文件扫描生成器，扫 /usr/include/dtk6/DWidget/*.h 重新生成绑定
GEN_REPORT.md 覆盖报告：哪些方法生成了，哪些跳过（含原因）
```

## 用法

```rust
use dtk::*;

let app = DApplication::new("my-app");
let win = DMainWindow::new();
win.titlebar().set_title("Hello");
let btn = DSuggestButton::new("点我");
btn.on_clicked(|| println!("clicked"));
btn.show();
win.resize(400, 300);
win.show();
std::process::exit(app.exec());
```

任意信号都能接（运行时按名字连接）：

```rust
widget.connect_signal("windowRadiusChanged()", || { ... });
widget.connect_signal_i32("currentRowChanged(int)", |row| { ... });
```

生成器覆盖的 60 个类在 `dtk::widgets` 下（DComboBox、DSpinner、DDialog……）。
跑 demo：`cargo run --example demo`；无头冒烟：`QT_QPA_PLATFORM=offscreen ./target/debug/examples/demo --smoke`。

## 重新生成

```
python3 tools/gen.py && cargo build
```

## 设计

- **生命周期**：Qt parent-child 管理，Rust wrapper 只是非拥有裸指针（!Send，GUI 单线程）。
- **信号**：`DtkRelay`（Q_OBJECT + SLOT）字符串式 connect 任意信号 → Rust 回调 id → thread_local 注册表里的闭包。
- **事件重写**：`DtkAppEx`（QEvent::Quit 守卫）/ `DtkMainWindowEx`（showEvent/closeEvent）shim 子类 → Rust 回调。入口：`DApplication::new_with_quit_guard`、`DMainWindow::new_with_events`。
- **自定义绘制**：`PaintDelegate`（QStyledItemDelegate 子类）paint 全转 Rust，配 `Painter` 原语 + `ModelIndex::data_*`。
- **类型映射**：QString↔&str/String、数值直映、枚举/QFlags→i32（`dtk::qt` 常量模块）、QColor/QFont/QPalette/QPixmap/QSize/QPoint/QRect 值类型→堆分配 wrapper、QWidget* 与 DTK 类指针↔wrapper。
- **QSocketNotifier**：`QSocketNotifier::new(fd)` + `on_activated`，配 signalfd/pipe/eventfd 用。
- **生成器**：正则解析 DTK 头文件（风格规整），所有参数/返回类型都可映射的方法才会生成，其余进 GEN_REPORT.md。

## 跳过项（需要时再加）

- Qt 容器类型（QList/QMap/QVariant 通用化）
- 非导出嵌套类（DPrinter 等）
- DGui/DCore 两个库（同套路再扫两个目录）
