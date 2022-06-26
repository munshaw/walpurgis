const WINDOW_SCALE: i32 = 2;
const WINDOW_WIDTH: i32 = 144;
const WINDOW_HEIGHT: i32 = 160;

fn main() {
    let window = MainWindow::new();

    window.set_window_scale(WINDOW_SCALE);
    window.set_window_width(WINDOW_WIDTH);
    window.set_window_height(WINDOW_HEIGHT);

    window.run();
}

slint::slint! {
    MainWindow := Window {
        property<int> window_scale;
        property<int> window_width;
        property<int> window_height;

        width: window_width * window_scale * 1px;
        height: window_height * window_scale * 1px;
        title: "Walpurgis";
    }
}
