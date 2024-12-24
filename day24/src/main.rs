use std::{
    collections::HashMap,
    io::{self, Read},
    ops::{BitAnd, BitOr, BitXor},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Opcode {
    Xor,
    Or,
    And,
}

fn compute(
    constants: &HashMap<String, bool>,
    gates: &Vec<(String, Opcode, String, String)>,
) -> Vec<bool> {
    fn dfs(
        constants: &HashMap<String, bool>,
        gates: &Vec<(String, Opcode, String, String)>,
        target: &String,
    ) -> Option<bool> {
        for gate in gates {
            if &gate.3 == target {
                let a = dfs(constants, gates, &gate.0)?;
                let b = dfs(constants, gates, &gate.2)?;
                return Some(match gate.1 {
                    Opcode::Xor => a.bitxor(b),
                    Opcode::Or => a.bitor(b),
                    Opcode::And => a.bitand(b),
                });
            }
        }

        for constant in constants {
            if constant.0 == target {
                return Some(*constant.1);
            }
        }

        None
    }

    let max = gates
        .iter()
        .map(|gate| &gate.3)
        .filter(|name| name.starts_with("z"))
        .map(|s| s[1..].parse::<u64>().unwrap())
        .max()
        .unwrap();

    let mut bits = Vec::new();

    for i in (0..=max).rev() {
        let out = format!("z{i:02}");
        let var = dfs(constants, gates, &out);
        if let Some(var) = var {
            let b = var;
            bits.push(b);
        }
    }

    bits
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let (constants_str, rules) = input.split_once("\n\n").unwrap();

    let mut constants = HashMap::new();
    constants_str.lines().for_each(|str| {
        let (name, value) = str.split_once(": ").unwrap();
        constants.insert(name.to_owned(), value.parse::<u8>().unwrap() == 1);
    });

    let gates = rules
        .lines()
        .map(|str| {
            let mut it = str.split(" ");
            let var1 = it.next().unwrap();
            let op = it.next().unwrap();
            let op = match op {
                "XOR" => Opcode::Xor,
                "OR" => Opcode::Or,
                "AND" => Opcode::And,
                _ => panic!("unhandled opcode"),
            };
            let var2 = it.next().unwrap();
            it.next().unwrap();
            let res = it.next().unwrap();
            (var1.to_owned(), op, var2.to_owned(), res.to_owned())
        })
        .collect::<Vec<_>>();

    let z = compute(&constants, &gates);

    fn bools_to_number(bools: &[bool]) -> u64 {
        let mut number: u64 = 0;
        for &bit in bools {
            number = (number << 1) | (bit as u64);
        }
        number
    }

    println!("Part 1: {}", bools_to_number(&z));
    // I can't think of an automatic solution to part 2...
    // I provide a visualizer which makes it slightly easier to spot irregularities
}
