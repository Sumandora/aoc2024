use std::io::{self, Read};

use memoize::memoize;

#[memoize]
fn mutate(num: u64) -> Vec<u64> {
    if num == 0 {
        vec![1]
    } else {
        let num_str = num.to_string();
        if num_str.len() % 2 == 0 {
            let (a, b) = num_str.split_at(num_str.len() / 2);
            vec![a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()]
        } else {
            vec![num * 2024]
        }
    }
}

#[memoize]
fn run_simulation(num: u64, depth: u64) -> usize {
    if depth == 0 {
        return 1;
    }
    let mut len = 0;
    for num in mutate(num) {
        len += run_simulation(num, depth - 1);
    }
    len
}

fn accumulate_len(nums: &[u64], depth: u64) -> usize {
    nums.iter()
        .map(|num| run_simulation(*num, depth))
        .sum::<usize>()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let nums = input
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .map(|str| str.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let part1 = accumulate_len(&nums, 25);
    println!("Part 1: {part1}");
    let part2 = accumulate_len(&nums, 75);
    println!("Part 2: {part2}");
}
