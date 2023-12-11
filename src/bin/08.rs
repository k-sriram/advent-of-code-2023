use std::collections::HashMap;

use lazy_static::lazy_static;
use num::integer::lcm;
use regex::Regex;

advent_of_code::solution!(8);

lazy_static! {
    static ref MAP_RE: Regex =
        Regex::new(r"([A-Z1-9]{3}) = \(([A-Z1-9]{3}), ([A-Z1-9]{3})\)").unwrap();
}

pub fn part_one(input: &str) -> Option<u64> {
    let (dir, map) = parse_input(input)?;
    traverse(&dir, &map).unwrap().try_into().ok()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (dir, map) = parse_input(input).expect("Failed to parse input");

    let starts = map
        .0
        .keys()
        .filter(|k| k.ends_with("A"))
        .collect::<Vec<_>>();

    let cycles = starts
        .iter()
        .map(|&start| traversal_cycle(&dir, &map, start).unwrap())
        .collect::<Vec<_>>();

    let cycle_len = cycles.iter().map(|&(l, _)| l).fold(1, lcm);

    for (i, start) in starts.iter().enumerate() {
        println!("Start: {}", start);
        println!("Cycle: {:?}", cycles[i].0);
        // println!("Cycle: {:?}", expand_cycle(cycles[i].0, cycle_len, &cycles[i].1));
    }
    println!("lcm: {}", cycle_len);
    Some(cycle_len)
}

#[derive(Debug)]
struct Network(HashMap<String, (String, String)>);

#[derive(Debug)]
struct Directions(Vec<usize>);

fn traverse(dir: &Directions, map: &Network) -> Result<u64, &'static str> {
    let mut visited = 0;
    let mut pos = "AAA".to_string();
    for &d in dir.0.iter().cycle() {
        let (to1, to2) = map.0.get(&pos).ok_or("Position not in map")?;
        let next = if d == 0 { to1 } else { to2 };
        visited += 1;
        pos = next.clone();
        if pos == "ZZZ" {
            break;
        }
    }
    Ok(visited)
}

fn traversal_cycle(
    dir: &Directions,
    map: &Network,
    start: &str,
) -> Result<(u64, Vec<(String, u64)>), &'static str> {
    let num_dir = dir.0.len() as u64;
    let mut visited = Vec::new();
    let mut pos = start.to_string();
    let mut cycle_len = 0;
    for (i, &d) in dir.0.iter().cycle().enumerate() {
        let cycle_i = i as u64 % num_dir;
        let (to1, to2) = map.0.get(&pos).ok_or("Position not in map")?;
        let next = if d == 0 { to1 } else { to2 };
        if visited.contains(&(next.clone(), cycle_i)) {
            let first = visited
                .iter()
                .position(|(p, i)| p == next && i == &cycle_i)
                .unwrap();
            let cur = visited.len();
            cycle_len = cur - first;
            visited.push((next.to_owned(), cycle_i));
            break;
        }
        visited.push((next.to_owned(), cycle_i));
        pos = next.clone();
    }
    Ok((cycle_len as u64, visited))
}

fn parse_input(input: &str) -> Option<(Directions, Network)> {
    let mut dir = Vec::new();
    let mut lines = input.lines();
    for c in lines.next()?.chars() {
        match c {
            'L' => dir.push(0),
            'R' => dir.push(1),
            _ => return None,
        }
    }

    let mut map = HashMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let caps = MAP_RE.captures(line)?;
        let (from, to1, to2) = (
            caps[1].to_string(),
            caps[2].to_string(),
            caps[3].to_string(),
        );
        map.insert(from, (to1, to2));
    }
    Some((Directions(dir), Network(map)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(7));
    }
}
