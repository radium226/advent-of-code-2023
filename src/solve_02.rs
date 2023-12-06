use std::collections::HashMap;
use std::result::Result;
use fancy_regex::Regex;
use std::error::Error;

pub fn solve_02(lines: impl IntoIterator<Item=String>) -> () {
    let solutions = do_solve_02(lines);
    let solutions = solutions.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(", ");
    println!("solution: {}", solutions);
}

fn do_solve_02(lines: impl IntoIterator<Item=String>) -> Vec<u32> {
    return vec![1, 2, 3];
}

struct Game {
    id: u32,
    revelations: Vec<Revelation>,
}

impl Game {

    fn parse(line: String) -> Result<Game, Box<dyn Error>> {

        let regex = Regex::new(r"Game (?P<id>\d+): (?P<revelations>.+)")?;
        let captures = regex.captures(&line)?;
        let captures = captures.expect("Unable parse the line to a game! ");
        let game_id = captures.name("id").expect("Unable to parse the game id! ");
        let game_id = game_id.as_str();
        let game_id = game_id.parse::<u32>()?;

        let revelations = captures.name("revelations").expect("Unable to parse the revelations! ");
        let revelations = revelations.as_str();
        let ref revelations = revelations.split(";");
        dbg!(revelations);

        let game_revelations: Vec<Revelation> = revelations
            .to_owned()
            .collect::<Vec<&str>>()
            .iter()
            .map(|part| part.trim().to_string())
            .map(Revelation::parse)
            .map(|result| result.expect("Unable to parse the revelation! "))
            .collect();

        dbg!(revelations);

        Ok(Game {
            id: game_id,
            revelations: game_revelations,
        })
    }

}


#[derive(Debug)]
struct Revelation(HashMap<Color, u32>);

impl Revelation {

    fn parse(text: String) -> Result<Revelation, Box<dyn Error>> {
        let regex = Regex::new(r"(?P<color>red|green|blue): (?P<count>\d+)(, ?)")?;
        Ok(Revelation(regex
            .captures_iter(text.as_str())
            .map(|capture| {
                let capture = capture.expect("Unable to parse the capture! ");
                let color = capture.name("color").expect("Unable to parse the color! ");
                let color = color.as_str();
                let color = match color {
                    "red" => Color::Red,
                    "green" => Color::Green,
                    "blue" => Color::Blue,
                    _ => panic!("Unknown color! "),
                };

                let count = capture.name("count").expect("Unable to parse the count! ");
                let count = count.as_str();
                let count = count.parse::<u32>().expect("Unable to parse the count! ");

                (color, count)
            })
            .collect()))
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}


#[cfg(test)]
mod tests {
    
        #[test]
        fn parse_game_should_work() {
            use super::*;
            let line = "Game 1: 1 red, 2 green, 3 blue".to_string();
            let game = Game::parse(line).expect("Unable to parse the game! ");
            assert_eq!(game.id, 1);

            assert_eq!(game.revelations.len(), 1);
            assert_eq!(game.revelations[0].0.len(), 3);
            assert_eq!(game.revelations[0].0[&Color::Red], 1);
            assert_eq!(game.revelations[0].0[&Color::Green], 2);
            assert_eq!(game.revelations[0].0[&Color::Blue], 3);
        }
}