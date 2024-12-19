// This code is awful, but for the sake of my sanity I will leave it here and not optimize further
use std::io::{self, Read};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
    Box,
    Robot,
    Wall,
    DoubleBoxLeft,
    DoubleBoxRight,
    None,
}

impl Cell {
    fn is_box(&self) -> bool {
        match self {
            Cell::Box => true,
            Cell::Robot => false,
            Cell::Wall => false,
            Cell::DoubleBoxLeft => true,
            Cell::DoubleBoxRight => true,
            Cell::None => false,
        }
    }

    fn find_counterpart(&self, pos: (i32, i32)) -> (i32, i32) {
        if matches!(self, Cell::DoubleBoxLeft) {
            return (pos.0 + 1, pos.1);
        }
        if matches!(self, Cell::DoubleBoxRight) {
            return (pos.0 - 1, pos.1);
        }
        panic!("no counterpart");
    }
}

#[derive(Debug)]
enum MoveInst {
    Up,
    Left,
    Down,
    Right,
}

impl MoveInst {
    fn dir(&self) -> (i32, i32) {
        match self {
            MoveInst::Up => (0, -1),
            MoveInst::Left => (-1, 0),
            MoveInst::Down => (0, 1),
            MoveInst::Right => (1, 0),
        }
    }
}

#[allow(clippy::ptr_arg)]
fn find_guard(map: &Vec<Vec<Cell>>) -> (i32, i32) {
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == Cell::Robot {
                return (x as i32, y as i32);
            }
        }
    }
    panic!("no guard");
}

fn part1(mut map: Vec<Vec<Cell>>, rules: &Vec<MoveInst>) -> Vec<Vec<Cell>> {
    for rule in rules {
        let mut new_map = map.clone();
        let guard = find_guard(&map);
        let dir = rule.dir();

        let mut new_pos = (guard.0 + dir.0, guard.1 + dir.1);

        new_map[new_pos.1 as usize][new_pos.0 as usize] = Cell::Robot;
        new_map[guard.1 as usize][guard.0 as usize] = Cell::None;

        while map[new_pos.1 as usize][new_pos.0 as usize] == Cell::Box {
            let new_box_pos = (new_pos.0 + dir.0, new_pos.1 + dir.1);

            new_map[new_box_pos.1 as usize][new_box_pos.0 as usize] = Cell::Box;
            new_pos = new_box_pos;
        }

        if map[new_pos.1 as usize][new_pos.0 as usize] != Cell::Wall {
            map = new_map;
        }
    }

    map
}

fn part2(map: Vec<Vec<Cell>>, rules: &Vec<MoveInst>) -> Vec<Vec<Cell>> {
    let mut map = map
        .iter()
        .map(|line| {
            line.iter()
                .flat_map(|c| match c {
                    Cell::Box => vec![Cell::DoubleBoxLeft, Cell::DoubleBoxRight],
                    Cell::Robot => vec![Cell::Robot, Cell::None],
                    Cell::Wall => vec![Cell::Wall, Cell::Wall],
                    Cell::None => vec![Cell::None, Cell::None],
                    _ => panic!("input error"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for rule in rules {
        let mut new_map = map.clone();
        let guard = find_guard(&map);
        let dir = rule.dir();

        let new_pos = (guard.0 + dir.0, guard.1 + dir.1);

        if dir.1 == 0 {
            let mut new_pos_temp = new_pos;
            let mut was_wall = false;
            while map[new_pos_temp.1 as usize][new_pos_temp.0 as usize].is_box() {
                was_wall = true;
                let new_box_pos = (new_pos_temp.0 + dir.0, new_pos_temp.1 + dir.1);

                new_pos_temp = new_box_pos;
            }
            if map[new_pos_temp.1 as usize][new_pos_temp.0 as usize] == Cell::Wall {
                continue;
            }
            if was_wall {
                while new_pos_temp != new_pos {
                    let prev_pos = (new_pos_temp.0 - dir.0, new_pos_temp.1 - dir.1);

                    new_map[new_pos_temp.1 as usize][new_pos_temp.0 as usize] =
                        map[prev_pos.1 as usize][prev_pos.0 as usize].clone();
                    new_pos_temp = prev_pos;
                }
            }
        } else {
            fn move_double_box(
                map: &Vec<Vec<Cell>>,
                boxes: &mut Vec<(i32, i32)>,
                move_goal: (i32, i32),
                dir: (i32, i32),
            ) -> bool {
                let cell = map[move_goal.1 as usize][move_goal.0 as usize].clone();
                if cell.is_box() && !boxes.contains(&move_goal) {
                    let counterpart = cell.find_counterpart(move_goal);
                    boxes.push(move_goal);
                    return move_double_box(map, boxes, (move_goal.0, move_goal.1 + dir.1), dir)
                        && move_double_box(map, boxes, counterpart, dir);
                } else if matches!(cell, Cell::Wall) {
                    return false;
                }

                true
            }

            if map[new_pos.1 as usize][new_pos.0 as usize].is_box() {
                let mut boxes = Vec::new();
                if !move_double_box(&map, &mut boxes, new_pos, dir) {
                    continue;
                }
                if !boxes.is_empty() {
                    boxes.sort_by_key(|(_, y)| *y * -dir.1);
                    for box_pos in boxes {
                        new_map[(box_pos.1 + dir.1) as usize][box_pos.0 as usize] =
                            map[box_pos.1 as usize][box_pos.0 as usize].clone();
                        new_map[box_pos.1 as usize][box_pos.0 as usize] = Cell::None;
                    }
                }
            }
        }

        new_map[new_pos.1 as usize][new_pos.0 as usize] = Cell::Robot;
        new_map[guard.1 as usize][guard.0 as usize] = Cell::None;

        if map[new_pos.1 as usize][new_pos.0 as usize] != Cell::Wall {
            map = new_map;
        }
    }

    map
}

#[allow(clippy::ptr_arg)]
fn sum_of_gps(map: &Vec<Vec<Cell>>) -> u64 {
    map.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, c)| match *c {
                    Cell::Box | Cell::DoubleBoxLeft => 100u64 * y as u64 + x as u64,
                    _ => 0,
                })
                .sum::<u64>()
        })
        .sum::<u64>()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let (map, rules) = input.split_once("\n\n").unwrap();

    let map = map
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Cell::Box,
                    '@' => Cell::Robot,
                    '#' => Cell::Wall,
                    '.' => Cell::None,
                    _ => panic!("input error"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let rules = rules
        .chars()
        .filter_map(|c| match c {
            '^' => Some(MoveInst::Up),
            'v' => Some(MoveInst::Down),
            '<' => Some(MoveInst::Left),
            '>' => Some(MoveInst::Right),
            '\n' => None,
            _ => panic!("rules have errors"),
        })
        .collect::<Vec<_>>();

    println!("Part 1: {}", sum_of_gps(&part1(map.clone(), &rules)));
    println!("Part 2: {}", sum_of_gps(&part2(map.clone(), &rules)));
}
