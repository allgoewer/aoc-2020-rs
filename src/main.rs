mod day1;
mod day2;

struct Input<'s>(&'s str);

trait Day {
    fn part1(&self, _input: &Input) -> String {
        String::new()
    }
    fn part2(&self, _input: &Input) -> String {
        String::new()
    }
}

fn main() {
    let days: Vec<(Box<dyn Day>, _)> = vec![
        (Box::new(day1::Day), Input(include_str!("inputs/1"))),
        (Box::new(day2::Day), Input(include_str!("inputs/2"))),
    ];

    for (mut i, (day, input)) in days.iter().enumerate() {
        i += 1;
        println!("day{:0>2}-part1\t{}", i, day.part1(&input));
        println!("day{:0>2}-part2\t{}", i, day.part2(&input));
        println!();
    }
}
