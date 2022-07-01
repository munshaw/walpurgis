use slint::{SharedString, VecModel};
use std::rc::Rc;
use cartridge::cartridge::Cartridge;

slint::include_modules!();

pub struct PlayerSlint {
    pub screen: Screen,
    pub cartridge: Box<dyn Cartridge>,
}

impl PlayerSlint {
    pub fn new(title: &str, scale: f32, cartridge: Box<dyn Cartridge>) -> Self {
        let screen = Screen::new();

        let (tile_width, tile_height) = cartridge.get_tile_size();
        let (grid_width, grid_height) = cartridge.get_grid_size();

        let tile_width = (tile_width as f32 * scale) as i32;
        let tile_height = (tile_height as f32 * scale) as i32;

        screen.set_window_title(SharedString::from(title));
        screen.set_tile_width(tile_width);
        screen.set_tile_height(tile_height);
        screen.set_grid_width(grid_width as i32);
        screen.set_grid_height(grid_height as i32);

        let mut tiles: Vec<Tile> = vec![Tile {}; grid_width * grid_height];
        let tiles_model = Rc::new(VecModel::from(tiles));
        screen.set_tiles(tiles_model.clone().into());

        Self { screen, cartridge }
    }

    pub fn run(&self) {
        self.screen.run();
    }
}
