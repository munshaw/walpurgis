use std::slice::Iter;

/// Tilesets should implement this. Consumed by `Cartridge`s.
trait Tileset {
    /// Tile size in pixels, returning (width, height).
    fn get_tile_size(&self) -> (usize, usize);

    /// Iterator for all tiles in (name, &[u8]) format, where &[u8] is rgba8 data.
    fn iter(&self) -> Iter<(&str, &[u8])>;
}