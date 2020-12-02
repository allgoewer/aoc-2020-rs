use super::{Day as DayTrait, Input};
use itertools::Itertools;

fn parse(line: &str) -> Option<u32> {
    line.parse().ok()
}

fn with_combination_size(input: &Input, n: usize) -> String {
    let combinations = input.0.lines().filter_map(parse).combinations(n);

    for c in combinations {
        // this call to 'fold' is equivalent to a sum over the elements of 'c'
        if itertools::fold(&c, 0, |a, b| a + b) == 2020 {
            let product: u32 = c.into_iter().product();
            return product.to_string();
        }
    }

    String::new()
}

pub struct Day;

impl DayTrait for Day {
    fn part1(&self, input: &Input) -> String {
        with_combination_size(input, 2)
    }

    fn part2(&self, input: &Input) -> String {
        with_combination_size(input, 3)
    }
}
