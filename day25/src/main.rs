use std::io::{self, Read};

use kust::ScopeFunctions;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let (locks, keys): (Vec<_>, Vec<_>) = input
        .split("\n\n")
        .map(|lock_or_key| {
            let mut heights = [0u32; 5];
            let is_lock = lock_or_key.starts_with("#");

            lock_or_key
                .lines()
                .using(|iter| {
                    if !is_lock {
                        iter.rev().collect::<Vec<_>>()
                    } else {
                        iter.collect::<Vec<_>>()
                    }
                })
                .iter()
                .skip(1)
                .for_each(|line| {
                    line.chars().enumerate().for_each(|(i, c)| {
                        if c == '#' {
                            heights[i] += 1;
                        }
                    });
                });

            (heights, is_lock)
        })
        .partition(|(_, is_lock)| *is_lock);

    println!(
        "Part 1: {}",
        locks
            .iter()
            .map(|lock| {
                keys.iter()
                    .filter(|key| lock.0.iter().zip(key.0).all(|(a, b)| a + b < 6))
                    .count()
            })
            .sum::<usize>()
    );
}
