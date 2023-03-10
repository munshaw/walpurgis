use crate::data::{HEIGHT, PALETTE, TILES, WIDTH};
use pixmap::pixmap::{pixmap_to_rgba8, Rgba8};
use std::collections::HashMap;
use tileset::tileset::{TileData, Tileset};

/// `Tileset` implementation for use with `CartridgeDebug`.
#[derive(Debug, Default)]
pub struct TilesetDebug {}

/// `Tileset` iterator for generating rgba8 pixmaps.
#[derive(Debug)]
pub struct TilesetDebugIntoIter {
    palette: HashMap<char, Rgba8>,
    index: usize,
}

impl Default for TilesetDebugIntoIter {
    fn default() -> Self {
        let mut palette = HashMap::new();
        if !PALETTE.iter().all(|a| palette.insert(a.0, a.1).is_none()) {
            panic!() // Repeated element. This will not fail if tests pass.
        }
        Self { palette, index: 0 }
    }
}

impl Iterator for TilesetDebugIntoIter {
    type Item = TileData;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < TILES.len() {
            let tile = TILES[self.index];
            self.index += 1;
            Some(TileData {
                tile_id: tile.0,
                // This will not fail if tests pass.
                rgba8: pixmap_to_rgba8(&self.palette, tile.1).unwrap(),
            })
        } else {
            None
        }
    }
}

impl IntoIterator for TilesetDebug {
    type Item = TileData;
    type IntoIter = TilesetDebugIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        TilesetDebugIntoIter::default()
    }
}

impl Tileset<TilesetDebugIntoIter> for TilesetDebug {
    fn get_tile_size(&self) -> (usize, usize) {
        (WIDTH, HEIGHT)
    }

    fn iter(&self) -> Self::IntoIter {
        TilesetDebugIntoIter::default()
    }

    fn len(&self) -> usize {
        TILES.len()
    }

    fn is_empty(&self) -> bool {
        TILES.is_empty()
    }
}
