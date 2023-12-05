use nom::{IResult, sequence::{preceded, separated_pair}, multi::separated_list1};
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, digit1, alpha1, u32 as nom_u64};

use crate::{Game, Round, Cube};

// 3 blue
fn parse_cube(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, color)) = separated_pair(nom_u64, tag(" "), alpha1)(input)?;

    Ok((input, Cube { amount, color: color.parse().unwrap() }))
}

// 3 blue, 4 red
fn parse_rounds(input: &str) -> IResult<&str, Round> {
    let (input, cubes) = separated_list1(tag(", "), parse_cube)(input)?;
    Ok((input, Round { cubes }))
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn parse_game(input: &str) -> IResult<&str, Game> {
    // Find one or more ASCI characters (digit1) preceded by "Game " and assign it to `id` variable
    let (input, game_id) = preceded(tag("Game "), digit1)(input)?;

    // Parse Game Id from &str to u32
    let id = game_id.parse::<u32>().unwrap();

    // Find one or more Rounds (parse_rounds) preceded by ": " and assign it to `rounds` variable
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), parse_rounds))(input)?;

    Ok((input, Game { id, rounds }))
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
pub fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, parse_game)(input)?;
    Ok((input, games))
}
