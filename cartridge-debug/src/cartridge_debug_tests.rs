use crate::cartridge_debug::{CartridgeDebug, GRID_HEIGHT, GRID_WIDTH, WINDOW_TITLE};
use cartridge::cartridge::Cartridge;

#[test]
fn get_grid_size() {
    let cartridge = CartridgeDebug::default();

    assert_eq!(cartridge.get_grid_size(), (GRID_WIDTH, GRID_HEIGHT));
}

#[test]
fn get_window_title() {
    let cartridge = CartridgeDebug::default();

    assert_eq!(cartridge.get_window_title(), WINDOW_TITLE)
}
