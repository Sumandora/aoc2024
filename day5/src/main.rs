use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let rules = input
        .lines()
        .filter(|line| line.contains("|"))
        .map(|line| line.split_once("|").unwrap())
        .map(|pair| {
            (
                pair.0.parse::<u64>().unwrap(),
                pair.1.parse::<u64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let get_rules_for = |num: u64| {
        rules
            .iter()
            .filter(|(a, _)| *a == num)
            .map(|(_, b)| b)
            .collect::<Vec<_>>()
    };

    let updates = input
        .lines()
        .filter(|line| line.contains(","))
        .map(|update_line| {
            update_line
                .split(",")
                .map(|str| str.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        });

    let (correct_order, wrong_order): (Vec<_>, Vec<_>) = updates.partition(|updates| {
        let mut passed = Vec::new();
        for update in updates {
            if get_rules_for(*update)
                .iter()
                .any(|rule| passed.contains(rule))
            {
                return false;
            }

            passed.push(update);
        }
        true
    });

    let wrong_order = wrong_order
        .iter()
        .map(|updates| {
            // Amateur sorting:

            let mut new_updates = Vec::<u64>::new();

            for update in updates {
                let mut index = new_updates.len();
                get_rules_for(*update).into_iter().for_each(|rule| {
                    if new_updates.contains(rule) {
                        let new_index = new_updates.iter().position(|val| val == rule).unwrap();
                        if index > new_index {
                            index = new_index;
                        }
                    }
                });
                new_updates.insert(index, *update);
            }
            new_updates
        })
        .collect::<Vec<_>>();

    fn sum_middles(vec: Vec<Vec<u64>>) -> u64 {
        vec.iter()
            .map(|updates| *updates.get(updates.len() / 2).unwrap())
            .sum::<u64>()
    }

    let part1 = sum_middles(correct_order);
    let part2 = sum_middles(wrong_order);

    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
