/// Tiles that can be provided.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TileId {
    BLACK,
    WHITE,
}

/// Id and rgba8 data for a single sprite.
#[derive(Debug)]
pub struct TileData {
    /// The identifier for the tile.
    pub tile_id: TileId,

    /// Raw rgba8 data for the tile.
    pub rgba8: Vec<u8>,
}

/// `Tileset`s implement this. Consumed by `Cartridge`s.
pub trait Tileset<I: Iterator<Item = TileData>>:
    IntoIterator<Item = TileData, IntoIter = I>
{
    /// Tile size in pixels, returning (width, height).
    fn get_tile_size(&self) -> (usize, usize);

    /// Return `Tileset` iterator.
    fn iter(&self) -> I;

    /// Return the number of tiles in this `Tileset`.
    fn len(&self) -> usize;

    /// Return true if the tileset is empty.
    fn is_empty(&self) -> bool;
}
