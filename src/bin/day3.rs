use std::{collections::HashMap, fmt::Write};

fn main() {
    let input = include_str!("input_day3.txt");

    let (gamma, epsilon) = parse_power(input);

    print!(
        "gamma is {}, epsilon is {}\nmultiplied is {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    let (oxygen, c02) = parse_life_support(input);

    print!(
        "oxygen is {}, c02 is {}\nmultiplied is {}",
        oxygen,
        c02,
        oxygen * c02
    );
}

fn parse_power(input: &str) -> (i32, i32) {
    let bit_length = input.lines().next().unwrap().len();

    let mut gamma_binary = String::new();
    let mut epsilon_binary = String::new();

    for i in 0..bit_length {
        let (most, min) = find_most_least(i, input);

        gamma_binary.write_char(most).unwrap();
        epsilon_binary.write_char(min).unwrap();
    }

    let gamma = i32::from_str_radix(&gamma_binary, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon_binary, 2).unwrap();
    (gamma, epsilon)
}

fn parse_life_support(input: &str) -> (i32, i32) {
    let last = input.lines().next().unwrap().len() - 1;

    let nums: Vec<i32> = input
        .lines()
        .map(|s| i32::from_str_radix(s, 2).unwrap())
        .collect();

    let mut o_nums = nums.clone();
    let mut c_nums = nums;
    for bit in (0..=last).rev().map(|i| 1 << i) {
        let count = |nums: &[i32]| {
            nums.iter().fold((0, 0), |(z, o), n| {
                if (n & bit) == bit {
                    (z, o + 1)
                } else {
                    (z + 1, o)
                }
            })
        };
        let reduce = |nums: &mut Vec<i32>, want| {
            let mut i = 0;
            while i < nums.len() {
                if nums[i] & bit == want {
                    i += 1;
                } else {
                    nums.swap_remove(i);
                }
            }
        };

        if o_nums.len() > 1 {
            let (z, o) = count(&o_nums);
            reduce(&mut o_nums, if o >= z { bit } else { 0 });
        }
        if c_nums.len() > 1 {
            let (z, o) = count(&c_nums);
            reduce(&mut c_nums, if o < z { bit } else { 0 });
        }
    }

    (o_nums[0], c_nums[0])
}

fn find_most_least(idx: usize, input: &str) -> (char, char) {
    let mut occurences = HashMap::new();

    input.lines().for_each(|line| {
        *occurences
            .entry(line.chars().nth(idx).unwrap())
            .or_insert(0) += 1
    });

    let most = occurences
        .iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| value)
        .unwrap();

    let min = occurences
        .iter()
        .min_by_key(|&(_, count)| count)
        .map(|(value, _)| value)
        .unwrap();

    (most.clone(), min.clone())
}
