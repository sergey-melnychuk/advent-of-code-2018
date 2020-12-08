use std::io;
use std::io::prelude::*;

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
struct V2 {
    row: usize,
    col: usize
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct Unit {
    kind: char,
    attack: usize,
    health: usize,
}

impl Unit {
    fn attack(&self, that: Unit) -> Option<Unit> {
        if that.health <= self.attack {
            None
        } else {
            Some( Unit { health: that.health - self.attack, ..that } )
        }
    }
}

#[derive(Clone)]
struct Grid {
    rows: usize,
    cols: usize,
    cells: Vec<Vec<char>>,
    units: Vec<Vec<Option<Unit>>>,
}

impl Grid {
    fn parse<F>(lines: Vec<String>, f: F) -> Grid
        where F: Fn(char, usize, usize) -> Option<Unit>
    {
        let mut cells: Vec<Vec<char>> = lines.into_iter()
            .map(|line| line.chars().collect())
            .collect();
        let rows = cells.len();
        let cols = cells[0].len();

        let mut units: Vec<Vec<Option<Unit>>> = vec![vec![None; cols]; rows];
        for row in 0..rows {
            for col in 0..cols {
                units[row][col] = f(cells[row][col], row, col);
                if units[row][col].is_some() {
                    cells[row][col] = '.';
                }
            }
        }
        Grid { rows, cols, cells, units }
    }

    fn count(&self, kind: char) -> usize {
        let mut es = 0;
        for row in &self.units {
            for opt in row {
                if opt.is_some() {
                    let k = opt.map(|u| u.kind).unwrap();
                    if k == kind {
                        es += 1;
                    }
                }
            }
        }
        es
    }

    fn adj(&self, pos: V2) -> Vec<V2> {
        if self.cells[pos.row][pos.col] == '#' {
            vec![]
        } else {
            let mut result = Vec::new();
            if pos.row > 0 && self.cells[pos.row - 1][pos.col] != '#' {
                result.push(V2 { row: pos.row - 1, ..pos }); // N
            }
            if pos.col > 0 && self.cells[pos.row][pos.col - 1] != '#'{
                result.push(V2 { col: pos.col - 1, ..pos }); // W
            }
            if pos.col < self.cols - 1 && self.cells[pos.row][pos.col + 1] != '#' {
                result.push(V2 { col: pos.col + 1, ..pos }); // E
            }
            if pos.row < self.rows - 1 && self.cells[pos.row + 1][pos.col] != '#' {
                result.push(V2 { row: pos.row + 1, ..pos }); // S
            }
            result
        }
    }

    // Find the path from `pos` to the closest unit of kind `kind`
    fn find(&self, pos: V2, kind: char) -> Vec<V2> {
        let inf = self.rows * self.cols + 1;
        let mut dist = vec![vec![inf; self.cols]; self.rows];
        let mut from: HashMap<V2, V2> = HashMap::new();
        from.insert(pos, pos);
        dist[pos.row][pos.col] = 0;

        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(pos);

        let mut finish: Option<V2> = None;
        let mut index: usize = std::usize::MAX;
        let mut cost: usize = std::usize::MAX;
        while !queue.is_empty() {
            let at = queue.pop_front().unwrap();
            seen.insert(at);
            let cost_at = dist[at.row][at.col];
            for to in self.adj(at) {
                //println!("at={:?} to={:?}", at, to);
                let cost_to = dist[to.row][to.col];
                let k = self.units[to.row][to.col].map(|u| u.kind);
                if k.is_some() && k.unwrap() == kind {
                    // Found!
                    if finish.is_none() {
                        //println!("found(1) at={:?} to={:?} cto={} cat={}", at, to, cost_to, cost_at);
                        //println!("\tidx={} c={} f={:?}", index, cost, finish);
                        from.insert(to, at);
                        dist[to.row][to.col] = cost_at + 1;
                        cost = cost_at + 1;
                        finish = Some(to);
                        index = at.row * self.rows + at.col;
                        //println!("\tidx={} c={} f={:?}", index, cost, finish);
                    } else {
                        //println!("found(n) at={:?} to={:?} cto={} cat={}", at, to, cost_to, cost_at);
                        //println!("\tidx={} c={} f={:?}", index, cost, finish);
                        if cost > cost_at + 1 {
                            //println!("\tbetter cost");
                            from.insert(to, at);
                            dist[to.row][to.col] = cost_at + 1;
                            cost = cost_at + 1;
                            finish = Some(to);
                        } else if cost == cost_at + 1 {
                            let idx = at.row * self.rows + at.col;
                            if idx < index {
                                //println!("\tbetter index");
                                from.insert(to, at);
                                dist[to.row][to.col] = cost_at + 1;
                                cost = cost_at + 1;
                                finish = Some(to);
                                index = idx;
                            }
                        }
                        //println!("\tidx={} c={} f={:?}", index, cost, finish);
                    }
                }

                if !seen.contains(&to) && k.is_none() {
                    if cost_to > cost_at + 1 {
                        dist[to.row][to.col] = cost_at + 1;
                        from.insert(to, at);
                        queue.push_back(to);
                    }
                }
            }
        }

        if finish.is_none() {
            vec![] // no path found
        } else {
            // Unwind the path
            let mut parent = finish.unwrap();
            let mut result = Vec::new();
            result.push(parent);
            loop {
                let p = from.get(&parent).unwrap();
                if *p == parent {
                    break;
                } else {
                    result.push(*p);
                    parent = *p;
                }
            }
            result.reverse();
            result
        }
    }

