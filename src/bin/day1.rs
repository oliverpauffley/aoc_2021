fn main() {
    let input = include_str!("input_day1.txt");

    let depth_count = find_depth_count_part_1(input);

    println!("depth count is {} for single windows", depth_count);

    let depth_count = find_depth_count_part_2(input);

    println!("depth count is {} for 3 length windows", depth_count)
}

fn find_depth_count_part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(2)
        .fold(0, |acc, win| if win[0] < win[1] { acc + 1 } else { acc })
}

fn find_depth_count_part_2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(3)
        .map(|win| win[0] + win[1] + win[2])
        .collect::<Vec<i32>>()
        .windows(2)
        .fold(0, |acc, win| if win[0] < win[1] { acc + 1 } else { acc })
}
