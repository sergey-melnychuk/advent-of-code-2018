
fn fuel(x: i64, y: i64, nr: i64) -> i64 {
    let rack = x + 10;
    let power = rack * y;
    let val = (power + nr) * rack;

    let hundreds_digit = (val / 100) % 10;
    hundreds_digit - 5
}

fn conv(x: usize, y: usize, size: usize, mat: &Vec<Vec<i64>>) -> i64 {
    let mut sum = 0;
    for i in x..(x+size) {
        for j in y..(y+size) {
            sum += mat.get(i).unwrap().get(j).unwrap();
        }
    }
    sum
}

fn window(row: &Vec<i64>, size: usize) -> Vec<i64> {
    let mut window: Vec<i64> = vec![0; row.len() - size + 1];
    let mut first = 0;
    for i in 0..size {
        first += row.get(i).unwrap();
    }
    *window.get_mut(0).unwrap() = first;
    for step in 1..window.len() {
        let prev: i64 = *window.get(step - 1).unwrap();
        let this = prev - row.get(step - 1).unwrap() + row.get(step + size - 1).unwrap();
        *window.get_mut(step).unwrap() = this;
    }
    window
}

fn transpose(mat: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let rows = mat.len();
    let cols = mat.get(0).unwrap().len();
    let mut result = vec![vec![0; rows]; cols];
    for i in 0..cols {
        for j in 0..rows {
            *result.get_mut(i).unwrap().get_mut(j).unwrap() =
                *mat.get(j).unwrap().get(i).unwrap();
        }
    }
    result
}

fn window2d(mat: &Vec<Vec<i64>>, hsize: usize, vsize: usize) -> Vec<Vec<i64>> {
    let mut hwindow: Vec<Vec<i64>> = Vec::new();
    for i in 0..mat.len() {
        hwindow.push(window(mat.get(i).unwrap(), hsize));
    }

    let transposed: Vec<Vec<i64>> = transpose(&hwindow);

    let mut vwindow: Vec<Vec<i64>> = Vec::new();
    for i in 0..transposed.len() {
        vwindow.push(window(transposed.get(i).unwrap(), vsize));
    }

    transpose(&vwindow)
}

fn main() {
    let nr: i64 = 7511;
    let len: usize = 300;

    let mut cells: Vec<Vec<i64>> = vec![vec![0; len]; len];

    for i in 0..len {
        for j in 0..len {
            *cells.get_mut(i).unwrap().get_mut(j).unwrap() =
                fuel((i+1) as i64, (j+1) as i64, nr);
        }
    }

    let (mut maxx, mut maxy) = (0, 0);
    let mut max = std::i64::MIN;
    for i in 0..(len-3) {
        for j in 0..(len-3) {
            let c = conv(i, j, 3, &cells);
            if c > max {
                max = c;
                maxx = i + 1;
                maxy = j + 1;
            }
        }
    }
    println!("{},{}", maxx, maxy);

    let mut maxsize = 0;
    max = 0;
    for size in 1..len {
        //println!("size: {}", size);
        let fuels = window2d(&cells, size, size);
        for i in 0..fuels.len() {
            let row = fuels.get(i).unwrap();
            for j in 0..row.len() {
                let f = *row.get(j).unwrap();
                if f > max {
                    max = f;
                    maxx = i + 1;
                    maxy = j + 1;
                    maxsize = size;
                }
            }
        }
    }
    println!("{},{},{}", maxx, maxy, maxsize);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fuel() {
        assert_eq!(fuel(122, 79, 57), -5);
        assert_eq!(fuel(217, 196, 39), 0);
        assert_eq!(fuel(101, 153, 71), 4);
    }

    #[test]
    fn test_window() {
        assert_eq!(window(&vec![1, 1, 1, 1, 1, 1, 1, 1, 1], 3),
                   vec![3, 3, 3, 3, 3, 3, 3]);

        assert_eq!(window(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3),
                   vec![6, 9, 12, 15, 18, 21, 24]);

        assert_eq!(window(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 9),
                   vec![45]);
    }

    #[test]
    fn test_transpose_3x3() {
        let mat = vec![
            vec![1, 2, 3],
            vec![1, 2, 3],
            vec![1, 2, 3],
        ];
        let exp = vec![
            vec![1, 1, 1],
            vec![2, 2, 2],
            vec![3, 3, 3],
        ];
        assert_eq!(transpose(&mat), exp);
    }

    #[test]
    fn test_transpose_2x4() {
        let mat = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8]
        ];
        let exp = vec![
            vec![1, 5],
            vec![2, 6],
            vec![3, 7],
            vec![4, 8]
        ];
        assert_eq!(transpose(&mat), exp);
    }
}

/*
e$ time ./target/release/advent-of-code
21,22
235,288,13

real    0m0.144s
user    0m0.120s
sys     0m0.024s
*/