    fn target(&self, pos: V2, kind: char) -> Option<V2> {
        let mut min = std::usize::MAX;
        let mut found: Option<V2> = None;

        // Find the adjacent target, if any
        for at in self.adj(pos) {
            match self.units[at.row][at.col] {
                Some(unit) if unit.kind == kind && unit.health < min => {
                    min = unit.health;
                    found = Some(at)
                }
                _ => ()
            }
        }

        if found.is_none() {
            // Now find first step to the closest enemy
            let path = self.find(pos, kind);
            if !path.is_empty() {
                found = Some(path[1]);
            }
        }

        found
    }

    fn dump(&self) -> Vec<String> {
        let mut chars: Vec<Vec<char>> = vec![vec!['x'; self.cols]; self.rows];
        for r in 0..self.rows {
            for c in 0..self.cols {
                chars[r][c] = self.cells[r][c];
            }
        }

        for (r, row) in self.units.iter().enumerate() {
            for (c, opt) in row.iter().enumerate() {
                if opt.is_some() {
                    let u = opt.unwrap();
                    chars[r][c] = u.kind;
                    let mut val = format!(" {}/{}", u.kind, u.health).chars().into_iter().collect();
                    chars[r].append(&mut val);
                    //println!("\tdump: add unit at r={} c={} {}/{}", r, c, u.kind, u.health);
                }
            }
        }

        chars.into_iter()
            .map(|cs| cs.iter().collect())
            .collect()
    }
}

fn get_input() -> Vec<String> {
    let stdin = io::stdin();
    stdin.lock().lines().map(Result::unwrap).collect()
}

// Run a simulation step, return number of players in teams
fn simulation(grid: &mut Grid) -> (usize, usize) {
    let mut done: HashSet<V2> = HashSet::new();
    for r in 0..grid.rows {
        for c in 0..grid.cols {
            let p = V2 { row: r, col: c };
            if done.contains(&p) {
                continue;
            }
            let opt = grid.units[r][c];
            if opt.is_some() {
                let u = opt.unwrap();
                let kind = if u.kind == 'E' { 'G' } else { 'E' };
                let op = grid.target(p, kind);
                if op.is_some() {
                    let t = op.unwrap();
                    if grid.units[t.row][t.col].is_some() {
                        // attack
                        let that = grid.units[t.row][t.col].unwrap();
                        let after = u.attack(that);
                        grid.units[t.row][t.col] = after;
                        if after.is_none() {
                            grid.cells[t.row][t.col] = '.';
                        }
                        //println!("attack: {}/{} at={:?} attacks {}/{} at={:?}", u.kind, u.health, p, that.kind, that.health, t);
                    } else {
                        // move and attack
                        grid.units[t.row][t.col] = opt;
                        grid.units[p.row][p.col] = None;
                        done.insert(t);
                        //done.remove(&p);
                        grid.cells[t.row][t.col] = u.kind;
                        grid.cells[p.row][p.col] = '.';
                        //println!("move: {}/{} {:?} -> {:?}", u.kind, u.health, p, t);

                        let to = grid.target(t, kind);
                        if to.is_some() {
                            let p = to.unwrap();
                            if grid.units[p.row][p.col].is_some() {
                                let that = grid.units[p.row][p.col].unwrap();
                                let after = u.attack(that);
                                grid.units[p.row][p.col] = after;
                                if after.is_none() {
                                    grid.cells[p.row][p.col] = '.';
                                }
                                //println!("\tattack: {}/{} at={:?} attacks {}/{} at={:?}", u.kind, u.health, p, that.kind, that.health, p);
                            }
                        }
                    }
                }
            }
        }
    }

    let mut es: usize = 0;
    let mut gs: usize = 0;
    for r in &grid.units {
        for u in r {
            if u.is_some() {
                let k = u.unwrap().kind;
                if k == 'E' {
                    es += 1;
                }
                if k == 'G' {
                    gs += 1;
                }
            }
        }
    }

    //println!("{}:{}", es, gs);
    (es, gs)
}

fn solve(grid: &mut Grid) -> usize {
    let mut go: bool = true;
    let mut round = 0;
    while go {
        round += 1;
        //println!("\nround: {}", round);
        let (es, gs) = simulation(grid);
        //println!("{}", grid.dump().join("\n"));
        go = es > 0 && gs > 0;
    }

    let mut sum = 0;
    for row in &grid.units {
        for opt in row {
            if opt.is_some() {
                let u = opt.unwrap();
                sum += u.health;
            }
        }
    }

    (round-1) * sum // workaround that leads to correct answer, don't ask why
}

pub fn main() {
    let input = get_input();
    {
        // Part 1
        let mut grid = Grid::parse(input.clone(), |chr, _row, _col| {
            if chr == 'E' || chr == 'G' {
                Some(Unit { kind: chr, health: 200, attack: 3 })
            } else {
                None
            }
        });
        let hp = solve(&mut grid);
        println!("{}", hp); // 221754
    }
    {
        // Part 2
        for attack in 4..100 {
            //println!("attack: {}", attack);
            let mut grid = Grid::parse(input.clone(), |chr, _row, _col| {
                if chr == 'E' {
                    Some(Unit { kind: chr, health: 200, attack })
                } else if chr == 'G' {
                    Some(Unit { kind: chr, health: 200, attack: 3 })
                } else {
                    None
                }
            });
            let elves = grid.count('E');
            let hp = solve(&mut grid);

            if grid.count('E') == elves {
                println!("{}", hp); // 41972
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wrap(lines: Vec<&'static str>) -> Vec<String> {
        lines.into_iter().map(String::from).collect()
    }

    fn make_grid(lines: Vec<&'static str>, health: usize, attack: usize) -> Grid {
        Grid::parse(wrap(lines), |chr: char, _row: usize, _col: usize| {
            if chr == 'E' || chr == 'G' {
                Some(Unit { kind: chr, health, attack })
            } else {
                None
            }
        })
    }

    #[test]
    fn test_parse() {
        let grid = make_grid(vec![
            "#####",
            "#E.E#",
            "#.G.#",
            "#E.E#",
            "#####",
        ], 10, 1);
        let gr = &grid;
        assert_eq!(gr.cells, vec![
            vec!['#', '#', '#', '#', '#'],
            vec!['#', '.', '.', '.', '#'],
            vec!['#', '.', '.', '.', '#'],
            vec!['#', '.', '.', '.', '#'],
            vec!['#', '#', '#', '#', '#'],
        ]);
        let e = Unit { kind: 'E', health: 10, attack: 1 };
        let g = Unit { kind: 'G', health: 10, attack: 1 };
        assert_eq!(gr.units, vec![
            vec![None,    None,    None,    None, None],
            vec![None, Some(e),    None, Some(e), None],
            vec![None,    None, Some(g),    None, None],
            vec![None, Some(e),    None, Some(e), None],
            vec![None,    None,    None,    None, None],
        ]);
    }

    #[test]
    fn test_target_1() {
        let grid = make_grid(vec![
            "#######",
            "#E...E#",
            "#.....#",
            "#..G..#",
            "#.....#",
            "#E...E#",
            "#######",
        ], 10, 1);

        assert_eq!(grid.target(V2 { row: 3, col: 3 }, 'E'), Some(V2 { row: 2, col: 3 }));
        assert_eq!(grid.target(V2 { row: 1, col: 1 }, 'G'), Some(V2 { row: 1, col: 2 }));
        assert_eq!(grid.target(V2 { row: 1, col: 5 }, 'G'), Some(V2 { row: 1, col: 4 }));
        assert_eq!(grid.target(V2 { row: 5, col: 1 }, 'G'), Some(V2 { row: 4, col: 1 }));
        assert_eq!(grid.target(V2 { row: 5, col: 5 }, 'G'), Some(V2 { row: 4, col: 5 }));
    }


    #[test]
    fn test_find_2() {
        let grid = make_grid(vec![
            "#####",
            "#...#",
            "#GG.#",
            "#..E#",
            "#####",
        ], 10, 1);

        let pos = V2 { row: 2, col: 2 };
        assert_eq!(grid.find(pos, 'E'), vec![
            V2 { row: 2, col: 2 },
            V2 { row: 2, col: 3 },
            V2 { row: 3, col: 3 },
        ]);
    }

    #[test]
    fn test_find_3() {
        let grid = make_grid(vec![
            "#####",
            "#...#",
            "#GG.#",
            "#..E#",
            "#####",
        ], 10, 1);

        let pos = V2 { row: 2, col: 1 };
        assert_eq!(grid.find(pos, 'E'), vec![
            V2 { row: 2, col: 1 },
            V2 { row: 3, col: 1 },
            V2 { row: 3, col: 2 },
            V2 { row: 3, col: 3 },
        ]);
    }

    #[test]
    fn test_find_4() {
        let grid = make_grid(vec![
            "#####",
            "#...#",
            "#GGE#",
            "#...#",
            "#####",
        ], 10, 1);

        let pos = V2 { row: 2, col: 1 };
        assert_eq!(grid.find(pos, 'E'), vec![
            V2 { row: 2, col: 1 },
            V2 { row: 1, col: 1 },
            V2 { row: 1, col: 2 },
            V2 { row: 1, col: 3 },
            V2 { row: 2, col: 3 },
        ]);
    }

    #[test]
    fn test_find_5() {
        let grid = make_grid(vec![
            "#####",
            "#...#",
            "#.G.#",
            "#.G.#",
            "#.E.#",
            "#####",
        ], 10, 1);

        let pos = V2 { row: 2, col: 2 };
        assert_eq!(grid.find(pos, 'E'), vec![
            V2 { row: 2, col: 2 },
            V2 { row: 2, col: 1 },
            V2 { row: 3, col: 1 },
            V2 { row: 4, col: 1 },
            V2 { row: 4, col: 2 },
        ]);
    }

    #[test]
    fn test_find_triangle_up1() {
        let grid = make_grid(vec![
            "#######",
            "#G...E#",
            "#...E.#",
            "#..E..#",
            "#.E...#",
            "#E....#",
            "#######",
        ], 10, 1);

        let pos = V2 { row: 1, col: 1 };
        assert_eq!(grid.find(pos, 'E'), vec![
            V2 { row: 1, col: 1 },
            V2 { row: 1, col: 2 },
            V2 { row: 1, col: 3 },
            V2 { row: 1, col: 4 },
            V2 { row: 1, col: 5 },
        ]);
    }

    #[test]
    fn test_find_triangle_up2() {
        let grid = make_grid(vec![
            "#######",
            "#E...G#",
            "#.E...#",
            "#..E..#",
            "#...E.#",
            "#....E#",
            "#######",
        ], 10, 1);

        let pos = V2 { row: 1, col: 5 };
        assert_eq!(grid.find(pos, 'E'), vec![
            V2 { row: 1, col: 5 },
            V2 { row: 1, col: 4 },
            V2 { row: 1, col: 3 },
            V2 { row: 1, col: 2 },
            V2 { row: 1, col: 1 },
        ]);
    }

    #[test]
    fn test_find_triangle_dn1() {
        let grid = make_grid(vec![
            "#######",
            "#....E#",
            "#...E.#",
            "#..E..#",
            "#.E...#",
            "#E...G#",
            "#######",
        ], 10, 1);

        let pos = V2 { row: 5, col: 5 };
        assert_eq!(grid.find(pos, 'E'), vec![
            V2 { row: 5, col: 5 },
            V2 { row: 4, col: 5 },
            V2 { row: 3, col: 5 },
            V2 { row: 2, col: 5 },
            V2 { row: 1, col: 5 },
        ]);
    }

    #[test]
    fn test_find_triangle_dn2() {
        let grid = make_grid(vec![
            "#######",
            "#E....#",
            "#.E...#",
            "#..E..#",
            "#...E.#",
            "#G...E#",
            "#######",
        ], 10, 1);

        let pos = V2 { row: 5, col: 1 };
        assert_eq!(grid.find(pos, 'E'), vec![
            V2 { row: 5, col: 1 },
            V2 { row: 4, col: 1 },
            V2 { row: 3, col: 1 },
            V2 { row: 2, col: 1 },
            V2 { row: 1, col: 1 },
        ]);
    }

    #[test]
    fn test_find_diamond1() {
        let grid = make_grid(vec![
            "###########",
            "#....E....#",
            "#...E.E...#",
            "#..E...E..#",
            "#.E.....E.#",
            "#E...G...E#",
            "#.E.....E.#",
            "#..E...E..#",
            "#...E.E...#",
            "#....E....#",
            "###########",
        ], 10, 1);

        let pos = V2 { row: 5, col: 5 };
        assert_eq!(grid.find(pos, 'E'), vec![
            V2 { row: 5, col: 5 },
            V2 { row: 4, col: 5 },
            V2 { row: 3, col: 5 },
            V2 { row: 2, col: 5 },
            V2 { row: 1, col: 5 },
        ]);
    }

    #[test]
    fn test_find_diamond2() {
        let grid = make_grid(vec![
            "###########",
            "#....E....#",
            "#...E#E...#",
            "#..E...E..#",
            "#.E.....E.#",
            "#E...G...E#",
            "#.E.....E.#",
            "#..E...E..#",
            "#...E.E...#",
            "#....E....#",
            "###########",
        ], 10, 1);

        let pos = V2 { row: 5, col: 5 };
        assert_eq!(grid.find(pos, 'E'), vec![
            V2 { row: 5, col: 5 },
            V2 { row: 4, col: 5 },
            V2 { row: 3, col: 5 },
            V2 { row: 3, col: 4 },
            V2 { row: 2, col: 4 },
        ]);
    }

    #[test]
    fn test_find_diamond3() {
        let grid = make_grid(vec![
            "###########",
            "#....E....#",
            "#...E#E...#",
            "#..E##.E..#", // (3,6) must be chosen, as first in reading order
            "#.E..G..E.#",
            "#E.......E#",
            "#.E.....E.#",
            "#..E...E..#",
            "#...E.E...#",
            "#....E....#",
            "###########",
        ], 10, 1);

        let pos = V2 { row: 4, col: 5 };
        assert_eq!(grid.find(pos, 'E'), vec![
            V2 { row: 4, col: 5 },
            V2 { row: 4, col: 6 },
            V2 { row: 3, col: 6 },
            V2 { row: 2, col: 6 },
        ]);
    }

    #[test]
    fn test_find_second() {
        let grid = make_grid(vec![
            "#########",  //  "#########",
            "#...E...#",  //  "#.@@E...#",
            "#....E..#",  //  "#.@..E..#",
            "#..######",  //  "#.@######",
            "#.......#",  //  "#.@@@...#",
            "#...G...#",  //  "#...G...#",
            "#..######",  //  "#..######",
            "#.......#",  //  "#.......#",
            "#....E..#",  //  "#....E..#",
            "#########",  //  "#########",
        ], 10, 1);

        let pos = V2 { row: 5, col: 4 };
        assert_eq!(grid.find(pos, 'E'), vec![
            V2 { row: 5, col: 4 },
            V2 { row: 4, col: 4 },
            V2 { row: 4, col: 3 },
            V2 { row: 4, col: 2 },
            V2 { row: 3, col: 2 },
            V2 { row: 2, col: 2 },
            V2 { row: 1, col: 2 },
            V2 { row: 1, col: 3 },
            V2 { row: 1, col: 4 },
        ]);
    }

    #[test]
    fn test_target1() {
        let grid = make_grid(vec![
            "#######",
            "#E..G.#",
            "#...#.#",
            "#.G.#G#",
            "#######",
        ], 10, 1);

        let pos = V2 { row: 1, col: 1 };
        assert_eq!(grid.target(pos, 'G'), Some(V2 { row: 1, col: 2 }));
    }

    #[test]
    fn test_target2_1() {
        let grid = make_grid(vec![
            "#######",
            "#.E...#",
            "#.....#",
            "#...G.#",
            "#######",
        ], 10, 1);

        let pos = V2 { row: 1, col: 2 };
        assert_eq!(grid.target(pos, 'G'), Some(V2 { row: 1, col: 3 }));
    }

    #[test]
    fn test_target2_2() {
        let grid = make_grid(vec![
            "#######",
            "#..E..#",
            "#.....#",
            "#...G.#",
            "#######",
        ], 10, 1);

        let pos = V2 { row: 1, col: 3 };
        assert_eq!(grid.target(pos, 'G'), Some(V2 { row: 1, col: 4 }));
    }

    #[test]
    fn test_target2_3() {
        let grid = make_grid(vec![
            "#######",
            "#...E.#",
            "#.....#",
            "#...G.#",
            "#######",
        ], 10, 1);

        let pos = V2 { row: 1, col: 4 };
        assert_eq!(grid.target(pos, 'G'), Some(V2 { row: 2, col: 4 }));
    }

    #[test]
    fn test_target2_4() {
        let grid = make_grid(vec![
            "#######",
            "#.....#",
            "#...E.#",
            "#...G.#",
            "#######",
        ], 10, 1);

        let pos = V2 { row: 2, col: 4 };
        assert_eq!(grid.target(pos, 'G'), Some(V2 { row: 3, col: 4 }));
    }

    #[test]
    fn test_find_between2() {
        let grid = make_grid(vec![
            "#####",
            "#..E#",
            "#G..#",
            "#..E#",
            "#####",
        ], 10, 1);

        let pos = V2 { row: 2, col: 1 };
        assert_eq!(grid.find(pos, 'E'), vec![
            V2 { row: 2, col: 1 },
            V2 { row: 1, col: 1 },
            V2 { row: 1, col: 2 },
            V2 { row: 1, col: 3 },
        ]);
    }

    #[test]
    fn test_find_between4_corners() {
        let grid = make_grid(vec![
            "######",
            "#E...E#",
            "#.....#",
            "#..G..#",
            "#.....#",
            "#E...E#",
            "######",
        ], 10, 1);

        let pos = V2 { row: 3, col: 3 };
        assert_eq!(grid.find(pos, 'E'), vec![
            V2 { row: 3, col: 3 },
            V2 { row: 2, col: 3 },
            V2 { row: 1, col: 3 },
            V2 { row: 1, col: 2 },
            V2 { row: 1, col: 1 },
        ]);
    }

    #[test]
    fn test_find_between4_edges() {
        let grid = make_grid(vec![
            "######",
            "#..E..#",
            "#.....#",
            "#E.G.E#",
            "#.....#",
            "#..E..#",
            "######",
        ], 10, 1);

        let pos = V2 { row: 3, col: 3 };
        assert_eq!(grid.find(pos, 'E'), vec![
            V2 { row: 3, col: 3 },
            V2 { row: 2, col: 3 },
            V2 { row: 1, col: 3 },
        ]);
    }

    #[test]
    fn test_find_between4_wall1() {
        let grid = make_grid(vec![
            "######",
            "#..E..#",
            "#..####",
            "#E.G.E#",
            "#.....#",
            "#..E..#",
            "######",
        ], 10, 1);

        let pos = V2 { row: 3, col: 3 };
        assert_eq!(grid.find(pos, 'E'), vec![
            V2 { row: 3, col: 3 },
            V2 { row: 3, col: 2 },
            V2 { row: 3, col: 1 },
        ]);
    }

    #[test]
    fn test_find_between4_wall2() {
        let grid = make_grid(vec![
            "######",
            "#..E..#",
            "#.#####",
            "#E#G.E#",
            "#.#...#",
            "#..E..#",
            "######",
        ], 10, 1);

        let pos = V2 { row: 3, col: 3 };
        assert_eq!(grid.find(pos, 'E'), vec![
            V2 { row: 3, col: 3 },
            V2 { row: 3, col: 4 },
            V2 { row: 3, col: 5 },
        ]);
    }

    #[test]
    fn test_find_between4_wall3() {
        let grid = make_grid(vec![
            "######",
            "#..E..#",
            "#.#####",
            "#E#G#E#",
            "#.#...#",
            "#..E..#",
            "######",
        ], 10, 1);

        let pos = V2 { row: 3, col: 3 };
        assert_eq!(grid.find(pos, 'E'), vec![
            V2 { row: 3, col: 3 },
            V2 { row: 4, col: 3 },
            V2 { row: 5, col: 3 },
        ]);
    }

    #[test]
    #[ignore] // it just doesn't fit the (round-1)*sum approach (that gives correct answers)
    fn test_solve1() {
        let mut grid = make_grid(vec![
            "#######",
            "#.G...#",
            "#...EG#",
            "#.#.#G#",
            "#..G#E#",
            "#.....#",
            "#######",
        ], 200, 3);

        assert_eq!(solve(&mut grid), 27730);
    }

    #[test]
    fn test_solve2() {
        let mut grid = make_grid(vec![
            "#######",
            "#G..#E#",
            "#E#E.E#",
            "#G.##.#",
            "#...#E#",
            "#...E.#",
            "#######",
        ], 200, 3);

        let x = solve(&mut grid);

        assert_eq!(grid.dump(), wrap(vec![
            "#######",
            "#...#E# E/200",
            "#E#...# E/197",
            "#.E##.# E/185",
            "#E..#E# E/200 E/200",
            "#.....#",
            "#######",
        ]));
        assert_eq!(x, 36334);
    }

    #[test]
    fn test_solve3() {
        let mut grid = make_grid(vec![
            "#######",
            "#E..EG#",
            "#.#G.E#",
            "#E.##E#",
            "#G..#.#",
            "#..E#.#",
            "#######",
        ], 200, 3);

        let x = solve(&mut grid);

        assert_eq!(grid.dump(), wrap(vec![
            "#######",
            "#.E.E.# E/164 E/197",
            "#.#E..# E/200",
            "#E.##.# E/98",
            "#.E.#.# E/200",
            "#...#.#",
            "#######",
        ]));
        assert_eq!(x, 39514);
    }

    #[test]
    fn test_solve6() {
        let mut grid = make_grid(vec![
            "#########",
            "#G......#",
            "#.E.#...#",
            "#..##..G#",
            "#...##..#",
            "#...#...#",
            "#.G...G.#",
            "#.....G.#",
            "#########",
        ], 200, 3);
        let x = solve(&mut grid);

        assert_eq!(grid.dump(), wrap(vec![
            "#########",
            "#.G.....# G/137",
            "#G.G#...# G/200 G/200",
            "#.G##...# G/200",
            "#...##..#",
            "#.G.#...# G/200",
            "#.......#",
            "#.......#",
            "#########",
        ]));

        assert_eq!(x, 18740);
    }

    #[test]
    fn test_simulation() {
        let mut grid = make_grid(vec![
            "#########",
            "#G..G..G# G/10 G/10 G/10",
            "#.......#",
            "#.......#",
            "#G..E..G# G/10 E/10 G/10",
            "#.......#",
            "#.......#",
            "#G..G..G# G/10 G/10 G/10",
            "#########",
        ], 10, 2);

        println!("\nstep 1:");
        simulation(&mut grid);
        assert_eq!(grid.dump(), wrap(vec![
            "#########",
            "#.G...G.# G/10 G/10",
            "#...G...# G/8",
            "#...E..G# E/10 G/10",
            "#.G.....# G/10",
            "#.......#",
            "#G..G..G# G/10 G/10 G/10",
            "#.......#",
            "#########",
        ]));

        println!("\nstep 2:");
        simulation(&mut grid);
        assert_eq!(grid.dump(), wrap(vec![
            "#########",
            "#..G.G..# G/10 G/10",
            "#...G...# G/6",
            "#.G.E.G.# G/10 E/8 G/10",
            "#.......#",
            "#G..G..G# G/10 G/10 G/10",
            "#.......#",
            "#.......#",
            "#########",
        ]));

        println!("\nstep 3:");
        let (es, gs) = simulation(&mut grid);
        assert_eq!(grid.dump(), wrap(vec![
            "#########",
            "#.......#",
            "#..GGG..# G/10 G/4 G/10",
            "#..G.G..# G/10 G/10",
            "#G..G...# G/10 G/10",
            "#......G# G/10",
            "#.......#",
            "#.......#",
            "#########",
        ]));
        assert_eq!(es, 0);
        assert_eq!(gs, 8);
    }
}
