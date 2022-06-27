use slint::SharedString;

use cartridge::cartridge::Cartridge;

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
        let grid_width = grid_width as i32;
        let grid_height = grid_height as i32;

        screen.set_window_title(SharedString::from(title));
        screen.set_tile_width(tile_width);
        screen.set_tile_height(tile_height);
        screen.set_grid_width(grid_width);
        screen.set_grid_height(grid_height);

        Self { screen, cartridge }
    }

    pub fn run(&self) {
        self.screen.run();
    }
}

slint::slint! {
    Screen := Window {
        property<int> tile_width;
        property<int> tile_height;
        property<int> grid_width;
        property<int> grid_height;
        property<string> window_title;

        width: tile_width * grid_width * 1px;
        height: tile_height * grid_height * 1px;
        title: window_title;
    }
}
