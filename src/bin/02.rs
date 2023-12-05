// use regex::Regex;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let games = input.lines().map(parse_line).collect::<Option<Vec<Game>>>().unwrap();
    Some(games.iter().filter(|g| g.is_valid(12,13,14)).map(|g| g.id).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = input.lines().map(parse_line).collect::<Option<Vec<Game>>>().unwrap();
    Some(games.iter().map(Game::calc_power).sum())
}

// static GAME_RE: Regex = Regex::new(r"Game (\d+):((?: \d+ (?:red|green|blue),?){1-3};)+").unwrap();


#[derive(Debug)]
struct Game{
    id: u32,
    reveal: Vec<Reveal>
}

impl Game {
    fn is_valid(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        for reveal in &self.reveal {
            if reveal.red > max_red || reveal.green > max_green || reveal.blue > max_blue {
                return false;
            }
        }
        true
    }
    
    fn calc_power(&self) -> u32 {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for reveal in &self.reveal {
            if reveal.red > max_red {
                max_red = reveal.red;
            }
            if reveal.green > max_green {
                max_green = reveal.green;
            }
            if reveal.blue > max_blue {
                max_blue = reveal.blue;
            }
        }
        max_red * max_green * max_blue
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Reveal{
    red: u32,
    green: u32,
    blue: u32,
}

impl Reveal {
    fn from_str(s: &str) -> Option<Reveal> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for part in s.split(",") {
            let mut part = part.trim().split(" ");
            let count = part.next().unwrap().parse::<u32>();
            if count.is_err() {
                return None;
            }
            let count = count.unwrap();
            let color = part.next().unwrap();
            match color {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => return None,
            }
        }
        Some(Reveal{
            red,
            green,
            blue,
        })
    }
}

fn parse_line(line: &str) -> Option<Game> {
    let mut parts = line.split(":");
    let id_part = parts.next().unwrap().split(" ").nth(1);
    if id_part.is_none() {
        return None;
    }
    let id = id_part.unwrap().parse::<u32>();
    if id.is_err() {
        println!("id is err");
        return None;
    }
    let id = id.unwrap();
    let reveals_str = parts.next().unwrap();
    let mut reveals = Vec::new();
    for reveal in reveals_str.split(";") {
        let reveal = Reveal::from_str(reveal);
        if reveal.is_none() {
            return None;
        }
        reveals.push(reveal.unwrap());
    }
    Some(Game{
        id,
        reveal: reveals,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
