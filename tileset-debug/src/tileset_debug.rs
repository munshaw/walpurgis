use tileset::tileset::{Sprite, Tileset, TilesetIntoIter};

/// `Tileset` for debugging `Player` manually.
pub struct TilesetDebug {}

/// Iterator for `Tileset debug`
pub struct TilesetDebugIntoIter {
    is_first: bool
}

impl TilesetDebugIntoIter {
    pub fn new() -> Self {
        TilesetDebugIntoIter {
            is_first: true
        }
    }
}

impl Iterator for TilesetDebugIntoIter {
    type Item = Sprite;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
            Some(("black".to_string(), vec![0, 0, 0, 255]))
        } else {
            None
        }
    }
}

impl TilesetIntoIter for TilesetDebugIntoIter {}

impl TilesetDebug {
    pub fn new() -> Self {
        TilesetDebug {}
    }
}

impl IntoIterator for TilesetDebug {
    type Item = Sprite;
    type IntoIter = Box<dyn TilesetIntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(TilesetDebugIntoIter::new())
    }
}

impl Tileset for TilesetDebug {
    fn get_tile_size(&self) -> (usize, usize) {
        (8, 8)
    }
}