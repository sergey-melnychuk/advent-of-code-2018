extern crate regex;
use self::regex::Regex;

use std::io;
use std::io::prelude::*;

use std::collections::VecDeque;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn pos(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Line {
    a: Pos,
    b: Pos,
}

#[derive(Debug)]
struct Grid {
    chars: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn min(&self) -> Pos {
        let (mut minx, mut miny) = (std::usize::MAX, std::usize::MAX);
        for (y, row) in self.chars.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '#' {
                    minx = min(minx, x);
                    miny = min(miny, y);
                    break;
                }
            }
        }
        Pos { x: minx, y: miny }
    }

    fn dump(&self) -> Vec<String> {
        let chars = self.chars.clone();
        chars.into_iter().map(|cs| cs.iter().collect()).collect()
    }

    fn dump_with_offset(&self, x: usize, y: usize) -> Vec<String> {
        let mut chars = vec![vec!['.'; self.cols - x]; self.rows - y];

        for r in 0..chars.len() {
            let n = chars[r].len();
            for c in 0..n {
                chars[r][c] = self.get(c + x, r + y);
            }
        }

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
}

fn max(a: usize, b: usize) -> usize {
    if a >= b {
        a
    } else {
        b
    }
}

fn min(a: usize, b: usize) -> usize {
    if a <= b {
        a
    } else {
        b
    }
}

fn get_input() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _n = handle.read_to_string(&mut buffer).unwrap();
    //println!("n: {}\n{}", n, buffer);
    buffer
}

fn parse_input(buffer: String) -> Vec<Line> {
    let re = Regex::new(r"(\w)=(\d+), \w=(\d+)..(\d+)").unwrap();
    let mut result = Vec::new();
    for cap in re.captures_iter(&buffer) {
        let axis = cap[1].chars().next().unwrap();
        let val: usize = cap[2].parse().unwrap();
        let at: usize = cap[3].parse().unwrap();
        let to: usize = cap[4].parse().unwrap();

        let line = if axis == 'x' {
            Line {
                a: Pos { x: val, y: at },
                b: Pos { x: val, y: to },
            }
        } else {
            Line {
                a: Pos { y: val, x: at },
                b: Pos { y: val, x: to },
            }
        };
        result.push(line);
    }
    result
}

fn build_grid(lines: Vec<Line>) -> Grid {
    let (mut xmax, mut ymax) = (0, 0);
    for line in &lines {
        xmax = max(xmax, max(line.a.x, line.b.x));
        ymax = max(ymax, max(line.a.y, line.b.y));
    }

    let mut chars = vec![vec!['.'; xmax + 1]; ymax + 1];
    for line in &lines {
        if line.a.x == line.b.x {
            // vertical
            let x = line.a.x;
            for y in line.a.y..=line.b.y {
                chars[y][x] = '#';
            }
        } else {
            // horizontal
            let y = line.a.y;
            for x in line.a.x..=line.b.x {
                chars[y][x] = '#';
            }
        }
    }
    Grid {
        chars,
        rows: ymax + 1,
        cols: xmax + 1,
    }
}

// get horizontal bounds
fn get_hbounds(pos: &Pos, grid: &Grid) -> (usize, usize, char) {
    let mut lborder: usize = 0;
    let mut rborder: usize = grid.cols - 1;
    let mut chr: char = '~';
    let mut spill = false;

    for x in (0..pos.x).rev() {
        let c = grid.get(x, pos.y);
        let d = grid.get(x, pos.y + 1);
        if c == '#' && d != '.' {
            lborder = x + 1;
            break;
        }
        if d == '.' || d == '|' {
            lborder = x;
            spill = true;
            break;
        }
    }

    for x in (pos.x + 1)..grid.cols {
        let c = grid.get(x, pos.y);
        let d = grid.get(x, pos.y + 1);
        if c == '#' && d != '.' {
            rborder = x - 1;
            break;
        }
        if d == '.' || d == '|' {
            rborder = x;
            spill = true;
            break;
        }
    }

    if spill {
        chr = '|';
    } else {
        chr = '~';
    }
    (lborder, rborder, chr)
}

// get lower bound
fn get_lbound(pos: &Pos, grid: &Grid) -> Option<Pos> {
    let mut opt: Option<Pos> = None;
    for y in (pos.y + 1)..grid.rows {
        //println!("L x={} y={} c={}", pos.x, y, grid.get(pos.x, y));
        let c = grid.get(pos.x, y);
        if c == '~' || c == '#' {
            opt = Some(Pos { x: pos.x, y: y - 1 });
            break;
        }
    }
    opt
}

