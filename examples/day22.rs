struct Pos {
    x: usize,
    y: usize,
}

// Geo Index:
// x=0 & y=0 -> 0
// y=0 -> x * 16807
// x=0 -> y * 48271
// [x-1] * [y-1] (erosion levels)
fn geo_index(grid: &Vec<Vec<u64>>, x: usize, y: usize) -> u64 {
    match (x, y) {
        (0, 0) => 0 as u64,
        (x, 0) => (x as u64) * 16807,
        (0, y) => (y as u64) * 48271,
        _ => grid[y-1][x] * grid[y][x-1]
    }
}

// Erosion Level:
// (geo index + depth) % 20183
fn erosion_level(gi: u64, d: u64) -> u64 {
    (gi + d) % 20183
}

fn make_grid(width: usize, height: usize, depth: u64) -> Vec<Vec<u64>> {
    let mut grid = vec![vec![0 as u64; width]; height];
    for y in 0..height {
        for x in 0..width {
            let gi = geo_index(&grid, x, y);
            let el = erosion_level(gi, depth);
            grid[y][x] = el;
            //println!("y={} x={} gi={} el={}", y, x, gi, el);
        }
    }
    grid
}

fn sum(grid: &Vec<Vec<u64>>, tx: usize, ty: usize) -> u64 {
    let mut result = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            result += grid[y][x] % 3;
        }
    }
    result -= grid[ty][tx] % 3;
    result
}

fn solve(target: Pos, depth: u64) -> u64 {
    let grid = make_grid(target.x + 1, target.y + 1, depth);
    sum(&grid, target.x, target.y)
}

pub fn main() {
    let depth = 11817;
    let target = Pos { x: 9, y: 751 };

    println!("{}", solve(target, depth)); // 7402
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(Pos { x: 10, y: 10 }, 510), 114);
    }
}
