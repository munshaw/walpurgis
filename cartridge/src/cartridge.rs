use std::sync::mpsc::{Receiver, Sender};

/// Commands sent from the player to the cartridge.
pub enum Input {}

/// Commands sent from the cartridge to the player.
pub enum Output {}

/// Games should implement this. Used by a player to create final gui.
pub trait Cartridge {
    /// Start the game.
    fn start(&self) -> (Sender<Input>, Receiver<Output>);

    /// Size of grid in tiles, returning (width, height).
    fn get_grid_size(&self) -> (usize, usize);

    /// Title of the window (hint).
    fn get_window_title(&self) -> &str;
}
