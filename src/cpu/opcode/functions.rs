use crate::cpu::Cpu;
use super::Args;
use rand;

// DXYN	Display	draw(Vx, Vy, N)	Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. Each row of 8 pixels is read as bit-coded starting 
// from memory location I; I value does not change after the execution of this instruction. 
// As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that does not happen
pub fn display(cpu: &mut Cpu, args: &Args) {
    let x_coord = (cpu.v[args.x as usize] % 64) as u16;
    let y_coord = (cpu.v[args.y as usize] % 32) as u16;

    cpu.v[0xF as usize] = 0;


    for y in 0..args.n {
        if y_coord + y > 31 {
            break;
        }
        let sprite_byte = cpu.memory[(cpu.i + y) as usize];

        for x in 0..8 {
            if x_coord + x > 63 {
                break;
            }

            let sprite_pixel: u8 = ((0b10000000 >> x) & sprite_byte) >> (7 - x);
            let pixel = &mut cpu.gfx[(x_coord + x + (y_coord + y) * 64) as usize];

            if sprite_pixel == 1 {
                if *pixel == 1 {
                    *pixel = 0;
                    cpu.v[0xF as usize] = 1;
                } else {
                    *pixel = 1;
                }
            }
        }
    }
    // cpu.draw_flag = 1;
}

// 1NNN	Flow	goto NNN;	Jumps to address NNN.
pub fn goto(cpu: &mut Cpu, args: &Args) {
    cpu.pc = args.nnn;
}

// 3XNN	Cond	if (Vx == NN)	Skips the next instruction if VX equals NN. (Usually the next instruction is a jump to skip a code block);
pub fn skip_eq(cpu: &mut Cpu, args: &Args) {

    if cpu.v[args.x as usize] == args.nn as u8 {
        cpu.pc += 2;
    }
}

// 0NNN	Call    Calls machine code routine (RCA 1802 for COSMAC VIP) at address NNN. Not necessary for most ROMs.
pub fn call(_cpu: &mut Cpu, _args: &Args) {
    panic!("Call not implemented!");
}

// 4XNN	Cond	if (Vx != NN)	Skips the next instruction if VX does not equal NN. (Usually the next instruction is a jump to skip a code block);
pub fn skip_not_eq(cpu: &mut Cpu, args: &Args) {
    if cpu.v[args.x as usize] != args.nn as u8 {
        cpu.pc += 2;
    }
}

// 2NNN	Flow	*(0xNNN)()	Calls subroutine at NNN.
pub fn call_sub(cpu: &mut Cpu, args: &Args) {
    cpu.stack[cpu.sp as usize] = cpu.pc;
    cpu.sp += 1;
    cpu.pc = args.nnn;
}

// 00E0	Display	disp_clear()	Clears the screen.
pub fn disp_clear(cpu: &mut Cpu, _args: &Args) {
    for i in cpu.gfx.iter_mut() {
        *i = 0;
    }
}

// 00EE	Flow	return;	Returns from a subroutine.
pub fn ret_sub(cpu: &mut Cpu, _args: &Args) {
    cpu.sp -= 1;
    cpu.pc = cpu.stack[cpu.sp as usize];
}

// 5XY0	Cond	if (Vx == Vy)	Skips the next instruction if VX equals VY. (Usually the next instruction is a jump to skip a code block);
pub fn skip_reg_eq(cpu: &mut Cpu, args: &Args) {
    if cpu.v[args.x as usize] == cpu.v[args.y as usize] {
        cpu.pc += 2;
    }
}

// 6XNN	Const	Vx = NN	Sets VX to NN.
pub fn set(cpu: &mut Cpu, args: &Args) {
    cpu.v[args.x as usize] = args.nn as u8;
}

// 7XNN	Const	Vx += NN	Adds NN to VX. (Carry flag is not changed);
pub fn add_const(cpu: &mut Cpu, args: &Args) {
    cpu.v[args.x as usize] += args.nn as u8;
}

// 9XY0	Cond	if (Vx != Vy)	Skips the next instruction if VX does not equal VY. (Usually the next instruction is a jump to skip a code block);
pub fn not_eq_reg(cpu: &mut Cpu, args: &Args) {
    if cpu.v[args.x as usize] != cpu.v[args.y as usize] {
        cpu.pc += 2;
    }
}

// ANNN	MEM	I = NNN	Sets I to the address NNN.
pub fn mvi(cpu: &mut Cpu, args: &Args)  {
    cpu.i = args.nnn;
}

// BNNN	Flow	PC = V0 + NNN	Jumps to the address NNN plus V0.
pub fn jmp_offset(cpu: &mut Cpu, args: &Args) {
    cpu.pc = cpu.v[0 as usize] as u16 + args.nnn;
}

// CXNN	Rand	Vx = rand() & NN	Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
pub fn bitwise_rand(cpu: &mut Cpu, args: &Args) {
    let random: u8 = rand::random::<u8>();
    cpu.v[args.x as usize] = args.nn as u8 & random;
}

// EX9E	KeyOp	if (key() == Vx)	Skips the next instruction if the key stored in VX is pressed. (Usually the next instruction is a jump to skip a code block);
pub fn skip_on_key(cpu: &mut Cpu, args: &Args) {
    if cpu.key[cpu.v[args.x as usize] as usize] == 1 {
        cpu.pc += 2;
    }
}

// EXA1	KeyOp	if (key() != Vx)	Skips the next instruction if the key stored in VX is not pressed. (Usually the next instruction is a jump to skip a code block);
pub fn skip_not_on_key(cpu: &mut Cpu, args: &Args) {
    if cpu.key[cpu.v[args.x as usize] as usize] != 1 {
        cpu.pc += 2;
    }
}