use cartridge::cartridge::Cartridge;
use slint::{Image, Rgba8Pixel, SharedPixelBuffer, SharedString, VecModel};
use std::rc::Rc;
use tileset::tileset::Tileset;

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
    pub fn new<T>(scale: f32, cartridge: C, tileset: T) -> Self
    where
        T: Tileset,
    {
        let screen = Screen::new();

        let (tile_width, tile_height) = tileset.get_tile_size();
        let (grid_width, grid_height) = cartridge.get_grid_size();

        let tile = tileset.into_iter().next().unwrap().1;
        let mut buffer =
            SharedPixelBuffer::<Rgba8Pixel>::new(tile_width as u32, tile_height as u32);
        buffer.make_mut_bytes().clone_from_slice(&tile);
        let image = Image::from_rgba8(buffer);

        screen.set_window_title(SharedString::from(cartridge.get_window_title()));
        let tile_width = (tile_width as f32 * scale) as i32;
        let tile_height = (tile_height as f32 * scale) as i32;
        screen.set_tile_width(tile_width);
        screen.set_tile_height(tile_height);
        screen.set_grid_width(grid_width as i32);
        screen.set_grid_height(grid_height as i32);

        let tiles: Vec<Tile> = vec![Tile { image }; grid_width * grid_height];
        let tiles_model = Rc::new(VecModel::from(tiles));
        screen.set_tiles(tiles_model.clone().into());

        Self { screen, cartridge }
    }

    /// Consider this to be the 'on' button.
    pub fn run(&self) {
        self.screen.run();
    }
}
