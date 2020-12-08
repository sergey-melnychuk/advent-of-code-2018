extern crate regex;

use std::io;
use std::io::prelude::*;
use regex::Regex;

use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::BTreeMap;

fn get_input() -> Vec<String> {
    let mut items = Vec::new();
    let stdin = io::stdin();
    for row in stdin.lock().lines() {
        items.push(row.unwrap());
    }
    items
}

fn parse_line(line: &str) -> (char, char) {
    let mut result: (char, char) = ('0', '0');
    let re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();
    for cap in re.captures_iter(line) {
        result.0 = cap[1].chars().next().unwrap();
        result.1 = cap[2].chars().next().unwrap();
    }
    result
}

struct Graph {
    n: usize,
    connectivity: Vec<Vec<usize>>
}

impl Graph {
    fn new(n: usize) -> Graph {
        let mut connectivity = Vec::with_capacity(n);
        for _i in 0..n {
            connectivity.push(Vec::with_capacity(n / 5));
        }
        Graph { n, connectivity }
    }

    fn adj(&self, u: usize) -> &Vec<usize> {
        self.connectivity.get(u).unwrap()
    }

    fn edge(&mut self, u: usize, v: usize) {
        let list = self.connectivity.get_mut(u).unwrap();
        list.push(v);
    }

    // Number of edges FROM the node u
    fn source(&self, u: usize) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        for item in self.connectivity.get(u).unwrap() {
            result.push(*item);
        }
        result.sort();
        result
    }

    // Number of edges TO the node u
    fn target(&self, u: usize) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        for list in &self.connectivity {
            for item in list {
                if *item == u {
                    result.push(*item);
                }
            }
        }
        result.sort();
        result
    }
}

struct GraphTools {}

impl GraphTools {
    fn sort(graph: &mut Graph) {
        for list in &mut graph.connectivity {
            list.sort();
        }
    }

    fn topological(graph: &Graph, roots: &Vec<usize>) -> Vec<usize> {
        let mut ordered = Vec::with_capacity(graph.n);

        let mut temp: HashSet<usize> = HashSet::with_capacity(graph.n);
        let mut perm: HashSet<usize> = HashSet::with_capacity(graph.n);

        fn visit(node: usize, temp: &mut HashSet<usize>, perm: &mut HashSet<usize>, graph: &Graph, out: &mut Vec<usize>) {
            if !temp.contains(&node) && !perm.contains(&node) {
                temp.insert(node);
                for next in graph.adj(node) {
                    visit(*next, temp, perm, graph, out);
                }
                perm.insert(node);
                out.push(node);
            }
        };

        let mut other: Vec<usize> = Vec::with_capacity(graph.n);
        for i in 0..graph.n {
            if !roots.contains(&i) {
                other.push(i);
            }
        }

        for node in roots {
            if perm.contains(&node) {
                continue;
            }
            visit(*node, &mut temp, &mut perm, graph, &mut ordered);
        }

        for node in other {
            if perm.contains(&node) {
                continue;
            }
            visit(node, &mut temp, &mut perm, graph, &mut ordered);
        }

        ordered
    }
}

struct WorkerPool {
    n: usize,
    ticks: usize,
}

impl WorkerPool {
    fn new(n: usize) -> WorkerPool {
        WorkerPool { n, ticks: 0 }
    }
}

