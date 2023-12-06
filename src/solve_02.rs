use std::collections::HashMap;
use std::result::Result;
use fancy_regex::Regex;
use std::error::Error;

pub fn solve_02_part_one(lines: impl IntoIterator<Item=String>) -> () {
    let solutions = do_solve_02_part1(lines);
    dbg!(&solutions);
    let solution: u32 = solutions.iter().sum();
    println!("solution: {}", solution);
}

pub fn solve_02_part_two(lines: impl IntoIterator<Item=String>) -> () {
    let solution = do_solve_02_part_two(lines);
    println!("solution: {}", solution);
}

fn do_solve_02_part1(lines: impl IntoIterator<Item=String>) -> Vec<u32> {
    let mut game_ids: Vec<u32> = Vec::new();
    for line in lines {
        let game = Game::parse(line).expect("Unable to parse the game! ");
        dbg!(&game);
        let content = find_content(&game.revelations);
        if content.get_count(Color::Red) <= 12 && content.get_count(Color::Green) <= 13 && content.get_count(Color::Blue) <= 14 {
            game_ids.push(game.id);
        }
    }

    return game_ids;
}

fn do_solve_02_part_two(lines: impl IntoIterator<Item=String>) -> u32 {
    lines
        .into_iter()
        .map(|line| Game::parse(line.to_string()).expect("Unable to parse the game! "))
        .map(|game| find_content(&game.revelations))
        .map(|content| content.get_count(Color::Red) * content.get_count(Color::Green) * content.get_count(Color::Blue))
        .sum()
}

fn find_content(revelations: &Vec<Revelation>) -> Content {
    let mut content = Content::default();
    for revelation in revelations {
        for (color, count) in revelation.content().iter() {
            let count = count;
            let current_count = &content.get_count(*color);
            dbg!(color, count, current_count);
            if current_count < count {
                content.set_count(*color, *count);
            }
        }
    }
    dbg!(&content);
    return content;
}

#[derive(Debug)]
struct Game {
    id: u32,
    revelations: Vec<Revelation>,
}

impl Game {

    fn new(id: u32, revelations: Vec<Revelation>) -> Game {
        Game {
            id,
            revelations,
        }
    }

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

        dbg!(&game_revelations);

        Ok(Game {
            id: game_id,
            revelations: game_revelations,
        })
    }

}

#[derive(Debug)]
struct Content(HashMap<Color, u32>);

impl Content {

    fn new(red_count: u32, green_count: u32, blue_count: u32) -> Content {
        let mut content = HashMap::new();
        content.insert(Color::Red, red_count);
        content.insert(Color::Green, green_count);
        content.insert(Color::Blue, blue_count);
        Content(content)
    }

    fn get_count(self: &Content, color: Color) -> u32 {
        self.0.get(&color).unwrap().to_owned()
    }

    fn set_count(self: &mut Content, color: Color, count: u32) -> () {
        self.0.insert(color, count);
    }

    fn iter(self: &Content) -> impl Iterator<Item=(&Color, &u32)> {
        self.0.iter()
    }
}

impl Default for Content {
    fn default() -> Self {
        Content({
            let mut map = HashMap::new();
            map.insert(Color::Red, 0);
            map.insert(Color::Green, 0);
            map.insert(Color::Blue, 0);
            map
        })
    }
}

impl Default for Bag {
    fn default() -> Self {
        Bag(Content::default())
    }
}

#[derive(Debug)]
struct Bag(Content);

#[derive(Debug)]
struct Revelation(Content);

impl Revelation {
    
    fn new(content: Content) -> Revelation {
        Revelation(content)
    }

    fn content(&self) -> &Content {
        &self.0
    }

    fn parse(text: String) -> Result<Revelation, Box<dyn Error>> {
        let regex = Regex::new(r"(?P<count>\d+) (?P<color>red|green|blue)(, )?")?;

        let counts_by_color: Result<HashMap<Color, u32>, Box<dyn Error>> = regex
            .captures_iter(text.as_str())
            .map(|capture| {
                let capture = capture?;
                let color = capture.name("color").ok_or("Unable to find color! ")?;
                let color = color.as_str();
                let color = Color::parse(color)?;

                let count = capture.name("count").ok_or("Unable to find count! ")?;
                let count = count.as_str();
                let count = count.parse::<u32>()?;

                Ok((color, count))
            })
            .collect();

        let counts_by_color = counts_by_color?;

        dbg!(&counts_by_color);

        let mut content = Content::default();
        for (color, count) in counts_by_color.iter() {
            content.set_count(*color, *count);
        }

        Ok(Revelation::new(content))
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn parse(text: &str) -> Result<Color, Box<dyn Error>> {
        match text {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err("Unknown color! ".into()),
        }
    }
}


#[cfg(test)]
mod tests {
    
        #[test]
        fn parse_game_should_work() {
            use super::*;
            let line = "Game 1: 1 red, 2 green".to_string();
            let game = Game::parse(line).expect("Unable to parse the game! ");
            assert_eq!(game.id, 1);

            assert_eq!(game.revelations.len(), 1);
            assert_eq!(game.revelations[0].0.0.len(), 3);
            assert_eq!(game.revelations[0].0.0[&Color::Red], 1);
            assert_eq!(game.revelations[0].0.0[&Color::Green], 2);
            assert_eq!(game.revelations[0].0.0[&Color::Blue], 0);
        }

        #[test]
        fn set_count_should_work() {
            use super::*;
            let mut content = Content::default();
            content.set_count(Color::Red, 1);
            assert_eq!(content.get_count(Color::Red), 1);
        }

        #[test]
        fn find_content_should_work() {
            use super::*;

            let game = Game::parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()).unwrap();
            let content = find_content(&game.revelations);
            assert_eq!(content.get_count(Color::Red), 4);
            assert_eq!(content.get_count(Color::Green), 2);
            assert_eq!(content.get_count(Color::Blue), 6);
        }
}