use std::io::{self, Read};

struct GridCell {
    c: char,
    taken: bool,
}

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

fn accumulate_plants(
    plant_type: char,
    x: usize,
    y: usize,
    grid: &mut Vec<Vec<GridCell>>,
    plants: &mut Vec<(usize, usize)>,
) {
    if grid[y][x].c != plant_type || grid[y][x].taken {
        return;
    }
    grid[y][x].taken = true;
    plants.push((x, y));

    for dir in DIRECTIONS {
        let new_x = x as i32 + dir.0;
        let new_y = y as i32 + dir.1;

        if new_y >= 0
            && new_y < grid.len() as i32
            && new_x >= 0
            && new_x < grid[new_y as usize].len() as i32
        {
            accumulate_plants(plant_type, new_x as usize, new_y as usize, grid, plants);
        }
    }
}

fn calc_perimeter(plants: &Vec<(usize, usize)>) -> usize {
    let mut perimeter = 0;
    for plant in plants {
        for dir in DIRECTIONS {
            let new_x = plant.0 as i32 + dir.0;
            let new_y = plant.1 as i32 + dir.1;

            if !plants.contains(&(new_x as usize, new_y as usize)) {
                perimeter += 1;
            }
        }
    }
    perimeter
}

// this is ugly:
fn calc_sides(plants: &[(usize, usize)]) -> usize {
    let mut sides = 0;

    let min_x = plants.iter().min_by_key(|p| p.0).unwrap().0 as isize;
    let min_y = plants.iter().min_by_key(|p| p.1).unwrap().1 as isize;
    let max_x = plants.iter().max_by_key(|p| p.0).unwrap().0 as isize;
    let max_y = plants.iter().max_by_key(|p| p.1).unwrap().1 as isize;

    for y in min_y - 1..=max_y {
        let mut last_has_side_down = false;
        let mut last_has_side_up = false;
        for x in min_x..=max_x {
            let next_y = y + 1;

            let down = plants.contains(&(x as usize, y as usize))
                && !plants.contains(&(x as usize, next_y as usize));
            let up = !plants.contains(&(x as usize, y as usize))
                && plants.contains(&(x as usize, next_y as usize));

            if !last_has_side_down && down {
                sides += 1;
            }
            if !last_has_side_up && up {
                sides += 1;
            }
            last_has_side_down = down;
            last_has_side_up = up;
        }
    }

    for x in min_x - 1..=max_x {
        let mut last_has_side_right = false;
        let mut last_has_side_left = false;
        for y in min_y..=max_y {
            let next_x = x + 1;

            let right = plants.contains(&(x as usize, y as usize))
                && !plants.contains(&(next_x as usize, y as usize));
            let left = !plants.contains(&(x as usize, y as usize))
                && plants.contains(&(next_x as usize, y as usize));

            if !last_has_side_right && right {
                sides += 1;
            }
            if !last_has_side_left && left {
                sides += 1;
            }
            last_has_side_right = right;
            last_has_side_left = left;
        }
    }

    sides
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| GridCell { c, taken: false })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut part1 = 0;
    let mut part2 = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x].taken {
                continue;
            }

            let mut plants = Vec::new();
            accumulate_plants(grid[y][x].c, x, y, &mut grid, &mut plants);

            let area = plants.len();
            let perimeter = calc_perimeter(&plants);
            part1 += area * perimeter;
            let sides = calc_sides(&plants);
            part2 += area * sides;
        }
    }

    println!("Part 1: {part1}\nPart 2: {part2}");
}
