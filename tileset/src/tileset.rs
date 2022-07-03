/// Name and rgba data for a single sprite
pub type Sprite = (String, Vec<u8>);

/// Tilesets should implement this. Consumed by `Cartridge`s.
pub trait Tileset: IntoIterator<Item = Sprite, IntoIter = Box<dyn TilesetIntoIter>> {
    /// Tile size in pixels, returning (width, height).
    fn get_tile_size(&self) -> (usize, usize);
}

pub trait TilesetIntoIter: Iterator<Item = Sprite> {}