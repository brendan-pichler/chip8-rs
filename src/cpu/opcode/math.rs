use crate::cpu::Cpu;
use super::Args;

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