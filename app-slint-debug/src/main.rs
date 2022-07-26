use player_slint::player_slint::PlayerSlint;

use cartridge_debug::cartridge_debug::CartridgeDebug;
use tileset_debug::tileset_debug::TilesetDebug;

const SCALE: f32 = 2.0;

fn main() {
    PlayerSlint::new(SCALE, CartridgeDebug::new(), TilesetDebug::new()).run();
}
