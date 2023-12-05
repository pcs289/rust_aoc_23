use day_2::*;

fn main() {
    println!("Day 2");
    let input = include_str!("../input.txt");

    println!("Part 1! What is the sum of the IDs of those games?");
    let p1 = part1(input.into());
    println!("{}", p1);

    println!("Part 2! What is the sum of the power of these sets?");
    let p2 = part2(input.into());
    println!("{}", p2);
}
