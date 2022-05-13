use minifb::{Key, Window};
use crate::cpu::Cpu;

pub fn map_inputs(window: &Window, cpu: &mut Cpu) {
    for key in cpu.key.iter_mut() {
        *key = 0;
    }

    if window.is_key_down(Key::X) {
        cpu.key[0] = 1;
    }

    if window.is_key_down(Key::Key1) {
        cpu.key[1] = 1;
    }

    if window.is_key_down(Key::Key2) {
        cpu.key[2] = 1;
    }

    if window.is_key_down(Key::Key3) {
        cpu.key[3] = 1;
    }

    if window.is_key_down(Key::Q) {
        cpu.key[4] = 1;
    }

    if window.is_key_down(Key::W) {
        cpu.key[5] = 1;
    }

    if window.is_key_down(Key::E) {
        cpu.key[6] = 1;
    }

    if window.is_key_down(Key::A) {
        cpu.key[7] = 1;
    }

    if window.is_key_down(Key::S) {
        cpu.key[8] = 1;
    }

    if window.is_key_down(Key::D) {
        cpu.key[9] = 1;
    }

    if window.is_key_down(Key::Z) {
        cpu.key[10] = 1;
    }

    if window.is_key_down(Key::C) {
        cpu.key[11] = 1;
    }

    if window.is_key_down(Key::Key4) {
        cpu.key[12] = 1;
    }

    if window.is_key_down(Key::R) {
        cpu.key[13] = 1;
    }

    if window.is_key_down(Key::F) {
        cpu.key[14] = 1;
    }

    if window.is_key_down(Key::V) {
        cpu.key[15] = 1;
    }
}