use std::fmt::Debug;
use std::io;
use std::io::prelude::*;

fn hash(value: [u8; 5]) -> usize {
    let mut result: usize = 0;
    for x in value.iter() {
        result <<= 1;
        result += *x as usize;
    }
    result
}

fn build_index(records: Vec<([u8; 5], u8)>) -> [u8; 32] {
    let mut result = [0; 32];
    for rec in records {
        let (pattern, value) = rec;
        let index = hash(pattern);
        result[index] = value;
    }
    result
}

fn parse_record(rec: &str) -> ([u8; 5], u8) {
    let mut bits = [0_u8; 5];
    let mut val = 0;

    let mut split = rec.split(" => ");
    for (i, c) in split.next().unwrap().chars().enumerate() {
        if c == '#' {
            bits[i] = 1;
        }
    }

    let out: char = split.next().unwrap().chars().next().unwrap();
    if out == '#' {
        val = 1;
    }

    (bits, val)
}

fn parse_state(line: &str) -> Vec<u8> {
    line.chars()
        .into_iter()
        .map(|c| u8::from(c == '#'))
        .collect()
}

// Window of size `size` and stride `stride` over input vector.
//
// window(input=[A B C D E F G], size=5, stride=4, zero=Z, f) =
// [
//  f(f(f(f(f(Z, Z), Z), Z), Z), A),
//  f(f(f(f(f(Z, Z), Z), Z), A), B),
//  f(f(f(f(f(Z, Z), Z), A), B), C),
//  f(f(f(f(f(Z, Z), A), B), C), D),
//  f(f(f(f(f(Z, A), B), C), D), E),
//  f(f(f(f(f(Z, B), C), D), E), F),
//  f(f(f(f(f(Z, C), D), E), F), G),
//  f(f(f(f(f(Z, D), E), F), G), Z),
//  f(f(f(f(f(Z, E), F), G), Z), Z),
//  f(f(f(f(f(Z, F), G), Z), Z), Z),
//  f(f(f(f(f(Z, G), Z), Z), Z), Z),
// ]
fn window<F, E: Clone + Debug, A: Clone + Debug>(
    input: Vec<E>,
    size: usize,
    stride: usize,
    zero_elem: E,
    zero_acc: A,
    reduce: F,
) -> Vec<A>
where
    F: Fn(A, E) -> A,
{
    let mut result = Vec::new();
    let mut expanded = vec![zero_elem.clone(); stride];
    expanded.extend(input);
    expanded.extend(vec![zero_elem; stride]);

    for start in 0..(expanded.len() - stride) {
        let mut acc = zero_acc.clone();
        for i in start..(start + size) {
            let e = expanded.get(i).unwrap().clone();
            acc = reduce(acc, e);
        }
        result.push(acc);
    }
    result
}

fn window_5bit(input: Vec<u8>, size: usize) -> Vec<u8> {
    let stride = size - 1;
    let zero: u8 = 0;
    let mask = (1_u8 << 5) - 1; // 0x1F = 00011111
    window(input, size, stride, zero, zero, |acc, e| {
        (acc << 1) & mask | e
    })
}

fn generation(state: Vec<u8>, window: usize, offset: isize, &index: &[u8; 32]) -> (Vec<u8>, isize) {
    let stride = window - 1;
    let matched = window_5bit(state, window);
    let updated = matched.into_iter().map(|i| index[i as usize]).collect();
    (updated, offset - (stride / 2) as isize)
}

fn read_input() -> (Vec<u8>, [u8; 32]) {
    let stdin = io::stdin();
    let mut state = Vec::new();
    let mut records = Vec::new();
    for row in stdin.lock().lines().enumerate() {
        let (i, line) = row;
        if i == 1 {
            state = parse_state(&line.unwrap());
        } else if i > 2 {
            records.push(parse_record(&line.unwrap()));
        }
    }

    (state, build_index(records))
}

