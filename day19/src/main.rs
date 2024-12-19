use std::io::{self, Read};

use memoize::memoize;

#[memoize(Ignore: stripes)]
fn find_solutions(stripes: &Vec<String>, stripe: String) -> u64 {
    stripes
        .iter()
        .map(|valid_stripe| {
            let mut c = 0;
            if stripe.starts_with(valid_stripe) {
                let cut = stripe[valid_stripe.len()..].to_owned();
                if cut.is_empty() {
                    c += 1;
                } else {
                    c += find_solutions(stripes, cut);
                }
            }
            c
        })
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let (possible, designs) = input.split_once("\n\n").unwrap();

    let possible_stripes = possible
        .split(", ")
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();

    let designs = designs
        .split("\n")
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();

    let solutions = designs
        .into_iter()
        .map(|stripe| find_solutions(&possible_stripes, stripe))
        .collect::<Vec<_>>();

    let part1 = solutions.iter().filter(|sols| **sols > 0).count();
    let part2 = solutions.iter().sum::<u64>();

    println!("Part 1: {part1}\nPart 2: {part2}");
}
