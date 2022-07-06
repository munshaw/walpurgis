/// Games should implement this. Used by a `Player` to create final gui.
pub trait Cartridge {
    /// Size of grid in tiles, returning (width, height).
    fn get_grid_size(&self) -> (usize, usize);

    /// Title of the window (hint).
    fn get_window_title(&self) -> &str;
}
