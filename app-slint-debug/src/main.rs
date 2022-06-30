use cartridge_debug::cartridge_debug::CartridgeDebug;
use constants::constants::{WINDOW_SCALE, WINDOW_TITLE};
use player_slint::player_slint::PlayerSlint;

fn main() {
    PlayerSlint::new(
        WINDOW_TITLE,
        WINDOW_SCALE,
        Box::new(CartridgeDebug::new()),
    ).run();
}