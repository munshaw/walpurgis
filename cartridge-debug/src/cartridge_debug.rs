use crate::game_loop::{GameLoop, GameLoopImpl};
use cartridge::cartridge::{Cartridge, Input, Output};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub(crate) const WINDOW_TITLE: &str = "Debug Cartridge";
pub(crate) const GRID_WIDTH: usize = 20;
pub(crate) const GRID_HEIGHT: usize = 18;

/// `Cartridge` implementation for debugging players.
#[derive(Debug)]
pub struct CartridgeDebug<G: 'static + GameLoop + Send> {
    game_loop: Option<G>,
}

impl Default for CartridgeDebug<GameLoopImpl> {
    fn default() -> Self {
        Self::new(GameLoopImpl::default())
    }
}

impl<G: 'static + GameLoop + Send> CartridgeDebug<G> {
    pub(crate) fn new(game_loop: G) -> Self {
        Self {
            game_loop: Some(game_loop),
        }
    }
}

impl<G: GameLoop + Send> Cartridge for CartridgeDebug<G> {
    fn start(&mut self) -> (Sender<Input>, Receiver<Output>) {
        let (to_player, from_cartridge) = channel();
        let (to_cartridge, from_player) = channel();

        // TODO: Don't need this unwrap!
        let game_loop = self.game_loop.take().unwrap();

        thread::spawn(move || game_loop.start(from_player, to_player));

        (to_cartridge, from_cartridge)
    }

    fn get_grid_size(&self) -> (usize, usize) {
        (GRID_WIDTH, GRID_HEIGHT)
    }

    fn get_window_title(&self) -> &str {
        WINDOW_TITLE
    }
}
