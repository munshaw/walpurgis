use cartridge::cartridge::Cartridge;
use cartridge_debug::cartridge_debug::CartridgeDebug;
use cartridge_debug::game_loop::{GameLoop, GameLoopImpl};
use slint::{Image, Rgba8Pixel, SharedPixelBuffer, SharedString, VecModel};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;
use tileset::tileset::{TileData, TileId, Tileset};
use tileset_debug::tileset_debug::TilesetDebug;

const SCALE: f32 = 2.0;

slint::include_modules!();

/// Insert a `Cartridge` into this slint-based player, and enjoy your game!
pub struct PlayerSlint<C: Cartridge> {
    screen: Screen,
    cartridge: C,
}

impl<C: Cartridge> PlayerSlint<C> {
    fn load_tiles<I: Iterator<Item = TileData>, T: Tileset<I>>(
        tileset: &T,
    ) -> HashMap<TileId, SharedPixelBuffer<Rgba8Pixel>> {
        let (tile_width, tile_height) = tileset.get_tile_size();
        let mut tile_buffers = HashMap::with_capacity(tileset.len());
        for tile in tileset.iter() {
            let mut buffer =
                SharedPixelBuffer::<Rgba8Pixel>::new(tile_width as u32, tile_height as u32);
            buffer.make_mut_bytes().clone_from_slice(&tile.rgba8);
            tile_buffers.insert(tile.tile_id, buffer);
        }
        tile_buffers
    }

    fn set_properties(
        screen: &Screen,
        title: &str,
        tile_height: usize,
        tile_width: usize,
        grid_width: usize,
        grid_height: usize,
        scale: f32,
    ) {
        screen.set_window_title(SharedString::from(title));
        let tile_width = (tile_width as f32 * scale) as i32;
        let tile_height = (tile_height as f32 * scale) as i32;
        screen.set_tile_width(tile_width);
        screen.set_tile_height(tile_height);
        screen.set_grid_width(grid_width as i32);
        screen.set_grid_height(grid_height as i32);
    }

    fn fill_tile_model(
        tile_buffers: HashMap<TileId, SharedPixelBuffer<Rgba8Pixel>>,
        grid_width: usize,
        grid_height: usize,
    ) -> Vec<Tile> {
        vec![
            Tile {
                image: Image::from_rgba8(tile_buffers[&TileId::BLACK].borrow().clone())
            };
            (grid_width * grid_height) as usize
        ]
    }

    pub fn new<I: Iterator<Item = TileData>, T: Tileset<I>>(
        scale: f32,
        cartridge: C,
        tileset: T,
    ) -> Self {
        let screen = Screen::new();
        let tile_buffers = Self::load_tiles(&tileset);

        let (tile_width, tile_height) = tileset.get_tile_size();
        let (grid_width, grid_height) = cartridge.get_grid_size();

        Self::set_properties(
            &screen,
            cartridge.get_window_title(),
            tile_height,
            tile_width,
            grid_height,
            grid_width,
            scale,
        );

        let mut tiles = Self::fill_tile_model(tile_buffers, grid_width, grid_height);
        let tiles_model = Rc::new(VecModel::from(tiles));
        screen.set_tiles(tiles_model.clone().into());

        Self { screen, cartridge }
    }

    /// Start playing ya game!
    pub fn run(&self) {
        self.screen.run();
    }
}

impl Default for PlayerSlint<CartridgeDebug<GameLoopImpl>> {
    fn default() -> Self {
        Self::new(SCALE, CartridgeDebug::default(), TilesetDebug::default())
    }
}
