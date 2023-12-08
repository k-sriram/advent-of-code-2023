use std::collections::HashMap;

use indicatif::ProgressIterator;
use lazy_static::lazy_static;
use regex::Regex;

advent_of_code::solution!(5);

lazy_static! {
    static ref MAP_START_RE: Regex = Regex::new(r"(\w+)-to-(\w+) map:").unwrap();
}
const DESTINATION: &str = "location";

pub fn part_one(input: &str) -> Option<i64> {
    let ((source_type, sources), maps) = parse_input(input);
    println!("Calculating {} destinations", sources.len());

    Some(
        sources
            .iter()
            .map(|s| maps.destination(*s, &source_type, DESTINATION).unwrap())
            .min()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let ((source_type, sources), maps) = parse_input(input);

    let mut num_sources = 0;
    for chunk in sources.chunks(2) {
        num_sources += chunk[1];
    }
    println!("Calculating {} destinations", num_sources);
    
    let mut new_source = Vec::new();
    for chunk in sources.chunks(2) {
        let (s, r) = (chunk[0], chunk[1]);
        for i in s..s+r {
            new_source.push(i);
        }
    }
    let sources = new_source;
    let num_sources = sources.len() as u64;


    Some(
        sources
            .iter()
            .progress_count(num_sources)
            .map(|s| maps.destination(*s, &source_type, DESTINATION).unwrap())
            .min()
            .unwrap(),
    )
}

struct Map {
    maps: Vec<(i64, i64, i64)>,
}

impl Map {
    fn calc_destination(&self, source: i64) -> i64 {
        for (dest_start, source_start, range) in &self.maps {
            if &source >= source_start && &source < &(source_start + range) {
                return dest_start + (source - source_start);
            }
        }
        source
    }

    // fn combine(&self, other: &Map) -> Map {
    //     let mut breakpoints = Vec::new();
    //     for (ods, oss, or) in &other.maps {
    //         breakpoints.push(*oss);
    //         breakpoints.push(*(oss + or));
    //     }

    //     let mut maps = Vec::new();
    //     for (ds, ss, r) in &self.maps {
            
    //     }
    // }
}

struct Maps {
    maps: HashMap<String, (String, Map)>,
}

impl Maps {
    fn destination(&self, mut source: i64, source_type: &str, dest_type: &str) -> Option<i64> {
        let mut source_type = source_type.to_string();
        loop {
            if !&self.maps.contains_key(&source_type) {
                return None;
            }
            let (next_source_type, map) = &self.maps[&source_type];
            source = map.calc_destination(source);
            source_type = next_source_type.clone();

            if source_type == dest_type {
                return Some(source);
            }
        }
    }
}

fn parse_input(input: &str) -> ((String, Vec<i64>), Maps) {
    let lines_vec = &input.lines().collect::<Vec<_>>();
    let (first, lines) = lines_vec.split_first().unwrap();
    let mut source_parts = first.split("s:");
    let source_type = source_parts.next().unwrap().to_string();
    let sources = source_parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i64>>();

    // Parse the maps
    let mut maps = HashMap::new();
    let mut in_map = false;
    let mut map_source = String::new();
    let mut map_dest = String::new();
    let mut map = Vec::new();
    for line in lines {
        if in_map {
            if line.is_empty() {
                in_map = false;
                maps.insert(map_source, (map_dest, Map { maps: map }));
                map_source = String::new();
                map_dest = String::new();
                map = Vec::new();
                continue;
            }
            let m = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>();
            map.push((m[0], m[1], m[2]));
        } else {
            if line.is_empty() {
                continue;
            }
            let caps = MAP_START_RE.captures(line).unwrap();
            map_source = caps.get(1).unwrap().as_str().to_string();
            map_dest = caps.get(2).unwrap().as_str().to_string();
            in_map = true;
        }
    }
    if in_map {
        maps.insert(map_source, (map_dest, Map { maps: map }));
    }

    ((source_type, sources), Maps { maps })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
