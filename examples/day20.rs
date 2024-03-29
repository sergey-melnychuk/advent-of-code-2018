use std::io;
use std::io::prelude::*;

use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn zero() -> Pos {
        Pos { x: 0, y: 0 }
    }

    fn step(&self, dir: char) -> Pos {
        match dir {
            'N' => Pos {
                y: self.y - 2,
                ..*self
            },
            'S' => Pos {
                y: self.y + 2,
                ..*self
            },
            'E' => Pos {
                x: self.x + 2,
                ..*self
            },
            'W' => Pos {
                x: self.x - 2,
                ..*self
            },
            _ => *self,
        }
    }

    fn back(&self, dir: char) -> Pos {
        match dir {
            'N' => Pos {
                y: self.y + 1,
                ..*self
            },
            'S' => Pos {
                y: self.y - 1,
                ..*self
            },
            'E' => Pos {
                x: self.x - 1,
                ..*self
            },
            'W' => Pos {
                x: self.x + 1,
                ..*self
            },
            _ => *self,
        }
    }
}

fn traverse_rec(rec: &Rec, at: Pos) -> Vec<(char, Pos)> {
    let mut acc = Vec::new();
    acc.push(('X', at));
    traverse_rec_tree(rec, &[at], &mut acc);
    acc
}

