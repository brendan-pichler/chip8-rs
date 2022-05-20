use crate::cpu::Cpu;
use super::Args;
use rand;

// 0NNN	Call    Calls machine code routine (RCA 1802 for COSMAC VIP) at address NNN. Not necessary for most ROMs.
pub fn call(_cpu: &mut Cpu, _args: &Args) {
    panic!("Call not implemented!");
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

// 1NNN	Flow	goto NNN;	Jumps to address NNN.
pub fn goto(cpu: &mut Cpu, args: &Args) {
    cpu.pc = args.nnn;
}

// 2NNN	Flow	*(0xNNN)()	Calls subroutine at NNN.
pub fn call_sub(cpu: &mut Cpu, args: &Args) {
    cpu.stack[cpu.sp as usize] = cpu.pc;
    cpu.sp += 1;
    cpu.pc = args.nnn;
}

// 3XNN	Cond	if (Vx == NN)	Skips the next instruction if VX equals NN. (Usually the next instruction is a jump to skip a code block);
pub fn skip_eq(cpu: &mut Cpu, args: &Args) {
    if cpu.v[args.x] == args.nn {
        cpu.pc += 2;
    }
}


// 4XNN	Cond	if (Vx != NN)	Skips the next instruction if VX does not equal NN. (Usually the next instruction is a jump to skip a code block);
pub fn skip_not_eq(cpu: &mut Cpu, args: &Args) {
    if cpu.v[args.x] != args.nn {
        cpu.pc += 2;
    }
}

// 5XY0	Cond	if (Vx == Vy)	Skips the next instruction if VX equals VY. (Usually the next instruction is a jump to skip a code block);
pub fn skip_reg_eq(cpu: &mut Cpu, args: &Args) {
    if cpu.v[args.x] == cpu.v[args.y] {
        cpu.pc += 2;
    }
}

// 6XNN	Const	Vx = NN	Sets VX to NN.
pub fn set(cpu: &mut Cpu, args: &Args) {
    cpu.v[args.x] = args.nn;
}

// 7XNN	Const	Vx += NN	Adds NN to VX. (Carry flag is not changed);
pub fn add_const(cpu: &mut Cpu, args: &Args) {
    cpu.v[args.x] = ((cpu.v[args.x] as u16 + args.nn as u16) % 256) as u8;
}

// 8XY0	Assig	Vx = Vy	Sets VX to the value of VY.
pub fn set_reg(cpu: &mut Cpu, args: &Args) {
    cpu.v[args.x] = cpu.v[args.y];
}

// 8XY1	BitOp	Vx |= Vy	Sets VX to VX or VY. (Bitwise OR operation);
pub fn bitwise_reg(cpu: &mut Cpu, args: &Args) {
    cpu.v[args.x] |= cpu.v[args.y];
}

// 8XY2	BitOp	Vx &= Vy	Sets VX to VX and VY. (Bitwise AND operation);
pub fn and_reg(cpu: &mut Cpu, args: &Args) {
    cpu.v[args.x] &= cpu.v[args.y];
}

// 8XY3[a]	BitOp	Vx ^= Vy	Sets VX to VX xor VY.
pub fn xor_reg(cpu: &mut Cpu, args: &Args) {
    cpu.v[args.x] ^= cpu.v[args.y];
}

// 8XY4	Math	Vx += Vy	Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there is not.
pub fn add_reg(cpu: &mut Cpu, args: &Args) {
    let a = cpu.v[args.x] as u16;
    let b = cpu.v[args.y] as u16;

    if a + b > 255 {
        cpu.v[0xF] = 1;
    } else {
        cpu.v[0xF] = 0;
    }

    cpu.v[args.x] = ((a + b) % 256) as u8;
}

// 8XY5	Math	Vx -= Vy	VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not.
pub fn sub_reg(cpu: &mut Cpu, args: &Args) {
    let a = cpu.v[args.x] as i16;
    let b = cpu.v[args.y] as i16;

    if a - b < 0 {
        cpu.v[0xF] = 0;
    } else {
        cpu.v[0xF] = 1;
    }

    cpu.v[args.x] = ((a - b) % 256) as u8;
}

// 8XY6[a]	BitOp	Vx >>= 1	Stores the least significant bit of VX in VF and then shifts VX to the right by 1.[b]
pub fn right_shift_reg(cpu: &mut Cpu, args: &Args) {
    cpu.v[0xF] = cpu.v[args.x] & 0b00000001;
    cpu.v[args.x] >>= 1;
}

// 8XY7[a]	Math	Vx = Vy - Vx	Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there is not.
pub fn sub_reg_store(cpu: &mut Cpu, args: &Args) {
    let a = cpu.v[args.x] as i16;
    let b = cpu.v[args.y] as i16;

    if b - a < 0 {
        cpu.v[0xF] = 0;
    } else {
        cpu.v[0xF] = 1;
    }

    cpu.v[args.x] = ((b - a) % 256) as u8;
}

// 8XYE[a]	BitOp	Vx <<= 1	Stores the most significant bit of VX in VF and then shifts VX to the left by 1.[b]
pub fn left_shift_reg(cpu: &mut Cpu, args: &Args) {
    cpu.v[0xF] = (cpu.v[args.x] & 0b10000000) >> 7;
    cpu.v[args.x] <<= 1;
}

// 9XY0	Cond	if (Vx != Vy)	Skips the next instruction if VX does not equal VY. (Usually the next instruction is a jump to skip a code block);
pub fn not_eq_reg(cpu: &mut Cpu, args: &Args) {
    if cpu.v[args.x] != cpu.v[args.y] {
        cpu.pc += 2;
    }
}

// ANNN	MEM	I = NNN	Sets I to the address NNN.
pub fn mvi(cpu: &mut Cpu, args: &Args)  {
    cpu.i = args.nnn;
}

// BNNN	Flow	PC = V0 + NNN	Jumps to the address NNN plus V0.
pub fn jmp_offset(cpu: &mut Cpu, args: &Args) {
    cpu.pc = cpu.v[0] as u16 + args.nnn;
}

// CXNN	Rand	Vx = rand() & NN	Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
pub fn bitwise_rand(cpu: &mut Cpu, args: &Args) {
    let random: u8 = rand::random::<u8>();
    cpu.v[args.x] = args.nn & random;
}

// DXYN	Display	draw(Vx, Vy, N)	Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. Each row of 8 pixels is read as bit-coded starting 
// from memory location I; I value does not change after the execution of this instruction. 
// As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that does not happen
pub fn display(cpu: &mut Cpu, args: &Args) {
    let x_coord = (cpu.v[args.x] % 64) as u16;
    let y_coord = (cpu.v[args.y] % 32) as u16;

    cpu.v[0xF] = 0;
    for y in 0..args.n {
        if y_coord + y as u16 > 31 {
            break;
        }
        let sprite_byte = cpu.memory[(cpu.i + y as u16) as usize];

        for x in 0..8 {
            if x_coord + x > 63 {
                break;
            }

            let sprite_pixel: u8 = ((0b10000000 >> x) & sprite_byte) >> (7 - x);
            let pixel = &mut cpu.gfx[(x_coord + x + (y_coord + y as u16) * 64) as usize];

            if sprite_pixel == 1 {
                if *pixel == 1 {
                    *pixel = 0;
                    cpu.v[0xF] = 1;
                } else {
                    *pixel = 1;
                }
            }
        }
    }
    // cpu.draw_flag = 1;
}

// EX9E	KeyOp	if (key() == Vx)	Skips the next instruction if the key stored in VX is pressed. (Usually the next instruction is a jump to skip a code block);
pub fn skip_on_key(cpu: &mut Cpu, args: &Args) {
    let key = cpu.v[args.x];
    if cpu.key[key as usize] == 1 {
        cpu.pc += 2;
    }
}

// EXA1	KeyOp	if (key() != Vx)	Skips the next instruction if the key stored in VX is not pressed. (Usually the next instruction is a jump to skip a code block);
pub fn skip_not_on_key(cpu: &mut Cpu, args: &Args) {
    let key = cpu.v[args.x];
    if cpu.key[key as usize] != 1 {
        cpu.pc += 2;
    }
}

// FX07	Timer	Vx = get_delay()	Sets VX to the value of the delay timer.
pub fn get_delay(cpu: &mut Cpu, args: &Args) {
    cpu.v[args.x] = cpu.delay_timer;
    cpu.pc += 2;
}

// FX0A	KeyOp	Vx = get_key()	A key press is awaited, and then stored in VX. (Blocking Operation. All instruction halted until next key event);
pub fn get_key_block(cpu: &mut Cpu, args: &Args) {
    let mut key_pressed = false;
    // Must block
    for key in cpu.key {
        if key == 1 {
            cpu.v[args.x] = key;
            key_pressed = true;
        }
    }

    if !key_pressed {
        cpu.pc -= 2;
    }
}
// FX15	Timer	delay_timer(Vx)	Sets the delay timer to VX.
pub fn set_delay_timer(cpu: &mut Cpu, args: &Args) {
    cpu.delay_timer = cpu.v[args.x];
}

// FX18	Sound	sound_timer(Vx)	Sets the sound timer to VX.
pub fn set_sound_timer(cpu: &mut Cpu, args: &Args) {
    cpu.sound_timer = cpu.v[args.x];
}

// FX1E	MEM	I += Vx	Adds VX to I. VF is not affected.[c]
pub fn add_reg_i(cpu: &mut Cpu, args: &Args) {
    cpu.i += cpu.v[args.x] as u16
}

// FX29	MEM	I = sprite_addr[Vx]	Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.
pub fn set_i_sprite(cpu: &mut Cpu, args: &Args) {
    cpu.i = cpu.v[args.x] as u16 * 5;
}

// FX33	BCD	
// set_BCD(Vx)
// *(I+0) = BCD(3);
// *(I+1) = BCD(2);
// *(I+2) = BCD(1);
// Stores the binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2. (In other words, take the decimal representation of VX, place the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.);
pub fn set_bcd(cpu: &mut Cpu, args: &Args) {
    cpu.memory[cpu.i as usize] = cpu.v[args.x] / 100;
    cpu.memory[cpu.i as usize + 1] = (cpu.v[args.x] / 10) % 10;
    cpu.memory[cpu.i as usize + 2] = (cpu.v[args.x] % 100) % 10;
}

// FX55	MEM	reg_dump(Vx, &I)	Stores from V0 to VX (including VX) in memory, starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.[d]
pub fn reg_dump(cpu: &mut Cpu, args: &Args) {
    for i in 0..args.x + 1 {
        cpu.memory[cpu.i as usize + i] = cpu.v[i];
    }
    cpu.i += args.x as u16 + 1;
}

// FX65	MEM	reg_load(Vx, &I)	Fills from V0 to VX (including VX) with values from memory, starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.[d]
pub fn reg_load(cpu: &mut Cpu, args: &Args) {
    for i in 0..args.x + 1 {
        cpu.v[i] = cpu.memory[cpu.i as usize + i];
    }
    cpu.i += args.x as u16 + 1;
}