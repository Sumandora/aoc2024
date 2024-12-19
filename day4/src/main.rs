use std::io::{self, Read};

const WORD: &str = "XMAS";
const DIRECTIONS: [[i32; 2]; 8] = [
    [1, 0],
    [-1, 0],
    [0, 1],
    [0, -1],
    [1, 1],
    [-1, -1],
    [-1, 1],
    [1, -1],
];

#[allow(clippy::ptr_arg)]
fn part1(lines: &Vec<Vec<char>>, x: usize, y: usize) -> u64 {
    let grid_x = lines.first().unwrap().len() as i32;
    let grid_y = lines.len() as i32;

    let mut sum = 0u64;

    for direction in DIRECTIONS {
        let mut matches = true;
        for (i, c) in WORD.char_indices() {
            let i = i as i32;
            let x_pos = x as i32 + direction[0] * i;
            let y_pos = y as i32 + direction[1] * i;
            if !(0..grid_x).contains(&x_pos) {
                matches = false;
                break;
            }

            if !(0..grid_y).contains(&y_pos) {
                matches = false;
                break;
            }

            let char = lines[y_pos as usize][x_pos as usize];

            if char != c {
                matches = false;
                break;
            }
        }
        if matches {
            sum += 1;
        }
    }
    sum
}

// I guess you could rotate it programmatically, but its only 3x3 and I value my time (to some extend that may be)
const XMAS_SHAPE: [[[Option<char>; 3]; 3]; 4] = [
    [
        [Some('M'), None, Some('S')],
        [None, Some('A'), None],
        [Some('M'), None, Some('S')],
    ],
    [
        [Some('S'), None, Some('M')],
        [None, Some('A'), None],
        [Some('S'), None, Some('M')],
    ],
    [
        [Some('S'), None, Some('S')],
        [None, Some('A'), None],
        [Some('M'), None, Some('M')],
    ],
    [
        [Some('M'), None, Some('M')],
        [None, Some('A'), None],
        [Some('S'), None, Some('S')],
    ],
];

const SHAPE_WIDTH: usize = 3;
const SHAPE_HEIGHT: usize = 3;

#[allow(clippy::ptr_arg)]
fn part2(lines: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let grid_width = lines.first().unwrap().len() as i32;
    let grid_height = lines.len() as i32;

    if x > (grid_width - SHAPE_WIDTH as i32) as usize
        || y > (grid_height - SHAPE_HEIGHT as i32) as usize
    {
        return false;
    }

    for shape in XMAS_SHAPE {
        let mut matches = true;
        for (shape_y, shape_line) in shape.iter().enumerate() {
            for (shape_x, shape_char) in shape_line.iter().enumerate() {
                if let Some(c) = shape_char {
                    let char = lines[x + shape_x][y + shape_y];
                    if char != *c {
                        matches = false;
                        break;
                    }
                }
            }
        }
        if matches {
            // If one shape matches, no other shape can match the same position
            return true;
        }
    }
    false
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut part1_count: u64 = 0;
    let mut part2_count: u64 = 0;

    lines.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, _)| {
            part1_count += part1(&lines, x, y);
            if part2(&lines, x, y) {
                part2_count += 1;
            }
        });
    });

    println!("Part 1: {}\nPart 2: {}", part1_count, part2_count);
}
