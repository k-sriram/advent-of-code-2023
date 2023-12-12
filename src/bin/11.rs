use std::collections::HashSet;

advent_of_code::solution!(11);

#[cfg(not(test))]
const EXPANSION: usize = 1_000_000;
#[cfg(test)]
const EXPANSION: usize = 100;


pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, EXPANSION)
}

fn solve(input: &str, expansion: usize) -> Option<u64> {
    let sky = parse(input);
    let galaxies = sky.get_galaxies();
    let (erows, ecols) = sky.get_empty_rowcols();
    let mut dist_sum = 0;

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let dist = galaxies[i].expanded_dist(
                &galaxies[j],
                erows.clone(),
                ecols.clone(),
                expansion - 1,
            );
            dist_sum += dist;
        }
    }
    Some(dist_sum as u64)
}

#[derive(Debug)]
struct Sky(Vec<Vec<bool>>);

impl Sky {
    fn get_empty_rowcols(&self) -> (HashSet<usize>, HashSet<usize>) {
        let mut empty_rows = HashSet::new();
        let mut empty_cols = HashSet::new();
        for (y, row) in self.0.iter().enumerate() {
            if row.iter().all(|n| !*n) {
                empty_rows.insert(y);
            }
        }
        for (x, col) in transpose(self.0.clone()).iter().enumerate() {
            if col.iter().all(|n| !*n) {
                empty_cols.insert(x);
            }
        }
        (empty_rows, empty_cols)
    }

    fn get_galaxies(&self) -> Vec<Galaxy> {
        let mut result = Vec::new();
        for (y, row) in self.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell {
                    result.push(Galaxy((x, y)));
                }
            }
        }
        result
    }

}


#[derive(Debug)]
struct Galaxy((usize, usize));

impl Galaxy {

    fn expanded_dist(
        &self,
        other: &Galaxy,
        erows: HashSet<usize>,
        ecols: HashSet<usize>,
        expansion: usize,
    ) -> usize {
        let (x1, y1) = self.0;
        let (x2, y2) = other.0;
        let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        let xdist = x2 - x1;
        let ydist = y2 - y1;
        let xexp = ecols.intersection(&HashSet::from_iter(x1..x2)).count() * expansion;
        let yexp = erows.intersection(&HashSet::from_iter(y1..y2)).count() * expansion;
        xdist + ydist + xexp + yexp
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn parse(input: &str) -> Sky {
    Sky(input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8410));
    }
}
