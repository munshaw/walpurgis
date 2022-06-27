use slint::SharedString;

use constants::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct MainWindow {
    pub window: ProtoWindow,
}

impl MainWindow {
    pub fn new(title: &str, scale: f32) -> Self {
        let window = ProtoWindow::new();

        window.set_window_title(SharedString::from(title));
        window.set_window_width((WINDOW_WIDTH as f32 * scale) as i32);
        window.set_window_height((WINDOW_HEIGHT as f32 * scale) as i32);

        Self { window }
    }

    pub fn run(&self) {
        self.window.run();
    }
}

slint::slint! {
    ProtoWindow := Window {
        property<int> window_width;
        property<int> window_height;
        property<string> window_title;

        width: window_width * 1px;
        height: window_height * 1px;
        title: window_title;
    }
}
