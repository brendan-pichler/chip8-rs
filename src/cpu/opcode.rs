use super::Cpu;

#[repr(u16)]
pub enum Opcode {
    Mvi(u16),
    Display(u16)
}

pub fn decode_opcode(opcode: u16) -> Opcode {
    let instruction = opcode & 0xF000;

    match instruction {
        0xA000 => Opcode::Mvi(opcode & 0x0FFF),
        0xD000 => Opcode::Display(opcode & 0x0FFF),
        _ => panic!("{}", format!("Opcode {:X} not found", opcode)),
    }
}

pub fn execute_opcode(opcode: Opcode, cpu: &mut Cpu) {
    match opcode {
        Opcode::Mvi(a) => mvi(cpu, a),
        Opcode::Display(a) => display(cpu, a & 0x0F00, a & 0x00F0, a & 0x000F),
    }
}

fn mvi(cpu: &mut Cpu, address: u16)  {
    cpu.i = address;
    cpu.pc += 2;
}

fn display(cpu: &mut Cpu, x: u16, y: u16, height: u16) {
    let mut pixel: u16;

    cpu.v[0xF as usize] = 0;
    for y_line in 0..height {
        pixel = cpu.memory[(cpu.i + y_line) as usize] as u16;

        for x_line in 0..8 {
            if (pixel & (0x80 >> x_line)) != 0 {
                cpu.v[0xF as usize] = 1;
            }
            cpu.gfx[(x + x_line + (y + y_line) * 64) as usize] ^= 1;
        }
    }

    cpu.draw_flag = 1;
    cpu.pc += 2;
}