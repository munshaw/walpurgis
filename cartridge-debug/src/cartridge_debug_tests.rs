use cartridge::cartridge::Cartridge;
use constants::constants::{GRID_WIDTH, GRID_HEIGHT};
use crate::cartridge_debug::CartridgeDebug;

#[test]
fn get_grid_size() {
    let cartridge = CartridgeDebug::new();

    assert_eq!(cartridge.get_grid_size(), (GRID_WIDTH, GRID_HEIGHT));
}