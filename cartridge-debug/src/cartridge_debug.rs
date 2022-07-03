use cartridge::cartridge::Cartridge;
use constants::constants::{GRID_HEIGHT, GRID_WIDTH};

/// `Cartridge` for debugging `Player`s.
pub struct CartridgeDebug {}

impl CartridgeDebug {
    pub fn new() -> Self {
        Self {}
    }
}

impl Cartridge for CartridgeDebug {
    fn get_grid_size(&self) -> (usize, usize) {
        (GRID_WIDTH, GRID_HEIGHT)
    }
}
