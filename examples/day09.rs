extern crate regex;

//use std::io;
use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
struct Input {
    players: usize,
    marble: usize,
}

fn get_input(line: &str) -> Input {
    let mut players: usize = 0;
    let mut marble: usize = 0;

    let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    for cap in re.captures_iter(line) {
        players = cap[1].parse().unwrap();
        marble = cap[2].parse().unwrap();
    }

    Input { players, marble }
}

fn seek(marbles: &Vec<usize>, current: usize, offset: i64) -> usize {
    let n = marbles.len();
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        let i = (current as i64 + offset + n as i64) as usize % n;
        if i > 0 {
            i
        } else {
            n
        }
    }
}

fn play(input: &Input) -> Vec<usize> {
    let mut scores = vec![0; input.players];

    //    let mut marbles: Vec<usize> = Vec::with_capacity(input.marble + 1);
    //    marbles.push(0); // Initial marble
    let mut list = DLList::new(input.marble + 1);
    list.insert(0, 0);

    let mut player: usize; // Number of current player (1-based)
    let _current: usize = 0; // Index of current marble (0-based) in marbles vector

    for m in 1..=input.marble {
        player = (m - 1) % input.players;

        if m % 23 == 0 {
            //            let index = seek(&marbles, current, -7);
            //            let points0 = marbles.remove(index);
            //            current = index;
            let points = list.remove(-7);
            *scores.get_mut(player).unwrap() += points + m;
        } else {
            //            let index = seek(&marbles, current, 2);
            //            marbles.insert(index, m);
            //            current = index;
            list.insert(1, m);
        }

        //        let dbg = list.vec();
        //        println!("\nm:{} eq={}", m, marbles == dbg);
        //        println!("current: {}", current);
        //        println!("r:{:?}", marbles);
        //        println!("w:{:?}", dbg);
    }
    scores
}

fn max(scores: &Vec<usize>) -> usize {
    let mut max: usize = 0;
    for s in scores {
        if *s > max {
            max = *s;
        }
    }
    max
}

#[derive(Debug)]
struct DLList {
    cap: usize,
    index: usize,
    current: usize,
    data: Vec<usize>,    // data[0] = default value?
    forward: Vec<usize>, // forward[0] is head
    reverse: Vec<usize>, // reverse[0] is tail
}

impl DLList {
    fn new(cap: usize) -> DLList {
        DLList {
            cap,
            index: 0,
            current: 0,
            data: vec![0; cap + 1],
            forward: vec![0; cap + 1],
            reverse: vec![0; cap + 1],
        }
    }

    fn after(&self, index: usize) -> usize {
        *self.forward.get(index).unwrap()
    }

    fn before(&self, index: usize) -> usize {
        *self.reverse.get(index).unwrap()
    }

    fn slide(&self, offset: i64) -> usize {
        if self.current == 0 {
            0
        } else {
            let mut node = self.current;
            let mut steps = offset.abs();
            while steps > 0 {
                if offset > 0 {
                    node = self.after(node);
                } else {
                    node = self.before(node);
                }
                if node == 0 {
                    if offset > 0 {
                        // Next after last node is addressed, rewind to head
                        node = *self.forward.first().unwrap();
                    } else {
                        //TODO FIXME
                        // Prev to the head node is addressed, rewind to tail
                        node = *self.reverse.first().unwrap();
                    }
                }
                steps -= 1;
            }
            node
        }
    }

