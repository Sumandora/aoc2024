use std::{
    collections::HashSet,
    io::{self, Read},
};

use pathfinding::directed::astar::astar_bag;

#[derive(Clone)]
enum Cell {
    None,
    Wall,
    Start,
    End,
}

impl Cell {
    fn can_be_walked_on(&self) -> bool {
        match self {
            Cell::None => true,
            Cell::Wall => false,
            Cell::Start => true,
            Cell::End => true,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Pos {
    direction: (i32, i32),
    pos: (i32, i32),
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => Cell::Start,
                    'E' => Cell::End,
                    '.' => Cell::None,
                    '#' => Cell::Wall,
                    _ => panic!("input error"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    fn find_start(grid: &Vec<Vec<Cell>>) -> (i32, i32) {
        for (y, line) in grid.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if matches!(c, Cell::Start) {
                    return (x as i32, y as i32);
                }
            }
        }
        panic!("no start");
    }

    fn find_end(grid: &Vec<Vec<Cell>>) -> (i32, i32) {
        for (y, line) in grid.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if matches!(c, Cell::End) {
                    return (x as i32, y as i32);
                }
            }
        }
        panic!("no end");
    }

    let start = Pos {
        direction: (1, 0),
        pos: find_start(&grid),
    };
    let end = find_end(&grid);

    let path = astar_bag(
        &start,
        |pos| {
            let mut vec = Vec::new();

            for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                let new_x = pos.pos.0 + dx;
                let new_y = pos.pos.1 + dy;
                if new_y < 0 && new_y >= grid.len() as i32 {
                    continue;
                }
                if new_x < 0 && new_x >= grid[new_y as usize].len() as i32 {
                    continue;
                }
                let c = grid[new_y as usize][new_x as usize].clone();
                if c.can_be_walked_on() {
                    let cost = if pos.direction != (dx, dy) {
                        1 + 1000
                    } else {
                        1
                    };

                    vec.push((
                        Pos {
                            pos: (new_x, new_y),
                            direction: (dx, dy),
                        },
                        cost,
                    ));
                }
            }

            vec
        },
        |pos| (pos.pos.0.abs_diff(end.0) + pos.pos.1.abs_diff(end.1)) / 3,
        |pos| matches!(grid[pos.pos.1 as usize][pos.pos.0 as usize], Cell::End),
    );
    let path = path.unwrap();
    let paths = path.0.collect::<Vec<_>>();

    println!("Part 1: {}", path.1);

    let path_positions = paths
        .iter()
        .flat_map(|pos| pos.iter().map(|p| p.pos).collect::<Vec<_>>())
        .collect::<HashSet<_>>();

    println!("Part 2: {}", path_positions.len())
}
