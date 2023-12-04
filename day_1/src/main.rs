use day_1::*;

fn main() {
    println!("Day 1");
    let input = include_str!("../input.txt");

    println!("Part 1! What is the sum of all of the calibration values?");
    let p1 = part1(input.into());
    println!("{}", p1);

    println!("Part 2! What is the sum of all of the calibration values?");
    let p2 = part2(input.into());
    println!("{}", p2);
}
