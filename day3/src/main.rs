use std::io::{self, Read};

use regex::Regex;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let re = Regex::new(r"(mul\((\d*),(\d*)\))|(don't\(\))|(do\(\))").unwrap();

    let mut active = true;
    let mut part1 = 0i64;
    let mut part2 = 0i64;

    for captures in re.captures_iter(input.as_str()) {
        if captures[0].starts_with("mul") {
            let num1 = &captures[2];
            let num2 = &captures[3];
            if !num1.is_empty() && num1.len() <= 3 && !num2.is_empty() && num2.len() <= 3 {
                let a = num1.parse::<i64>().unwrap();
                let b = num2.parse::<i64>().unwrap();

                part1 += a * b;
                if active {
                    part2 += a * b;
                }
            }
        } else if captures[0].starts_with("don't") {
            active = false;
        } else if captures[0].starts_with("do") {
            active = true;
        }
    }

    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
