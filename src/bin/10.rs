use std::collections::HashSet;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut pipes, start) = parse(input);
    trim_connections(&mut pipes);
    let loop_ = get_loop(&pipes, start);
    let max_dist = (loop_.len() as u32) / 2;
    Some(max_dist)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut pipes, start) = parse(input);
    trim_connections(&mut pipes);
    let loop_ = get_loop(&pipes, start);
    let size = (pipes[0].len(), pipes.len());
    let outer = get_outer(&loop_, &pipes);
    print_loop(&input, &loop_, &outer, size);
    let area = (size.0) * (size.1);
    println!("{} {} {}", area, loop_.len(), outer.len());
    Some((area - loop_.len() - outer.len()) as u32)
}

struct Pipe {
    x: usize,
    y: usize,
    connections: Vec<(usize, usize)>,
}

impl std::fmt::Debug for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Pipe")
            .field(&(self.x, self.y))
            .field(&self.connections)
            .finish()
    }
}

fn get_outer(loop_: &[(usize, usize)], pipes: &Vec<Vec<Pipe>>) -> HashSet<(usize, usize)> {
    let start = (0, 0);
    let size = (pipes[0].len(), pipes.len());
    let outer_verts = df_fill(pipes, start, size);
    let mut outer = HashSet::new();
    for (i, j) in outer_verts {
        for (di, dj) in &[(0, 0), (1, 0), (0, 1), (1, 1)] {
            if !loop_.contains(&(i + di, j + dj)) {
                outer.insert((i + di, j + dj));
            }
        }
    }
    outer
}

fn df_fill(
    pipes: &Vec<Vec<Pipe>>,
    start: (usize, usize),
    size: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut visited = Vec::new();
    let mut stack = Vec::new();
    stack.push(start);
    while let Some(current) = stack.pop() {
        visited.push(current);
        for neighbour in get_neighbours(pipes, current, size) {
            if visited.contains(&neighbour) {
                continue;
            }
            stack.push(neighbour);
        }
    }
    visited
}

fn get_neighbours(
    pipes: &Vec<Vec<Pipe>>,
    current: (usize, usize),
    size: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    let (i, j) = current;
    // Up

    if j > 0
        && !(pipes[j][i].connections.contains(&(i + 1, j))
            && pipes[j][i + 1].connections.contains(&(i, j)))
    {
        neighbours.push((i, j - 1));
    }
    // Down
    if j < size.1 - 2
        && !(pipes[j + 1][i].connections.contains(&(i + 1, j + 1))
            && pipes[j + 1][i + 1].connections.contains(&(i, j + 1)))
    {
        neighbours.push((i, j + 1));
    }
    // Left
    if i > 0
        && !(pipes[j][i].connections.contains(&(i, j + 1))
            && pipes[j + 1][i].connections.contains(&(i, j)))
    {
        neighbours.push((i - 1, j));
    }
    // Right
    if i < size.0 - 2
        && !(pipes[j][i + 1].connections.contains(&(i + 1, j + 1))
            && pipes[j + 1][i + 1].connections.contains(&(i + 1, j)))
    {
        neighbours.push((i + 1, j));
    }
    neighbours
}

fn get_loop(pipes: &[Vec<Pipe>], start: (usize, usize)) -> Vec<(usize, usize)> {
    let mut loop_ = Vec::new();
    loop_.push(start);
    let mut current = start;
    if pipes[current.1][current.0].connections.len() != 2 {
        panic!("Invalid start");
    }
    let mut last = current;
    current = pipes[current.1][current.0].connections[0];
    while current != start {
        loop_.push(current);
        let next = pipes[current.1][current.0]
            .connections
            .iter()
            .filter(|(x, y)| *x != last.0 || *y != last.1)
            .next()
            .unwrap()
            .clone();
        last = current;
        current = next;
    }
    loop_
}

