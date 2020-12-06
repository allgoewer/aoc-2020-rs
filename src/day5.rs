use super::{Day as DayTrait, Input};
use std::str::FromStr;


#[derive(Clone, Debug, PartialEq, Eq)]
struct Seat {
    row: usize,
    col: usize,
    id: usize,
}

fn parse<'a>(input: &Input<'a>) -> impl Iterator<Item = &'a str> {
    input.0.split_ascii_whitespace()
}

impl FromStr for Seat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s.as_bytes();

        let rows = &b[0..8];
        let cols = &b[7..10];

        let row = rows.iter().fold((0, 127), |rows, p| {
            if *p as char == 'F' {
                (rows.0, (rows.0 + rows.1) / 2)
            } else {
                ((rows.0 + rows.1 + 1) / 2, rows.1)
            }
        });

        let col = cols.iter().fold((0, 7), |cols, p| {
            if *p as char == 'L' {
                (cols.0, (cols.0 + cols.1) / 2)
            } else {
                ((cols.0 + cols.1 + 1) / 2, cols.1)
            }
        });

        let row = if rows[7] as char == 'B' { row.0 } else { row.1 };
        let col = if cols[2] as char == 'L' { col.0 } else { col.1 };
        let id = row * 8 + col;

        Ok(Seat {
            row,
            col,
            id,
        })
    }
}


pub struct Day;

impl DayTrait for Day {
    fn part1(&self, input: &Input) -> String {
        let max = parse(input)
            .filter_map(|i| i.parse::<Seat>().ok())
            .map(|s| s.id).max().unwrap();

        max.to_string()
    }

    fn part2(&self, input: &Input) -> String {
        let mut seats: Vec<_> = parse(input)
            .filter_map(|i| i.parse::<Seat>().ok())
            .map(|s| s.id)
            .collect();

        seats.sort_unstable();

        for seats in seats.windows(2) {
            let a = seats[0];
            let b = seats[1];

            if a + 1 != b {
                return (a + 1).to_string();
            }
        }


        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let s = "BFFFBBFRRR";
        let seat: Seat = s.parse().unwrap();
        assert_eq!(seat, Seat { row: 70, col: 7, id: 567 });

        let s = "FFFBBBFRRR";
        let seat: Seat = s.parse().unwrap();
        assert_eq!(seat, Seat { row: 14, col: 7, id: 119 });

        let s = "BBFFBBFRLL";
        let seat: Seat = s.parse().unwrap();
        assert_eq!(seat, Seat { row: 102, col: 4, id: 820 });
    }
}
