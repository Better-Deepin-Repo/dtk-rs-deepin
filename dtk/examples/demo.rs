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

    // --smoke 专用：PaintDelegate + QSocketNotifier + 事件窗口 链路验证
    let smoke = std::env::args().any(|a| a == "--smoke");
    let painted = Rc::new(Cell::new(false));
    let notified = Rc::new(Cell::new(false));
    let mut pipe_w = 0i32;
    if smoke {
        let table = QTableWidget::new(1, 2);
        table.set_cell_text(0, 0, "图标列");
        table.set_cell_text(0, 1, "内存列");
        {
            let painted = painted.clone();
            let delegate = PaintDelegate::new(move |p, idx, x, y, w, h, _state| {
                assert_eq!(idx.data_string(0), "图标列"); // DisplayRole
                p.fill_rect(x, y, w, h, &QColor::rgb(200, 220, 255));
                p.draw_text(x, y, w, h, qt::ALIGN_CENTER, "画过了");
                painted.set(true);
            });
            table.set_delegate_for_column(0, &delegate);
            std::mem::forget(delegate); // delegate 生命周期交给表（offscreen 测试进程短）
        }
        vbox.add_widget(&table.as_widget());
        std::mem::forget(table);

        // pipe 自写自读测 QSocketNotifier
        let mut fds = [0i32; 2];
        assert_eq!(unsafe { libc_pipe(fds.as_mut_ptr()) }, 0);
        pipe_w = fds[1];
        let notifier = QSocketNotifier::new(fds[0]);
        {
            let notified = notified.clone();
            let fd_r = fds[0];
            notifier.on_activated(move || {
                let mut buf = [0u8; 8];
                unsafe { libc_read(fd_r, buf.as_mut_ptr(), 8) }; // 排干，否则电平触发反复报
                notified.set(true);
                println!("notified");
            });
        }
        std::mem::forget(notifier);
    }

    win.set_central_widget(&central);
    win.resize(400, 200);
    win.show();

    if smoke {
        QTimer::single_shot(100, move || btn.click());
        QTimer::single_shot(150, move || unsafe {
            libc_write(pipe_w, b"x".as_ptr() as _, 1);
        });
        QTimer::single_shot(500, move || {
            assert!(clicked.get(), "clicked 回调没生效（信号链路断）");
            assert!(painted.get(), "PaintDelegate paint 回调没生效");
            assert!(notified.get(), "QSocketNotifier activated 没生效");
            println!("smoke ok");
            DApplication::quit();
        });
    }

    std::process::exit(app.exec());
}

// 不引 libc crate，两个 syscall 直接 extern
unsafe extern "C" {
    fn pipe(fds: *mut i32) -> i32;
    fn write(fd: i32, buf: *const u8, n: usize) -> isize;
    fn read(fd: i32, buf: *mut u8, n: usize) -> isize;
}
unsafe fn libc_pipe(fds: *mut i32) -> i32 {
    unsafe { pipe(fds) }
}
unsafe fn libc_write(fd: i32, buf: *const u8, n: usize) -> isize {
    unsafe { write(fd, buf, n) }
}
unsafe fn libc_read(fd: i32, buf: *mut u8, n: usize) -> isize {
    unsafe { read(fd, buf, n) }
}
