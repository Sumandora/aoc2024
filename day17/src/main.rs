use std::{
    io::{self, Read},
    ops::{BitOr, BitXor, Shl, Shr},
};

#[derive(Debug, Default, Clone)]
struct VmState {
    ip: usize,
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
}

fn as_combo(state: &VmState, operand: u8) -> i64 {
    match operand {
        0..=3 => operand as i64,
        4 => state.reg_a,
        5 => state.reg_b,
        6 => state.reg_c,
        7 => panic!("7 is reserved"),
        _ => panic!("invalid combo operand"),
    }
}

fn start_vm(mut state: VmState, insn_data: &Vec<u8>) -> Vec<u8> {
    let mut output = Vec::new();
    'next_insn: while state.ip < insn_data.len() {
        let opcode = *insn_data.get(state.ip).unwrap();
        let operand = *insn_data.get(state.ip + 1).unwrap();
        match opcode {
            0 => {
                let numerator = state.reg_a;
                let denominator = 2i64.pow(as_combo(&state, operand) as u32);
                state.reg_a = numerator / denominator;
            }
            1 => {
                state.reg_b = state.reg_b.bitxor(operand as i64);
            }
            2 => {
                state.reg_b = as_combo(&state, operand).rem_euclid(8);
            }
            3 => {
                if state.reg_a != 0 {
                    state.ip = operand as usize;
                    continue 'next_insn;
                }
            }
            4 => {
                state.reg_b = state.reg_b.bitxor(state.reg_c);
            }
            5 => {
                let value = as_combo(&state, operand).rem_euclid(8) as u8;
                output.push(value);
            }
            6 => {
                let numerator = state.reg_a;
                let denominator = 2i64.pow(as_combo(&state, operand) as u32);
                state.reg_b = numerator / denominator;
            }
            7 => {
                let numerator = state.reg_a;
                let denominator = 2i64.pow(as_combo(&state, operand) as u32);
                state.reg_c = numerator / denominator;
            }

            _ => panic!("unhandled opcode: {}", opcode),
        }

        state.ip += 2;
    }
    output
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut state = VmState::default();
    let mut insn_data = Vec::new();
    input
        .lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let (k, v) = line.split_once(": ").unwrap();
            match k {
                "Register A" => {
                    state.reg_a = v.parse::<i64>().unwrap();
                }
                "Register B" => {
                    state.reg_b = v.parse::<i64>().unwrap();
                }
                "Register C" => {
                    state.reg_c = v.parse::<i64>().unwrap();
                }
                "Program" => {
                    v.split(",")
                        .map(|op| op.parse::<u8>().unwrap())
                        .for_each(|op| insn_data.push(op));
                }
                _ => panic!("input error"),
            };
        });

    let part1 = start_vm(state.clone(), &insn_data);
    println!(
        "Part 1: {}",
        part1
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    // There is no way to solve part 2 correctly.
    // The solution wanted by AOC is to analyze the input by hand and basically brute force numbers in 3 bit parts from the right to left.
}
