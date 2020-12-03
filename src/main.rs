mod day1;
mod day2;
mod day3;

#[derive(Clone, Debug)]
struct Input<'s>(&'s str);

trait Day {
    fn part1(&self, _input: &Input) -> String {
        String::new()
    }
    fn part2(&self, _input: &Input) -> String {
        String::new()
    }
}

fn timed<F>(day: usize, part: usize, func: F)
where
    F: Fn() -> String
{
    let start = std::time::Instant::now();
    let result = func();
    let elapsed = start.elapsed();

    println!("day{:0>2}-part{} {:>9} us {:>12}", day, part, elapsed.as_micros(), result);
}

fn main() {
    let days: Vec<(Box<dyn Day>, _)> = vec![
        (Box::new(day1::Day), Input(include_str!("inputs/1"))),
        (Box::new(day2::Day), Input(include_str!("inputs/2"))),
        (Box::new(day3::Day), Input(include_str!("inputs/3"))),
    ];

    for (mut i, (day, input)) in days.iter().enumerate() {
        i += 1;

        timed(i, 1, || day.part1(&input));
        timed(i, 2, || day.part2(&input));
        println!();
    }
}
