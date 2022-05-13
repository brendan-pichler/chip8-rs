mod opcode;
mod timers;

use opcode::{execute_opcode, construct_opcodes, Opcodes};
use timers::{update_timers};

// MMAP
// 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
// 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
// 0x200-0xFFF - Program ROM and work RAM
pub struct Cpu {
    memory: [u8; 4096],
    v: [u8; 16], // General purpose registers, last bit is carry flag
    i: u16, // Index register
    pc: u16, // Program counter
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: u16,
    opcodes: Opcodes,
    pub key: [u8; 16], // State of keyboard
    pub draw_flag: u8,
    pub gfx: [u8; 64 * 32],
}

impl Cpu {
    pub fn initialize(fontset: &Vec<u8>, program: &Vec<u8>) -> Self {

        // Initialise registers and memory
        let mut cpu = Cpu {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            gfx: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
            draw_flag: 0,
            opcodes: construct_opcodes(),
        };

        for i in 0..80 {
            cpu.memory[i] = fontset[i];
        }

        for i in 0..program.len() {
            cpu.memory[i + 512] = program[i];
        }

        cpu
    }

    pub fn emulate_cycle(&mut self) {
        // Fetch opcode
        let left_op = (self.memory[self.pc as usize] as u16) << 8;
        let right_op = self.memory[(self.pc + 1) as usize] as u16;

        let opcode: u16 =  left_op | right_op;

        // Execute opcode
        execute_opcode(self, opcode);

        // Update timers
        update_timers(self);
    }
}