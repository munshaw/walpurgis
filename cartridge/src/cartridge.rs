/// Games should implent this. Used by a `Player` to create final gui.
pub trait Cartridge {
    /// Size of grid in tiles, returning (width, height).
    fn get_grid_size(&self) -> (usize, usize);
}
