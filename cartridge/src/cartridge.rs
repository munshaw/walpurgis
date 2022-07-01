/// Your game should implement this. Used by player to create final gui.
pub trait Cartridge {
    /// Tile size in pixels, returning (width, height).
    fn get_tile_size(&self) -> (usize, usize);

    // Size of grin in tiles, returning (width, height).
    fn get_grid_size(&self) -> (usize, usize);
}