    fn insert(&mut self, offset: i64, value: usize) {
        self.index += 1;
        let this = self.index;
        *self.data.get_mut(this).unwrap() = value;

        //        println!("\ninsert: value={} this={}", value, this);

        let prev: usize = self.slide(offset);
        let next: usize = *self.forward.get(prev).unwrap();

        //        println!("prev: {:?}", prev);
        //        println!("next: {:?}", next);

        *self.forward.get_mut(this).unwrap() = next;
        *self.reverse.get_mut(this).unwrap() = prev;
        if next != 0 {
            *self.reverse.get_mut(next).unwrap() = this;
        }
        if prev != 0 {
            *self.forward.get_mut(prev).unwrap() = this;
        }
        self.current = this;

        // Set 'head' if empty
        if *self.forward.first().unwrap() == 0 {
            *self.forward.get_mut(0).unwrap() = this;
        }

        // Set 'tail'
        if next == 0 {
            *self.reverse.get_mut(0).unwrap() = this;
        }

        //        println!("fw: {:?}", self.forward);
        //        println!("rv: {:?}", self.reverse);
        //        println!("data: {:?}", self.data);
    }

    fn remove(&mut self, offset: i64) -> usize {
        let this = self.slide(offset);
        let value = *self.data.get(this).unwrap();

        let next: usize = *self.forward.get(this).unwrap();
        let prev: usize = *self.reverse.get(this).unwrap();

        *self.forward.get_mut(prev).unwrap() = next;
        *self.reverse.get_mut(next).unwrap() = prev;

        self.current = next;
        value
    }

    fn reduce<F, ACC>(&self, zero: ACC, f: F) -> ACC
    where
        F: Fn(ACC, usize) -> ACC,
    {
        let head: usize = *self.forward.first().unwrap();
        //println!("reduce: head={}", head);

        let mut next = head;
        let mut acc: ACC = zero;
        while next != 0 {
            //println!("reduce: next={}", next);
            let current: usize = *self.data.get(next).unwrap();
            let reduced = f(acc, current);
            acc = reduced;
            next = *self.forward.get(next).unwrap();
        }
        acc
    }

    fn vec(&self) -> Vec<usize> {
        let mut vec: Vec<usize> = Vec::with_capacity(self.cap);
        self.reduce(&mut vec, |acc: &mut Vec<usize>, x: usize| {
            acc.push(x);
            acc
        });
        vec
    }
}

fn main() {
    //    let mut line = String::new();
    //    let n_bytes = io::stdin().read_line(&mut line).unwrap();
    //    println!("input: {} bytes", n_bytes);

    // This is the whole input for the task
    let line = "426 players; last marble is worth 72058 points";
    let input = get_input(line);
    println!("{:?}", input);

    let scores = play(&input);
    let score = max(&scores);
    println!("max: {}", score); // max: 424112

    let new_input = Input {
        marble: input.marble * 100,
        ..input
    };
    println!("{:?}", new_input);
    let new_score = max(&play(&new_input));
    println!("max: {}", new_score);
}

#[cfg(test)]
mod list {
    use super::*;

    #[test]
    fn game_steps() {
        let n = 25;
        let mut list: DLList = DLList::new(n + 1);
        list.insert(0, 0);

        let state = vec![
            vec![0],
            vec![0, 1],
            vec![0, 2, 1],
            vec![0, 2, 1, 3],
            vec![0, 4, 2, 1, 3],
            vec![0, 4, 2, 5, 1, 3],
            vec![0, 4, 2, 5, 1, 6, 3],
            vec![0, 4, 2, 5, 1, 6, 3, 7],
            vec![0, 8, 4, 2, 5, 1, 6, 3, 7],
            vec![0, 8, 4, 9, 2, 5, 1, 6, 3, 7],
            vec![0, 8, 4, 9, 2, 10, 5, 1, 6, 3, 7],
            vec![0, 8, 4, 9, 2, 10, 5, 11, 1, 6, 3, 7],
            vec![0, 8, 4, 9, 2, 10, 5, 11, 1, 12, 6, 3, 7],
            vec![0, 8, 4, 9, 2, 10, 5, 11, 1, 12, 6, 13, 3, 7],
            vec![0, 8, 4, 9, 2, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7],
            vec![0, 8, 4, 9, 2, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15],
            vec![0, 16, 8, 4, 9, 2, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15],
            vec![0, 16, 8, 17, 4, 9, 2, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15],
            vec![
                0, 16, 8, 17, 4, 18, 9, 2, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15,
            ],
            vec![
                0, 16, 8, 17, 4, 18, 9, 19, 2, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15,
            ],
            vec![
                0, 16, 8, 17, 4, 18, 9, 19, 2, 20, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15,
            ],
            vec![
                0, 16, 8, 17, 4, 18, 9, 19, 2, 20, 10, 21, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15,
            ],
            vec![
                0, 16, 8, 17, 4, 18, 9, 19, 2, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15,
            ],
            vec![
                0, 16, 8, 17, 4, 18, 19, 2, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15,
            ],
            vec![
                0, 16, 8, 17, 4, 18, 19, 2, 24, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15,
            ],
            vec![
                0, 16, 8, 17, 4, 18, 19, 2, 24, 20, 25, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7,
                15,
            ],
        ];

        assert_eq!(list.vec(), *state.get(0).unwrap());

        for m in 1..=(state.len() - 1) {
            println!("m={}", m);
            if m % 23 == 0 {
                assert_eq!(list.remove(-7), 9);
            } else {
                list.insert(1, m);
            }
            assert_eq!(list.vec(), *state.get(m).unwrap());
        }
    }

