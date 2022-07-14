use crate::tileset_debug::{TilesetDebug, HEIGHT, TILES, WIDTH};
use tileset::tileset::Tileset;

#[test]
fn no_duplicate_characters_in_pallete() {
    let _ = TilesetDebug::new().into_iter();
}

#[test]
fn no_unregistered_characters_in_tiles() {
    let tileset = TilesetDebug::new();

    tileset.into_iter().for_each(|_| ());
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
    let tileset = TilesetDebug::new();

    assert_eq!((WIDTH, HEIGHT), tileset.get_tile_size());
}

#[test]
fn len_ok() {
    let tileset = TilesetDebug::new();

    assert_eq!(TILES.len(), tileset.len())
}

#[test]
fn iterator_right_len() {
    let tileset = TilesetDebug::new();

    assert_eq!(TILES.len(), tileset.iter().count())
}

#[test]
fn iter_ok() {
    let tileset = TilesetDebug::new();

    tileset.iter().for_each(|_| ());
}

#[test]
fn is_empty_ok() {
    assert!(!TilesetDebug::new().is_empty());
}
