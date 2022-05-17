use minifb::{Window, WindowOptions};
use crate::cpu::Cpu;

pub const WIDTH: usize = 640;
pub const HEIGHT: usize = 320;

#[non_exhaustive]
struct Color;

impl Color {
    const WHITE: u32 = 0x00FFFFFF;
    // const RED: u32 = 0x00FF0000;
    // const BLUE: u32 = 0x000000FF;
    // const GREEN: u32 = 0x0000FF00;
    const BLACK: u32 = 0x00000000;
}

pub fn create_window(clock_time: std::time::Duration) -> Window {
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.limit_update_rate(Some(clock_time));

    window
}

pub fn draw_pixels(window: &mut Window, cpu: &mut Cpu, buffer: &mut Vec<u32>) {
    if cpu.draw_flag == 1 {
        for i in 0..buffer.len() {
            let x0 = i % WIDTH;
            let y0 = i / WIDTH;

            let x1 = x0 / 10;
            let y1 = y0 / 10;

            let gfx_i = x1 + y1 * 64;

            let color: u32;
            if cpu.gfx[gfx_i] == 1 {
                color = Color::WHITE;
            } else {
                color = Color::BLACK;
            }
            buffer[i] = color;
        }
    }

    cpu.draw_flag = 0;

    window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
}