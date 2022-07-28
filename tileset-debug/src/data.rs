use pixmap::pixmap::Rgba8;
use tileset::tileset::TileId;

pub const WIDTH: usize = 8;
pub const HEIGHT: usize = 8;
pub const TILES: &[(TileId, &str)] = &[
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
pub const PALETTE: &[(char, Rgba8)] = &[
    ('.', (0x00, 0x00, 0x00, 0xff)),
    (',', (0xff, 0xff, 0xff, 0xff)),
];
