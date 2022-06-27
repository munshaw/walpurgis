use slint::SharedString;

use constants::constants::{WINDOW_SCALE, WINDOW_TITLE};

use crate::main_window::MainWindow;

pub mod main_window;

fn main() {
    MainWindow::new(WINDOW_TITLE, WINDOW_SCALE).run();
}
