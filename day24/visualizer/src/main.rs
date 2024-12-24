use std::{
    collections::HashMap,
    io::{self, Read},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Opcode {
    Xor,
    Or,
    And,
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

    println!("flowchart TD");
    for (constant, _) in constants {
        println!("    {constant}[{constant}]");
    }

    for (c, (a, op, b, res)) in gates.into_iter().enumerate() {
        println!("    {res}[{res}]");
        let block_name;
        match op {
            Opcode::Xor => {
                block_name = format!("XOR{c}");
                println!("    XOR{c}[XOR]")
            }
            Opcode::Or => {
                block_name = format!("OR{c}");
                println!("    OR{c}(OR)")
            }
            Opcode::And => {
                block_name = format!("AND{c}");
                println!("    AND{c}{{AND}}")
            }
        }
        println!("    {a}[{a}] --> {block_name}");
        println!("    {b}[{b}] --> {block_name}");
        println!("    {block_name} --> {res}[{res}]");
    }
}
