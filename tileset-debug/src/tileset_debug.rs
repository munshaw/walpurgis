use pixmap::pixmap::{pixmap_to_rgba8, Rgba8};
use std::collections::HashMap;
use std::slice::Iter;
use tileset::tileset::{TileData, TileId, Tileset};

pub(crate) const WIDTH: usize = 8;
pub(crate) const HEIGHT: usize = 8;

pub(crate) const TILES: &[(TileId, &str)] = &[
    (
        TileId::BLACK,
        "\
        ........\
        ........\
        ........\
        ........\
        ........\
        ........\
        ........\
        ........",
    ),
    (
        TileId::WHITE,
        "\
        ,,,,,,,,\
        ,,,,,,,,\
        ,,,,,,,,\
        ,,,,,,,,\
        ,,,,,,,,\
        ,,,,,,,,\
        ,,,,,,,,\
        ,,,,,,,,",
    ),
];

const PALETTE: &[(char, Rgba8)] = &[
    ('.', (0x00, 0x00, 0x00, 0xff)),
    (',', (0xff, 0xff, 0xff, 0xff)),
];

/// `Tileset` implementation for use with `CartridgeDebug`.
#[derive(Debug)]
pub struct TilesetDebug {}

/// Tileset iterator for generating rgba8 pixmaps.
#[derive(Debug)]
pub struct TilesetDebugIntoIter<'a> {
    palette: HashMap<char, Rgba8>,
    tile_iter: Iter<'a, (TileId, &'static str)>,
}

impl<'a> TilesetDebugIntoIter<'a> {
    fn new() -> Self {
        let mut palette = HashMap::new();
        if !PALETTE.iter().all(|a| palette.insert(a.0, a.1).is_none()) {
            panic!() // Repeated element. If tests pass, this will not happen.
        }
        Self {
            palette,
            tile_iter: TILES.iter(),
        }
    }
}

impl<'a> Iterator for TilesetDebugIntoIter<'a> {
    type Item = TileData;

    fn next(&mut self) -> Option<Self::Item> {
        match self.tile_iter.next() {
            None => None,
            Some(tile) => Some(TileData {
                tile_id: tile.0,
                // Unregistered character. If tests pass, this will not happen
                rgba8: pixmap_to_rgba8(&self.palette, tile.1).unwrap(),
            }),
        }
    }
}

impl Default for TilesetDebug {
    fn default() -> Self {
        Self::new()
    }
}

impl TilesetDebug {
    pub fn new() -> Self {
        Self {}
    }
}

impl IntoIterator for TilesetDebug {
    type Item = TileData;
    type IntoIter = TilesetDebugIntoIter<'static>;

    fn into_iter(self) -> Self::IntoIter {
        TilesetDebugIntoIter::new()
    }
}

impl Tileset<TilesetDebugIntoIter<'static>> for TilesetDebug {
    fn get_tile_size(&self) -> (usize, usize) {
        (WIDTH, HEIGHT)
    }

    fn iter(&self) -> Self::IntoIter {
        TilesetDebugIntoIter::new()
    }

    fn len(&self) -> usize {
        TILES.len()
    }

    fn is_empty(&self) -> bool {
        TILES.is_empty()
    }
}
