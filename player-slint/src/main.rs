use cartridge_debug::cartridge_debug::CartridgeDebug;
use constants::constants::{WINDOW_SCALE, WINDOW_TITLE};

use crate::main_window::PlayerSlint;

pub mod main_window;

fn main() {
    PlayerSlint::new(
        WINDOW_TITLE,
        WINDOW_SCALE,
        Box::new(CartridgeDebug::new()),
    ).run();
}
