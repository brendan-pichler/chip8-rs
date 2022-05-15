mod functions;
mod math;
mod timer;

use super::Cpu;
use functions::*;
use math::*;
use timer::*;

type OpFn = fn(&mut Cpu, u16);

pub struct Opcode {
    args: u16,
    op_fn: OpFn,
}

impl Opcode {
    pub fn execute_opcode(&self, cpu: &mut Cpu) {
        (self.op_fn)(cpu, self.args);
    }
}

pub fn decode_opcode(opcode: u16) -> Opcode {
    let mut  d_op = Opcode {
        args: opcode,
        op_fn: call,
    };

    d_op.args = 0x0FFF & opcode;

    match 0xF000 & opcode {
        0x0000 => {
            match opcode & 0x00FF {
                0x00E0 => d_op.op_fn = disp_clear,
                0x00EE => d_op.op_fn = ret_sub,
                _ => opcode_not_found(opcode),
            };
        },
        0x1000 => d_op.op_fn = goto,
        0x3000 => d_op.op_fn = skip_eq,
        0x4000 => d_op.op_fn = skip_not_eq,
        0x5000 => d_op.op_fn = skip_reg_eq,
        0x6000 => d_op.op_fn = set,
        0x7000 => d_op.op_fn = add_const,
        0x8000 => {
            match opcode & 0x000F {
                0x0000 => d_op.op_fn = set_reg,
                0x0001 => d_op.op_fn = bitwise_reg,
                0x0002 => d_op.op_fn = and_reg,
                0x0003 => d_op.op_fn = xor_reg,
                0x0004 => d_op.op_fn = add_reg,
                0x0005 => d_op.op_fn = sub_reg,
                0x0006 => d_op.op_fn = right_shift_reg,
                0x0007 => d_op.op_fn = sub_reg_store,
                0x000E => d_op.op_fn = left_shift_reg,
                _ => opcode_not_found(opcode),
            };
        },
        0x9000 => d_op.op_fn = not_eq_reg,
        0xA000 => d_op.op_fn = mvi,
        0xB000 => d_op.op_fn = jmp_offset,
        0xD000 => d_op.op_fn = display,
        0xC000 => d_op.op_fn = bitwise_rand,
        0xE000 => {
            match opcode & 0x00FF {
                0x009E => d_op.op_fn = skip_on_key,
                0x00A1 => d_op.op_fn = skip_not_on_key,
                _ => opcode_not_found(opcode),
            }
        },
        0xF000 => {
            match opcode & 0x00FF {
                0x0007 => d_op.op_fn = get_delay,
                0x000A => d_op.op_fn = get_key_block,
                0x0015 => d_op.op_fn = set_delay_timer,
                0x0018 => d_op.op_fn = set_sound_timer,
                0x001E => d_op.op_fn = add_reg_i,
                0x0029 => d_op.op_fn = set_i_sprite,
                0x0033 => d_op.op_fn = set_bcd,
                0x0055 => d_op.op_fn = reg_dump,
                0x0065 => d_op.op_fn = reg_load,
                _ => opcode_not_found(opcode),
            }
        }
        _ => opcode_not_found(opcode),
    };

    d_op
}

fn opcode_not_found(opcode: u16) {
    panic!("{}", format!("Opcode {:X} not found.", opcode));
}