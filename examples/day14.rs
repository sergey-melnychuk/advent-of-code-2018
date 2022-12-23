fn digits(number: usize) -> Vec<usize> {
    if number < 10 {
        vec![number]
    } else {
        vec![1, number - 10]
    }
}

fn answer(mut state: Vec<usize>, count: usize, query: usize) -> Vec<usize> {
    let mut fst = 0;
    let mut snd = 1;

    loop {
        let one = state[fst];
        let two = state[snd];

        let ds = digits(one + two);
        for d in ds {
            state.push(d);
        }
        let len = state.len();

        fst = (fst + one + 1) % len;
        snd = (snd + two + 1) % len;

        if len >= count + query {
            break;
        }
    }

    let mut result = vec![];
    for item in state.iter().skip(count).take(query) {
        result.push(*item);
    }
    result
}

fn checksum(items: &[usize], offset: usize, length: usize) -> usize {
    if items.len() < offset + length {
        0
    } else {
        let mut acc = 0;
        for item in items.iter().skip(offset).take(length) {
            let val = acc * 10 + item;
            acc = val;
        }
        acc
    }
}

fn reverse(mut state: Vec<usize>, mask: Vec<usize>) -> usize {
    let mask_checksum = checksum(&mask, 0, mask.len());
    //println!("\nmask: {}", mask_checksum);

    let mut divider: usize = 1;
    for _ in 0..mask.len() {
        divider *= 10;
    }
    divider /= 10;

    let mut sum: usize = checksum(&state, 0, state.len());

    let mut fst = 0;
    let mut snd = 1;
    let mut len = state.len();

    'outer: loop {
        let one = state[fst];
        let two = state[snd];

        let ds = digits(one + two);
        for d in ds {
            state.push(d);
            len = state.len();

            let val = sum % divider;
            sum = val * 10 + d;

            if sum == mask_checksum {
                break 'outer;
            }
        }

        fst = (fst + one + 1) % len;
        snd = (snd + two + 1) % len;
    }

    //println!("state={:?} sum={}", state, sum);
    state.len() - mask.len()
}

pub fn main() {
    let count: usize = 0o030121; // input
    let query: usize = 10; // 10 recepies after 'count' of recepies is available

    let state: Vec<usize> = vec![3, 7];

    {
        let result = answer(state.clone(), count, query);
        println!("{:?}", result); // 5101271252
    }
    {
        let result = reverse(state, vec![0, 3, 0, 1, 2, 1]);
        println!("{}", result); // 20287556
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(
            answer(vec![3, 7], 5, 10),
            vec![0, 1, 2, 4, 5, 1, 5, 8, 9, 1]
        );
        assert_eq!(
            answer(vec![3, 7], 9, 10),
            vec![5, 1, 5, 8, 9, 1, 6, 7, 7, 9]
        );
        assert_eq!(
            answer(vec![3, 7], 18, 10),
            vec![9, 2, 5, 1, 0, 7, 1, 0, 8, 5]
        );
        assert_eq!(
            answer(vec![3, 7], 2018, 10),
            vec![5, 9, 4, 1, 4, 2, 9, 8, 8, 2]
        );
    }

    #[test]
    fn test_checksum() {
        assert_eq!(checksum(&vec![5, 1, 5, 8, 9], 3, 5), 0);
        assert_eq!(checksum(&vec![5, 1, 5, 8, 9], 0, 5), 51589);
        assert_eq!(checksum(&vec![5, 9, 4, 1, 4], 0, 5), 59414);
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(vec![3, 7], vec![5, 1, 5, 8, 9]), 9);
        assert_eq!(reverse(vec![3, 7], vec![0, 1, 2, 4, 5]), 5);
        assert_eq!(reverse(vec![3, 7], vec![9, 2, 5, 1, 0]), 18);
        assert_eq!(reverse(vec![3, 7], vec![5, 9, 4, 1, 4]), 2018);
    }
}
