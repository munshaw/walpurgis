use cartridge::cartridge::Cartridge;
use slint::{Image, Rgba8Pixel, SharedPixelBuffer, SharedString, VecModel};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;
use tileset::tileset::{TileId, Tileset};

slint::include_modules!();

/// Insert a `Cartridge` and some stuff into this slint-based `Player`,
/// and play your game!
pub struct PlayerSlint<C>
where
    C: Cartridge,
{
    pub screen: Screen,
    pub cartridge: C,
}

impl<C> PlayerSlint<C>
where
    C: Cartridge,
{
    fn load_tiles<T>(tileset: T) -> HashMap<TileId, SharedPixelBuffer<Rgba8Pixel>>
    where
        T: Tileset,
    {
        let (tile_width, tile_height) = tileset.get_tile_size();
        let mut tile_buffers = HashMap::with_capacity(tileset.len());
        for tile in tileset {
            let mut buffer =
                SharedPixelBuffer::<Rgba8Pixel>::new(tile_width as u32, tile_height as u32);
            buffer.make_mut_bytes().clone_from_slice(&tile.rgba8);
            tile_buffers.insert(tile.tile_id, buffer);
        }
        tile_buffers
    }

    pub fn new<T>(scale: f32, cartridge: C, tileset: T) -> Self
    where
        T: Tileset,
    {
        let screen = Screen::new();
        let tile_buffers = Self::load_tiles(tileset);

        let (tile_width, tile_height) = tileset.get_tile_size() as (f32, f32);
        let (grid_width, grid_height) = cartridge.get_grid_size() as (i32, i32);

        screen.set_window_title(SharedString::from(cartridge.get_window_title()));
        let tile_width = (tile_width * scale) as i32;
        let tile_height = (tile_height * scale) as i32;
        screen.set_tile_width(tile_width);
        screen.set_tile_height(tile_height);
        screen.set_grid_width(grid_width);
        screen.set_grid_height(grid_height);

        let mut tiles: Vec<Tile> = vec![
            Tile {
                image: Image::from_rgba8(tile_buffers[&TileId::BLACK].borrow().clone())
            };
            (grid_width * grid_height) as usize
        ];
        tiles[33].image = Image::from_rgba8(tile_buffers[&TileId::WHITE].borrow().clone());

        let tiles_model = Rc::new(VecModel::from(tiles));
        screen.set_tiles(tiles_model.clone().into());

        Self { screen, cartridge }
    }

    /// Consider this to be the 'on' button.
    pub fn run(&self) {
        self.screen.run();
    }
}
