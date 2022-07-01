use cartridge::cartridge::Cartridge;
use constants::constants::{GRID_HEIGHT, GRID_WIDTH, TILE_HEIGHT, TILE_WIDTH};

/// Cartridge for debugging players.
pub struct CartridgeDebug {}

impl CartridgeDebug {
    pub fn new() -> Self {
        CartridgeDebug {}
    }
}

impl Cartridge for CartridgeDebug {
    fn get_tile_size(&self) -> (usize, usize) {
        (TILE_WIDTH, TILE_HEIGHT)
    }

    fn get_grid_size(&self) -> (usize, usize) {
        (GRID_WIDTH, GRID_HEIGHT)
    }
}
