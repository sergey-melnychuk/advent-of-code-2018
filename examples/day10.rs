// extern crate regex;
// use regex::Regex;

use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct V2 {
    x: i64,
    y: i64
}

#[derive(Debug)]
struct Star {
    pos: V2,
    vel: V2
}

#[derive(Debug)]
struct Size {
    min: V2,
    max: V2
}

impl Size {
    fn square(&self) -> usize {
        (self.max.x - self.min.x) as usize * (self.max.y - self.min.y) as usize
    }

    fn width(&self) -> usize {
        (self.max.x - self.min.x) as usize
    }

    fn height(&self) -> usize {
        (self.max.y - self.min.y) as usize
    }
}

// fn get_input() -> Vec<Star> {
//     let mut result = Vec::new();
//     let re = Regex::new(r"position=<\s?(\-?\d+),\s+(\-?\d+)> velocity=<\s?(\-?\d+),\s+(\-?\d+)>").unwrap();
//     let stdin = io::stdin();
//     for line in stdin.lock().lines() {
//         for cap in re.captures_iter(&line.unwrap()) {
//             result.push(Star { pos: {
//                 V2 {x: cap[1].parse().unwrap(), y: cap[2].parse().unwrap()}
//             }, vel: {
//                 V2 {x: cap[3].parse().unwrap(), y: cap[4].parse().unwrap()}
//             } });
//         }
//     }
//     result
// }

fn parse_line(line: &str) -> (i64, i64, i64, i64) {
    let mut s0 = line.split("<");
    s0.next();
    let mut s1 = s0.next().unwrap()
        .split(">").next().unwrap().split(",");
    let x1: i64 = s1.next().unwrap().trim().parse().unwrap();
    let y1: i64 = s1.next().unwrap().trim().parse().unwrap();

    let mut s2 = s0.next().unwrap()
        .split(">").next().unwrap().split(",");
    let x2: i64 = s2.next().unwrap().trim().parse().unwrap();
    let y2: i64 = s2.next().unwrap().trim().parse().unwrap();

    (x1, y1, x2, y2)
}

fn fetch_input() -> Vec<Star> {
    let mut result = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let row = &line.unwrap();
        if row.is_empty() {
            break;
        }
        let (px, py, vx, vy) = parse_line(&row);
        result.push(Star {
            pos: V2 {x: px, y: py},
            vel: V2 {x: vx, y: vy} });
        println!("{:?}", (px, py, vx, vy));
    }
    result
}

fn move_stars(stars: &Vec<Star>, seconds: isize) -> (Vec<(i64, i64)>, Size) {
    let (mut xmin, mut xmax, mut ymin, mut ymax) = (std::i64::MAX, std::i64::MIN, std::i64::MAX, std::i64::MIN);
    let mut result = Vec::new();
    for star in stars {
        let x = star.pos.x + seconds as i64 * star.vel.x;
        let y = star.pos.y + seconds as i64 * star.vel.y;
        result.push((x, y));

        if x > xmax { xmax = x; }
        if x < xmin { xmin = x; }
        if y > ymax { ymax = y; }
        if y < ymin { ymin = y; }
    }
    (result, Size { min: V2 { x: xmin, y: ymin } , max: V2{ x: xmax, y: ymax } })
}

fn main() {
    let lines = vec![
        "xxxx<1,2>yyyy<3,4>zzzz",
        "position=< 43869, -10792> velocity=<-4,  1>"
    ];
    for line in lines {
        println!("'{}' -> {:?}", line, parse_line(line));
    }

    let input = fetch_input();
    println!("input: {} records", input.len());

    let (mut dots, mut size) = move_stars(&input, 0);
    println!("size: {:?}", size);

    let mut current: usize = std::usize::MAX;
    let mut seconds: isize = 0;
    loop {
        let (ds, sz) = move_stars(&input, seconds);
        let square = sz.square();
        if square < current {
            current = square;
            seconds += 1;
            dots = ds;
            size = sz;
        } else {
            break;
        }
        println!("sec:{} sq:{} sz:{:?}", seconds, square, size);
    }

    println!("{:?}", dots);

    let width = size.width();
    let height = size.height();
    let mut matrix = vec![vec![' '; width + 1]; height + 1];

    for dot in dots {
        let (x, y) = dot;
        let row = (y - size.min.y) as usize;
        let col = (x - size.min.x) as usize;
        *matrix.get_mut(row).unwrap().get_mut(col).unwrap() = '#';
    }

    for i in 0..=height {
        for j in 0..=width {
            print!("{}", matrix.get(i).unwrap().get(j).unwrap());
        }
        println!();
    }

    // KZHGRJGZ
    // 10932
}
