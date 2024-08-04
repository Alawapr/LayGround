use crate::backend::{HEIGHT, SCRBUFF, WIDTH};

pub fn set_pixel(x: usize, y: usize, color: u32) {
    if x + y * WIDTH >= WIDTH * HEIGHT * 4 {
        eprintln!("set_pixel out of bounds: ({}, {})", x, y);
        return;
    }
    unsafe { SCRBUFF[x + y * WIDTH] = color };
}

pub fn set_pixels(x: usize, y: usize, color: u32, count: usize) {
    for i in 0..count {
        set_pixel(x + i, y, color);
    }
}
