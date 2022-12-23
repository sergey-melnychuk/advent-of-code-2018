#![allow(dead_code)]
use std::io;
use std::io::prelude::*;

use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

struct Field {
    chars: Vec<Vec<char>>,
}

impl Field {
    fn new(lines: Vec<String>) -> Field {
        let chars = lines
            .into_iter()
            .map(|line| line.chars().into_iter().collect())
            .collect();
        Field { chars }
    }

    fn get(&self, x: usize, y: usize) -> char {
        *self.chars.get(y).unwrap().get(x).unwrap()
    }

    //    fn mirror<A, F>(&self, f: F) -> Vec<Vec<A>>
    //        where F: Fn(usize, usize, char) -> A {
    //        let rows = self.chars.len();
    //        let mut result = Vec::with_capacity(rows);
    //        for y in 0..rows {
    //            let cols = self.chars[y].len();
    //            let mut items: Vec<A> = Vec::with_capacity(cols);
    //            for x in 0..cols {
    //                let val = f(x, y, self.chars[y][x]);
    //                items.push(val);
    //            }
    //            result.push(items);
    //        }
    //        result
    //    }

    //    fn reduce<A, F>(&self, f: F) -> Vec<A>
    //        where F: Fn(usize, usize, char) -> Option<A> {
    //        let rows = self.chars.len();
    //        let mut result = Vec::with_capacity(rows);
    //        for y in 0..rows {
    //            let cols = self.chars[y].len();
    //            for x in 0..cols {
    //                let val = f(x, y, self.chars[y][x]);
    //                if val.is_some() {
    //                    result.push(val.unwrap());
    //                }
    //            }
    //        }
    //        result
    //    }
}

// Each time a cart has the option to turn (by arriving at any intersection),
// - it turns left the first time,
// - goes straight the second time,
// - turns right the third time,
// and then repeats those directions starting again
fn step(
    x: usize,
    y: usize,
    dir: char,
    cell: char,
    turn: usize,
) -> Option<((usize, usize), char, usize)> {
    match (dir, cell, turn % 3) {
        ('>', '-', _) => Some(((x + 1, y), '>', turn)),
        ('>', '\\', _) => Some(((x, y + 1), 'v', turn)),
        ('>', '/', _) => Some(((x, y - 1), '^', turn)),
        ('>', '+', 0) => Some(((x, y - 1), '^', turn + 1)),
        ('>', '+', 1) => Some(((x + 1, y), '>', turn + 1)),
        ('>', '+', 2) => Some(((x, y + 1), 'v', turn + 1)),

        ('<', '-', _) => Some(((x - 1, y), '<', turn)),
        ('<', '\\', _) => Some(((x, y - 1), '^', turn)),
        ('<', '/', _) => Some(((x, y + 1), 'v', turn)),
        ('<', '+', 0) => Some(((x, y + 1), 'v', turn + 1)),
        ('<', '+', 1) => Some(((x - 1, y), '<', turn + 1)),
        ('<', '+', 2) => Some(((x, y - 1), '^', turn + 1)),

        ('^', '|', _) => Some(((x, y - 1), '^', turn)),
        ('^', '\\', _) => Some(((x - 1, y), '<', turn)),
        ('^', '/', _) => Some(((x + 1, y), '>', turn)),
        ('^', '+', 0) => Some(((x - 1, y), '<', turn + 1)),
        ('^', '+', 1) => Some(((x, y - 1), '^', turn + 1)),
        ('^', '+', 2) => Some(((x + 1, y), '>', turn + 1)),

        ('v', '|', _) => Some(((x, y + 1), 'v', turn)),
        ('v', '\\', _) => Some(((x + 1, y), '>', turn)),
        ('v', '/', _) => Some(((x - 1, y), '<', turn)),
        ('v', '+', 0) => Some(((x + 1, y), '>', turn + 1)),
        ('v', '+', 1) => Some(((x, y + 1), 'v', turn + 1)),
        ('v', '+', 2) => Some(((x - 1, y), '<', turn + 1)),

        (_, _, _) => None,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Cart {
    pos: Pos,
    dir: char,
    turn: usize,
}

impl Cart {
    fn go(&self, cell: char) -> Option<Cart> {
        step(self.pos.x, self.pos.y, self.dir, cell, self.turn).map(|((x, y), d, t)| Cart {
                pos: Pos { x, y },
                dir: d,
                turn: t,
            })
    }
}

fn get_input() -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        result.push(line.unwrap());
    }
    result
}

fn fetch_data(lines: Vec<String>) -> (Field, Vec<Cart>) {
    let mut chars: Vec<Vec<char>> = Vec::with_capacity(lines.len());
    let mut carts = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        let mut row = Vec::with_capacity(line.len());
        for (x, c) in line.chars().enumerate() {
            if c == '<' || c == '>' {
                carts.push(Cart {
                    pos: Pos { x, y },
                    dir: c,
                    turn: 0,
                });
                row.push('-');
            } else if c == '^' || c == 'v' {
                carts.push(Cart {
                    pos: Pos { x, y },
                    dir: c,
                    turn: 0,
                });
                row.push('|');
            } else {
                row.push(c);
            }
        }
        chars.push(row);
    }
    (Field { chars }, carts)
}

