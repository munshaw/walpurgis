use std::sync::mpsc::{Receiver, Sender};
use tileset::tileset::TileId;

/// Commands sent from the player to the cartridge.
pub enum Input {
    /// TThe user typed a character.
    Char(char),
}

/// Commands sent from the cartridge to the player.
pub enum Output {
    /// Draw a tile at an (x, y) coordinate. (x, y) coordinates work
    /// left to right, top to bottom.
    Draw(TileId, (usize, usize)),
}

/// Games implement this. Consumed by a player to create final game.
pub trait Cartridge {
    /// Start the game, return IO channels to communicate with the game loop.
    fn start(&self) -> (Sender<Input>, Receiver<Output>);

    /// Size of grid in tiles, returning (width, height).
    fn get_grid_size(&self) -> (usize, usize);

    /// Title of the window (hint).
    fn get_window_title(&self) -> &str;
}
