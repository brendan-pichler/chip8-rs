use crate::cpu::Cpu;

pub fn mvi(cpu: &mut Cpu, arg: u16)  {
    cpu.i = arg;
    cpu.pc += 2;
}

// DXYN	Display	draw(Vx, Vy, N)	Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. Each row of 8 pixels is read as bit-coded starting from memory location I; I value does not change after the execution of this instruction. As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that does not happen
pub fn display(cpu: &mut Cpu, arg: u16) {
    let x = arg & 0x0F00;
    let y = arg & 0x00F0;
    let height = arg & 0x000F;

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

pub fn goto(cpu: &mut Cpu, arg: u16) {
    cpu.pc = arg;
}

// 3XNN	Cond	if (Vx == NN)	Skips the next instruction if VX equals NN. (Usually the next instruction is a jump to skip a code block);
pub fn skip_eq(cpu: &mut Cpu, arg: u16) {
    let x = arg & 0x0F00;
    let nn = arg & 0x00FF;

    if cpu.v[x as usize] == nn as u8 {
        cpu.pc += 2;
    }
    cpu.pc += 2;
}

// 0NNN	Call    Calls machine code routine (RCA 1802 for COSMAC VIP) at address NNN. Not necessary for most ROMs.
pub fn call(cpu: &mut Cpu, arg: u16) {
    let nnn = arg & 0x0FFF;

    if nnn == 0x00E0 {
        disp_clear(cpu, arg)
    } else if nnn == 0x0EE {
        ret_sub(cpu, arg);
    }

    // TODO: Implement
}

// 4XNN	Cond	if (Vx != NN)	Skips the next instruction if VX does not equal NN. (Usually the next instruction is a jump to skip a code block);
pub fn skip_not_eq(cpu: &mut Cpu, arg: u16) {
    let x = arg & 0x0F00;
    let nn = arg & 0x00FF;

    if cpu.v[x as usize] != nn as u8 {
        cpu.pc += 2;
    }
    cpu.pc += 2;
}

// 2NNN	Flow	*(0xNNN)()	Calls subroutine at NNN.
pub fn call_sub(cpu: &mut Cpu, arg: u16) {
    let nnn = arg & 0x0FFF;
    cpu.stack[cpu.sp as usize] = cpu.pc;
    cpu.sp += 1;
    cpu.pc = nnn;
}

// 00E0	Display	disp_clear()	Clears the screen.
pub fn disp_clear(cpu: &mut Cpu, _arg: u16) {
    for i in cpu.gfx.iter_mut() {
        *i = 0;
    }
    cpu.pc += 2;
}

// 00EE	Flow	return;	Returns from a subroutine.
pub fn ret_sub(cpu: &mut Cpu, _arg: u16) {
    cpu.sp -= 1;
    cpu.pc = cpu.stack[cpu.sp as usize];
    cpu.pc += 2;
}

// 5XY0	Cond	if (Vx == Vy)	Skips the next instruction if VX equals VY. (Usually the next instruction is a jump to skip a code block);
pub fn skip_reg_eq(cpu: &mut Cpu, arg: u16) {
    let x = arg & 0x0F00;
    let y = arg & 0x00F0;

    if cpu.v[x as usize] != cpu.v[y as usize] {
        cpu.pc += 2;
    }
    cpu.pc += 2;
}

// 6XNN	Const	Vx = NN	Sets VX to NN.
pub fn set_reg(cpu: &mut Cpu, arg: u16) {
    let x = arg & 0x0F00;
    let nn = arg & 0x00FF;

    cpu.v[x as usize] = nn as u8;
    cpu.pc += 2;
}

// 7XNN	Const	Vx += NN	Adds NN to VX. (Carry flag is not changed);
pub fn add_const_reg(cpu: &mut Cpu, arg: u16) {
    let x = arg & 0x0F00;
    let nn = arg & 0x00FF;

    cpu.v[x as usize] += nn as u8;
    cpu.pc += 2;
}



// 8XY0	Assig	Vx = Vy	Sets VX to the value of VY.
pub fn add_reg_reg(cpu: &mut Cpu, arg: u16) {
    let x = arg & 0x0F00;
    let y = arg & 0x00F0;

    cpu.v[x as usize] = cpu.v[y as usize];
    cpu.pc += 2;
}

// 8XY1	BitOp	Vx |= Vy	Sets VX to VX or VY. (Bitwise OR operation);
pub fn bitwise_reg_reg(cpu: &mut Cpu, arg: u16) {
    let x = arg & 0x0F00;
    let y = arg & 0x00F0;

    cpu.v[x as usize] = cpu.v[y as usize];
    cpu.pc += 2;
}