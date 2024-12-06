use std::io::{self, Read};

#[derive(Debug, Eq, PartialEq, Clone)]
enum GridType {
    Empty,
    Blocked,
    SteppedOn(Vec<Direction>),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn to_vec(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }

    fn rotate(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

struct Simulation {
    grid: Vec<Vec<GridType>>,
    guard_rotation: Direction,
    guard_pos: (i32, i32),
}

#[derive(Debug)]
enum SimulationError {
    StuckInLoop,
}

impl Simulation {
    fn simulate(mut self) -> Result<Vec<Vec<GridType>>, SimulationError> {
        let width = self.grid.iter().nth(0).unwrap().len();
        let height = self.grid.len();

        loop {
            let vec = self.guard_rotation.to_vec();
            let new_guard_pos = (self.guard_pos.0 + vec.0, self.guard_pos.1 + vec.1);

            if new_guard_pos.0 < 0
                || new_guard_pos.0 as usize >= width
                || new_guard_pos.1 < 0
                || new_guard_pos.1 as usize >= height
            {
                self.grid[self.guard_pos.1 as usize][self.guard_pos.0 as usize] =
                    GridType::SteppedOn(vec![self.guard_rotation]);
                break;
            }

            let grid_type = self.grid[new_guard_pos.1 as usize][new_guard_pos.0 as usize].clone();
            let curr_grid_type =
                self.grid[self.guard_pos.1 as usize][self.guard_pos.0 as usize].clone();

            match grid_type {
                GridType::Empty | GridType::SteppedOn(_) => {
                    match curr_grid_type {
                        GridType::SteppedOn(mut prev_directions) => {
                            if prev_directions.contains(&self.guard_rotation) {
                                return Err(SimulationError::StuckInLoop);
                            }
                            prev_directions.push(self.guard_rotation);
                            self.grid[self.guard_pos.1 as usize][self.guard_pos.0 as usize] =
                                GridType::SteppedOn(prev_directions);
                        }
                        _ => {
                            self.grid[self.guard_pos.1 as usize][self.guard_pos.0 as usize] =
                                GridType::SteppedOn(vec![self.guard_rotation]);
                        }
                    }
                    self.guard_pos = new_guard_pos;
                }
                GridType::Blocked => {
                    self.guard_rotation = self.guard_rotation.rotate();
                }
            }
        }

        return Ok(self.grid);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut guard_pos = (0i32, 0i32);
    let grid: Vec<Vec<GridType>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let line_vec = line
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '^' {
                        guard_pos.0 = x as i32;
                        guard_pos.1 = y as i32;
                    }
                    match c {
                        '.' => GridType::Empty,
                        '#' => GridType::Blocked,
                        '^' => GridType::Empty,
                        _ => panic!("Unknown grid type"),
                    }
                })
                .collect::<Vec<_>>();
            line_vec
        })
        .collect::<Vec<_>>();

    let simulation = Simulation {
        grid: grid.clone(),
        guard_rotation: Direction::Up,
        guard_pos,
    };

    let finished = simulation
        .simulate()
        .expect("Can't solve part 1, because there is a loop");

    let part1 = finished
        .iter()
        .map(|line| {
            line.iter()
                .filter(|grid_type| matches!(**grid_type, GridType::SteppedOn(_)))
                .count()
        })
        .sum::<usize>();
    println!("Part 1: {}", part1);

    let part2 = finished
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(x, pos)| {
                    if !matches!(pos, GridType::SteppedOn(_)) {
                        return false;
                    }
                    let grid_type = grid[y][*x].clone();
                    // This is neccessary to not override the guard itself
                    if grid_type != GridType::Empty {
                        return false;
                    }
                    let mut new_grid = grid.clone();
                    new_grid[y][*x] = GridType::Blocked;
                    let new_simulation = Simulation {
                        grid: new_grid,
                        guard_rotation: Direction::Up,
                        guard_pos,
                    };
                    return new_simulation.simulate().is_err();
                })
                .count()
        })
        .sum::<usize>();
    println!("Part 2: {}", part2);
}
