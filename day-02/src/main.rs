use std::{collections::BTreeMap, ops::Not};

use nom::{
    bytes::complete::tag,
    character::complete::{
        self, alpha1, digit1, line_ending,
    },
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

struct Cube<'a>{
    color: &'a str,
    count: u32,
}

struct Game<'a>{
    id: &'a str,
    rounds: Vec<Vec<Cube<'a>>>,
}

impl<'a> Game<'a> {
    fn valid_for_cube_set(
        &self,
        map: &BTreeMap<&str, u32>,
    ) -> Option<u32> {
        self.rounds
            .iter()
            .any(|round| {
                round.iter().any(|shown_cube| {
                    shown_cube.count
                        > *map
                            .get(shown_cube.color)
                            .expect("a valid cube")
                })
            })
            .not()
            .then_some(
                self.id.parse::<u32>().expect(
                    "game id should a parsable u32",
                ),
            )
    }
}
        

fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = 
        preceded(tag("Game "), digit1)(input)?;
    let (input, rounds) = preceded(
        tag(": "),
        separated_list1(tag("; "), round),
    )(input)?;
    Ok((input, Game{id, rounds}))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>>{
    let (input, games) = separated_list1(line_ending, game)(input)?;
    Ok((input, games))
}

fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, (count, color)) = 
    separated_pair(complete::u32, tag(" "), alpha1)(input)?;
    Ok((input, Cube{color, count}))
}

fn round(input: &str) -> IResult<&str, Vec<Cube>>{
    let (input, cubes) = separated_list1(tag(", "), cube)(input)?;
    Ok((input, cubes))
}

fn part1(input: &str) -> String {
    let map = BTreeMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);
    let games = parse_games(input).expect("should parse");
    games.1.iter()
        .filter_map(|game| game.valid_for_cube_set(&map))
        .sum::<u32>()
        .to_string()
}





#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part1(input).to_string();
        assert_eq!(result, "8");
    }
}