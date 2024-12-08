use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut map = HashMap::<char, Vec<(i32, i32)>>::new();
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '.' {
                return;
            }
            map.entry(c)
                .or_insert(Vec::new())
                .push((x as i32, y as i32));
        });
    });

    let mut part1 = HashSet::new();
    let mut part2 = HashSet::new();

    map.iter().for_each(|(_, v)| {
        v.iter().enumerate().for_each(|(idx, (x1, y1))| {
            v.iter().skip(idx + 1).for_each(|(x2, y2)| {
                let diff = (x2 - x1, y2 - y1);

                let mut mult = 0;
                loop {
                    let a = (x1 - diff.0 * mult, y1 - diff.1 * mult);
                    if a.0 >= 0 && a.0 < width && a.1 >= 0 && a.1 < height {
                        if mult == 1 {
                            part1.insert(a);
                        }
                        part2.insert(a);
                        mult += 1;
                    } else {
                        break;
                    }
                }
                mult = 0;
                loop {
                    let b = (x2 + diff.0 * mult, y2 + diff.1 * mult);
                    if b.0 >= 0 && b.0 < width && b.1 >= 0 && b.1 < height {
                        if mult == 1 {
                            part1.insert(b);
                        }
                        part2.insert(b);
                        mult += 1;
                    } else {
                        break;
                    }
                }
            });
        });
    });

    println!("Part 1: {}\nPart 2: {}", part1.len(), part2.len());
}
