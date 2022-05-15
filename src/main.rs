// https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
extern crate piet_common;
extern crate device_query;
extern crate piet;
extern crate minifb;

pub mod cpu;
pub mod fontset;
pub mod draw;
pub mod input;

use std::{path::Path, fs::read, time};
use cpu::Cpu;
use fontset::CHIP8_FONTSET;
use draw::{create_window, draw_pixels, HEIGHT, WIDTH};
use input::{map_inputs};
use minifb::Key;

fn main() {
    let filename = Path::new("/Users/bpichler/Documents/rust/chip-8/space_invaders.ch8");
    let freq: f32 = 60.;
    // Load program
    let buffer = read(filename).unwrap();
    let fontset = CHIP8_FONTSET.to_vec();

    let mut cpu = Cpu::initialize(&fontset, &buffer);

    let clock_time = time::Duration::from_secs_f32(1.0/freq);


    let mut window = create_window(clock_time);
    let mut draw_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    
    draw_pixels(&mut window, &cpu, &mut draw_buffer);


    while window.is_open() && !window.is_key_down(Key::Escape) {

        map_inputs(&window, &mut cpu);
        cpu.emulate_cycle();

        draw_pixels(&mut window, &cpu, &mut draw_buffer);
    }

}
