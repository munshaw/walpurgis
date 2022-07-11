use pixmap::pixmap::{pixmap_to_rgba8, Rgba8};
use std::collections::HashMap;
use tileset::tileset::{Sprite, Tileset};

/// `Tileset` for debugging `Player` manually.
#[derive(Debug)]
pub struct TilesetDebug {}

#[derive(Debug)]
struct TilesetDebugIntoIter {
    is_first: bool,
    palette: HashMap<char, Rgba8>,
    pixels: &'static str,
}

impl TilesetDebugIntoIter {
    fn new() -> Self {
        let mut palette = HashMap::new();
        palette.insert('.', (0x00, 0x00, 0x00, 0xff));
        palette.insert('X', (0x33, 0x99, 0xcc, 0xff));
        Self {
            is_first: true,
            palette,
            pixels: "\
            XXXXXXXX\
            XX.....X\
            X.X....X\
            X..X...X\
            X...X..X\
            X....X.X\
            X.....XX\
            XXXXXXXX",
        }
    }
}

impl Iterator for TilesetDebugIntoIter {
    type Item = Sprite;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
            // If tests pass then unwrap will not fail.
            Some((0, pixmap_to_rgba8(&self.palette, self.pixels).unwrap()))
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
