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

fn char_at(lines: &Vec<&str>, x: usize, y: usize) -> char {
    return lines.iter().nth(y).unwrap().chars().nth(x).unwrap();
}

fn part1(lines: &Vec<&str>, x: usize, y: usize) -> u64 {
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

            let char = char_at(lines, y_pos as usize, x_pos as usize);

            if char != c {
                matches = false;
                break;
            }
        }
        if matches {
            sum += 1;
        }
    }
    return sum;
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

fn char_at_shape(lines: &[[Option<char>; 3]; 3], x: usize, y: usize) -> &Option<char> {
    return lines.iter().nth(y).unwrap().iter().nth(x).unwrap();
}

const SHAPE_WIDTH: usize = 3;
const SHAPE_HEIGHT: usize = 3;

fn part2(lines: &Vec<&str>, x: usize, y: usize) -> bool {
    let grid_x = lines.first().unwrap().len() as i32;
    let grid_y = lines.len() as i32;

    if x > (grid_x - SHAPE_WIDTH as i32) as usize || y > (grid_y - SHAPE_HEIGHT as i32) as usize {
        return false;
    }

    for shape in XMAS_SHAPE {
        let mut matches = true;
        for shape_x in 0..SHAPE_WIDTH {
            for shape_y in 0..SHAPE_HEIGHT {
                let shape_char = char_at_shape(&shape, shape_x, shape_y);
                if let Some(c) = shape_char {
                    let char = char_at(&lines, x as usize + shape_x, y as usize + shape_y);
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
    return false;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let lines = input.lines().collect::<Vec<&str>>();

    let mut part1_count: u64 = 0;
    let mut part2_count: u64 = 0;

    lines.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, _)| {
            part1_count += part1(&lines, x, y);
            if part2(&lines, x, y) {
                part2_count += 1;
            }
        });
    });

    println!("Part 1: {}\nPart 2: {}", part1_count, part2_count);
}
