use crate::cpu::Cpu;
use super::Args;

// FX07	Timer	Vx = get_delay()	Sets VX to the value of the delay timer.
pub fn get_delay(cpu: &mut Cpu, args: &Args) {
    cpu.v[args.x as usize] = cpu.delay_timer;
    cpu.pc += 2;
}

// FX0A	KeyOp	Vx = get_key()	A key press is awaited, and then stored in VX. (Blocking Operation. All instruction halted until next key event);
pub fn get_key_block(cpu: &mut Cpu, args: &Args) {
    let mut key_pressed = false;
    // Must block
    for key in cpu.key {
        if key == 1 {
            cpu.v[args.x as usize] = key;
            key_pressed = true;
        }
    }

    if !key_pressed {
        cpu.pc -= 2;
    }
}
// FX15	Timer	delay_timer(Vx)	Sets the delay timer to VX.
pub fn set_delay_timer(cpu: &mut Cpu, args: &Args) {
    cpu.delay_timer = cpu.v[args.x as usize];
}

// FX18	Sound	sound_timer(Vx)	Sets the sound timer to VX.
pub fn set_sound_timer(cpu: &mut Cpu, args: &Args) {
    cpu.sound_timer = cpu.v[args.x as usize];
}

// FX1E	MEM	I += Vx	Adds VX to I. VF is not affected.[c]
pub fn add_reg_i(cpu: &mut Cpu, args: &Args) {
    cpu.i += cpu.v[args.x as usize] as u16;
}

// FX29	MEM	I = sprite_addr[Vx]	Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.
pub fn set_i_sprite(cpu: &mut Cpu, args: &Args) {
    cpu.i = cpu.v[args.x as usize] as u16 * 5;
}

// FX33	BCD	
// set_BCD(Vx)
// *(I+0) = BCD(3);
// *(I+1) = BCD(2);
// *(I+2) = BCD(1);
// Stores the binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2. (In other words, take the decimal representation of VX, place the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.);
pub fn set_bcd(cpu: &mut Cpu, args: &Args) {
    cpu.memory[cpu.i as usize] = cpu.v[args.x as usize] / 100;
    cpu.memory[cpu.i as usize] = cpu.v[args.x as usize] / 10 % 10;
    cpu.memory[cpu.i as usize] = cpu.v[args.x as usize] % 100 % 10;
}

// FX55	MEM	reg_dump(Vx, &I)	Stores from V0 to VX (including VX) in memory, starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.[d]
pub fn reg_dump(cpu: &mut Cpu, args: &Args) {
    for i in 0..args.x {
        cpu.memory[cpu.i as usize + i as usize] = cpu.v[i as usize];
    }
}

// FX65	MEM	reg_load(Vx, &I)	Fills from V0 to VX (including VX) with values from memory, starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.[d]
pub fn reg_load(cpu: &mut Cpu, args: &Args) {
    for i in 0..args.x {
        cpu.v[i as usize] = cpu.memory[cpu.i as usize + i as usize];
    }
}