use super::{Day as DayTrait, Input};
use regex::Regex;
use std::collections::HashMap;

const REGEX_CONTAINER: &str = r"^([[:alpha:][:space:]]+?) bags contain";
const REGEX_CONTAINEES: &str = r",? ([[:digit:]]+?) ([[:alpha:][:space:]]+?) bags?";

#[derive(Clone, Debug, PartialEq, Eq)]
struct Rule<'a> {
    color: &'a str,
    others: HashMap<&'a str, usize>,
}

impl<'a> Rule<'a> {
    fn try_from_str(rule: &'a str, regexes: &(Regex, Regex)) -> Result<Self, &'static str> {
        let color = if let Some(container) = regexes
            .0 // regex for the container color
            .captures_iter(rule)
            .filter_map(|cap| cap.get(1))
            .next()
        // there can be only one match
        {
            &rule[container.range()]
        } else {
            return Err("Parsing container failed");
        };

        let others: HashMap<_, _> = regexes
            .1 // regex for containees
            .captures_iter(rule)
            .filter_map(|cap| match (cap.get(1), cap.get(2)) {
                // get the first (quantity) and second (color) match
                (Some(a), Some(b)) => Some((&rule[a.range()], &rule[b.range()])),
                _ => None,
            })
            .filter_map(|(quantity, color)| Some((color, quantity.parse::<usize>().ok()?)))
            .collect();

        Ok(Rule { color, others })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ReducedRule<'a> {
    color: &'a str,
    count: usize,
}

fn reduce<'a, I: Iterator<Item = Rule<'a>>>(rules: I, color: &str) -> usize {
    let mut bags: Vec<_> = rules
        .filter_map(|mut bag| {
            if let Some(count) = bag.others.remove(color) {
                Some((bag, count))
            } else {
                Some((bag, 0))
            }
        })
        .collect();

    let mut reduced: HashMap<&str, usize> = HashMap::new();

    while bags.len() > 0 {
        for (Rule { color: _, others }, count) in bags.iter_mut() {
            for (red_color, red_count) in reduced.iter() {
                if let Some(c) = others.remove(red_color) {
                    *count += c * red_count;
                }
            }
        }

        bags.retain(|(Rule { color, others }, count)| {
            if others.len() == 0 {
                reduced.insert(color, *count);
                false
            } else {
                true
            }
        });
    }

    reduced.iter().filter(|(_color, count)| **count > 0).count()
}

fn count_bags<'a, I: Iterator<Item = Rule<'a>>>(rules: I, color: &str) -> usize {
    let mut bags: Vec<_> = rules.filter_map(|bag| Some((bag, 1))).collect();

    let mut reduced: HashMap<&str, usize> = HashMap::new();

    while bags.len() > 0 {
        for (Rule { color: _, others }, count) in bags.iter_mut() {
            for (red_color, red_count) in reduced.iter() {
                if let Some(c) = others.remove(red_color) {
                    *count += c * red_count;
                }
            }
        }

        bags.retain(|(Rule { color, others }, count)| {
            if others.len() == 0 {
                reduced.insert(color, *count);
                false
            } else {
                true
            }
        });
    }

    reduced.get(color).unwrap() - 1
}

pub struct Day;

impl DayTrait for Day {
    fn part1(&self, input: &Input) -> String {
        let re = (
            Regex::new(REGEX_CONTAINER).unwrap(),
            Regex::new(REGEX_CONTAINEES).unwrap(),
        );

        let parsed_rules = input
            .0
            .lines()
            .filter_map(|rule| Rule::try_from_str(rule, &re).ok());

        reduce(parsed_rules, "shiny gold").to_string()
    }

    fn part2(&self, input: &Input) -> String {
        let re = (
            Regex::new(REGEX_CONTAINER).unwrap(),
            Regex::new(REGEX_CONTAINEES).unwrap(),
        );

        let parsed_rules = input
            .0
            .lines()
            .filter_map(|rule| Rule::try_from_str(rule, &re).ok());

        count_bags(parsed_rules, "shiny gold").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RULES: [&str; 9] = [
        "light red bags contain 1 bright white bag, 2 muted yellow bags.",
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
        "bright white bags contain 1 shiny gold bag.",
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
        "faded blue bags contain no other bags.",
        "dotted black bags contain no other bags.",
    ];

    #[test]
    fn reduce_part1() {
        let re = (
            Regex::new(REGEX_CONTAINER).unwrap(),
            Regex::new(REGEX_CONTAINEES).unwrap(),
        );

        let parsed_rules = RULES
            .iter()
            .filter_map(|rule| Rule::try_from_str(rule, &re).ok());

        assert_eq!(reduce(parsed_rules, "shiny gold"), 4);
    }

    #[test]
    fn count_part2() {
        let re = (
            Regex::new(REGEX_CONTAINER).unwrap(),
            Regex::new(REGEX_CONTAINEES).unwrap(),
        );

        let parsed_rules = RULES
            .iter()
            .filter_map(|rule| Rule::try_from_str(rule, &re).ok());

        assert_eq!(count_bags(parsed_rules, "shiny gold"), 32);
    }

    #[test]
    fn rule_samples_part1() {
        let re = (
            Regex::new(REGEX_CONTAINER).unwrap(),
            Regex::new(REGEX_CONTAINEES).unwrap(),
        );

        let parsed_rules = [
            Rule {
                color: "light red",
                others: vec![("bright white", 1), ("muted yellow", 2)]
                    .into_iter()
                    .collect(),
            },
            Rule {
                color: "dark orange",
                others: vec![("bright white", 3), ("muted yellow", 4)]
                    .into_iter()
                    .collect(),
            },
            Rule {
                color: "bright white",
                others: vec![("shiny gold", 1)].into_iter().collect(),
            },
            Rule {
                color: "muted yellow",
                others: vec![("shiny gold", 2), ("faded blue", 9)]
                    .into_iter()
                    .collect(),
            },
            Rule {
                color: "shiny gold",
                others: vec![("dark olive", 1), ("vibrant plum", 2)]
                    .into_iter()
                    .collect(),
            },
            Rule {
                color: "dark olive",
                others: vec![("faded blue", 3), ("dotted black", 4)]
                    .into_iter()
                    .collect(),
            },
            Rule {
                color: "vibrant plum",
                others: vec![("faded blue", 5), ("dotted black", 6)]
                    .into_iter()
                    .collect(),
            },
            Rule {
                color: "faded blue",
                others: HashMap::new(),
            },
            Rule {
                color: "dotted black",
                others: HashMap::new(),
            },
        ];

        for (rule, parsed) in RULES.iter().zip(parsed_rules.iter()) {
            assert_eq!(Rule::try_from_str(rule, &re).unwrap(), *parsed);
        }
    }

    #[test]
    fn samples_part2() {}
}
