use cartridge::cartridge::Cartridge;
use constants::constants::{GRID_HEIGHT, GRID_WIDTH};
use tileset::tileset::Tileset;

/// `Cartridge` for debugging `Player`s.
pub struct CartridgeDebug {
    tileset: Box<dyn Tileset>
}

impl CartridgeDebug {
    pub fn new(tileset: Box<dyn Tileset>) -> Self {
        CartridgeDebug {
            tileset
        }
    }
}

impl Cartridge for CartridgeDebug {
    fn get_tile_size(&self) -> (usize, usize) {
        self.tileset.get_tile_size()
    }

    fn get_grid_size(&self) -> (usize, usize) {
        (GRID_WIDTH, GRID_HEIGHT)
    }
}
