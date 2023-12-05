use std::str::FromStr;

mod parser;

#[derive(Debug)]
pub struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    cubes: Vec<Cube>,
}

#[derive(Debug)]
struct Cube {
    color: Color,
    amount: u32,
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}
impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(())
        }
    }
}

pub fn part1(input: &str) -> u32 {
    // Parse Games
    let (_, games) = parser::parse_games(&input).unwrap();

    // Sum the Game IDs of the possible games available with 12 red cubes, 13 green cubes, and 14 blue cubes
    games
        .into_iter()
        .filter_map(validate_game)
        .sum()
}

fn validate_game(game: Game) -> Option<u32> {
    let old_rounds_length = game.rounds.len();
    // Filter out invalid rounds
    let rounds: Vec<Round> = game.rounds
        .into_iter()
        .filter_map(validate_rounds)
        .collect();

    if old_rounds_length - rounds.len() == 0 {
        // No Discarded Rounds => Valid Game => Keep
        Some(game.id)
    } else {
        // Some Discarded Rounds => Invalid Game => Discard
        None
    }
}

fn validate_rounds(round: Round) -> Option<Round> {
    let old_cubes_length = round.cubes.len();
    // Filter out invalid cubes
    let cubes: Vec<Cube> = round.cubes
        .into_iter()
        .filter_map(validate_cubes)
        .collect();

    if old_cubes_length - cubes.len() == 0 {
        // No Discarded Cubes => Valid Round => Keep
        Some(Round { cubes })
    } else {
        // Some Discarded Cubes => Invalid Round => Discard
        None
    }
}

fn validate_cubes(cube: Cube) -> Option<Cube> {
    // Get limit to validate for color of cube
    let limit = cube_color_limit(&cube.color);
    if cube.amount > limit {
        // Invalid Cube => Discard
        None
    } else {
        // Valid Cube => Keep
        Some(cube)
    }
}

fn cube_color_limit(color: &Color) -> u32 {
    match color {
        Color::Red => 12,
        Color::Green => 13,
        Color::Blue => 14,
    }
}

pub fn part2(input: &str) -> u32 {
    // Parse Games
    let (_, games) = parser::parse_games(&input).unwrap();

    // Sum powers of minimal set of cubes required for game to be valid
    games
        .into_iter()
        .filter_map(minimum_cubes)
        .sum()
}

// We aim to find the minimum cubes to have to make the game valid
// Worst Round for Game 1: Round { cubes: [Cube { color: Red, amount: 4}, Cube { color: Green, amount: 2}, Cube { color: Blue, amount: 6}]}
fn minimum_cubes(game: Game) -> Option<u32> {
    // Find worst rounds
    let worst_rounds: Vec<(u32, u32, u32)> = game.rounds
        .into_iter()
        .map(|round| {
            // Find worst (red, green, blue) across cubes for each round
            let (mut worst_r, mut worst_g, mut worst_b) = (0, 0, 0);
            round.cubes
                .into_iter()
                .for_each(|cube| {
                    match cube.color {
                        Color::Red => {
                            if cube.amount > worst_r {
                                worst_r = cube.amount;
                            }
                        },
                        Color::Green => {
                            if cube.amount > worst_g {
                                worst_g = cube.amount;
                            }
                        },
                        Color::Blue => {
                            if cube.amount > worst_b {
                                worst_b = cube.amount;
                            }
                        },
                    };
                });
            (worst_r, worst_g, worst_b)
        })
        .collect();
    
    // Find worst (red, green, blue) across rounds for a game
    let (mut worst_r, mut worst_g, mut worst_b) = (0, 0, 0);
    worst_rounds
        .into_iter()
        .for_each(|(r, g, b)| {
            if r > worst_r {
                worst_r = r;
            }
            if g > worst_g {
                worst_g = g;
            }
            if b > worst_b {
                worst_b = b;
            }
        });
    
    // Return power of worst (red, green, blue) in the game
    Some(worst_r * worst_g * worst_b)
}

#[cfg(test)]
mod day_2 {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 8);
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 2286);
    }
}
