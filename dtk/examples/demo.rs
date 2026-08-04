use dtk::*;
use std::cell::Cell;
use std::rc::Rc;

fn main() {
    let app = DApplication::new("dtk-rs-demo");

    let win = DMainWindow::new();
    win.set_window_title("dtk-rs demo");
    let tb = win.titlebar();
    tb.set_title("dtk-rs demo");
    tb.set_icon(&QIcon::from_theme("deepin-liferaft"));

    let central = QWidget::new(None);
    let vbox = QVBoxLayout::new(Some(&central));
    vbox.set_contents_margins(20, 20, 20, 20);
    vbox.set_spacing(12);

    let label = DLabel::new("DTK6 Rust binding 跑通了。");
    vbox.add_widget(&label.as_widget());

    let btn = DSuggestButton::new("点我");
    let clicked = Rc::new(Cell::new(false));
    {
        let clicked = clicked.clone();
        btn.on_clicked(move || {
            label.set_text("按钮点过了 ✓");
            clicked.set(true);
            println!("clicked");
        });
    }
    vbox.add_widget(&btn.as_widget());

    win.set_central_widget(&central);
    win.resize(400, 200);
    win.show();

    // --smoke：offscreen 冒烟测试。程序化点按钮 → 回调触发 → 退出
    if std::env::args().any(|a| a == "--smoke") {
        QTimer::single_shot(100, move || btn.click());
        QTimer::single_shot(300, move || {
            assert!(clicked.get(), "clicked 回调没生效（信号链路断）");
            println!("smoke ok");
            DApplication::quit();
        });
    }

    std::process::exit(app.exec());
}