fn trim_connections(pipes: &mut Vec<Vec<Pipe>>) {
    let (rows, cols) = (pipes.len(), pipes[0].len());
    for j in 0..rows {
        for i in 0..cols {
            let mut connections = Vec::new();
            for (x, y) in pipes[j][i].connections.iter() {
                if pipes[*y][*x].connections.contains(&(i, j)) {
                    connections.push((*x, *y));
                }
            }
            pipes[j][i].connections = connections;
        }
    }
}

fn parse(input: &str) -> (Vec<Vec<Pipe>>, (usize, usize)) {
    let mut pipes = Vec::new();
    let mut start = (0, 0);
    pipes.push(Vec::new());
    for (j, line) in input.lines().enumerate() {
        let mut pipe_row = vec![Pipe {
            x: 0,
            y: j + 1,
            connections: Vec::new(),
        }];
        for (i, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    pipe_row.push(Pipe {
                        x: i + 1,
                        y: j + 1,
                        connections: Vec::new(),
                    });
                }
                '|' => {
                    pipe_row.push(Pipe {
                        x: i + 1,
                        y: j + 1,
                        connections: vec![(i + 1, j), (i + 1, j + 2)],
                    });
                }
                '-' => {
                    pipe_row.push(Pipe {
                        x: i + 1,
                        y: j + 1,
                        connections: vec![(i, j + 1), (i + 2, j + 1)],
                    });
                }
                'L' => {
                    pipe_row.push(Pipe {
                        x: i + 1,
                        y: j + 1,
                        connections: vec![(i + 1, j), (i + 2, j + 1)],
                    });
                }
                'J' => {
                    pipe_row.push(Pipe {
                        x: i + 1,
                        y: j + 1,
                        connections: vec![(i + 1, j), (i, j + 1)],
                    });
                }
                '7' => {
                    pipe_row.push(Pipe {
                        x: i + 1,
                        y: j + 1,
                        connections: vec![(i + 1, j + 2), (i, j + 1)],
                    });
                }
                'F' => {
                    pipe_row.push(Pipe {
                        x: i + 1,
                        y: j + 1,
                        connections: vec![(i + 1, j + 2), (i + 2, j + 1)],
                    });
                }
                'S' => {
                    start = (i + 1, j + 1);
                    pipe_row.push(Pipe {
                        x: i + 1,
                        y: j + 1,
                        connections: vec![(i + 1, j + 2), (i + 2, j + 1), (i + 1, j), (i, j + 1)],
                    });
                }
                _ => {
                    unreachable!("Invalid character: {}", c)
                }
            }
        }
        pipe_row.push(Pipe {
            x: line.len() + 1,
            y: j + 1,
            connections: Vec::new(),
        });
        pipes.push(pipe_row);
    }
    let rowlen = pipes[1].len();
    pipes[0] = (0..rowlen)
        .map(|i| Pipe {
            x: i,
            y: 0,
            connections: Vec::new(),
        })
        .collect();
    pipes.push(
        (0..rowlen)
            .map(|i| Pipe {
                x: i,
                y: pipes.len(),
                connections: Vec::new(),
            })
            .collect(),
    );
    if start == (0, 0) {
        panic!("No start found");
    }
    (pipes, start)
}

fn print_loop(
    input: &str,
    loop_: &[(usize, usize)],
    outer: &HashSet<(usize, usize)>,
    size: (usize, usize),
) {
    let input_vec = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut grid = vec![vec![' '; size.0 - 2]; size.1 - 2];
    let mut i_counter = 0;
    for i in 1..size.0 - 1 {
        for j in 1..size.1 - 1 {
            if outer.contains(&(i, j)) {
                grid[j - 1][i - 1] = 'O';
            } else if loop_.contains(&(i, j)) {
                grid[j - 1][i - 1] = input_vec[j - 1][i - 1];
            } else {
                i_counter += 1;
                grid[j - 1][i - 1] = 'I';
            }
        }
    }
    for line in grid {
        println!("{}", line.iter().collect::<String>());
    }
    println!("{}", i_counter);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(4));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(10));
    }
}