fn main() {
    println!("{:?}", parse_line("Step S must be finished before step B can begin.")); // (S, B)

    // 1. Build graph, 2. Apply Topological Sorting - NOPE!
    // 3. Seems like priority-queue based BFS works out instead

    let raw: Vec<(char, char)> = get_input().iter()
        .map(|x| parse_line(x))
        .collect();

    let input: Vec<(usize, usize)> = (&raw).into_iter()
        .map(|pair| (pair.0 as usize - 'A' as usize, pair.1 as usize - 'A' as usize))
        .collect();

    println!("input: {}", input.len());

    let mut seen: HashSet<usize> = HashSet::new();
    for (a, b) in &input {
        seen.insert(*a);
        seen.insert(*b);
    }

    let mut graph: Graph = Graph::new(seen.len());
    for (u, v) in &input {
        graph.edge(*u, *v);
    }

    for i in 0..graph.n {
        println!("{} in={} out={}", (i as u8 + 'A' as u8) as char, graph.target(i).len(), graph.source(i).len());
    }

    let mut roots: Vec<usize> = Vec::with_capacity(graph.n / 2);
    // find roots
    for (u, v) in &input {
        if graph.target(*u).is_empty() && !roots.contains(u) {
            roots.push(*u);
        }
        if graph.target(*v).is_empty() && !roots.contains(v) {
            roots.push(*v);
        }
    }
    println!("roots: {:?}", roots);
    println!("seen {} nodes: {:?}", seen.len(), seen);

    GraphTools::sort(&mut graph);
    let dag = GraphTools::topological(&graph, &roots);
    let chars: Vec<char> = dag.iter().map(|x| (*x as u8 + 'A' as u8) as char).collect();
    println!("{:?}", chars);

    // node -> [nodes waiting for the key]
    let mut blocks: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();

    // node -> [nodes that block the key]
    let mut blocked: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();

    for (u, v) in &input {
        if !blocks.contains_key(u) {
            blocks.insert(*u, BTreeSet::new());
        }
        blocks.get_mut(u).unwrap().insert(*v);

        if !blocked.contains_key(v) {
            blocked.insert(*v, BTreeSet::new());
        }
        blocked.get_mut(v).unwrap().insert(*u);
    }

    println!("blocked: {:?}", blocked);
    println!("blocks: {:?}", blocks);

    let mut ordered = Vec::with_capacity(graph.n);
    let mut queue: BTreeSet<usize> = BTreeSet::new();

    for r in &roots {
        queue.insert(*r);
    }
    while !queue.is_empty() {
        let step = *queue.iter().next().unwrap();
        let chr = (step as u8 + 'A' as u8) as char;
        queue.remove(&step);
        println!("step={} queue={:?} blocked={}", chr, queue, blocked.contains_key(&step));
        if !blocked.contains_key(&step) {
            ordered.push(step);

            if blocks.contains_key(&step) {
                for next in blocks.get(&step).unwrap() {
                    if blocked.contains_key(next) {
                        blocked.get_mut(next).unwrap().remove(&step);
                        if blocked.get(next).unwrap().is_empty() {
                            blocked.remove(next);
                        }
                    }
                    queue.insert(*next);
                }
            }
        }
    }

    println!("{:?}", ordered);
    for step in ordered {
        print!("{}", (step as u8 + 'A' as u8) as char);
    }
    println!();

    // part 2

    fn time(c: char) -> usize {
        (c as u8 - 'A' as u8) as usize + 61
    }

    let mut active: HashMap<char, usize> = HashMap::new();
    let mut clock: usize = 0;

    const MAX_ACTIVE_TASKS: usize = 5;

    let mut is_blocked_by: HashMap<char, HashSet<char>> = HashMap::new();
    let mut is_blocking: HashMap<char, HashSet<char>> = HashMap::new();
    for (u, v) in &raw {
        if !is_blocking.contains_key(u) {
            is_blocking.insert(*u, HashSet::new());
        }
        is_blocking.get_mut(u).unwrap().insert(*v);

        if !is_blocked_by.contains_key(v) {
            is_blocked_by.insert(*v, HashSet::new());
        }
        is_blocked_by.get_mut(v).unwrap().insert(*u);
    }

    let mut ready: HashSet<char> = HashSet::new();
    for r in &roots {
        ready.insert((*r as u8 + 'A' as u8) as char);
    }

    while !ready.is_empty() || !active.is_empty() {
        println!("\n{}", clock);
        println!("is_blocked_by: {:?}", is_blocked_by);
        println!("ready : {:?}", ready);
        println!("active: {:?}", active);

        let mut done = HashSet::new();
        for (c, t) in &active {
            if clock >= *t {
                done.insert(*c);

                if is_blocking.contains_key(c) {
                    for next in is_blocking.get(c).unwrap() {
                        if is_blocked_by.contains_key(next) {
                            is_blocked_by.get_mut(next).unwrap().remove(c);
                            if is_blocked_by.get(next).unwrap().is_empty() {
                                is_blocked_by.remove(next);
                                ready.insert(*next);
                            }
                        }
                    }
                }

            }
        }
        println!("done : {:?}", done);
        for d in done {
            active.remove(&d);
        }

        let mut start: HashSet<char> = HashSet::new();
        for r in &ready {
            // TODO Check if r is not blocked
            if is_blocked_by.contains_key(r) {
                continue;
            }

            if active.len() < MAX_ACTIVE_TASKS {
                start.insert(*r);
            }
        }
        println!("start: {:?}", start);
        for s in start {
            active.insert(s, clock + time(s));
            ready.remove(&s);
        }

        clock += 1;
    }

    println!("{}", clock-1);
}
