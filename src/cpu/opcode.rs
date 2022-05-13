mod functions;

use super::Cpu;
use std::collections::HashMap;
use functions::*;

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
    let mut d_op: Opcode;

    d_op.args = 0x0FFF & opcode;

    match 0xF000 & opcode {
        0xA000 => d_op.op_fn = mvi,
        0xD000 => d_op.op_fn = display,
        0x0000 => {
            match opcode & 0x00FF {
                0x00E0 => d_op.op_fn = disp_clear,
                0x00EE => d_op.op_fn = ret_sub,
            };
        },
        0x1000 => d_op.op_fn = goto,
        0x3000 => d_op.op_fn = skip_eq,
        0x4000 => d_op.op_fn = skip_not_eq,
        0x5000 => d_op.op_fn = skip_reg_eq,
        0x6000 => d_op.op_fn = set_reg,
        0x7000 => d_op.op_fn = add_const_reg,
        0x8000 => {
            match opcode & 0x000F {
                0x0000 => d_op.op_fn = add_reg_reg,
                0x0001 => d_op.op_fn = bitwise_reg_reg,
            };
        },
        _ => panic!("{}", format!("Opcode {:X} not found.", opcode)),
    };

    d_op
}