fn go(carts: Vec<Cart>, field: &Field) -> (Vec<Cart>, Vec<Pos>) {
    let mut result: Vec<Cart> = Vec::with_capacity(carts.len());
    let mut collisions: Vec<Pos> = Vec::new();

    let mut been: HashSet<Pos> = HashSet::new();
    for c in &carts {
        been.insert(c.pos);
    }

    let mut seen: HashSet<Pos> = HashSet::new();
    for cart in carts {
        if collisions.contains(&cart.pos) {
            continue;
        }
        let cell = field.get(cart.pos.x, cart.pos.y);
        let next = cart.go(cell).unwrap();
        let pos = next.pos;

        if seen.contains(&pos) {
            collisions.push(pos);
        } else {
            seen.insert(pos);
        }
        if !been.contains(&pos) {
            result.push(next);
        } else {
            collisions.push(pos);
        }
        been.remove(&cart.pos);
    }

    // Remove collided carts
    if !collisions.is_empty() {
        let mut remaining = Vec::new();
        for c in result {
            if !collisions.contains(&c.pos) {
                remaining.push(c);
            }
        }
        result = remaining;
    }

    result.sort_by_key(|c| c.pos.y * field.chars.len() + c.pos.x);
    (result, collisions)
}

pub fn main() {
    let (field, mut carts) = fetch_data(get_input());

    //let mut i = 0;
    loop {
        let (moved, collisions) = go(carts, &field);

        if !collisions.is_empty() {
            println!("collision: {:?}", collisions); // 38,57
                                                     //break;
        }

        if moved.len() == 1 {
            println!("last cart: {:?}", moved[0]); // 4,92
            break;
        }

        carts = moved;
        //println!("{}", i);
        //i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_field_new() {
        assert_eq!(
            Field::new(vec![
                String::from("abc"),
                String::from("def"),
                String::from("gh")
            ])
            .chars,
            vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f'], vec!['g', 'h']]
        );
    }

    #[test]
    fn test_field_get() {
        let field = Field::new(vec![
            String::from("abc"),
            String::from("def"),
            String::from("gh"),
        ]);
        assert_eq!(field.get(0, 0), 'a');
        assert_eq!(field.get(1, 0), 'b');
        assert_eq!(field.get(2, 0), 'c');
        assert_eq!(field.get(0, 1), 'd');
        assert_eq!(field.get(1, 1), 'e');
        assert_eq!(field.get(2, 1), 'f');
        assert_eq!(field.get(0, 2), 'g');
        assert_eq!(field.get(1, 2), 'h');
    }

    #[test]
    fn test_step_w() {
        assert_eq!(step(10, 10, '>', '-', 0), Some(((11, 10), '>', 0)));
        assert_eq!(step(10, 10, '>', '/', 0), Some(((10, 09), '^', 0)));
        assert_eq!(step(10, 10, '>', '\\', 0), Some(((10, 11), 'v', 0)));
        assert_eq!(step(10, 10, '>', '+', 0), Some(((10, 09), '^', 1)));
        assert_eq!(step(10, 10, '>', '+', 1), Some(((11, 10), '>', 2)));
        assert_eq!(step(10, 10, '>', '+', 2), Some(((10, 11), 'v', 3)));
    }

    #[test]
    fn test_step_e() {
        assert_eq!(step(10, 10, '<', '-', 0), Some(((09, 10), '<', 0)));
        assert_eq!(step(10, 10, '<', '/', 0), Some(((10, 11), 'v', 0)));
        assert_eq!(step(10, 10, '<', '\\', 0), Some(((10, 09), '^', 0)));
        assert_eq!(step(10, 10, '<', '+', 0), Some(((10, 11), 'v', 1)));
        assert_eq!(step(10, 10, '<', '+', 1), Some(((09, 10), '<', 2)));
        assert_eq!(step(10, 10, '<', '+', 2), Some(((10, 09), '^', 3)));
    }

    #[test]
    fn test_step_n() {
        assert_eq!(step(10, 10, '^', '|', 0), Some(((10, 09), '^', 0)));
        assert_eq!(step(10, 10, '^', '/', 0), Some(((11, 10), '>', 0)));
        assert_eq!(step(10, 10, '^', '\\', 0), Some(((09, 10), '<', 0)));
        assert_eq!(step(10, 10, '^', '+', 0), Some(((09, 10), '<', 1)));
        assert_eq!(step(10, 10, '^', '+', 1), Some(((10, 09), '^', 2)));
        assert_eq!(step(10, 10, '^', '+', 2), Some(((11, 10), '>', 3)));
    }

    #[test]
    fn test_step_s() {
        assert_eq!(step(10, 10, 'v', '|', 0), Some(((10, 11), 'v', 0)));
        assert_eq!(step(10, 10, 'v', '/', 0), Some(((09, 10), '<', 0)));
        assert_eq!(step(10, 10, 'v', '\\', 0), Some(((11, 10), '>', 0)));
        assert_eq!(step(10, 10, 'v', '+', 0), Some(((11, 10), '>', 1)));
        assert_eq!(step(10, 10, 'v', '+', 1), Some(((10, 11), 'v', 2)));
        assert_eq!(step(10, 10, 'v', '+', 2), Some(((09, 10), '<', 3)));
    }

    #[test]
    fn test_step_illegal() {
        assert_eq!(step(0, 0, 'v', '-', 0), None);
        assert_eq!(step(0, 0, '^', '-', 0), None);
        assert_eq!(step(0, 0, '>', '|', 0), None);
        assert_eq!(step(0, 0, '<', '|', 0), None);
    }

    #[test]
    fn test_fetch_data() {
        let (field, carts) = fetch_data(vec![
            String::from("  /->-\\ "),
            String::from("  |   | "),
            String::from("  ^   v "),
            String::from("  |   | "),
            String::from("  \\-<-/ "),
        ]);

        assert_eq!(
            field.chars,
            vec![
                vec![' ', ' ', '/', '-', '-', '-', '\\', ' '],
                vec![' ', ' ', '|', ' ', ' ', ' ', '|', ' '],
                vec![' ', ' ', '|', ' ', ' ', ' ', '|', ' '],
                vec![' ', ' ', '|', ' ', ' ', ' ', '|', ' '],
                vec![' ', ' ', '\\', '-', '-', '-', '/', ' '],
            ]
        );

        assert_eq!(
            carts,
            vec![
                Cart {
                    pos: Pos { x: 4, y: 0 },
                    dir: '>',
                    turn: 0
                },
                Cart {
                    pos: Pos { x: 2, y: 2 },
                    dir: '^',
                    turn: 0
                },
                Cart {
                    pos: Pos { x: 6, y: 2 },
                    dir: 'v',
                    turn: 0
                },
                Cart {
                    pos: Pos { x: 4, y: 4 },
                    dir: '<',
                    turn: 0
                },
            ]
        );
    }

    #[test]
    fn test_go() {
        let (field, carts0) = fetch_data(vec![
            String::from("  /---\\ "),
            String::from("  |   | "),
            String::from("  v   v "),
            String::from("  |   | "),
            String::from("  \\---/ "),
        ]);
        assert_eq!(
            carts0,
            vec![
                Cart {
                    pos: Pos { x: 2, y: 2 },
                    dir: 'v',
                    turn: 0
                },
                Cart {
                    pos: Pos { x: 6, y: 2 },
                    dir: 'v',
                    turn: 0
                },
            ]
        );

        let (carts1, c1) = go(carts0, &field);
        assert_eq!(
            carts1,
            vec![
                Cart {
                    pos: Pos { x: 2, y: 3 },
                    dir: 'v',
                    turn: 0
                },
                Cart {
                    pos: Pos { x: 6, y: 3 },
                    dir: 'v',
                    turn: 0
                },
            ]
        );
        assert_eq!(c1, vec![]);

        let (carts2, c2) = go(carts1, &field);
        assert_eq!(
            carts2,
            vec![
                Cart {
                    pos: Pos { x: 2, y: 4 },
                    dir: 'v',
                    turn: 0
                },
                Cart {
                    pos: Pos { x: 6, y: 4 },
                    dir: 'v',
                    turn: 0
                },
            ]
        );
        assert_eq!(c2, vec![]);

        let (carts3, c3) = go(carts2, &field);
        assert_eq!(
            carts3,
            vec![
                Cart {
                    pos: Pos { x: 3, y: 4 },
                    dir: '>',
                    turn: 0
                },
                Cart {
                    pos: Pos { x: 5, y: 4 },
                    dir: '<',
                    turn: 0
                },
            ]
        );
        assert_eq!(c3, vec![]);

        let (carts4, c4) = go(carts3, &field);
        assert_eq!(carts4, vec![]);
        assert_eq!(c4, vec![Pos { x: 4, y: 4 }]);
    }

    #[test]
    fn test_go_jump() {
        let (field, carts) = fetch_data(vec![String::from("-><-")]);
        assert_eq!(go(carts, &field), (vec![], vec![Pos { x: 2, y: 0 }]));
    }

    #[test]
    fn test_go_follow() {
        let (field, carts) = fetch_data(vec![String::from("-<<-")]);
        assert_eq!(
            go(carts, &field),
            (
                vec![
                    Cart {
                        pos: Pos { x: 0, y: 0 },
                        dir: '<',
                        turn: 0
                    },
                    Cart {
                        pos: Pos { x: 1, y: 0 },
                        dir: '<',
                        turn: 0
                    },
                ],
                vec![]
            )
        );
    }
}
