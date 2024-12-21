use std::{
    collections::HashMap,
    io::{self, Read},
    iter::{self, zip},
    sync::LazyLock,
};

use itertools::Itertools;
use memoize::memoize;
use pathfinding::prelude::astar_bag;

#[allow(clippy::declare_interior_mutable_const, clippy::type_complexity)]
static FIRST_BUTTON_POSITIONS: LazyLock<HashMap<char, (i32, i32)>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    for (i, x) in ('7'..='9').enumerate() {
        map.insert(x, (i as i32, 0));
    }
    for (i, x) in ('4'..='6').enumerate() {
        map.insert(x, (i as i32, 1));
    }
    for (i, x) in ('1'..='3').enumerate() {
        map.insert(x, (i as i32, 2));
    }
    map.insert('0', (1, 3));
    map.insert('A', (2, 3));
    map
});

#[allow(clippy::declare_interior_mutable_const, clippy::type_complexity)]
static SECOND_BUTTON_POSITIONS: LazyLock<HashMap<char, (i32, i32)>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert('^', (1, 0));
    map.insert('A', (2, 0));

    map.insert('<', (0, 1));
    map.insert('v', (1, 1));
    map.insert('>', (2, 1));

    map
});

#[allow(clippy::declare_interior_mutable_const, clippy::type_complexity)]
static MOVEMENTS: LazyLock<HashMap<(i32, i32), char>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    map.insert((-1, 0), '<');
    map.insert((1, 0), '>');
    map.insert((0, 1), 'v');
    map.insert((0, -1), '^');

    map
});

fn all_movements(positions: &HashMap<char, (i32, i32)>) -> HashMap<(char, char), Vec<Vec<char>>> {
    let mut map = HashMap::new();
    for (from, from_pos) in positions {
        for (to, to_pos) in positions {
            let paths = astar_bag(
                from_pos,
                |pos| {
                    MOVEMENTS
                        .keys()
                        .map(|key| ((pos.0 + key.0, pos.1 + key.1), 1))
                        .filter(|&((x, y), _)| {
                            positions.values().any(|&(x2, y2)| x == x2 && y == y2)
                        })
                        .collect::<Vec<_>>()
                },
                |pos| (to_pos.0.abs_diff(pos.0) + to_pos.1.abs_diff(pos.1)) / 3,
                |pos| pos == to_pos,
            )
            .unwrap()
            .0
            .map(|path| {
                path.iter()
                    .tuple_windows()
                    .map(|(movement, movement2)| {
                        *MOVEMENTS
                            .get(&(movement2.0 - movement.0, movement2.1 - movement.1))
                            .unwrap()
                    })
                    .chain(iter::once('A'))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
            map.insert((*from, *to), paths);
        }
    }
    map
}

#[allow(clippy::type_complexity)]
static FIRST_STEP_MOVEMENTS: LazyLock<HashMap<(char, char), Vec<Vec<char>>>> =
    LazyLock::new(|| all_movements(&FIRST_BUTTON_POSITIONS));
#[allow(clippy::type_complexity)]
static SECOND_STEP_MOVEMENTS: LazyLock<HashMap<(char, char), Vec<Vec<char>>>> =
    LazyLock::new(|| all_movements(&SECOND_BUTTON_POSITIONS));

fn first_step(str: &str) -> Vec<String> {
    iter::once('A')
        .chain(str.chars())
        .tuple_windows()
        .map(|(a, b)| FIRST_STEP_MOVEMENTS.get(&(a, b)).unwrap().clone())
        .map(|vec| {
            vec.iter()
                .map(|vec| vec.iter().collect::<String>())
                .collect::<Vec<_>>()
        })
        .multi_cartesian_product()
        .map(|strings| strings.join(""))
        .collect::<Vec<_>>()
}

#[memoize]
fn second_step(solution: String, steps: usize) -> u64 {
    if steps == 1 {
        return zip(iter::once('A').chain(solution.chars()), solution.chars())
            .map(|(a, b)| {
                SECOND_STEP_MOVEMENTS
                    .get(&(a, b))
                    .unwrap()
                    .first()
                    .unwrap()
                    .len() as u64
            })
            .sum::<u64>();
    }

    zip(iter::once('A').chain(solution.chars()), solution.chars())
        .map(|(from, to)| {
            SECOND_STEP_MOVEMENTS
                .get(&(from, to))
                .unwrap()
                .iter()
                .map(|str| second_step(str.iter().collect::<String>(), steps - 1))
                .min()
                .unwrap()
        })
        .sum::<u64>()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let solutions = input
        .lines()
        .map(|str| (str, first_step(str)))
        .map(|(orig, str)| {
            (
                orig,
                str.iter()
                    .map(|sol| second_step((*sol).clone(), 2))
                    .min()
                    .unwrap(),
                str.iter()
                    .map(|sol| second_step((*sol).clone(), 25))
                    .min()
                    .unwrap(),
            )
        })
        .map(|(orig, len1, len2)| {
            let c = orig[0..orig.len() - 1].parse::<u64>().unwrap();
            (len1 * c, len2 * c)
        })
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    println!("Part 1: {}\nPart 2: {}", solutions.0, solutions.1);
}
