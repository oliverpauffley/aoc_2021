use std::panic;

use ndarray::{Array2, ArrayView};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card {
    pub numbers: ndarray::Array2<i32>,
}

impl Card {
    pub fn new(input: &str) -> Self {
        let mut numbers = Array2::zeros((0, 5));

        for line in input.lines() {
            let line_nums = line
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            match numbers.push_row(ArrayView::from(&line_nums)) {
                Ok(_) => (),
                Err(err) => {
                    println!("could not load this card:\n {}\n", input);
                    panic!("err loading card {}", err);
                }
            }
        }
        Self { numbers }
    }

    pub fn check_rows(&self, drawn_numbers: &Vec<i32>) -> bool {
        for row in self.numbers.rows() {
            let complete = row.iter().all(|num| drawn_numbers.contains(num));
            if complete {
                return true;
            }
        }
        false
    }

    pub fn check_columns(&self, drawn_numbers: &Vec<i32>) -> bool {
        for column in self.numbers.columns() {
            let complete = column.iter().all(|num| drawn_numbers.contains(num));
            if complete {
                return true;
            }
        }
        false
    }

    pub fn is_winning_card(&self, drawn_numbers: &Vec<i32>) -> bool {
        self.check_columns(drawn_numbers) || self.check_rows(drawn_numbers)
    }

    pub fn unmarked_numbers(&self, drawn_numbers: &Vec<i32>) -> Vec<i32> {
        let mut unmarked: Vec<i32> = Vec::new();
        for num in self.numbers.iter() {
            if !drawn_numbers.contains(num) {
                unmarked.push(*num)
            }
        }
        unmarked
    }
}

fn main() {
    let input = include_str!("input_day4.txt");

    let (drawn, cards) = input.split_once("\n").unwrap();

    let drawn_numbers = drawn
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let cards = get_cards(cards.trim());

    println!("part 1:");
    'outer: for window_size in 1..drawn_numbers.len() {
        for card in cards.iter() {
            let drawn_so_far: Vec<i32> = drawn_numbers[0..window_size].to_vec();
            if card.is_winning_card(&drawn_so_far) {
                println!("card:\n{}\n drawn: {:?}", card.numbers, drawn_so_far);

                let unmarked = card.unmarked_numbers(&drawn_so_far);
                let last_drawn = drawn_so_far.last().unwrap();
                println!("unmarked: {:?}\nlast: {:?}", unmarked, last_drawn);

                let sum: i32 = unmarked.iter().sum();
                println!("sum: {}\nmultplied: {}", sum, sum * last_drawn);
                break 'outer;
            }
        }
    }

    println!("\npart 2:");
    let mut winning_cards: Vec<&Card> = Vec::new();
    let mut last_numbers = Vec::new();
    'outer2: for window_size in 1..drawn_numbers.len() {
        for card in cards.iter() {
            let drawn_so_far: Vec<i32> = drawn_numbers[0..window_size].to_vec();
            if !winning_cards.contains(&card) && card.is_winning_card(&drawn_so_far) {
                winning_cards.push(card);
                last_numbers = drawn_so_far;
                if winning_cards.len() == cards.len() {
                    break 'outer2;
                }
            }
        }
    }

    let last_winner = winning_cards.pop().unwrap();
    println!("card:\n{}\n drawn: {:?}", last_winner.numbers, last_numbers);

    let unmarked = last_winner.unmarked_numbers(&last_numbers);
    let last_drawn = last_numbers.last().unwrap();
    println!("unmarked: {:?}\nlast: {:?}", unmarked, last_drawn);

    let sum: i32 = unmarked.iter().sum();
    println!("sum: {}\nmultplied: {}", sum, sum * last_drawn);
}

fn get_cards(input: &str) -> Vec<Card> {
    input
        .split("\n\n")
        .map(|card| Card::new(card))
        .collect::<Vec<Card>>()
}

#[cfg(test)]
mod test_card {
    use ndarray::array;

    use super::*;

    #[test]
    fn test_get_cards() {
        let input = "1 2 3 4 5
6 7 8 9 10
11 12 13 14 15

1 2 3 4 5
6 7 8 9 10
11 12 13 14 16";
        let got = get_cards(input);
        let want1 = array![[1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15]];
        let want2 = array![[1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 16]];
        assert_eq!(got[0].numbers, want1);
        assert_eq!(got[1].numbers, want2);
    }

    #[test]
    fn test_new() {
        let input = "1 2 3 4 5
            6 7 8 9 10
            11 12 13 14 15";
        let got = Card::new(input);
        let want = array![[1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15]];

        assert_eq!(got.numbers, want)
    }

    #[test]
    fn test_check_rows_complete() {
        let input = "1 2 3 4 5
            6 7 8 9 10
            11 12 13 14 15";
        let card = Card::new(input);
        let drawn = vec![1, 50, 2, 3, 4, 5, 100, 121];
        assert!(card.check_rows(&drawn))
    }
    #[test]
    fn test_check_rows_incomplete() {
        let input = "1 2 3 4 5
            6 7 8 9 10
            11 12 13 14 15";
        let card = Card::new(input);
        let drawn = vec![1, 50, 2, 3, 5, 100, 121];
        assert!(!card.check_rows(&drawn))
    }
    #[test]
    fn test_check_columns_complete() {
        let input = "1 2 3 4 5
            6 7 8 9 10
            11 12 13 14 15";
        let card = Card::new(input);
        let drawn = vec![5, 10, 15, 1];
        assert!(card.check_columns(&drawn))
    }
    #[test]
    fn test_check_columns_incomplete() {
        let input = "1 2 3 4 5
            6 7 8 9 10
            11 12 13 14 15";
        let card = Card::new(input);
        let drawn = vec![1, 6, 12];
        assert!(!card.check_columns(&drawn))
    }
    #[test]
    fn test_is_winning() {
        let input = "1 2 3 4 5
            6 7 8 9 10
            11 12 13 14 15";
        let card = Card::new(input);
        let drawn = vec![5, 10, 15, 1];
        assert!(card.is_winning_card(&drawn))
    }

    #[test]
    fn test_unmarked() {
        let input = "1 2 3 4 5
            6 7 8 9 10
            11 12 13 14 15";
        let card = Card::new(input);
        let drawn = vec![1, 2, 3, 4, 5, 6];
        let want = vec![7, 8, 9, 10, 11, 12, 13, 14, 15];
        assert_eq!(card.unmarked_numbers(&drawn), want)
    }
}
