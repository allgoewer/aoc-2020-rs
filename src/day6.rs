use super::{Day as DayTrait, Input};

pub struct Day;

fn count_anyone(s: &str) -> usize {
    let mut counter: usize = 0;

    for c in s.bytes() {
        if c >= b'a' && c <= b'z' {
            let pos = c - b'a';
            counter |= 1<<pos;
        }
    }

    counter.count_ones() as usize
}

fn count_everyone(s: &str) -> usize {
    let mut counter = [0usize; 26];

    for line in s.lines() {
        for c in line.bytes() {
            counter[(c - b'a') as usize] += 1;
        }
    }

    let n_lines = s.lines().count();

    counter.iter().filter(|count| **count == n_lines).count()
}

fn groups<'a>(input: &Input<'a>) -> impl Iterator<Item = &'a str> {
    input.0.split("\n\n")
}

impl DayTrait for Day {
    fn part1(&self, input: &Input) -> String {
        let anyones: usize = groups(input).map(count_anyone).sum();
        anyones.to_string()
    }

    fn part2(&self, input: &Input) -> String {
        let everyones: usize = groups(input).map(count_everyone).sum();
        everyones.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(count_anyone("abc"), 3);
        assert_eq!(count_anyone("a\nb\nc"), 3);
        assert_eq!(count_anyone("ab\nac"), 3);
        assert_eq!(count_anyone("a\na\na\na"), 1);
        assert_eq!(count_anyone("b"), 1);
    }

    #[test]
    fn samples_part2() {
        assert_eq!(count_everyone("abc"), 3);
        assert_eq!(count_everyone("a\nb\nc"), 0);
        assert_eq!(count_everyone("ab\nac"), 1);
        assert_eq!(count_everyone("a\na\na\na"), 1);
        assert_eq!(count_everyone("b"), 1);
    }
}
