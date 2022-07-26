use cartridge::cartridge::{Cartridge, Input, Output};
use std::sync::mpsc::{Receiver, Sender};

pub const WINDOW_TITLE: &str = "Debug Cartridge";

pub const GRID_WIDTH: usize = 20;
pub const GRID_HEIGHT: usize = 18;

/// `Cartridge` for debugging players.
#[derive(Debug)]
pub struct CartridgeDebug {}

impl CartridgeDebug {
    pub fn new() -> Self {
        Self {}
    }
}

impl Cartridge for CartridgeDebug {
    fn start(&self) -> (Sender<Input>, Receiver<Output>) {
        todo!()
    }

    fn get_grid_size(&self) -> (usize, usize) {
        (GRID_WIDTH, GRID_HEIGHT)
    }

    fn get_window_title(&self) -> &str {
        WINDOW_TITLE
    }
}
