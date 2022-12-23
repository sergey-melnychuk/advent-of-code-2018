use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Grid {
    chars: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn make(lines: Vec<String>) -> Grid {
        let rows = lines.len();
        let cols = lines.len();
        let chars = lines.into_iter().map(|l| l.chars().collect()).collect();
        Grid { chars, rows, cols }
    }

    fn dump(&self) -> Vec<String> {
        let chars = self.chars.clone();
        chars.into_iter().map(|cs| cs.iter().collect()).collect()
    }

    fn count<F>(&self, f: F) -> usize
    where
        F: Fn(char) -> bool,
    {
        let mut acc = 0;
        for row in &self.chars {
            for c in row {
                if f(*c) {
                    acc += 1;
                }
            }
        }
        acc
    }

    fn set(&mut self, x: usize, y: usize, c: char) {
        *self.chars.get_mut(y).unwrap().get_mut(x).unwrap() = c;
    }

    fn get(&self, x: usize, y: usize) -> char {
        *self.chars.get(y).unwrap().get(x).unwrap()
    }

    fn adj(&self, x: isize, y: isize) -> Vec<char> {
        let cells = vec![
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            /*skip*/ (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];
        let (rs, cs) = (self.rows as isize, self.cols as isize);
        cells
            .into_iter()
            .filter_map(|p| {
                let (x, y) = p;
                if x >= 0 && y >= 0 && x < cs && y < rs {
                    Some(p)
                } else {
                    None
                }
            })
            .map(|p| {
                let (x, y) = p;
                self.get(x as usize, y as usize)
            })
            .collect()
    }
}

// Each acre can be either open ground (.), trees (|), or a lumberyard (#).
// Here, "adjacent" means any of the eight acres surrounding that acre.

/*
An open acre will become filled with trees if three or more adjacent acres contained trees.
Otherwise, nothing happens.

An acre filled with trees will become a lumberyard if three or more adjacent acres were lumberyards.
Otherwise, nothing happens.

An acre containing a lumberyard will remain a lumberyard if it was adjacent to at least one other
lumberyard and at least one acre containing trees. Otherwise, it becomes open.
*/

fn count(grid: &Grid, x: usize, y: usize) -> (usize, usize, usize) {
    let adj = grid.adj(x as isize, y as isize);
    let gr = adj.iter().filter(|c| **c == '.').count();
    let tr = adj.iter().filter(|c| **c == '|').count();
    let ly = adj.iter().filter(|c| **c == '#').count();
    (gr, tr, ly)
}

fn mutate(grid: &Grid, x: usize, y: usize) -> char {
    let c = grid.get(x, y);
    match (c, count(grid, x, y)) {
        ('.', (_gr, tr, _ly)) if tr >= 3 => '|',
        ('|', (_gr, _tr, ly)) if ly >= 3 => '#',
        ('#', (_gr, tr, ly)) if ly >= 1 && tr >= 1 => '#',
        ('#', (_gr, _tr, _ly)) => '.',
        _ => c,
    }
}

fn simulate(this: Grid) -> Grid {
    let mut next = this.clone();
    for y in 0..this.rows {
        for x in 0..this.cols {
            let _c = this.get(x, y);
            let m = mutate(&this, x, y);
            next.set(x, y, m);
        }
    }
    next
}

fn iterate(this: Grid, k: usize) -> Grid {
    let mut seen: HashMap<Grid, usize> = HashMap::new();
    seen.insert(this.clone(), 0);

    let mut next = this;
    let mut i = 0;
    while i < k {
        //println!("iteration: {}", i);

        let temp = simulate(next);
        next = temp;

        if seen.contains_key(&next) {
            let p = *seen.get(&next).unwrap();
            //println!("loop detected: i={} p={}", i, p);
            let period = i - p;

            let full_periods = (k - i + 1) / period;
            i += full_periods * period;
        } else {
            seen.insert(next.clone(), i);
        }

        i += 1;
    }
    next
}

fn get_input() -> Vec<String> {
    let stdin = io::stdin();
    let lines = stdin
        .lock()
        .lines()
        .into_iter()
        .map(Result::unwrap)
        .collect();
    lines
}

pub fn main() {
    let grid = Grid::make(get_input());
    //println!("\n{}", grid.dump().join("\n"));

    let n = 10;
    let next = iterate(grid.clone(), n);
    //println!("\n{}", next.dump().join("\n"));
    {
        let tr = next.count(|c| c == '|');
        let ly = next.count(|c| c == '#');
        println!("{}", tr * ly); // 637550
    }

    let k = 1000000000;
    let last = iterate(grid, k);
    {
        let tr = last.count(|c| c == '|');
        let ly = last.count(|c| c == '#');
        println!("{}", tr * ly); // 201465
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wrap(lines: Vec<&'static str>) -> Vec<String> {
        lines.into_iter().map(String::from).collect()
    }

    struct Data {
        s0: Vec<String>,
        s1: Vec<String>,
    }

    fn make_data() -> Data {
        Data {
            s0: wrap(vec![
                ".#.#...|#.",
                ".....#|##|",
                ".|..|...#.",
                "..|#.....#",
                "#.#|||#|#|",
                "...#.||...",
                ".|....|...",
                "||...#|.#|",
                "|.||||..|.",
                "...#.|..|.",
            ]),
            s1: wrap(vec![
                ".......##.",
                "......|###",
                ".|..|...#.",
                "..|#||...#",
                "..##||.|#|",
                "...#||||..",
                "||...|||..",
                "|||||.||.|",
                "||||||||||",
                "....||..|.",
            ]),
        }
    }

    #[test]
    fn test_iter1() {
        let data = make_data();
        let grid = Grid::make(data.s0);
        let next = simulate(grid);
        assert_eq!(next.dump(), data.s1);
    }
}