fn trim(items: Vec<u8>, offset: isize) -> (Vec<u8>, isize) {
    let mut off = offset;
    let mut prefix_nonzero: usize = 0;
    let mut suffix_nonzero: usize = 0;
    for (i, item) in items.iter().enumerate() {
        if *item > 0 {
            if prefix_nonzero == 0 {
                prefix_nonzero = i;
            }
            suffix_nonzero = i;
        }
    }
    off += prefix_nonzero as isize;
    let mut trimmed = Vec::with_capacity(items.len());
    for i in prefix_nonzero..=suffix_nonzero {
        trimmed.push(*items.get(i).unwrap());
    }
    (trimmed, off)
}

fn run(initial: Vec<u8>, window: usize, generations: usize, index: &[u8; 32]) -> isize {
    let mut state = initial;
    let mut offset = 0;

    for i in 0..generations {
        let (st, off) = generation(state.clone(), window, offset, index);
        let (tr, cut) = trim(st, off);
        if tr == state {
            let _d = cut - offset;
            println!(
                "converged at: i={}, offset={}, cut={}, offset-cut={}",
                i,
                offset,
                cut,
                offset - cut
            );
            offset += (generations - i) as isize; // d=1 for the given input
            break;
        } else {
            state = tr;
            offset = cut;
        }
    }

    let mut sum: isize = 0;
    for (i, x) in state.into_iter().enumerate() {
        if x > 0 {
            sum += (i as isize) + offset;
        }
    }

    sum
}

