use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self, Read},
    ops::BitXor,
};

fn mix(a: u64, b: u64) -> u64 {
    a.bitxor(b)
}

fn prune(x: u64) -> u64 {
    x.rem_euclid(16777216)
}

fn mutate(mut x: u64) -> u64 {
    x = prune(mix(x * 64, x));
    x = prune(mix(x / 32, x));
    x = prune(mix(x * 2048, x));

    x
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let nums = input
        .lines()
        .map(|str| str.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let part1 = nums
        .iter()
        .map(|&num| {
            let mut num = num;
            for _ in 0..2000 {
                num = mutate(num);
            }
            num
        })
        .sum::<u64>();

    println!("Part 1: {part1}");

    let mut scores = HashMap::<_, u64>::new();

    for num in nums {
        let mut seen = HashSet::new();
        let mut num = num;

        let mut last = num;
        let mut queue = VecDeque::new();
        for _ in 0..2000 {
            num = mutate(num);

            let last_digit = num.rem_euclid(10);

            let diff = last_digit as i64 - last as i64;
            queue.push_back(diff);
            if queue.len() > 4 {
                queue.pop_front();
            }

            if queue.len() == 4 && !seen.contains(&queue) {
                *scores.entry(queue.clone()).or_default() += last_digit;
                seen.insert(queue.clone());
            }

            last = last_digit;
        }
    }

    println!("Part 2: {}", scores.values().max().unwrap());
}
