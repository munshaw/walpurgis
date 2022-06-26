use slint::SharedString;

use crate::constants::*;

pub mod constants;

fn main() {
    let window = MainWindow::new();

    window.set_window_scale(WINDOW_SCALE);
    window.set_window_width(WINDOW_WIDTH);
    window.set_window_height(WINDOW_HEIGHT);
    window.set_window_title(SharedString::from(WINDOW_TITLE));

    window.run();
}

slint::slint! {
    MainWindow := Window {
        property<int> window_scale;
        property<int> window_width;
        property<int> window_height;
        property<string> window_title;

        width: window_width * window_scale * 1px;
        height: window_height * window_scale * 1px;
        title: window_title;
    }
}
