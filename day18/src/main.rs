use std::io::{self, Read};

use pathfinding::{directed::astar, matrix::directions::DIRECTIONS_4};

#[derive(PartialEq, Eq, Clone, Copy)]
enum GridCell {
    None,
    Corrupted,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Real input:
    let width = 70 + 1;
    let height = 70 + 1;
    let steps = 1024;
    // Example:
    // let width = 6 + 1;
    // let height = 6 + 1;
    // let steps = 12;

    let mut grid = Vec::new();

    for _ in 0..height {
        let mut line = Vec::new();
        for _ in 0..width {
            line.push(GridCell::None);
        }
        grid.push(line);
    }

    let corruptions = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(",").unwrap();
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>();

    corruptions
        .clone()
        .into_iter()
        .take(steps)
        .for_each(|(x, y)| {
            grid[y as usize][x as usize] = GridCell::Corrupted;
        });

    let start = (0, 0);
    let end = (width - 1, height - 1);

    fn find_path(
        start: &(isize, isize),
        end: &(isize, isize),
        #[allow(clippy::ptr_arg)] grid: &Vec<Vec<GridCell>>,
    ) -> Option<(Vec<(isize, isize)>, isize)> {
        astar::astar(
            start,
            |&pos| {
                let mut vec = Vec::new();

                for dir in DIRECTIONS_4 {
                    let new_x = pos.0 + dir.0;
                    let new_y = pos.1 + dir.1;

                    if new_y < 0 || new_y as usize >= grid.len() {
                        continue;
                    }

                    if new_x < 0 || new_x as usize >= grid[new_y as usize].len() {
                        continue;
                    }

                    if grid[new_y as usize][new_x as usize] == GridCell::None {
                        vec.push(((new_x, new_y), 1));
                    }
                }

                vec
            },
            |&(x, y)| ((end.0 - x).abs() + (end.1 - y).abs()) / 3,
            |pos| pos == end,
        )
    }

    let path = find_path(&start, &end, &grid).unwrap();

    println!("Part 1: {}", path.1);

    // I recommend running p2 in release mode, there might a smarter solution but this works fine in reasonable time.
    for (i, (x, y)) in corruptions.into_iter().enumerate() {
        if i <= steps {
            continue;
        }
        grid[y as usize][x as usize] = GridCell::Corrupted;
        let path = find_path(&start, &end, &grid);
        if path.is_none() {
            println!("Part 2: {},{}", x, y);
            break;
        }
    }
}
