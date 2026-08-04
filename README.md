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
- **类型映射**：QString↔&str/String、数值类型直映、QWidget* 与 DTK 类指针↔wrapper。其余（QColor、QRect、枚举、模板容器……）暂不支持，见 GEN_REPORT.md，按需扩 `tools/gen.py` 的类型表。
- **生成器**：正则解析 DTK 头文件（风格规整），所有参数/返回类型都可映射的方法才会生成，其余进报告，不生成编译不过的代码。

## 跳过项（需要时再加）

- 值类型 QColor/QPoint/QRect/QIcon 参数（shim 转 POD struct 即可扩）
- 枚举参数/返回（映射 i32 + 常量）
- DGui/DCore 两个库（同套路再扫两个目录）
