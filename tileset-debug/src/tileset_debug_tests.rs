use crate::data::{HEIGHT, TILES, WIDTH};
use crate::tileset_debug::TilesetDebug;
use tileset::tileset::Tileset;

#[test]
fn no_duplicate_characters_in_palette() {
    TilesetDebug::new().into_iter();
}

#[test]
fn no_unregistered_characters_in_tiles() {
    TilesetDebug::new().iter().for_each(|_| ());
}

#[test]
fn all_right_size() {
    let tileset = TilesetDebug::new();
    let (width, height) = tileset.get_tile_size();
    let expected_size = 4 * width * height;
    for p in tileset {
        assert_eq!(p.rgba8.len(), expected_size)
    }
}

#[test]
fn get_tile_size_ok() {
    assert_eq!((WIDTH, HEIGHT), TilesetDebug::new().get_tile_size());
}

#[test]
fn len_ok() {
    assert_eq!(TILES.len(), TilesetDebug::new().len());
}

#[test]
fn iterator_right_len() {
    assert_eq!(TILES.len(), TilesetDebug::new().iter().count())
}

#[test]
fn is_empty_ok() {
    assert!(!TilesetDebug::new().is_empty());
}

#[test]
fn default_ok() {
    TilesetDebug::default();
}
