use slint::SharedString;

use crate::constants::{WINDOW_SCALE, WINDOW_TITLE};
use crate::main_window::MainWindow;

pub mod constants;
pub mod main_window;

fn main() {
    MainWindow::new(WINDOW_TITLE, WINDOW_SCALE).run();
}
