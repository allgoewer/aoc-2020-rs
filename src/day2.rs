use super::{Day as DayTrait, Input};

#[derive(Clone, Debug)]
struct Parsed<'s> {
    policy: (usize, usize),
    character: char,
    password: &'s str,
}

fn parse(line: &str) -> Option<Parsed<'_>> {
    let line: Vec<_> = line.splitn(3, ' ').collect();

    match &line[..] {
        [policy, character, password] => Some(Parsed {
            // If the following unwraps panic, something is wrong with the input dataset
            policy: {
                let policy: Vec<_> = policy.splitn(2, '-').collect();
                let a = policy[0].parse().unwrap();
                let b = policy[1].parse().unwrap();
                (a, b)
            },
            character: character.chars().next().unwrap(),
            password,
        }),
        _ => None,
    }
}

fn validate1(parsed: &Parsed<'_>) -> bool {
    let count = parsed
        .password
        .chars()
        .filter(|c| *c == parsed.character)
        .count();

    parsed.policy.0 <= count && count <= parsed.policy.1
}

fn validate2(parsed: &Parsed<'_>) -> bool {
    let c = parsed.character;
    if let (Some(a), Some(b)) = (
        parsed.password.chars().nth(parsed.policy.0 - 1),
        parsed.password.chars().nth(parsed.policy.1 - 1),
    ) {
        return a == c && b != c || a != c && b == c;
    }

    false
}

fn validate_with<P>(input: &Input, predicate: P) -> String
where
    P: FnMut(&Parsed<'_>) -> bool,
{
    input
        .0
        .lines()
        .filter_map(parse)
        .filter(predicate)
        .count()
        .to_string()
}

pub struct Day;

impl DayTrait for Day {
    fn part1(&self, input: &Input) -> String {
        validate_with(input, validate1)
    }

    fn part2(&self, input: &Input) -> String {
        validate_with(input, validate2)
    }
}
