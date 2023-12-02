// Copyright 2023 Louis Royer. All rights reserved.
// Use of this source code is governed by a MIT-style license that can be
// found in the LICENSE file.
// SPDX-License-Identifier: MIT

use regex::Regex;
use std::cmp;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct ParseGameDataError;

/// A bag contains a certain amount of colored cubes.
/// There is 3 kinds of cubes: red, green, and blue.
#[derive(Clone, Copy)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

/// A game consist in taking several times a random number of cubes out of the bag.
/// We only store relevant data:
/// - the game id
/// - the maximum number of cubes showed of each color (red, green, and blue)
#[derive(Debug, PartialEq)]
struct GameData {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for GameData {
    type Err = ParseGameDataError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re_id = Regex::new(r"Game (\d+):(.*)").unwrap();
        let re_cube = Regex::new(r"(\d+) (blue|red|green)").unwrap();

        // retrieve id of the game
        let id: u32 = {
            match re_id
                .captures(s)
                .ok_or(ParseGameDataError)?
                .get(1)
                .ok_or(ParseGameDataError)?
                .as_str()
                .parse()
            {
                Ok(v) => v,
                Err(_) => return Err(ParseGameDataError),
            }
        };

        // get maximums for red, green, and blue cubes
        let content = re_id
            .captures(s)
            .ok_or(ParseGameDataError)?
            .get(2)
            .ok_or(ParseGameDataError)?
            .as_str()
            .split(";")
            .collect::<Vec<&str>>();
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for info in content {
            for cube_info in info.trim().split(",").collect::<Vec<&str>>() {
                let cube_cap = re_cube.captures(cube_info).ok_or(ParseGameDataError)?;
                match (
                    cube_cap.get(1).ok_or(ParseGameDataError)?.as_str(),
                    cube_cap.get(2).ok_or(ParseGameDataError)?.as_str(),
                ) {
                    (num, "red") => {
                        red = cmp::max(
                            red,
                            match num.parse() {
                                Ok(v) => v,
                                _ => return Err(ParseGameDataError),
                            },
                        )
                    }
                    (num, "green") => {
                        green = cmp::max(
                            green,
                            match num.parse() {
                                Ok(v) => v,
                                _ => return Err(ParseGameDataError),
                            },
                        )
                    }
                    (num, "blue") => {
                        blue = cmp::max(
                            blue,
                            match num.parse() {
                                Ok(v) => v,
                                _ => return Err(ParseGameDataError),
                            },
                        )
                    }
                    (_, _) => return Err(ParseGameDataError),
                }
            }
        }

        Ok(GameData {
            id: id,
            red: red,
            green: green,
            blue: blue,
        })
    }
}

/// Day 2: Cube Conundrum
fn main() {
    let file_path = "inputs/02.in";
    let file = read_to_string(file_path).expect("Could not open game records");

    // Create a bag
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    // TODO: use a buffer instead
    let sum = game_id_sum(file.lines(), bag).expect("Could not parse game records");
    println!("Sum of ids of valid games: {sum}");

    let p_sum = power_sum(file.lines()).expect("Could not parse game records");
    println!("Sum of cube power of games: {p_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_game_id_sum() {
        let bag = Bag {
            red: 12,
            green: 13,
            blue: 14,
        };
        assert_eq!(
            game_id_sum(
                vec![
                    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                    "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                    "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
                ]
                .into_iter(),
                bag
            ),
            Ok(8)
        );
    }

    #[test]
    fn it_validate() {
        let bag = Bag {
            red: 12,
            green: 13,
            blue: 14,
        };
        assert!(validate(
            &GameData::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap(),
            bag
        ));
        assert!(validate(
            &GameData::from_str("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
                .unwrap(),
            bag
        ));
        assert!(!validate(
            // too many red cubes in this game
            &GameData::from_str(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            )
            .unwrap(),
            bag
        ));
        assert!(!validate(
            // too many blue cubes in this game
            &GameData::from_str(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            )
            .unwrap(),
            bag
        ));
        assert!(validate(
            &GameData::from_str("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").unwrap(),
            bag
        ));
    }

    // Part 2
    #[test]
    fn it_game_data_from_str() {
        assert_eq!(
            GameData::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap(),
            GameData {
                id: 1,
                red: 4,
                green: 2,
                blue: 6
            }
        );
        assert_eq!(
            GameData::from_str("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
                .unwrap(),
            GameData {
                id: 2,
                red: 1,
                green: 3,
                blue: 4
            }
        );
        assert_eq!(
            GameData::from_str(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            )
            .unwrap(),
            GameData {
                id: 3,
                red: 20,
                green: 13,
                blue: 6
            }
        );
        assert_eq!(
            GameData::from_str(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            )
            .unwrap(),
            GameData {
                id: 4,
                red: 14,
                green: 3,
                blue: 15,
            }
        );
        assert_eq!(
            GameData::from_str("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").unwrap(),
            GameData {
                id: 5,
                red: 6,
                green: 3,
                blue: 2,
            }
        );
    }

    #[test]
    fn it_cube_power() {
        assert_eq!(
            cube_power(GameData {
                id: 1,
                red: 4,
                green: 2,
                blue: 6
            }),
            48
        );
        assert_eq!(
            cube_power(GameData {
                id: 2,
                red: 1,
                green: 3,
                blue: 4
            }),
            12
        );
        assert_eq!(
            cube_power(GameData {
                id: 3,
                red: 20,
                green: 13,
                blue: 6
            }),
            1560
        );
        assert_eq!(
            cube_power(GameData {
                id: 4,
                red: 14,
                green: 3,
                blue: 15,
            }),
            630
        );
        assert_eq!(
            cube_power(GameData {
                id: 5,
                red: 6,
                green: 3,
                blue: 2,
            }),
            36
        );
    }

    #[test]
    fn it_power_sum() {
        assert_eq!(
            power_sum(
                vec![
                    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                    "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                    "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
                ]
                .into_iter()
            ),
            Ok(2286)
        )
    }
}

/// Returns the sum of game ids which would have been possible with this game data
fn game_id_sum<'a>(
    lines: impl Iterator<Item = &'a str>,
    bag: Bag,
) -> Result<u32, ParseGameDataError> {
    lines
        .filter(|x| !x.is_empty()) // exclude empty lines,
        .map(|x| GameData::from_str(x)) // parse into game data,
        .collect::<Result<Vec<GameData>, ParseGameDataError>>()?
        .into_iter()
        .filter(|x| validate(x, bag)) // check validity of the game data according to bag content
        .map(|x| Ok(x.id)) // get bag id
        .sum() // and sum them all
}

/// A game data is valid if maximum cubes shown is lower than number of cubes in the bag.
fn validate(data: &GameData, bag: Bag) -> bool {
    data.red <= bag.red && data.green <= bag.green && data.blue <= bag.blue
}

/// Product of number of cubes that must have been present in a game
fn cube_power(data: GameData) -> u32 {
    data.red * data.green * data.blue
}

fn power_sum<'a>(lines: impl Iterator<Item = &'a str>) -> Result<u32, ParseGameDataError> {
    lines
        .filter(|x| !x.is_empty()) // exclude empty lines,
        .map(|x| GameData::from_str(x)) // parse into game data,
        .collect::<Result<Vec<GameData>, ParseGameDataError>>()?
        .into_iter()
        .map(|x| Ok(cube_power(x)))
        .sum() // and sum them all
}
