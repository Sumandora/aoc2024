use std::io::{self, Read};

fn check<I>(mut iter: I, test_val: i64, curr: i64, concat_operator: bool) -> bool
where
    I: Iterator<Item = i64>,
{
    let num = iter.next();
    if num.is_none() {
        // end reached
        return curr == test_val;
    }
    let num = num.unwrap();

    let vec = iter.collect::<Vec<i64>>();

    // This is a bit slow, but rusts release mode gets both parts done in 3 seconds

    return check(
        vec.clone().into_iter(),
        test_val,
        curr * num,
        concat_operator,
    ) || check(
        vec.clone().into_iter(),
        test_val,
        curr + num,
        concat_operator,
    ) || (concat_operator
        && check(
            vec.into_iter(),
            test_val,
            (curr.to_string() + num.to_string().as_str()) // if it ain't broke, dont fix it
                .parse::<i64>()
                .unwrap(),
            concat_operator,
        ));
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let equations = input
        .lines()
        .map(|line| {
            let (test_val, nums) = line.split_once(": ").unwrap();

            (
                test_val.parse::<i64>().unwrap(),
                nums.split(" ")
                    .map(|str| str.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let part1 = equations
        .iter()
        .filter(|(test_val, nums)| check(nums.clone().into_iter(), *test_val, 0, false))
        .map(|(test_val, _)| test_val)
        .sum::<i64>();
    println!("Part 1: {}", part1);

    let part2 = equations
        .iter()
        .filter(|(test_val, nums)| check(nums.clone().into_iter(), *test_val, 0, true))
        .map(|(test_val, _)| test_val)
        .sum::<i64>();
    println!("Part 2: {}", part2);
}
