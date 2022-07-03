use pixmap::pixmap::PixMap;
use tileset::tileset::{Sprite, Tileset};

/// `Tileset` for debugging `Player` manually.
pub struct TilesetDebug {}

/// Iterator for `Tileset debug`
pub struct TilesetDebugIntoIter {
    is_first: bool,
    pixmap: PixMap,
}

impl TilesetDebugIntoIter {
    pub fn new() -> Self {
        Self {
            is_first: true,
            pixmap: PixMap {
                colours: vec![
                    (' ', (0x00, 0x00, 0x00, 0xff)),
                    ('.', (0x22, 0x44, 0x66, 0xff)),
                ],
                pixels: "\
                .       \
                 .      \
                  .     \
                   .    \
                    .   \
                     .  \
                      . \
                       ."
                .to_string(),
            },
        }
    }
}

impl Iterator for TilesetDebugIntoIter {
    type Item = Sprite;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
            Some((0, self.pixmap.to_rgba8()))
        } else {
            None
        }
    }
}

impl TilesetDebug {
    pub fn new() -> Self {
        Self {}
    }
}

impl IntoIterator for TilesetDebug {
    type Item = Sprite;
    type IntoIter = Box<dyn Iterator<Item = Sprite>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(TilesetDebugIntoIter::new())
    }
}

impl Tileset for TilesetDebug {
    fn get_tile_size(&self) -> (usize, usize) {
        (8, 8)
    }
}