    #[test]
    fn new() {
        let cap: usize = 100500;
        let list: DLList = DLList::new(cap);
        assert_eq!(list.cap, cap);
        assert_eq!(list.index, 0);
        assert_eq!(list.current, 0);
        assert_eq!(list.data.len(), cap + 1);
        assert_eq!(list.forward.len(), cap + 1);
        assert_eq!(list.reverse.len(), cap + 1);
    }

    #[test]
    fn insert_one() {
        let mut list = DLList::new(10);
        list.insert(0, 101);

        assert_eq!(list.vec(), vec![101]);
    }

    #[test]
    fn insert_two() {
        let mut list = DLList::new(10);
        list.insert(0, 0);
        list.insert(2, 1);

        assert_eq!(list.vec(), vec![0, 1]);
    }

    #[test]
    fn insert_n() {
        let N = 100;
        let mut list = DLList::new(N);
        for i in 1..=N {
            list.insert(0, i);
        }

        let exp: Vec<usize> = (1..=N).collect();
        assert_eq!(list.vec(), exp);
    }

    #[test]
    fn insert_small() {
        let mut list = DLList::new(10);
        list.insert(0, 1);
        list.insert(0, 2);
        list.insert(0, 3);

        assert_eq!(list.vec(), vec![1, 2, 3]);
    }

    #[test]
    fn insert_mid() {
        let mut list = DLList::new(10);
        list.insert(0, 0);
        list.insert(1, 1);
        list.insert(1, 2);

        assert_eq!(list.vec(), vec![0, 2, 1]);
    }

    fn assert_size(n: usize) {
        let mut list = DLList::new(n);
        for i in 0..n {
            list.insert(0, 1);

            if i > 100 && i % 10 == 0 {
                list.remove(-7);
            }
        }

        let sum = list.reduce(0, |acc: usize, val: usize| acc + val);
        assert_eq!(sum, n - (n - 100) / 10 + 1);
    }

    #[test]
    fn remove_one() {
        let mut list = DLList::new(10);
        list.insert(0, 1);
        list.insert(0, 2);
        list.insert(0, 3);
        assert_eq!(list.vec(), vec![1, 2, 3]);

        assert_eq!(list.remove(-1), 2);
        assert_eq!(list.vec(), vec![1, 3]);

        list.insert(0, 10);
        assert_eq!(list.vec(), vec![1, 3, 10]);

        list.insert(-2, 20);
        assert_eq!(list.vec(), vec![1, 20, 3, 10]);

        assert_eq!(list.remove(0), 20);
        assert_eq!(list.vec(), vec![1, 3, 10]);
    }

