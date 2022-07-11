use crate::tileset_debug::TilesetDebug;
use tileset::tileset::Tileset;

#[test]
fn no_bad_character() {
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

    assert_eq!((8, 8), tileset.get_tile_size());
}
