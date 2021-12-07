fn main() {
    let input = include_str!("input_day7.txt");

    let mut crab_positions = input
        .trim()
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    // part 1
    crab_positions.sort();
    let mid = crab_positions.len() / 2;
    let median = crab_positions[mid];

    let mut fuel = 0;

    for crab in &crab_positions {
        let distance = (crab - median).abs();
        fuel += distance;
    }

    println!(
        "the  median was {} giving a fuel cost to align of {}",
        median, fuel
    );

    let length = crab_positions.len();

    // part 2 - brute force
    let mut lowest_fuel: i32 = i32::MAX;
    for possible_pos in 0..length {
        let f = get_fuel_cost(&crab_positions, possible_pos as i32);
        lowest_fuel = if f < lowest_fuel { f } else { lowest_fuel }
    }

    println!("the fuel cost was {}", lowest_fuel);
}

pub fn get_fuel_cost(crab_pos: &Vec<i32>, pos: i32) -> i32 {
    crab_pos.iter().fold(0, |fuel, crab| {
        let movement = (crab - pos).abs();
        fuel + (movement * (movement + 1)) / 2
    })
}
