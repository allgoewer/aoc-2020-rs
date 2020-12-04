use super::{Day as DayTrait, Input};
use std::collections::HashMap;

const MAX_ENTRIES: usize = 8;

fn parse<'a>(input: &'a Input) -> impl Iterator<Item = HashMap<&'a str, &'a str>> {
    input.0.split("\n\n").map(|record| {
        let mut set = HashMap::with_capacity(MAX_ENTRIES);
        for entry in record.split_ascii_whitespace() {
            let mut components = entry.split(':');
            let kind = components.next().unwrap(); // if unwrap panics, input is invalid
            let value = components.next().unwrap();
            set.insert(kind, value);
        }

        set
    })
}

fn is_valid1(record: &HashMap<&str, &str>) -> bool {
    if record.len() == MAX_ENTRIES {
        true
    } else {
        (record.len() == MAX_ENTRIES - 1) && !record.contains_key("cid")
    }
}

fn is_valid2(record: &HashMap<&str, &str>) -> bool {
    let byr = record.get("byr").unwrap().parse::<i32>().unwrap();
    if byr < 1920 || byr > 2002 {
        return false;
    }

    let iyr = record.get("iyr").unwrap().parse::<i32>().unwrap();
    if iyr < 2010 || iyr > 2020 {
        return false;
    }

    let eyr = record.get("eyr").unwrap().parse::<i32>().unwrap();
    if eyr < 2020 || eyr > 2030 {
        return false;
    }

    let hgt = record.get("hgt").unwrap();
    if let Some(hgt) = hgt.strip_suffix("cm") {
        let hgt: i32 = hgt.parse().unwrap();
        if hgt < 150 || hgt > 193 {
            return false;
        }
    } else if let Some(hgt) = hgt.strip_suffix("in") {
        let hgt: i32 = hgt.parse().unwrap();
        if hgt < 59 || hgt > 76 {
            return false;
        }
    } else {
        return false;
    }

    let hcl = record.get("hcl").unwrap();
    if let Some(hcl) = hcl.strip_prefix("#") {
        if hcl
            .chars()
            .filter(|c| (*c >= '0' && *c <= '9') || (*c >= 'a' && *c <= 'f'))
            .count() != 6
        {
            return false;
        }
    } else {
        return false;
    }

    let ecl = record.get("ecl").unwrap();
    if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .filter(|color| *color == ecl)
        .count() == 0
    {
        return false;
    }

    let pid = record.get("pid").unwrap();
    if pid.len() != 9 || pid.parse::<i32>().is_err() {
        return false;
    }

    true
}

pub struct Day;

impl DayTrait for Day {
    fn part1(&self, input: &Input) -> String {
        let count: usize = parse(input).filter(is_valid1).count();
        count.to_string()
    }

    fn part2(&self, input: &Input) -> String {
        let count: usize = parse(input).filter(|r| is_valid1(r) && is_valid2(r)).count();
        count.to_string()
    }
}
