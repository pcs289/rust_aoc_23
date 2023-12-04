pub fn part1(input: String) -> u32 {
    input
        .lines()
        .map(parse_line1)
        .sum::<u32>()
}

fn parse_line1(line: &str) -> u32 {
    let numerics_in_line: Vec<u32> = (0..line.len())
        .into_iter()
        .filter_map(|index| {
            let subline = &line[index..index+1];
            subline.to_string().parse::<u32>().ok()
        })
        .collect();
    let result = format!("{}{}", numerics_in_line.first().unwrap(), numerics_in_line.last().unwrap());
    result.parse().unwrap()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(parse_line2)
        .sum()
}

fn parse_line2(line: &str) -> u32 {
    let numerics_in_line: Vec<i32> = (0..line.len())
        .into_iter()
        .filter_map(|index| {
            let subline = &line[index..];
            if subline.starts_with("one") {
                Some(1)
            } else if subline.starts_with("two") {
                Some(2)
            } else if subline.starts_with("three") {
                Some(3)
            } else if subline.starts_with("four") {
                Some(4)
            } else if subline.starts_with("five") {
                Some(5)
            } else if subline.starts_with("six") {
                Some(6)
            } else if subline.starts_with("seven") {
                Some(7)
            } else if subline.starts_with("eight") {
                Some(8)
            } else if subline.starts_with("nine") {
                Some(9)
            } else if let Ok(numeric) = subline[0..1].parse::<i32>() {
                Some(numeric)
            } else {
                None
            }
        })
        .collect();
    let result = format!("{}{}", numerics_in_line.first().unwrap(), numerics_in_line.last().unwrap());
    result.parse().unwrap()
}

#[cfg(test)]
mod day_1 {
    use super::*;

    const INPUT1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const INPUT2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT1.to_string()), 142);
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT2), 281);
    }
}
