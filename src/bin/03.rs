use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let engine = Engine::from_str(input);
    Some(engine.get_symbol_adjacent_numbers().iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let engine = Engine::from_str(input);
    let gears = engine.get_gears();
    let ratios = gears.iter().map(|(_, [a, b])| *a * *b).collect::<Vec<_>>();
    Some(ratios.iter().sum())
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Pos")
            .field(&self.row)
            .field(&self.col)
            .finish()
    }
}

impl Pos {
    fn neighbours(&self) -> Vec<Pos> {
        let mut result = Vec::new();
        for drow in -1..=1 {
            for dcol in -1..=1 {
                if drow == 0 && dcol == 0 {
                    continue;
                }
                let row = self.row as i32 + drow;
                let col = self.col as i32 + dcol;
                if row < 0 || col < 0 {
                    continue;
                }
                result.push(Pos {
                    row: row as usize,
                    col: col as usize,
                });
            }
        }
        result
    }
}

#[derive(Debug)]
struct Engine {
    parts: Vec<EnginePart>,
}

#[derive(Debug)]
enum EnginePart {
    Number { value: u32, poss: Vec<Pos> },
    Symbol { value: char, pos: Pos },
}

impl Engine {
    fn from_str(input: &str) -> Engine {
        let mut parts = Vec::new();
        for (row, line) in input.lines().enumerate() {
            let mut in_number = false;
            let mut cur_number = 0;
            let mut cur_number_cols = Vec::new();
            for (col, c) in line.chars().enumerate() {
                if !in_number {
                    if c == '.' {
                        continue;
                    }
                    if c.is_digit(10) {
                        in_number = true;
                        cur_number = c.to_digit(10).unwrap();
                        cur_number_cols.push(col);
                        continue;
                    }
                    parts.push(EnginePart::Symbol {
                        value: c,
                        pos: Pos { row, col },
                    });
                } else {
                    if c.is_digit(10) {
                        cur_number = cur_number * 10 + c.to_digit(10).unwrap();
                        cur_number_cols.push(col);
                        continue;
                    }
                    parts.push(EnginePart::Number {
                        value: cur_number,
                        poss: cur_number_cols
                            .into_iter()
                            .map(|col| Pos { row, col })
                            .collect(),
                    });
                    in_number = false;
                    cur_number = 0;
                    cur_number_cols = Vec::new();
                    if c != '.' {
                        parts.push(EnginePart::Symbol {
                            value: c,
                            pos: Pos { row, col },
                        });
                    }
                }
            }
            if in_number {
                parts.push(EnginePart::Number {
                    value: cur_number,
                    poss: cur_number_cols
                        .into_iter()
                        .map(|col| Pos { row, col })
                        .collect(),
                });
            }
        }
        Engine { parts }
    }

    fn get_symbol_neighbours(&self) -> HashSet<Pos> {
        let mut result = HashSet::new();
        for part in &self.parts {
            if let EnginePart::Symbol { pos, .. } = part {
                for neighbour in pos.neighbours() {
                    result.insert(neighbour);
                }
            }
        }
        result
    }

    fn get_symbol_adjacent_numbers(&self) -> Vec<u32> {
        let mut result = Vec::new();
        let neighbours = self.get_symbol_neighbours();
        for part in &self.parts {
            if let EnginePart::Number { value, poss } = part {
                if poss.iter().any(|pos| neighbours.contains(pos)) {
                    result.push(*value);
                }
            }
        }
        result
    }

    fn get_adjacent_numbers(&self, pos: Pos) -> Vec<u32> {
        let mut result = Vec::new();
        let neighbour_pos = pos.neighbours();
        for part in &self.parts {
            if let EnginePart::Number { value, poss } = part {
                if poss.iter().any(|pos| neighbour_pos.contains(pos)) {
                    result.push(*value);
                }
            }
        }
        result
    }

    fn get_gears(&self) -> Vec<(Pos, [u32; 2])> {
        let mut result = Vec::new();
        for part in &self.parts {
            if let EnginePart::Symbol { value: '*', pos } = part {
                let neighbours = &self.get_adjacent_numbers(pos.clone());
                if neighbours.len() == 2 {
                    result.push((pos.clone(), [neighbours[0], neighbours[1]]));
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
