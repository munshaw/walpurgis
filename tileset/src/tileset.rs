/// Tiles that can be provided.
#[derive(Debug)]
pub enum TileId {
    DEBUG,
}

/// ID and rgba data for a single sprite.
#[derive(Debug)]
pub struct TileData {
    /// The identifier for the tile.
    pub tile: TileId,

    /// Raw rgba8 data for the tile.
    pub rgba8: Vec<u8>,
}

/// Tilesets implement this. Consumed by `Cartridge`s.
pub trait Tileset:
    IntoIterator<Item = TileData, IntoIter = Box<dyn Iterator<Item = TileData>>>
{
    /// Tile size in pixels, returning (width, height).
    fn get_tile_size(&self) -> (usize, usize);
}
