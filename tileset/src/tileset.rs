/// Name and rgba data for a single sprite
pub type Sprite = (isize, Vec<u8>);

/// Tilesets implement this. Consumed by `Cartridge`s.
pub trait Tileset:
    IntoIterator<Item = (isize, Vec<u8>), IntoIter = Box<dyn Iterator<Item = (isize, Vec<u8>)>>>
{
    /// Tile size in pixels, returning (width, height).
    fn get_tile_size(&self) -> (usize, usize);
}
