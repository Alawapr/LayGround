use backend::{create_window, HEIGHT, WIDTH};
use drawing::set_pixels;

mod backend;
mod drawing;

fn main() {
    unsafe { create_window() };
}

fn real_main() {
    set_pixels(0, 0, 0xFF00FFFF, (WIDTH * HEIGHT) / 2);
}
