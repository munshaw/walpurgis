/// Tiles that can be provided.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TileId {
    BLACK,
    WHITE,
}

/// ID and rgba data for a single sprite.
#[derive(Debug)]
pub struct TileData {
    /// The identifier for the tile.
    pub tile_id: TileId,

    /// Raw rgba8 data for the tile.
    pub rgba8: Vec<u8>,
}

/// `Tileset`s implement this. Consumed by `Cartridge`s.
pub trait Tileset:
    IntoIterator<Item = TileData, IntoIter = Box<dyn Iterator<Item = TileData>>>
{
    /// Tile size in pixels, returning (width, height).
    fn get_tile_size(&self) -> (usize, usize);

    /// Return `Tileset` iterator.
    fn iter(&self) -> Box<dyn Iterator<Item = TileData>>;

    /// Return the number of tiles in this `Tileset`.
    fn len(&self) -> usize;

    /// Return false if this tileset is not empty.
    fn is_empty(&self) -> bool;
}
