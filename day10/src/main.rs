use std::{
    collections::HashSet,
    io::{self, Read},
};

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

fn recursively_spiral_out(
    grid: &Vec<Vec<u8>>,
    x: usize,
    y: usize,
    curr_num: u8,
    seen: &mut HashSet<(usize, usize)>,
    respect_seen: bool,
) -> u64 {
    if grid[y][x] != curr_num {
        return 0;
    }
    if curr_num == 9 {
        seen.insert((x, y));
        return 1;
    }

    return DIRECTIONS
        .into_iter()
        .filter_map(|(dir_x, dir_y)| {
            let new_x = x as i32 + dir_x;
            let new_y = y as i32 + dir_y;

            if new_x < 0 || new_x >= grid[0].len() as i32 {
                return None;
            }
            if new_y < 0 || new_y >= grid.len() as i32 {
                return None;
            }

            if respect_seen {
                if seen.contains(&(new_x as usize, new_y as usize)) {
                    return None;
                }
            }

            Some(recursively_spiral_out(
                grid,
                new_x as usize,
                new_y as usize,
                curr_num + 1,
                seen,
                respect_seen,
            ))
        })
        .sum();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut grid = Vec::<Vec<u8>>::new();

    input.lines().for_each(|line| {
        let mut line_vec = Vec::new();
        line.chars().for_each(|c| {
            line_vec.push(c as u8 - '0' as u8);
        });
        grid.push(line_vec);
    });

    let (part1, part2) = grid
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, _)| {
                    let mut seen = HashSet::new();
                    // combining those is quite hard... anyways
                    let part1 = recursively_spiral_out(&grid, x, y, 0, &mut seen, true);
                    let part2 = recursively_spiral_out(&grid, x, y, 0, &mut seen, false);

                    (part1, part2)
                })
                .collect::<Vec<_>>()
        })
        .fold((0, 0), |acc, (part1, part2)| {
            return (acc.0 + part1, acc.1 + part2);
        });

    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
