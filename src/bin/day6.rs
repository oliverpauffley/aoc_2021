use std::panic;

fn main() {
    let input = include_str!("input_day6.txt");

    let numbers = input
        .trim()
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let days = 256;

    let mut fishes = FishTable::new(numbers);

    for _ in 0..days {
        fishes.simulate_day()
    }

    println!("{:?}", fishes.sum());
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FishTable {
    day0: i64,
    day1: i64,
    day2: i64,
    day3: i64,
    day4: i64,
    day5: i64,
    day6: i64,
    day7: i64,
    day8: i64,
}

impl FishTable {
    pub fn new(fishes: Vec<i32>) -> Self {
        let mut table = Self {
            day0: 0,
            day1: 0,
            day2: 0,
            day3: 0,
            day4: 0,
            day5: 0,
            day6: 0,
            day7: 0,
            day8: 0,
        };
        for fish in fishes {
            match fish {
                0 => table.day0 += 1,
                1 => table.day1 += 1,
                2 => table.day2 += 1,
                3 => table.day3 += 1,
                4 => table.day4 += 1,
                5 => table.day5 += 1,
                6 => table.day6 += 1,
                7 => table.day7 += 1,
                8 => table.day8 += 1,
                _ => panic!("not sure what to do with this number"),
            }
        }
        table
    }

    pub fn simulate_day(&mut self) {
        let copy = &self.clone();
        self.day0 = copy.day1;
        self.day1 = copy.day2;
        self.day2 = copy.day3;
        self.day3 = copy.day4;
        self.day4 = copy.day5;
        self.day5 = copy.day6;
        self.day6 = copy.day7 + copy.day0;
        self.day7 = copy.day8;
        self.day8 = copy.day0;
    }

    pub fn sum(&self) -> i64 {
        self.day0
            + self.day1
            + self.day2
            + self.day3
            + self.day4
            + self.day5
            + self.day6
            + self.day7
            + self.day8
    }
}