fn traverse_rec_tree(rec: &Rec, from: &[Pos], acc: &mut Vec<(char, Pos)>) -> Vec<Pos> {
    let mut last: Vec<Pos> = Vec::new();

    if !rec.leaf.is_empty() {
        for f in from {
            let mut p = *f;
            for c in &rec.leaf {
                let n = p.step(*c);
                acc.push((*c, n));
                p = n;
            }
            last.push(p);
        }
    } else if !rec.list.is_empty() {
        last = from.to_vec();
        for r in &rec.list {
            let next = traverse_rec_tree(r, &last, acc);
            last = next;
        }
    } else if !rec.fork.is_empty() {
        for r in &rec.fork {
            let mut next = traverse_rec_tree(r, from, acc);
            last.append(&mut next);
        }
    }

    last
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    chars: Vec<char>,
    nodes: Vec<Node>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Rec {
    leaf: Vec<char>,
    fork: Vec<Rec>,
    list: Vec<Rec>,
}

impl Rec {
    fn new() -> Rec {
        Rec {
            leaf: vec![],
            fork: vec![],
            list: vec![],
        }
    }
}

fn reduce_list(rec: Rec) -> Rec {
    if rec.list.len() == 1 {
        rec.list[0].to_owned()
    } else {
        rec
    }
}

fn reduce_fork(rec: Rec) -> Rec {
    if rec.fork.len() == 1 {
        rec.fork[0].to_owned()
    } else {
        rec
    }
}

fn fetch_leaf(chars: &[char], at: usize) -> (Rec, usize) {
    let mut rec: Rec = Rec::new();
    let mut i = at;
    loop {
        let c = chars[i];
        //println!("leaf: i={} c={} rec={:?}", i, c, rec);
        match c {
            'N' | 'S' | 'E' | 'W' => rec.leaf.push(c),
            _ => break,
        }
        i += 1;
    }
    (rec, i)
}

fn fetch_list(chars: &[char], at: usize) -> (Rec, usize) {
    let mut rec: Rec = Rec::new();
    let mut i = at;
    loop {
        let c = chars[i];
        //println!("list: i={} c={} rec={:?}", i, c, rec);
        match c {
            'N' | 'S' | 'E' | 'W' => {
                let (r, j) = fetch_leaf(chars, i);
                rec.list.push(r);
                i = j;
            }
            '(' => {
                let (f, j) = fetch_fork(chars, i + 1);
                rec.list.push(f);
                i = j;
            }
            '|' => {
                //println!("Char '|' not expected in list at {}", i);
                //i += 1;
                break;
            }
            _ => break,
        }
    }
    (reduce_list(rec), i)
}

fn fetch_fork(chars: &[char], at: usize) -> (Rec, usize) {
    let mut rec: Rec = Rec::new();
    let mut i = at;

    let mut stack: Vec<Rec> = Vec::new();
    loop {
        if i >= chars.len() {
            break;
        }
        let c = chars[i];
        //println!("fork: i={} c={} rec={:?}", i, c, rec);
        match c {
            'N' | 'S' | 'W' | 'E' => {
                let (r, j) = fetch_list(chars, i);
                rec.fork.push(r);
                i = j;
            }
            '|' => {
                let (l, j) = fetch_list(chars, i + 1);
                rec.fork.push(l);
                i = j;
            }
            '(' => {
                stack.push(rec);
                rec = Rec::new();
                let (f, j) = fetch_fork(chars, i + 1);
                rec.fork.push(f);
                i = j;
            }
            ')' => {
                if !stack.is_empty() {
                    rec = stack.pop().unwrap();
                    i += 1;
                } else {
                    i += 1;
                    break;
                }
            }
            _ => break,
        }
    }
    //println!("\tfork: rec={:?}", rec);
    (reduce_fork(rec), i)
}

fn fetch_tree(chars: &[char]) -> Rec {
    let (rec, _) = fetch_list(chars, 1); // skip '^' at index 0
    rec
}

#[derive(Debug)]
struct Size {
    minx: isize,
    miny: isize,
    width: usize,
    height: usize,
}

impl Size {
    fn at(&self, p: Pos) -> Pos {
        Pos {
            x: p.x - self.minx,
            y: p.y - self.miny,
        }
    }
}

fn get_size(steps: &[(char, Pos)]) -> Size {
    fn max(a: isize, b: isize) -> isize {
        if a >= b {
            a
        } else {
            b
        }
    }
    fn min(a: isize, b: isize) -> isize {
        if a <= b {
            a
        } else {
            b
        }
    }

    let (mut minx, mut miny, mut maxx, mut maxy) = (
        std::isize::MAX,
        std::isize::MAX,
        std::isize::MIN,
        std::isize::MIN,
    );

    for s in steps {
        let (_, p) = s;
        minx = min(minx, p.x);
        maxx = max(maxx, p.x);
        miny = min(miny, p.y);
        maxy = max(maxy, p.y);
    }

    Size {
        minx: minx - 1,
        miny: miny - 1,
        width: ((maxx - minx).abs() + 3) as usize,
        height: ((maxy - miny).abs() + 3) as usize,
    }
}

fn make_grid(size: &Size, steps: &[(char, Pos)]) -> Vec<Vec<char>> {
    let mut grid = vec![vec!['#'; size.width]; size.height];
    for s in steps {
        let (d, p) = s;
        {
            let x = (p.x - size.minx) as usize;
            let y = (p.y - size.miny) as usize;
            grid[y][x] = '.';
        }
        {
            let f = p.back(*d);
            let x = (f.x - size.minx) as usize;
            let y = (f.y - size.miny) as usize;
            let c = match *d {
                'N' | 'S' => '-',
                'E' | 'W' => '|',
                x => x,
            };
            grid[y][x] = c;
        }
    }
    grid
}

fn bfs(grid: &[Vec<char>], at: Pos) -> Vec<Vec<usize>> {
    let rows = grid.len();
    let cols = grid[0].len();

    fn get(grid: &[Vec<char>], p: Pos) -> char {
        grid[p.y as usize][p.x as usize]
    }

    fn get_cost(dist: &[Vec<usize>], p: Pos) -> usize {
        dist[p.y as usize][p.x as usize]
    }

    fn set_cost(dist: &mut [Vec<usize>], p: Pos, d: usize) {
        dist[p.y as usize][p.x as usize] = d;
    }

    fn adj(grid: &[Vec<char>], p: Pos) -> Vec<Pos> {
        fn check(grid: &[Vec<char>], p: Pos, d: char, w: char, acc: &mut Vec<Pos>) {
            if get(grid, p.step(d).back(d)) == w {
                acc.push(p.step(d));
            }
        }

        let mut res = Vec::new();
        check(grid, p, 'N', '-', &mut res);
        check(grid, p, 'W', '|', &mut res);
        check(grid, p, 'E', '|', &mut res);
        check(grid, p, 'S', '-', &mut res);
        res
    }

    let mut dist: Vec<Vec<usize>> = vec![vec![std::usize::MAX; cols]; rows];
    dist[at.y as usize][at.x as usize] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(at);
    let mut seen: HashSet<Pos> = HashSet::new();
    seen.insert(at);
    while !queue.is_empty() {
        let p = queue.pop_front().unwrap();
        for a in adj(grid, p) {
            let d = get_cost(&dist, a);
            let c = get_cost(&dist, p) + 1;
            if c < d {
                set_cost(&mut dist, a, c)
            }
            if !seen.contains(&a) {
                queue.push_back(a);
                seen.insert(a);
            }
        }
    }

    dist
}

fn max(grid: &[Vec<char>], dist: &[Vec<usize>]) -> usize {
    fn max(a: usize, b: usize) -> usize {
        if a >= b {
            a
        } else {
            b
        }
    }
    let mut val = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let c = grid[y][x];
            if c == '.' {
                let d = dist[y][x];
                val = max(val, d);
            }
        }
    }

    val
}