    #[test]
    fn test_repro_case() {
        // Remove with offset = -7 and current < 7
        let mut list = DLList::new(10);
        list.insert(0, 1);
        list.insert(0, 2);
        list.insert(0, 3);
        list.insert(0, 4);
        list.insert(0, 5);
        list.insert(0, 6);
        list.insert(0, 7);
        list.insert(0, 8);
        list.insert(0, 9);
        assert_eq!(list.vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        list.insert(-7, 100);
        assert_eq!(list.vec(), vec![1, 2, 100, 3, 4, 5, 6, 7, 8, 9]);

        assert_eq!(list.remove(-7), 5);
        assert_eq!(list.vec(), vec![1, 2, 100, 3, 4, 6, 7, 8, 9]);
    }

    //    #[test]
    //    fn insert_1k() {
    //        assert_size(1000);
    //    }
    //
    //    #[test]
    //    fn insert_100k() {
    //        assert_size(100000);
    //    }
    //
    //    #[test]
    //    fn insert_10m() {
    //        assert_size(10000000);
    //    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let line = "426 players; last marble is worth 72058 points";
        let input = Input {
            players: 426,
            marble: 72058,
        };
        assert_eq!(get_input(line), input);
    }

    #[test]
    fn test_play_example() {
        assert_eq!(
            play(&Input {
                players: 9,
                marble: 25
            }),
            vec![0, 0, 0, 0, 32, 0, 0, 0, 0,]
        )
    }

    #[test]
    fn test_play_10() {
        assert_eq!(
            max(&play(&Input {
                players: 10,
                marble: 1618
            })),
            8317
        );
    }

    #[test]
    fn test_play_13() {
        assert_eq!(
            max(&play(&Input {
                players: 13,
                marble: 7999
            })),
            146373
        );
    }

    #[test]
    fn test_play_17() {
        assert_eq!(
            max(&play(&Input {
                players: 17,
                marble: 1104
            })),
            2764
        );
    }

    #[test]
    fn test_play_21() {
        assert_eq!(
            max(&play(&Input {
                players: 21,
                marble: 6111
            })),
            54718
        );
    }

    #[test]
    fn test_play_30() {
        assert_eq!(
            max(&play(&Input {
                players: 30,
                marble: 5807
            })),
            37305
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            max(&play(&Input {
                players: 426,
                marble: 72058
            })),
            424112
        );
    }

    //    #[test]
    //    fn test_seek_clockwise_0() {
    //        assert_eq!(seek(&vec![],0, 2), 0);
    //    }
    //
    //    #[test]
    //    fn test_seek_clockwise_1() {
    //        assert_eq!(seek(&vec![0],0, 2), 1);
    //    }
    //
    //    #[test]
    //    fn test_seek_clockwise_2() {
    //        assert_eq!(seek(&vec![0, 1],1, 2), 1);
    //    }
    //
    //    #[test]
    //    fn test_seek_clockwise_3() {
    //        assert_eq!(seek(&vec![0, 2, 1],1, 2), 3);
    //    }
    //
    //    #[test]
    //    fn test_seek_clockwise_4() {
    //        assert_eq!(seek(&vec![0, 2, 1, 3],3, 2), 1);
    //    }
    //
    //    #[test]
    //    fn test_seek_clockwise_5() {
    //        assert_eq!(seek(&vec![0, 4, 2, 1, 3],1, 2), 3);
    //    }
    //
    //    #[test]
    //    fn test_seek_clockwise_6() {
    //        assert_eq!(seek(&vec![0, 4, 2, 5, 1, 3],3, 2), 5);
    //    }
    //
    //    #[test]
    //    fn test_seek_clockwise_7() {
    //        assert_eq!(seek(&vec![0, 4, 2, 5, 1, 6, 3],5, 2), 7);
    //    }
    //
    //    #[test]
    //    fn test_seek_counter_clockwise22() {
    //        assert_eq!(
    //            seek(&vec![
    //                0, 16,  8, 17,  4, 18,  9, 19,  2, 20, 10, 21,  5, 22, 11,  1, 12,  6, 13,  3, 14,  7, 15,
    //            ], 13, -7),
    //            6
    //        )
    //    }
}
