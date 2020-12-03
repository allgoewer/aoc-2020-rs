use super::{Day as DayTrait, Input};
use std::convert::{TryFrom, TryInto};

#[derive(Clone, Debug)]
struct Forest {
    data: String,
    width: usize,
    height: usize,
}

impl TryFrom<&Input<'_>> for Forest {
    type Error = &'static str;

    fn try_from(input: &Input) -> Result<Self, Self::Error> {
        let data = input.0.to_string();
        let width = input.0.find('\n').ok_or("Can't find \\n in input")?;
        let height = input
            .0
            .as_bytes()
            .iter()
            .filter(|c| **c as char == '\n') // search for newlines
            .count();

        Ok(Forest {
            data: data.replace('\n', ""), // remove newlines
            width,
            height,
        })
    }
}

impl Forest {
    fn traverse(&self, slope: (usize, usize)) -> impl Iterator<Item = usize> + '_ {
        (0..)
            .map(move |i| {
                // generate all possible coordinates for the given slope
                // columns wrap around
                let coord = ((i * slope.0) % self.width, i * slope.1);
                // calculate the array index of the coordinate
                coord.0 + coord.1 * self.width
            })
            .take_while(move |pos| *pos < self.height * self.width)
    }

    fn get(&self, index: usize) -> char {
        self.data.as_bytes()[index] as char
    }

    fn count_with_slope(&self, slope: (usize, usize)) -> usize {
        self.traverse(slope)
            .filter(|pos| self.get(*pos) == '#')
            .count()
    }
}

pub struct Day;

impl DayTrait for Day {
    fn part1(&self, input: &Input) -> String {
        let forest: Forest = input.try_into().unwrap();
        forest.count_with_slope((3, 1)).to_string()
    }

    fn part2(&self, input: &Input) -> String {
        let forest: Forest = input.try_into().unwrap();
        let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let product: usize = slopes.iter().map(|s| forest.count_with_slope(*s)).product();
        product.to_string()
    }
}
