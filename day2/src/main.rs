use std::io::{self, Read};

fn verify<'a, I>(mut levels: I) -> bool
where
    I: Iterator<Item = &'a i64>,
{
    let mut last_diff = 0i64;
    let mut last_num = *levels.next().unwrap();
    return levels.all(|num| {
        let diff = last_num - *num;
        if diff == 0 || diff.abs() > 3 {
            return false;
        }

        if last_diff != 0 && ((last_diff > 0 && diff < 0) || (last_diff < 0 && diff > 0)) {
            return false;
        }

        last_diff = diff;
        last_num = *num;
        return true;
    });
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut part1 = 0u64;
    let mut part2 = 0u64;

    input
        .lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let levels = line
                .split_whitespace()
                .map(|word| word.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            if verify(levels.iter()) {
                part1 += 1;
                part2 += 1;
            } else {
                // You could probably count errors and then do a <= 1, but my verify algorithm isn't very happy with that
                for i in 0..levels.len() {
                    let mut new = levels.clone();
                    new.remove(i);
                    if verify(new.iter()) {
                        part2 += 1;
                        break;
                    }
                }
            }
        });

    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
