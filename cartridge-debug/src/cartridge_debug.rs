use crate::game_loop::{GameLoop, GameLoopImpl};
use cartridge::cartridge::{Cartridge, Input, Output};
use std::marker::PhantomData;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub(crate) const WINDOW_TITLE: &str = "Debug Cartridge";

pub(crate) const GRID_WIDTH: usize = 20;
pub(crate) const GRID_HEIGHT: usize = 18;

/// `Cartridge` for debugging players.
#[derive(Debug)]
pub struct CartridgeDebug<G: GameLoop> {
    phantom: PhantomData<G>,
}

impl Default for CartridgeDebug<GameLoopImpl> {
    fn default() -> Self {
        Self::new()
    }
}

impl CartridgeDebug<GameLoopImpl> {
    pub fn new() -> Self {
        Self::new_root()
    }
}

impl<G: GameLoop> CartridgeDebug<G> {
    pub(crate) fn new_root() -> Self {
        Self {
            phantom: Default::default(),
        }
    }
}

impl<G: GameLoop> Cartridge for CartridgeDebug<G> {
    fn start(&self) -> (Sender<Input>, Receiver<Output>) {
        let (to_player, from_cartridge) = channel();
        let (to_cartridge, from_player) = channel();

        thread::spawn(move || G::new(from_player, to_player).start());

        (to_cartridge, from_cartridge)
    }

    fn get_grid_size(&self) -> (usize, usize) {
        (GRID_WIDTH, GRID_HEIGHT)
    }

    fn get_window_title(&self) -> &str {
        WINDOW_TITLE
    }
}
