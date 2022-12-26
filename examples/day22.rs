use std::collections::{HashMap, VecDeque};

pub fn main() {
    let mut grid = Grid::new(11817, (9, 751));
    // let mut grid = Grid::new(510, (10, 10)); // test example

    let part1 = sum(&mut grid);
    println!("{}", part1);

    let time = bfs(&mut grid);
    println!("{}", time);
}

struct Grid {
    depth: i64,
    target: (i64, i64),
    cache: HashMap<(i64, i64), u64>,
    cells: HashMap<(i64, i64), char>,
}

impl Grid {
    fn new(depth: i64, target: (i64, i64)) -> Self {
        let mut cache = HashMap::new();
        cache.insert(target, 0);
        Self {
            depth,
            target,
            cache,
            cells: HashMap::new(),
        }
    }

    fn geo(&mut self, x: i64, y: i64) -> u64 {
        if let Some(ret) = self.cache.get(&(x, y)) {
            return *ret;
        }

        let geo_idx = match (x, y) {
            (0, 0) => 0,
            (x, 0) => x as u64 * 16807,
            (0, y) => y as u64 * 48271,
            _ => {
                let a = (self.geo(x, y - 1) + self.depth as u64) % 20183;
                let b = (self.geo(x - 1, y) + self.depth as u64) % 20183;
                a * b
            }
        };

        self.cache.insert((x, y), geo_idx);
        geo_idx
    }

    fn cell(&mut self, x: i64, y: i64) -> char {
        if let Some(cell) = self.cells.get(&(x, y)) {
            return *cell;
        }

        //println!("x={} y={}, geo: {}", x, y, geo_idx);
        let level = (self.geo(x, y) + self.depth as u64) % 20183;
        let cell = match level % 3 {
            0 => '.', // rocky
            1 => '=', // wet
            2 => '|', // narrow
            x => panic!("X % 3 not in [0, 1, 2]! {} % 3 = {}.", level, x),
        };

        self.cells.insert((x, y), cell);
        cell
    }
}

type State = (u8, u32, (i64, i64)); // tool, time, (x, y)

fn bfs(grid: &mut Grid) -> u32 {
    let state: State = (2, 0, (0, 0));

    let mut queue = VecDeque::new();
    queue.push_back(state);

    let mut len: HashMap<((i64, i64), u8), u32> = HashMap::new();
    len.insert(((0, 0), 2), 0);

    let mut best = u32::MAX;

    while !queue.is_empty() {
        let (tool, time, (x, y)) = queue.pop_front().unwrap();

        if time > best {
            continue;
        }

        if time > len.get(&((x, y), tool)).copied().unwrap_or(u32::MAX) {
            continue;
        }

        if (x, y) == grid.target && tool == 2 {
            let time = time + 7; // final switch to torch
            best = best.min(time);
            //println!("best: {}, q: {}", best, queue.len());
            continue;
        }

        for next in adj(x, y) {
            let cell = grid.cell(next.0, next.1);

            for (tool, time, next) in steps((tool, time, next), cell) {
                if time < len.get(&(next, tool)).copied().unwrap_or(u32::MAX) {
                    len.insert((next, tool), time);
                    queue.push_back((tool, time, next));
                }
            }
        }
    }

    best
}

// "climbing gear" = 1, "torch" = 2, "neither" = 0

// rocky '.': "climbing gear" 1 or "torch" 2
// wet '=': "climbing gear" 1 or "neither" 0
// narrow '|': "torch" 2 or "neither" 0

fn steps(state: State, cell: char) -> Vec<State> {
    let (tool, time, next) = state;
    match (cell, tool) {
        // In rocky regions, you can use the climbing gear or the torch.
        // You cannot use neither (you'll likely slip and fall).
        ('.', 2) => vec![(1, time + 1 + 7, next), (2, time + 1, next)],
        ('.', 1) => vec![(1, time + 1, next), (2, time + 1 + 7, next)],
        ('.', 0) => vec![(1, time + 1 + 7, next), (2, time + 1 + 7, next)],

        // In wet regions, you can use the climbing gear or neither tool.
        // You cannot use the torch (if it gets wet, you won't have a light source).
        ('=', 2) => vec![(0, time + 1 + 7, next), (1, time + 1 + 7, next)],
        ('=', 1) => vec![(1, time + 1, next), (0, time + 1 + 7, next)],
        ('=', 0) => vec![(0, time + 1, next), (1, time + 1 + 7, next)],

        // In narrow regions, you can use the torch or neither tool.
        // You cannot use the climbing gear (it's too bulky to fit).
        ('|', 2) => vec![(0, time + 1 + 7, next), (2, time + 1, next)],
        ('|', 1) => vec![(0, time + 1 + 7, next), (2, time + 1 + 7, next)],
        ('|', 0) => vec![(0, time + 1, next), (2, time + 1 + 7, next)],

        _ => unreachable!(),
    }
}

fn adj(x: i64, y: i64) -> Vec<(i64, i64)> {
    let mut ret = Vec::with_capacity(4);
    if x - 1 >= 0 {
        ret.push((x - 1, y));
    }
    if y - 1 >= 0 {
        ret.push((x, y - 1));
    }
    ret.push((x + 1, y));
    ret.push((x, y + 1));
    ret
}

fn sum(grid: &mut Grid) -> u32 {
    let mut sum = 0;
    for x in 0..=grid.target.0 {
        for y in 0..=grid.target.1 {
            let x = match grid.cell(x as i64, y as i64) {
                '.' => 0,
                '=' => 1,
                '|' => 2,
                _ => unreachable!(),
            };
            sum += x;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let mut grid = Grid::new(510, (10, 10));
        assert_eq!(sum(&mut grid), 114);
    }
}
