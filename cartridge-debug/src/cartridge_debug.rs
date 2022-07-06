use cartridge::cartridge::Cartridge;

pub const WINDOW_TITLE: &str = "Debug Cartridge";

pub const GRID_WIDTH: usize = 20;
pub const GRID_HEIGHT: usize = 18;

/// `Cartridge` for debugging `Player`s.
#[derive(Debug)]
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

    fn get_window_title(&self) -> &str {
        WINDOW_TITLE
    }
}