pub fn main() {
    let window = 5;
    let (initial, index) = read_input();

    {
        let generations = 20; // 2767
        let sum = run(initial.clone(), window, generations, &index);
        println!("{}", sum);
    }

    {
        let generations = 50000000000;
        let sum = run(initial, window, generations, &index);
        println!("{}", sum); // 2650000001362
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash([0, 0, 0, 0, 0]), 0);
        assert_eq!(hash([0, 0, 0, 0, 1]), 1);
        assert_eq!(hash([0, 0, 0, 1, 0]), 2);
        assert_eq!(hash([0, 0, 0, 1, 1]), 3);
        assert_eq!(hash([0, 0, 1, 0, 0]), 4);
        assert_eq!(hash([0, 0, 1, 0, 1]), 5);
        assert_eq!(hash([0, 0, 1, 1, 0]), 6);
        assert_eq!(hash([0, 0, 1, 1, 1]), 7);
        assert_eq!(hash([0, 1, 0, 0, 0]), 8);
        assert_eq!(hash([0, 1, 0, 0, 1]), 9);
        assert_eq!(hash([0, 1, 0, 1, 0]), 10);
        assert_eq!(hash([0, 1, 0, 1, 1]), 11);
        assert_eq!(hash([0, 1, 1, 0, 0]), 12);
        assert_eq!(hash([0, 1, 1, 0, 1]), 13);
        assert_eq!(hash([0, 1, 1, 1, 0]), 14);
        assert_eq!(hash([0, 1, 1, 1, 1]), 15);
        assert_eq!(hash([1, 0, 0, 0, 0]), 16);
        assert_eq!(hash([1, 0, 0, 0, 1]), 17);
        assert_eq!(hash([1, 0, 0, 1, 0]), 18);
        assert_eq!(hash([1, 0, 0, 1, 1]), 19);
        assert_eq!(hash([1, 0, 1, 0, 0]), 20);
        assert_eq!(hash([1, 0, 1, 0, 1]), 21);
        assert_eq!(hash([1, 0, 1, 1, 0]), 22);
        assert_eq!(hash([1, 0, 1, 1, 1]), 23);
        assert_eq!(hash([1, 1, 0, 0, 0]), 24);
        assert_eq!(hash([1, 1, 0, 0, 1]), 25);
        assert_eq!(hash([1, 1, 0, 1, 0]), 26);
        assert_eq!(hash([1, 1, 0, 1, 1]), 27);
        assert_eq!(hash([1, 1, 1, 0, 0]), 28);
        assert_eq!(hash([1, 1, 1, 0, 1]), 29);
        assert_eq!(hash([1, 1, 1, 1, 0]), 30);
        assert_eq!(hash([1, 1, 1, 1, 1]), 31);
    }

    #[test]
    fn test_build_index() {
        assert_eq!(
            build_index(vec![
                ([0, 0, 0, 0, 1], 1),
                ([0, 0, 0, 1, 0], 1),
                ([0, 0, 1, 0, 0], 1),
                ([0, 1, 0, 0, 0], 1),
                ([1, 0, 0, 0, 0], 1),
                ([1, 1, 1, 1, 1], 1),
            ]),
            [
                0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, // 0-15
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 // 16-31
            ]
        );
    }

    #[test]
    fn test_parse_record() {
        assert_eq!(parse_record(".#.#. => #"), ([0, 1, 0, 1, 0], 1));
        assert_eq!(parse_record("..#.. => #"), ([0, 0, 1, 0, 0], 1));
        assert_eq!(parse_record("#.#.# => #"), ([1, 0, 1, 0, 1], 1));
        assert_eq!(parse_record("..... => #"), ([0, 0, 0, 0, 0], 1));
        assert_eq!(parse_record("##### => #"), ([1, 1, 1, 1, 1], 1));

        assert_eq!(parse_record(".#.#. => ."), ([0, 1, 0, 1, 0], 0));
        assert_eq!(parse_record("..#.. => ."), ([0, 0, 1, 0, 0], 0));
        assert_eq!(parse_record("#.#.# => ."), ([1, 0, 1, 0, 1], 0));
        assert_eq!(parse_record("..... => ."), ([0, 0, 0, 0, 0], 0));
        assert_eq!(parse_record("##### => ."), ([1, 1, 1, 1, 1], 0));
    }

    #[test]
    fn test_parse_records() {
        assert_eq!(
            parse_records(vec![".#.#. => #", "##### => .",]),
            vec![([0, 1, 0, 1, 0], 1), ([1, 1, 1, 1, 1], 0),]
        );
    }

    #[test]
    fn test_parse_state() {
        assert_eq!(
            parse_state("#..#.#..##......###...###"),
            vec![1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1]
        );
    }

    #[test]
    fn test_window_5bit() {
        let size = 3;
        let input = vec![1, 1, 1, 1];
        let expected = vec![1, 3, 7, 7, 6, 4];

        assert_eq!(window_5bit(input, 3), expected);
    }

    #[test]
    fn test_generation_small() {
        let state: Vec<u8> = vec![1, 0, 0, 1, 0, 0, 1];
        let expected: Vec<u8> = vec![1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1];
        let index = build_index(vec![
            ([0, 0, 0, 0, 1], 1),
            ([1, 0, 0, 0, 0], 1),
            ([0, 0, 1, 0, 0], 1),
        ]);

        let (actual, offset) = generation(state, 5, 0, &index);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_generation_1() {
        let initial = parse_state("#..#.#..##......###...###");
        let index = build_index(parse_records(vec![
            "...## => #",
            "..#.. => #",
            ".#... => #",
            ".#.#. => #",
            ".#.## => #",
            ".##.. => #",
            ".#### => #",
            "#.#.# => #",
            "#.### => #",
            "##.#. => #",
            "##.## => #",
            "###.. => #",
            "###.# => #",
            "####. => #",
        ]));
        let (state, offset) = generation(initial, 5, 0, &index);

        // input:                  __#..#.#..##......###...###__
        let expected = "..#...#....#.....#..#..#..#.."
            .chars()
            .into_iter()
            .map(|c| if c == '#' { 1 } else { 0 })
            .collect::<Vec<u8>>();

        assert_eq!(state, expected);
        assert_eq!(offset, -2);
    }

    #[test]
    fn test_trim() {
        assert_eq!(
            trim(vec![0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0], -1),
            (vec![1, 0, 0, 1], 2)
        );
    }

    #[test]
    fn test_run() {
        let initial = parse_state("#..#.#..##......###...###");
        let index = build_index(parse_records(vec![
            "...## => #",
            "..#.. => #",
            ".#... => #",
            ".#.#. => #",
            ".#.## => #",
            ".##.. => #",
            ".#### => #",
            "#.#.# => #",
            "#.### => #",
            "##.#. => #",
            "##.## => #",
            "###.. => #",
            "###.# => #",
            "####. => #",
        ]));

        assert_eq!(run(initial, 5, 20, &index), 325);
    }
}
