use super::{Day as DayTrait, Input};
use itertools::Itertools;
use std::ops::{Add, AddAssign};

fn first_invalid<T>(values: &[T], window_size: usize) -> Option<T>
where
    T: Copy + Default + PartialEq + Add<Output = T>,
{
    'outer: for window in values.windows(window_size + 1) {
        let next = window[window_size];

        for c in window[0..window_size].iter().combinations(2) {
            if itertools::fold(&c, T::default(), |a, b| a + **b) == next {
                continue 'outer;
            }
        }

        return Some(next);
    }

    None
}

/// Find the first sub-range of values which's sum equals target
fn find_range<T>(values: &[T], target: T) -> Option<&[T]>
where
    T: Copy + Default + PartialEq + PartialOrd + AddAssign,
{
    for a in 0..values.len() {
        for b in a..values.len() {
            let mut sum = T::default();

            for c in a..b {
                sum += values[c];
                if sum == target {
                    return Some(&values[a..c]);
                } else if sum > target {
                    break;
                }
            }
        }
    }

    None
}

/// Calculate the sum of the smallest and the biggest value in values
fn min_max_sum<T>(values: &[T]) -> Option<T>
where
    T: Copy + Ord + Add<Output = T>,
{
    let min = values.iter().min();
    let max = values.iter().max();

    match (min, max) {
        (Some(min), Some(max)) => Some(*min + *max),
        _ => None,
    }
}

pub struct Day;

impl DayTrait for Day {
    fn part1(&self, input: &Input) -> String {
        let values: Vec<_> = input
            .0
            .lines()
            .filter_map(|line| line.parse::<u64>().ok())
            .collect();

        first_invalid(&values, 25).unwrap().to_string()
    }

    fn part2(&self, input: &Input) -> String {
        let values: Vec<_> = input
            .0
            .lines()
            .filter_map(|line| line.parse::<u64>().ok())
            .collect();

        let invalid = first_invalid(&values, 25).unwrap();
        let range = find_range(&values, invalid).unwrap();

        min_max_sum(range).unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &[u64] = &[
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    #[test]
    fn example_part1() {
        assert_eq!(first_invalid(SAMPLE1, 5), Some(127));
    }

    #[test]
    fn example_part2() {
        let invalid = first_invalid(SAMPLE1, 5).unwrap();
        let range = find_range(SAMPLE1, invalid).unwrap();

        assert_eq!(min_max_sum(range), Some(62));
    }
}
