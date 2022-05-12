// https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
extern crate piston_window;
pub mod cpu;
pub mod fontset;
pub mod draw;

use std::{path::Path, fs::read, thread, time};
use cpu::Cpu;
use fontset::CHIP8_FONTSET;
use draw::{create_window, draw_screen};


fn main() {
    let filename = Path::new("/Users/bpichler/Documents/rust/chip-8/space_invaders.ch8");
    let freq: f32 = 60.0;
    // Load program
    let buffer = read(filename).unwrap();
    let fontset = CHIP8_FONTSET.to_vec();

    let mut cpu = Cpu::initialize(&fontset, &buffer);

    let clock_time = time::Duration::from_secs_f32(1.0/freq);

    let mut window = create_window();
    loop {
        let event = window.next().unwrap();

        let now = time::Instant::now();
        
        // cpu.emulate_cycle();
        draw_screen(&mut window, &event, &cpu.gfx);

        thread::sleep(clock_time - now.elapsed());
    }

}
