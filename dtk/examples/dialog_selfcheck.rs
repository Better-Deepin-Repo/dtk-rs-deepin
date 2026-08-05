// ponytail: compile-time self-check for the three dialog classes.
// Verifies DDialog + DMessageManager (generated) and DMessageBox (hand-written) link + have the API surface
// the user asked for (exec / addButton / setTitle / setMessage / setIcon for confirmation dialogs).
use dtk::{DApplication, DMessageBox, QIcon, QMargins, DDciIcon, QWidget, qt};
use dtk::widgets::{DDialog, DMessageManager};

fn _ddialog_usage() {
    let d = DDialog::new();
    d.set_title("Confirm");
    d.set_message("Proceed?");
    d.set_icon(&QIcon::from_theme("dialog-warning"));
    d.add_button("OK", true, 0); // ButtonNormal
    d.add_button("Cancel", false, 1); // ButtonWarning
    let _r: i32 = d.exec();
}

fn _dmessagebox_usage() {
    // static helpers (most common confirmation path)
    let _r = DMessageBox::information(None, "Info", "Done", qt::MSG_BTN_OK, qt::MSG_BTN_OK);
    let _r = DMessageBox::question(None, "Ask", "Yes or no?", qt::MSG_BTN_YES_NO, qt::MSG_BTN_YES);

    // full instance API
    let mb = DMessageBox::with(qt::MSG_ICON_QUESTION, "Ask", "Sure?", qt::MSG_BTN_YES_NO, None);
    mb.set_informative_text("This cannot be undone.");
    mb.set_default_button(qt::MSG_BTN_NO);
    let _btn = mb.add_button("Custom", qt::MSG_ROLE_ACTION);
    let _clicked: i32 = mb.exec();
}

fn _messagemanager_usage() {
    let _mm = DMessageManager::instance();
    let _margins = QMargins::new(10, 10, 10, 10);
    let _dci = DDciIcon::new();
    let _dci2 = DDciIcon::from_file("/usr/share/dsg/icons/example.dci");
}

fn main() {
    // smoke: methods exist + types resolve (no actual dialog shown without an event loop)
    let _app = DApplication::new("selfcheck");
    _ddialog_usage();
    _dmessagebox_usage();
    _messagemanager_usage();
    let _w = QWidget::new(None);
}
