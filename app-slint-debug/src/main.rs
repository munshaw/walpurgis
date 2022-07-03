use cartridge_debug::cartridge_debug::CartridgeDebug;
use constants::constants::{WINDOW_SCALE, WINDOW_TITLE};
use player_slint::player_slint::PlayerSlint;
use tileset_debug::tileset_debug::TilesetDebug;

fn main() {
    PlayerSlint::new(
        WINDOW_TITLE,
        WINDOW_SCALE,
        Box::new(CartridgeDebug::new()),
        Box::new(TilesetDebug::new()),
    ).run();
}