fn count<F>(grid: &[Vec<char>], dist: &[Vec<usize>], f: F) -> usize
where
    F: Fn(usize) -> bool,
{
    let mut n = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let c = grid[y][x];
            if c == '.' {
                let d = dist[y][x];
                if f(d) {
                    n += 1;
                }
            }
        }
    }
    n
}

fn dump(grid: &[Vec<char>]) -> Vec<String> {
    grid.iter().map(|row| row.iter().collect()).collect()
}

fn get_input() -> Result<String, std::io::Error> {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.lock().read_to_string(&mut buffer)?;
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wrap(lines: Vec<&'static str>) -> Vec<String> {
        lines.into_iter().map(String::from).collect()
    }

    #[test]
    fn test_fetch_tree() {
        let input: Vec<char> = "^NE(W|E(N|S))WS$".chars().collect();
        println!("input: {:?}", input);
        let exp = Rec::list(vec![
            Rec::leaf(vec!['N', 'E']),
            Rec::fork(vec![
                Rec::leaf(vec!['W']),
                Rec::list(vec![
                    Rec::leaf(vec!['E']),
                    Rec::fork(vec![Rec::leaf(vec!['N']), Rec::leaf(vec!['S'])]),
                ]),
            ]),
            Rec::leaf(vec!['W', 'S']),
        ]);
        // println!("exp:\n{}", print_rec_tree(&exp).join("\n"));

        let rec = fetch_tree(&input);
        println!("rec:\n{}", print_rec_tree(&rec).join("\n"));
        println!("exp:\n{}", print_rec_tree(&exp).join("\n"));

        assert_eq!(rec, exp);
    }

    #[test]
    fn test_parse_tree1() {
        let input: Vec<char> = "^NN(EE|WW)SS$".chars().collect();
        assert_eq!(
            parse_tree(input),
            Node::node(vec![
                Node::leaf(vec!['N', 'N']),
                Node::node(vec![Node::leaf(vec!['E', 'E']), Node::leaf(vec!['W', 'W']),]),
                Node::leaf(vec!['S', 'S']),
            ])
        );
    }

    #[test]
    fn test_parse_tree2() {
        let input: Vec<char> = "^NEWS(SWEN|EWNS)$".chars().collect();
        assert_eq!(
            parse_tree(input),
            Node::node(vec![
                Node::leaf(vec!['N', 'E', 'W', 'S']),
                Node::node(vec![
                    Node::leaf(vec!['S', 'W', 'E', 'N']),
                    Node::leaf(vec!['E', 'W', 'N', 'S']),
                ]),
            ])
        );
    }

    #[test]
    fn test_parse_tree3_1() {
        let input: Vec<char> = "^NN(E|NEWS)SS$".chars().collect();
        assert_eq!(
            parse_tree(input),
            Node::node(vec![
                Node::leaf(vec!['N', 'N']),
                Node::node(vec![
                    Node::leaf(vec!['E']),
                    Node::leaf(vec!['N', 'E', 'W', 'S']),
                ]),
                Node::leaf(vec!['S', 'S']),
            ])
        );
    }

    #[test]
    fn test_parse_tree3_2() {
        let input: Vec<char> = "^NN(E|NEWS|S)SS$".chars().collect();
        assert_eq!(
            parse_tree(input),
            Node::node(vec![
                Node::leaf(vec!['N', 'N']),
                Node::node(vec![
                    Node::leaf(vec!['E']),
                    Node::leaf(vec!['N', 'E', 'W', 'S']),
                    Node::leaf(vec!['S'])
                ]),
                Node::leaf(vec!['S', 'S']),
            ])
        );
    }

    #[test]
    fn test_parse_tree3_3() {
        let input: Vec<char> = "^NN(E|NE(W|E)WS|S)SS$".chars().collect();
        assert_eq!(
            parse_tree(input),
            Node::node(vec![
                Node::leaf(vec!['N', 'N']),
                Node::node(vec![
                    Node::leaf(vec!['E']),
                    Node::leaf(vec!['N', 'E']),
                    Node::node(vec![Node::leaf(vec!['W']), Node::leaf(vec!['E']),]),
                    Node::leaf(vec!['W', 'S']),
                    Node::leaf(vec!['S'])
                ]),
                Node::leaf(vec!['S', 'S']),
            ])
        );
    }

    #[test]
    fn test_traverse() {
        // NN(E|W)SS
        let nn = Node::leaf(vec!['N', 'N']);
        let ss = Node::leaf(vec!['S', 'S']);
        let e = Node::leaf(vec!['E']);
        let w = Node::leaf(vec!['W']);
        let ew = Node::node(vec![e, w]);
        let root = Node::node(vec![nn, ew, ss]);

        let z = Pos::zero();
        let ps = traverse(root);
        assert_eq!(
            ps,
            vec![
                ('X', z),
                ('N', z.step('N')),
                ('N', z.step('N').step('N')),
                ('E', z.step('N').step('N').step('E')),
                ('W', z.step('N').step('N').step('W')),
                ('S', z.step('N').step('N').step('E').step('S')),
                ('S', z.step('N').step('N').step('E').step('S').step('S')),
                ('S', z.step('N').step('N').step('W').step('S')),
                ('S', z.step('N').step('N').step('W').step('S').step('S')),
            ]
        );
    }

    #[test]
    fn test_traverse_rec() {
        // NN(E|W)SS
        let rec = Rec::list(vec![
            Rec::leaf(vec!['N', 'N']),
            Rec::fork(vec![Rec::leaf(vec!['E']), Rec::leaf(vec!['W'])]),
            Rec::leaf(vec!['S', 'S']),
        ]);

        let z = Pos::zero();
        let ps = traverse_rec(&rec, z);
        assert_eq!(
            ps,
            vec![
                ('X', z),
                ('N', z.step('N')),
                ('N', z.step('N').step('N')),
                ('E', z.step('N').step('N').step('E')),
                ('W', z.step('N').step('N').step('W')),
                ('S', z.step('N').step('N').step('E').step('S')),
                ('S', z.step('N').step('N').step('E').step('S').step('S')),
                ('S', z.step('N').step('N').step('W').step('S')),
                ('S', z.step('N').step('N').step('W').step('S').step('S')),
            ]
        );
    }

    #[test]
    fn test_solve1() {
        let input = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$".to_string();
        let (grid, d) = solve(input);
        assert_eq!(d, 18);
        assert_eq!(
            dump(&grid),
            wrap(vec![
                "###########",
                "#.|.#.|.#.#",
                "#-###-#-#-#",
                "#.|.|.#.#.#",
                "#-#####-#-#",
                "#.#.#X|.#.#",
                "#-#-#####-#",
                "#.#.|.|.|.#",
                "#-###-###-#",
                "#.|.|.#.|.#",
                "###########",
            ])
        );
    }

    #[test]
    fn test_solve2_fetch_tree() {
        let input: Vec<char> = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"
            .chars()
            .collect();
        println!("{:?}", input);
        let rec = fetch_tree(&input);
        let exp = Rec::list(vec![
            Rec::leaf(vec!['E', 'S', 'S', 'W', 'W', 'N']),
            Rec::fork(vec![
                Rec::leaf(vec!['E']),
                Rec::list(vec![
                    Rec::leaf(vec!['N', 'N', 'E', 'N', 'N']),
                    Rec::fork(vec![
                        Rec::list(vec![
                            Rec::leaf(vec!['E', 'E', 'S', 'S']),
                            Rec::fork(vec![Rec::leaf(vec!['W', 'N', 'S', 'E']), Rec::leaf(vec![])]),
                            Rec::leaf(vec!['S', 'S', 'S']),
                        ]),
                        Rec::list(vec![
                            Rec::leaf(vec!['W', 'W', 'W', 'S', 'S', 'S', 'S', 'E']),
                            Rec::fork(vec![
                                Rec::leaf(vec!['S', 'W']),
                                Rec::leaf(vec!['N', 'N', 'N', 'E']),
                            ]),
                        ]),
                    ]),
                ]),
            ]),
        ]);
        println!("\nrec:\n{}", print_rec_tree(&rec).join("\n"));
        println!("\nexp:\n{}", print_rec_tree(&exp).join("\n"));

        assert_eq!(rec, exp);
        assert_eq!(print_rec_tree(&rec), print_rec_tree(&exp));
    }

    #[test]
    fn test_solve2() {
        let input = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$".to_string();
        let (grid, d) = solve(input);
        assert_eq!(
            dump(&grid),
            wrap(vec![
                "#############",
                "#.|.|.|.|.|.#",
                "#-#####-###-#",
                "#.#.|.#.#.#.#",
                "#-#-###-#-#-#",
                "#.#.#.|.#.|.#",
                "#-#-#-#####-#",
                "#.#.#.#X|.#.#",
                "#-#-#-###-#-#",
                "#.|.#.|.#.#.#",
                "###-#-###-#-#",
                "#.|.#.|.|.#.#",
                "#############",
            ])
        );
        assert_eq!(d, 23);
    }

    #[test]
    fn test_solve3() {
        let input = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$".to_string();
        let (grid, d) = solve(input);
        assert_eq!(
            dump(&grid),
            wrap(vec![
                "###############",
                "#.|.|.|.#.|.|.#",
                "#-###-###-#-#-#",
                "#.|.#.|.|.#.#.#",
                "#-#########-#-#",
                "#.#.|.|.|.|.#.#",
                "#-#-#########-#",
                "#.#.#.|X#.|.#.#",
                "###-#-###-#-#-#",
                "#.|.#.#.|.#.|.#",
                "#-###-#####-###",
                "#.|.#.|.|.#.#.#",
                "#-#-#####-#-#-#",
                "#.#.|.|.|.#.|.#",
                "###############",
            ])
        );
        assert_eq!(d, 31);
    }
}

fn solve(input: String) -> (Vec<Vec<char>>, usize, usize) {
    // let tree = parse_tree(input.chars().collect());
    // let cells = traverse(tree);
    let rec = fetch_tree(&input.chars().collect::<Vec<_>>());
    let cells = traverse_rec(&rec, Pos::zero());
    println!("cells: {}", cells.len());

    let size = get_size(&cells);
    println!("size: {:?}", size);

    let grid = make_grid(&size, &cells);
    println!("{}", dump(&grid).join("\n"));

    let dist = bfs(&grid, size.at(Pos::zero()));
    let d = max(&grid, &dist);
    let n = count(&grid, &dist, |x| x >= 1000);
    (grid, d, n)
}

pub fn main() {
    let input = get_input().expect("Failed to read input");
    println!("input: {}", input.len());

    let (_, d, n) = solve(input);
    println!("{}", d); // 3983
    println!("{}", n); // 8486
}
