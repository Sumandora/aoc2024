use std::io::{self, Read};

use pathfinding::{directed::astar, matrix::directions::DIRECTIONS_4};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GridCell {
    None,
    Wall,
    Start,
    End,
}
impl GridCell {
    fn can_be_walked_on(self) -> bool {
        match self {
            GridCell::None => true,
            GridCell::Wall => false,
            GridCell::Start => true,
            GridCell::End => true,
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let grid = input
        .lines()
        .map(|line| {
            let mut vec = Vec::new();
            for c in line.chars() {
                vec.push(match c {
                    'S' => GridCell::Start,
                    'E' => GridCell::End,
                    '.' => GridCell::None,
                    '#' => GridCell::Wall,
                    _ => panic!("input error"),
                });
            }
            vec
        })
        .collect::<Vec<_>>();

    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            match c {
                GridCell::Start => start = (x, y),
                GridCell::End => end = (x, y),
                _ => (),
            }
        }
    }

    let baseline_path = astar::astar(
        &start,
        |pos| {
            let mut vec = Vec::new();
            for dir in DIRECTIONS_4 {
                let new_pos = (pos.0 as i32 + dir.0 as i32, pos.1 as i32 + dir.1 as i32);

                if new_pos.1 < 0 || new_pos.1 >= grid.len() as i32 {
                    continue;
                }

                if new_pos.0 < 0 || new_pos.0 >= grid[new_pos.1 as usize].len() as i32 {
                    continue;
                }

                if grid[new_pos.1 as usize][new_pos.0 as usize].can_be_walked_on() {
                    vec.push(((new_pos.0 as usize, new_pos.1 as usize), 1));
                }
            }
            vec
        },
        |pos| (end.0.abs_diff(pos.0) + end.1.abs_diff(pos.1)) / 3,
        |pos| end.0 == pos.0 && end.1 == pos.1,
    )
    .unwrap();

    let minimum = 100;
    // Example:
    // let minimum = 50;

    let mut part1 = 0;
    let mut part2 = 0;

    for (idx, pos) in baseline_path.0.iter().enumerate() {
        for (idx2, other_pos) in baseline_path.0.iter().take(idx).enumerate() {
            let distance = other_pos.0.abs_diff(pos.0) + other_pos.1.abs_diff(pos.1);
            if idx - idx2 >= minimum + distance {
                if distance == 2 {
                    part1 += 1;
                }
                if distance <= 20 {
                    part2 += 1;
                }
            }
        }
    }

    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
