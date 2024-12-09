// WARNING: for running this on inputs you are required to use --release
// Must admit, this code is everything but good.

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut kind_count = 0u32;
    let line = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .flat_map(|(idx, c)| {
            let mut vec = Vec::new();
            let num = c as u8 - '0' as u8;
            let enabled = idx % 2 == 0;
            for _ in 0..num {
                if enabled {
                    vec.push(Some(kind_count));
                } else {
                    vec.push(None);
                }
            }
            if enabled {
                kind_count += 1;
            }
            vec
        })
        .collect::<Vec<_>>();

    fn checksum(vec: &Vec<Option<u32>>) -> usize {
        vec.into_iter()
            .enumerate()
            .map(|(idx, num)| {
                if *num == None {
                    0
                } else {
                    idx * num.unwrap() as usize
                }
            })
            .sum::<usize>()
    }

    let mut part1 = line.clone();
    for (idx, c) in part1.clone().iter().enumerate() {
        if *c == None {
            let last_num = part1.iter().rposition(|c| *c != None).unwrap();
            if last_num > idx {
                part1[idx] = part1[last_num];
                part1[last_num] = None;
            } else {
                break;
            }
        }
    }

    println!("Part 1: {}", checksum(&part1));

    fn find_space(line: &Vec<Option<u32>>, target_len: usize) -> Option<usize> {
        let mut idx = -1isize;
        let mut len = 0;

        for i in 0..line.len() {
            if line[i] == None {
                if idx == -1 {
                    idx = i as isize;
                }
                len += 1;
                if len >= target_len {
                    break;
                }
            } else {
                len = 0;
                idx = -1;
            }
        }

        return if idx != -1 { Some(idx as usize) } else { None };
    }
    fn find_block(line: &Vec<Option<u32>>, kind: u32) -> Option<(usize, usize)> {
        let mut idx = usize::MAX;

        for i in 0..line.len() {
            if line[i] != None && line[i].unwrap() == kind {
                if idx == usize::MAX {
                    idx = i;
                }
            } else if idx != usize::MAX {
                return Some((idx, i));
            }
        }
        if idx != usize::MAX {
            return Some((idx, line.len()));
        }

        return None;
    }

    let mut part2 = line.clone();
    for kind in (0..kind_count).rev() {
        let block = find_block(&part2, kind);
        if let Some(block) = block {
            let length = block.1 - block.0;
            let space = find_space(&part2, length);
            if let Some(space) = space {
                if space < block.0 {
                    for i in 0..length {
                        part2[space + i] = part2[block.0 + i];
                        part2[block.0 + i] = None;
                    }
                }
            }
        }
    }

    println!("Part 2: {}", checksum(&part2));
}
