use cartridge::cartridge::Cartridge;
use constants::constants::{TILE_HEIGHT, TILE_WIDTH, GRID_WIDTH, GRID_HEIGHT};
use crate::cartridge_debug::CartridgeDebug;

#[test]
fn get_tile_size() {
    let cartridge = CartridgeDebug::new();

    assert_eq!(cartridge.get_tile_size(), (TILE_WIDTH, TILE_HEIGHT));
}

#[test]
fn get_grid_size() {
    let cartridge = CartridgeDebug::new();

    assert_eq!(cartridge.get_grid_size(), (GRID_WIDTH, GRID_HEIGHT));
}