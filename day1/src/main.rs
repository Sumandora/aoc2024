use std::{
    io::{self, Read},
    iter::zip,
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (mut inputs1, mut inputs2): (Vec<u64>, Vec<u64>) = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut ints = line.split_whitespace().map(|it| it.parse::<u64>().unwrap());
            (ints.next().unwrap(), ints.next().unwrap())
        })
        .unzip();

    inputs1.sort();
    inputs2.sort();

    let (dists, sims): (Vec<_>, Vec<_>) = zip(inputs1, inputs2.clone())
        .map(|(it1, it2)| {
            (
                // Part 1:
                it2.abs_diff(it1),
                // Part 2:
                it1 * inputs2.iter().filter(|it| **it == it1).count() as u64,
            )
        })
        .unzip();

    let sum_of_dist: u64 = dists.iter().sum();
    let sum_of_sims: u64 = sims.iter().sum();

    println!("dist: {}\nsim: {}", sum_of_dist, sum_of_sims);
}