// get upper bound
fn get_ubound(pos: &Pos, grid: &Grid) -> Option<Pos> {
    let mut opt: Option<Pos> = None;
    for y in (0..pos.y).rev() {
        //println!("U x={} y={} c={}", pos.x, y, grid.get(pos.x, y));
        let c = grid.get(pos.x, y);
        if c != '|' {
            opt = Some(Pos { x: pos.x, y: y + 1 });
            break;
        }
    }
    opt
}

// get sub-stream that fills current horizontal line
fn get_sources(pos: &Pos, l: usize, r: usize, grid: &Grid) -> Vec<Pos> {
    let mut result = Vec::new();
    let y = pos.y;
    for x in l..=r {
        let c = grid.get(x, y - 1);
        if c == '|' {
            let p = Pos { y: y - 1, ..*pos };
            let opt = get_ubound(&p, grid);
            if opt.is_some() {
                result.push(opt.unwrap());
            }
        }
    }
    result
}

fn pour(from: &Pos, grid: &mut Grid) -> Vec<Pos> {
    let mut result = Vec::new();

    let opt = get_lbound(from, grid);
    if opt.is_some() {
        let p = opt.unwrap();
        let (l, r, c) = get_hbounds(&p, grid);
        //println!("from={:?} pos={:?} l={} r={} c={}", from, p, l, r, c);
        if *from == p && c == '~' {
            // sub-stream floods
            return get_sources(&p, l, r, grid);
        }

        for x in l..=r {
            grid.set(x, p.y, c);
        }
        if c == '~' {
            result.push(*from);
        } else {
            let ld = grid.get(l, p.y + 1);
            let rd = grid.get(r, p.y + 1);

            if ld == '.' {
                let ls = Pos { x: l, y: p.y };
                result.push(ls);
            }

            if rd == '.' {
                let rs = Pos { x: r, y: p.y };
                result.push(rs);
            }

            for y in max(from.y, 1)..p.y {
                grid.set(p.x, y, '|');
            }
        }
    } else {
        for y in max(from.y, 1)..grid.rows {
            grid.set(from.x, y, '|');
        }
    }

    result
}

fn simulate(from: Pos, grid: &mut Grid) {
    let mut queue = VecDeque::new();
    queue.push_back(from);

    let mut it = 0;
    while !queue.is_empty() {
        it += 1;
        let at = queue.pop_back().unwrap();
        //println!("it={} at={:?} c={}", it, at, grid.get(at.x, at.y));
        for p in pour(&at, grid) {
            queue.push_back(p);
        }
        //println!("\n{:?}", grid.dump_with_offset(grid.cols*6/10, 0).iter().take(100).collect::<Vec<&String>>());
        //if it > 1000 { break; }
    }
    println!("iterations: {}", it);
}

