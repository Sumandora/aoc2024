use std::{
    collections::HashSet,
    io::{self, Read},
};

use regex::Regex;

fn numbers(str: &str) -> Vec<i64> {
    let re = Regex::new(r"([-+]?\d+)").unwrap();
    let mut vec = Vec::new();
    for (_, [num]) in re.captures_iter(str).map(|cap| cap.extract()) {
        vec.push(num.parse::<i64>().unwrap());
    }
    vec
}

#[derive(Debug)]
struct Robot {
    pos: (i64, i64),
    velocity: (i64, i64),
}

// There are many solutions that can yield false positives, but are much faster.
// I just checked if there is a long contiguous line in the image
// That allows one to get the answer, but I wanted a detection that is actually _correct_
const CHRISTMAS_TREE: &str = "###############################
#.............................#
#.............................#
#.............................#
#.............................#
#..............#..............#
#.............###.............#
#............#####............#
#...........#######...........#
#..........#########..........#
#............#####............#
#...........#######...........#
#..........#########..........#
#.........###########.........#
#........#############........#
#..........#########..........#
#.........###########.........#
#........#############........#
#.......###############.......#
#......#################......#
#........#############........#
#.......###############.......#
#......#################......#
#.....###################.....#
#....#####################....#
#.............###.............#
#.............###.............#
#.............###.............#
#.............................#
#.............................#
#.............................#
#.............................#
###############################";

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let robots = input
        .lines()
        .map(|line| {
            let nums = numbers(line);
            Robot {
                pos: (nums.get(0).unwrap().clone(), nums.get(1).unwrap().clone()),
                velocity: (nums.get(2).unwrap().clone(), nums.get(3).unwrap().clone()),
            }
        })
        .collect::<Vec<_>>();

    let width = 101;
    let height = 103;

    let steps = 100;

    let part1 = robots
        .iter()
        .map(|robot| {
            let mut new_x = robot.pos.0 + robot.velocity.0 * steps;
            let mut new_y = robot.pos.1 + robot.velocity.1 * steps;

            new_x = new_x.rem_euclid(width);
            new_y = new_y.rem_euclid(height);

            (new_x, new_y)
        })
        .collect::<Vec<_>>();

    let quad1 = part1
        .iter()
        .filter(|pos| pos.0 < width / 2 && pos.1 < height / 2)
        .count();
    let quad2 = part1
        .iter()
        .filter(|pos| pos.0 < width / 2 && pos.1 > height / 2)
        .count();
    let quad3 = part1
        .iter()
        .filter(|pos| pos.0 > width / 2 && pos.1 < height / 2)
        .count();
    let quad4 = part1
        .iter()
        .filter(|pos| pos.0 > width / 2 && pos.1 > height / 2)
        .count();

    println!("Part 1: {}", quad1 * quad2 * quad3 * quad4);

    let mut step = 1;
    let tree_lines = CHRISTMAS_TREE
        .lines()
        .map(|str| str.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let tree_width = tree_lines.first().unwrap().len() as i64;
    let tree_height = tree_lines.len() as i64;

    let part2 = loop {
        let positions = robots
            .iter()
            .map(|robot| {
                let mut new_x = robot.pos.0 + robot.velocity.0 * step;
                let mut new_y = robot.pos.1 + robot.velocity.1 * step;

                new_x = new_x.rem_euclid(width);
                new_y = new_y.rem_euclid(height);

                (new_x, new_y)
            })
            .collect::<HashSet<_>>();

        let mut matches = false;

        'pattern_scan: for x in 0..width - tree_width {
            for y in 0..height - tree_height {
                let mut inner_match = true;

                'pattern_loop: for pat_x in 0..tree_width {
                    for pat_y in 0..tree_height {
                        if tree_lines[pat_y as usize][pat_x as usize] == '#'
                            && !positions.contains(&(x + pat_x, y + pat_y))
                        {
                            inner_match = false;
                            break 'pattern_loop;
                        }
                    }
                }

                if inner_match {
                    matches = true;
                    break 'pattern_scan;
                }
            }
        }

        if matches {
            break step;
        }

        step += 1;
    };

    println!("Part 2: {part2}");
}
