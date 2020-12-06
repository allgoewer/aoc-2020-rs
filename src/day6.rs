use super::{Day as DayTrait, Input};

pub struct Day;

fn count_anyone(s: &str) -> u32 {
    let mut counter: u32 = 0;

    for c in s.chars() {
        if c >= 'a' && c <= 'z' {
            let pos = c as u32 - 'a' as u32;
            counter |= 1<<pos;
        }
    }

    counter.count_ones()
}

fn count_everyone(s: &str) -> u32 {
    let mut counter = [0u32; 26];

    for line in s.lines() {
        for c in line.chars() {
            counter[c as usize - 'a' as usize] += 1;
        }
    }

    let n_lines = s.lines().count() as u32;

    counter.iter().filter(|count| **count == n_lines).count() as u32
}

fn groups<'a>(input: &Input<'a>) -> impl Iterator<Item = &'a str> {
    input.0.split("\n\n")
}

impl DayTrait for Day {
    fn part1(&self, input: &Input) -> String {
        let anyones: u32 = groups(input).map(count_anyone).sum();
        anyones.to_string()
    }

    fn part2(&self, input: &Input) -> String {
        let everyones: u32 = groups(input).map(count_everyone).sum();
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