pub fn main() {
    let lines = parse_input(get_input());
    println!("lines: {}", lines.len());
    let mut grid = build_grid(lines);
    let pos = Pos { x: 500, y: 0 };
    simulate(pos, &mut grid);
    println!("{:?}", grid.dump_with_offset(grid.cols * 6 / 10, 0));

    let count = grid.count(|c| c == '~' || c == '|');
    let miny = grid.min().y;
    let not_count = if miny == 0 { 1 } else { miny - 1 };
    println!("~|: {}", count - not_count); // 34244
    println!("~: {}", grid.count(|c| c == '~')); // 28202
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wrap(lines: Vec<&'static str>) -> Vec<String> {
        lines.into_iter().map(String::from).collect()
    }

    fn make_grid(lines: Vec<&str>) -> Grid {
        let rows = lines.len();
        let cols = lines.len();
        let chars = lines.into_iter().map(|l| l.chars().collect()).collect();
        Grid { chars, rows, cols }
    }

    fn get_grid() -> Grid {
        make_grid(vec![
            "..............",
            "............#.",
            ".#..#.......#.",
            ".#..#..#......",
            ".#..#..#......",
            ".#.....#......",
            ".#.....#......",
            ".#######......",
            "..............",
            "..............",
            "....#.....#...",
            "....#.....#...",
            "....#.....#...",
            "....#######...",
        ])
    }

    #[test]
    fn test_get_lbound_some() {
        let grid = get_grid();
        let p = Pos { x: 6, y: 0 };
        let m = Pos { x: 6, y: 6 };
        assert_eq!(get_lbound(&p, &grid), Some(m));
    }

    #[test]
    fn test_get_lbound2() {
        let grid = get_grid();
        let p = Pos { x: 8, y: 2 };
        let m = Pos { x: 8, y: 12 };
        assert_eq!(get_lbound(&p, &grid), Some(m));
    }

    #[test]
    fn test_get_lbound_none() {
        let grid = get_grid();
        let p = Pos { x: 0, y: 0 };
        assert_eq!(get_lbound(&p, &grid), None);
    }

    #[test]
    fn test_get_hbounds_still() {
        let grid = get_grid();
        let p = Pos { x: 6, y: 6 };
        assert_eq!(get_hbounds(&p, &grid), (2, 6, '~'));
    }

    #[test]
    fn test_get_hbounds_pour() {
        let grid = make_grid(vec![
            "..............",
            "............#.",
            ".#~~#.......#.",
            ".#..#..#......",
            ".#..#..#......",
            ".#.....#......",
            ".#.....#......",
            ".#######......",
            "..............",
            "..............",
            "....#.....#...",
            "....#.....#...",
            "....#.....#...",
            "....#######...",
        ]);
        let p = Pos { x: 1, y: 1 };
        assert_eq!(get_hbounds(&p, &grid), (0, 5, '|'));
    }

    #[test]
    fn test_pour1() {
        let mut grid = get_grid();
        let pos = Pos { x: 6, y: 0 };

        let res = pour(&pos, &mut grid);

        assert_eq!(
            grid.dump(),
            wrap(vec![
                "..............",
                "............#.",
                ".#..#.......#.",
                ".#..#..#......",
                ".#..#..#......",
                ".#.....#......",
                ".#~~~~~#......",
                ".#######......",
                "..............",
                "..............",
                "....#.....#...",
                "....#.....#...",
                "....#.....#...",
                "....#######...",
            ])
        );
        assert_eq!(res, vec![pos]);
    }

    #[test]
    fn test_pour2() {
        let mut grid = make_grid(vec![
            "..............",
            "............#.",
            ".#..#.......#.",
            ".#..#..#......",
            ".#..#..#......",
            ".#.....#......",
            ".#~~~~~#......",
            ".#######......",
            "..............",
            "..............",
            "....#.....#...",
            "....#.....#...",
            "....#.....#...",
            "....#######...",
        ]);

        let pos = Pos { x: 6, y: 0 };

        let res = pour(&pos, &mut grid);

        assert_eq!(
            grid.dump(),
            wrap(vec![
                "..............",
                "............#.",
                ".#..#.......#.",
                ".#..#..#......",
                ".#..#..#......",
                ".#~~~~~#......",
                ".#~~~~~#......",
                ".#######......",
                "..............",
                "..............",
                "....#.....#...",
                "....#.....#...",
                "....#.....#...",
                "....#######...",
            ])
        );
        assert_eq!(res, vec![pos]);
    }

    #[test]
    fn test_pour3() {
        let mut grid = make_grid(vec![
            "..............",
            "............#.",
            ".#..#.......#.",
            ".#..#..#......",
            ".#..#..#......",
            ".#~~~~~#......",
            ".#~~~~~#......",
            ".#######......",
            "..............",
            "..............",
            "....#.....#...",
            "....#.....#...",
            "....#.....#...",
            "....#######...",
        ]);

        let pos = Pos { x: 6, y: 0 };

        let res = pour(&pos, &mut grid);

        assert_eq!(
            grid.dump(),
            wrap(vec![
                "..............",
                "............#.",
                ".#..#.......#.",
                ".#..#..#......",
                ".#..#~~#......",
                ".#~~~~~#......",
                ".#~~~~~#......",
                ".#######......",
                "..............",
                "..............",
                "....#.....#...",
                "....#.....#...",
                "....#.....#...",
                "....#######...",
            ])
        );
        assert_eq!(res, vec![pos]);
    }

    #[test]
    fn test_pour4() {
        let mut grid = make_grid(vec![
            "..............",
            "............#.",
            ".#..#.......#.",
            ".#..#..#......",
            ".#..#~~#......",
            ".#~~~~~#......",
            ".#~~~~~#......",
            ".#######......",
            "..............",
            "..............",
            "....#.....#...",
            "....#.....#...",
            "....#.....#...",
            "....#######...",
        ]);

        let pos = Pos { x: 6, y: 0 };

        let res = pour(&pos, &mut grid);

        assert_eq!(
            grid.dump(),
            wrap(vec![
                "..............",
                "............#.",
                ".#..#.......#.",
                ".#..#~~#......",
                ".#..#~~#......",
                ".#~~~~~#......",
                ".#~~~~~#......",
                ".#######......",
                "..............",
                "..............",
                "....#.....#...",
                "....#.....#...",
                "....#.....#...",
                "....#######...",
            ])
        );
        assert_eq!(res, vec![pos]);
    }

    #[test]
    fn test_pour5() {
        let mut grid = make_grid(vec![
            "..............",
            "............#.",
            ".#..#.......#.",
            ".#..#~~#......",
            ".#..#~~#......",
            ".#~~~~~#......",
            ".#~~~~~#......",
            ".#######......",
            "..............",
            "..............",
            "....#.....#...",
            "....#.....#...",
            "....#.....#...",
            "....#######...",
        ]);

        let pos = Pos { x: 6, y: 0 };

        let res = pour(&pos, &mut grid);

        assert_eq!(
            grid.dump(),
            wrap(vec![
                "..............",
                "......|.....#.",
                ".#..#||||...#.",
                ".#..#~~#......",
                ".#..#~~#......",
                ".#~~~~~#......",
                ".#~~~~~#......",
                ".#######......",
                "..............",
                "..............",
                "....#.....#...",
                "....#.....#...",
                "....#.....#...",
                "....#######...",
            ])
        );
        assert_eq!(res, vec![Pos { x: 8, y: 2 }]);
    }

    #[test]
    fn test_pour6() {
        let mut grid = make_grid(vec![
            "......|.......",
            "......|.....#.",
            ".#..#||||...#.",
            ".#..#~~#......",
            ".#..#~~#......",
            ".#~~~~~#......",
            ".#~~~~~#......",
            ".#######......",
            "..............",
            "..............",
            "....#.....#...",
            "....#.....#...",
            "....#.....#...",
            "....#######...",
        ]);

        let pos = Pos { x: 8, y: 2 };

        let res = pour(&pos, &mut grid);

        assert_eq!(
            grid.dump(),
            wrap(vec![
                "......|.......",
                "......|.....#.",
                ".#..#||||...#.",
                ".#..#~~#......",
                ".#..#~~#......",
                ".#~~~~~#......",
                ".#~~~~~#......",
                ".#######......",
                "..............",
                "..............",
                "....#.....#...",
                "....#.....#...",
                "....#~~~~~#...",
                "....#######...",
            ])
        );
        assert_eq!(res, vec![Pos { x: 8, y: 2 }]);
    }

    #[test]
    fn test_pour7() {
        let mut grid = make_grid(vec![
            "......|.......",
            "......|.....#.",
            ".#..#||||...#.",
            ".#..#~~#......",
            ".#..#~~#......",
            ".#~~~~~#......",
            ".#~~~~~#......",
            ".#######......",
            "..............",
            "..............",
            "....#.....#...",
            "....#.....#...",
            "....#~~~~~#...",
            "....#######...",
        ]);

        let pos = Pos { x: 8, y: 2 };

        let res = pour(&pos, &mut grid);

        assert_eq!(
            grid.dump(),
            wrap(vec![
                "......|.......",
                "......|.....#.",
                ".#..#||||...#.",
                ".#..#~~#......",
                ".#..#~~#......",
                ".#~~~~~#......",
                ".#~~~~~#......",
                ".#######......",
                "..............",
                "..............",
                "....#.....#...",
                "....#~~~~~#...",
                "....#~~~~~#...",
                "....#######...",
            ])
        );
        assert_eq!(res, vec![Pos { x: 8, y: 2 }]);
    }

    #[test]
    fn test_pour8() {
        let mut grid = make_grid(vec![
            "......|.......",
            "......|.....#.",
            ".#..#||||...#.",
            ".#..#~~#......",
            ".#..#~~#......",
            ".#~~~~~#......",
            ".#~~~~~#......",
            ".#######......",
            "..............",
            "..............",
            "....#.....#...",
            "....#~~~~~#...",
            "....#~~~~~#...",
            "....#######...",
        ]);

        let pos = Pos { x: 8, y: 2 };

        let res = pour(&pos, &mut grid);

        assert_eq!(
            grid.dump(),
            wrap(vec![
                "......|.......",
                "......|.....#.",
                ".#..#||||...#.",
                ".#..#~~#......",
                ".#..#~~#......",
                ".#~~~~~#......",
                ".#~~~~~#......",
                ".#######......",
                "..............",
                "..............",
                "....#~~~~~#...",
                "....#~~~~~#...",
                "....#~~~~~#...",
                "....#######...",
            ])
        );
        assert_eq!(res, vec![Pos { x: 8, y: 2 }]);
    }

    #[test]
    fn test_pour9() {
        let mut grid = make_grid(vec![
            "......|.......",
            "......|.....#.",
            ".#..#||||...#.",
            ".#..#~~#......",
            ".#..#~~#......",
            ".#~~~~~#......",
            ".#~~~~~#......",
            ".#######......",
            "..............",
            "..............",
            "....#~~~~~#...",
            "....#~~~~~#...",
            "....#~~~~~#...",
            "....#######...",
        ]);

        let pos = Pos { x: 8, y: 2 };

        let res = pour(&pos, &mut grid);

        assert_eq!(
            grid.dump(),
            wrap(vec![
                "......|.......",
                "......|.....#.",
                ".#..#||||...#.",
                ".#..#~~#|.....",
                ".#..#~~#|.....",
                ".#~~~~~#|.....",
                ".#~~~~~#|.....",
                ".#######|.....",
                "........|.....",
                "...|||||||||..",
                "....#~~~~~#...",
                "....#~~~~~#...",
                "....#~~~~~#...",
                "....#######...",
            ])
        );
        assert_eq!(res, vec![Pos { x: 3, y: 9 }, Pos { x: 11, y: 9 }]);
    }

    #[test]
    fn test_pour_final1() {
        let mut grid = make_grid(vec![
            "......|.......",
            "......|.....#.",
            ".#..#||||...#.",
            ".#..#~~#|.....",
            ".#..#~~#|.....",
            ".#~~~~~#|.....",
            ".#~~~~~#|.....",
            ".#######|.....",
            "........|.....",
            "...|||||||||..",
            "....#~~~~~#...",
            "....#~~~~~#...",
            "....#~~~~~#...",
            "....#######...",
        ]);

        let pos = Pos { x: 3, y: 9 };

        let res = pour(&pos, &mut grid);

        assert_eq!(
            grid.dump(),
            wrap(vec![
                "......|.......",
                "......|.....#.",
                ".#..#||||...#.",
                ".#..#~~#|.....",
                ".#..#~~#|.....",
                ".#~~~~~#|.....",
                ".#~~~~~#|.....",
                ".#######|.....",
                "........|.....",
                "...|||||||||..",
                "...|#~~~~~#...",
                "...|#~~~~~#...",
                "...|#~~~~~#...",
                "...|#######...",
            ])
        );
        assert_eq!(res, vec![]);
    }

    #[test]
    fn test_pour_final2() {
        let mut grid = make_grid(vec![
            "......|.......",
            "......|.....#.",
            ".#..#||||...#.",
            ".#..#~~#|.....",
            ".#..#~~#|.....",
            ".#~~~~~#|.....",
            ".#~~~~~#|.....",
            ".#######|.....",
            "........|.....",
            "...|||||||||..",
            "....#~~~~~#...",
            "....#~~~~~#...",
            "....#~~~~~#...",
            "....#######...",
        ]);

        let pos = Pos { x: 11, y: 9 };

        let res = pour(&pos, &mut grid);

        assert_eq!(
            grid.dump(),
            wrap(vec![
                "......|.......",
                "......|.....#.",
                ".#..#||||...#.",
                ".#..#~~#|.....",
                ".#..#~~#|.....",
                ".#~~~~~#|.....",
                ".#~~~~~#|.....",
                ".#######|.....",
                "........|.....",
                "...|||||||||..",
                "....#~~~~~#|..",
                "....#~~~~~#|..",
                "....#~~~~~#|..",
                "....#######|..",
            ])
        );
        assert_eq!(res, vec![]);
    }

    #[test]
    fn test_count() {
        let grid = make_grid(vec![
            "..............",
            "......|.....#.",
            ".#..#||||...#.",
            ".#..#~~#|.....",
            ".#..#~~#|.....",
            ".#~~~~~#|.....",
            ".#~~~~~#|.....",
            ".#######|.....",
            "........|.....",
            "...|||||||||..",
            "...|#~~~~~#|..",
            "...|#~~~~~#|..",
            "...|#~~~~~#|..",
            "...|#######|..",
        ]);

        assert_eq!(grid.count(|c| c == '~' || c == '|'), 57);
    }

    #[test]
    fn test_parse() {
        let input = "
            |x=495, y=2..7\n\
            |y=7, x=495..501\n\
            |x=501, y=3..7\n\
            |x=498, y=2..4\n\
            |x=506, y=1..2\n\
            |x=498, y=10..13\n\
            |x=504, y=10..13\n\
            |y=13, x=498..504";

        let exp = vec![
            Line {
                a: Pos { x: 495, y: 2 },
                b: Pos { x: 495, y: 7 },
            },
            Line {
                a: Pos { x: 495, y: 7 },
                b: Pos { x: 501, y: 7 },
            },
            Line {
                a: Pos { x: 501, y: 3 },
                b: Pos { x: 501, y: 7 },
            },
            Line {
                a: Pos { x: 498, y: 2 },
                b: Pos { x: 498, y: 4 },
            },
            Line {
                a: Pos { x: 506, y: 1 },
                b: Pos { x: 506, y: 2 },
            },
            Line {
                a: Pos { x: 498, y: 10 },
                b: Pos { x: 498, y: 13 },
            },
            Line {
                a: Pos { x: 504, y: 10 },
                b: Pos { x: 504, y: 13 },
            },
            Line {
                a: Pos { x: 498, y: 13 },
                b: Pos { x: 504, y: 13 },
            },
        ];

        assert_eq!(parse_input(input.to_string()), exp);
    }

    #[test]
    fn test_build_grid() {
        let lines = vec![
            Line {
                a: Pos { x: 495, y: 2 },
                b: Pos { x: 495, y: 7 },
            },
            Line {
                a: Pos { x: 495, y: 7 },
                b: Pos { x: 501, y: 7 },
            },
            Line {
                a: Pos { x: 501, y: 3 },
                b: Pos { x: 501, y: 7 },
            },
            Line {
                a: Pos { x: 498, y: 2 },
                b: Pos { x: 498, y: 4 },
            },
            Line {
                a: Pos { x: 506, y: 1 },
                b: Pos { x: 506, y: 2 },
            },
            Line {
                a: Pos { x: 498, y: 10 },
                b: Pos { x: 498, y: 13 },
            },
            Line {
                a: Pos { x: 504, y: 10 },
                b: Pos { x: 504, y: 13 },
            },
            Line {
                a: Pos { x: 498, y: 13 },
                b: Pos { x: 504, y: 13 },
            },
        ];

        let grid = build_grid(lines);

        let dump = grid.dump_with_offset(494, 0);
        for s in &dump {
            println!("{}", s);
        }

        assert_eq!(
            dump,
            wrap(vec![
                ".............",
                "............#",
                ".#..#.......#",
                ".#..#..#.....",
                ".#..#..#.....",
                ".#.....#.....",
                ".#.....#.....",
                ".#######.....",
                ".............",
                ".............",
                "....#.....#..",
                "....#.....#..",
                "....#.....#..",
                "....#######..",
            ])
        );
    }

    #[test]
    fn test_issue() {
        let mut grid = make_grid(vec![
            //   0     | (6, 0)
            "....#.........",
            "....#..#......",
            "....#..#......",
            "....####......",
            "..............",
            "..............",
            ".#.........#..",
            ".#.........#..",
            ".#..####...#..",
            ".#..#..#...#..",
            ".#..#..#...#..",
            ".#..####...#..",
            ".#.........#..",
            ".###########..",
        ]);

        let pos = Pos::pos(6, 0);
        simulate(pos, &mut grid);

        assert_eq!(
            grid.dump(),
            wrap(vec![
                //   0     | (6, 0)
                "....#||||.....",
                "....#~~#|.....",
                "....#~~#|.....",
                "....####|.....",
                "........|.....",
                "|||||||||||||.", // "........|....."
                "|#~~~~~~~~~#|.", // ".#......|..#.."
                "|#~~~~~~~~~#|.",
                "|#~~####~~~#|.", // <- Merge back at this point is tricky,
                "|#~~#..#~~~#|.", //    as previous sub-stream continues,
                "|#~~#..#~~~#|.", //    not stops, and leads to mismatch above.
                "|#~~####~~~#|.", //
                "|#~~~~~~~~~#|.", //   Need stack of sub-streams, not queue! TODO FIXME
                "|###########|.",
            ])
        );
    }
}
