extern crate regex;

use std::io;
use std::io::prelude::*;

use std::collections::VecDeque;

fn get_input() -> Vec<usize> {
    let stdin = io::stdin();
    let mut line = String::new();
    let n = stdin.lock().read_line(&mut line).unwrap();
    println!("input bytes: {}", n);
    let mut result: Vec<usize> = Vec::new();
    for item in line.split_whitespace().into_iter() {
        result.push(item.parse().unwrap())
    }
    result
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Node {
    id: usize,
    n_children: usize, // number of child nodes
    n_metadata: usize, // number of metadata entries
    metadata: Vec<usize>,
    children: Vec<usize>,
}

impl Node {
    fn is_leaf(&self) -> bool {
        self.n_children == 0
    }

    fn is_full(&self) -> bool {
        self.n_children == self.children.len()
    }

    fn is_done(&self) -> bool {
        self.n_metadata == self.metadata.len()
    }

    fn metadata(&self) -> Vec<usize> {
        self.metadata.clone()
    }

    fn children(&self) -> Vec<usize> {
        self.children.clone()
    }
}

fn fetch_node(id: usize, offset: usize, stream: &Vec<usize>) -> (Node, usize) {
    let n_children = *stream.get(offset).unwrap();
    let n_metadata = *stream.get(offset + 1).unwrap();
    let mut node = Node {
        id,
        n_children,
        n_metadata,
        metadata: Vec::new(),
        children: Vec::new(),
    };
    let mut next = offset + 2;
    if n_children == 0 {
        next = fetch_metadata(&mut node, next, stream);
    }
    (node, next)
}

fn fetch_metadata(node: &mut Node, offset: usize, stream: &Vec<usize>) -> usize {
    for i in offset..(offset + node.n_metadata) {
        node.metadata.push(*stream.get(i).unwrap());
    }
    offset + node.n_metadata
}

fn fetch_all(stream: &Vec<usize>) -> Vec<Node> {
    let mut tree: Vec<Node> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();

    let (root, cutoff) = fetch_node(0, 0, stream);
    tree.push(root);
    stack.push(0);

    let mut offset: usize = cutoff;
    while offset < stream.len() {
        //println!();
        let is_parent_full = {
            let parent: &Node = tree.get(*stack.last().unwrap()).unwrap();
            parent.is_full()
        };

        if is_parent_full {
            let parent: &mut Node = tree.get_mut(*stack.last().unwrap()).unwrap();
            offset = fetch_metadata(parent, offset, stream);
            stack.pop();
            continue;
        }

        let id = tree.len();
        let (next, cutoff) = fetch_node(id, offset, stream);
        offset = cutoff;
        tree.push(next);
        //println!("node: {:?}", tree.last().unwrap());

        let is_done = tree.last().unwrap().is_done();

        let parent: &mut Node = tree.get_mut(*stack.last().unwrap()).unwrap();
        //println!("parent: {:?}", *parent);
        parent.children.push(id);

        if !is_done {
            stack.push(id);
        }

        //println!("offset: {}", offset);
    }

    tree
}

fn code(tree: &Vec<Node>, id: usize) -> usize {
    let mut result: usize = 0;
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(id);

    while !queue.is_empty() {
        let id = queue.pop_front().unwrap();
        //println!("id: {}", id);
        let node: &Node = tree.get(id).unwrap();
        if node.is_leaf() {
            for n in node.metadata() {
                result += n;
            }
        } else {
            for n in node.metadata() {
                if n <= node.n_children {
                    let index = *node.children().get(n - 1).unwrap();
                    queue.push_back(index);
                }
            }
        }
    }
    result
}

fn main() {
    let input = get_input();
    println!("input items: {}", input.len());

    let nodes = fetch_all(&input);

    let mut check: usize = 0;
    for node in nodes.clone() {
        for d in node.metadata {
            check += d;
        }
    }
    println!("check: {}", check);
    println!("code: {}", code(&nodes, 0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaf_node() {
        let input: Vec<usize> = vec![0, 3, 1, 2, 3];
        let node = Node {
            id: 0,
            n_children: 0,
            n_metadata: 3,
            metadata: vec![1, 2, 3],
            children: vec![],
        };

        assert_eq!(fetch_node(0, 0, &input), (node, input.len()));
    }

    #[test]
    fn test_two_nodes() {
        let input: Vec<usize> = vec![0, 3, 1, 2, 3, 0, 2, 10, 20];
        let n1 = Node {
            id: 0,
            n_children: 0,
            n_metadata: 3,
            metadata: vec![1, 2, 3],
            children: vec![],
        };
        let n2 = Node {
            id: 1,
            n_children: 0,
            n_metadata: 2,
            metadata: vec![10, 20],
            children: vec![],
        };

        assert_eq!(fetch_node(0, 0, &input), (n1, 5));
        assert_eq!(fetch_node(1, 5, &input), (n2, input.len()));
    }

    #[test]
    fn test_match_node_start() {
        let input: Vec<usize> = vec![2, 3, 0, 1, 201, 0, 1, 202, 101, 102, 103];
        let n = Node {
            id: 0,
            n_children: 2,
            n_metadata: 3,
            children: vec![],
            metadata: vec![],
        };

        assert_eq!(fetch_node(0, 0, &input), (n, 2));
    }

    #[test]
    fn fetch_root() {
        let root = Node {
            id: 0,
            n_children: 0,
            n_metadata: 1,
            children: vec![],
            metadata: vec![101],
        };
        assert_eq!(fetch_all(&vec![0, 1, 101]), vec![root]);
    }

    #[test]
    fn fetch_all_small() {
        let input = vec![3, 1, 0, 1, 201, 0, 1, 202, 0, 1, 203, 101];
        let tree = vec![
            Node {
                id: 0,
                n_children: 3,
                n_metadata: 1,
                metadata: vec![101],
                children: vec![1, 2, 3],
            },
            Node {
                id: 1,
                n_children: 0,
                n_metadata: 1,
                metadata: vec![201],
                children: vec![],
            },
            Node {
                id: 2,
                n_children: 0,
                n_metadata: 1,
                metadata: vec![202],
                children: vec![],
            },
            Node {
                id: 3,
                n_children: 0,
                n_metadata: 1,
                metadata: vec![203],
                children: vec![],
            },
        ];
        assert_eq!(fetch_all(&input), tree);
    }

    #[test]
    fn fetch_all_list() {
        let input = vec![1, 1, 1, 1, 1, 1, 0, 1, 401, 301, 201, 101];
        let tree = vec![
            Node {
                id: 0,
                n_children: 1,
                n_metadata: 1,
                metadata: vec![101],
                children: vec![1],
            },
            Node {
                id: 1,
                n_children: 1,
                n_metadata: 1,
                metadata: vec![201],
                children: vec![2],
            },
            Node {
                id: 2,
                n_children: 1,
                n_metadata: 1,
                metadata: vec![301],
                children: vec![3],
            },
            Node {
                id: 3,
                n_children: 0,
                n_metadata: 1,
                metadata: vec![401],
                children: vec![],
            },
        ];

        assert_eq!(fetch_all(&input), tree);
    }

    #[test]
    fn part1() {
        let input = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
        let tree = vec![
            Node {
                id: 0,
                n_children: 2,
                n_metadata: 3,
                metadata: vec![1, 1, 2],
                children: vec![1, 2],
            },
            Node {
                id: 1,
                n_children: 0,
                n_metadata: 3,
                metadata: vec![10, 11, 12],
                children: vec![],
            },
            Node {
                id: 2,
                n_children: 1,
                n_metadata: 1,
                metadata: vec![2],
                children: vec![3],
            },
            Node {
                id: 3,
                n_children: 0,
                n_metadata: 1,
                metadata: vec![99],
                children: vec![],
            },
        ];

        assert_eq!(fetch_all(&input), tree);
    }

    #[test]
    fn part2() {
        let input = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
        let tree = fetch_all(&input);
        assert_eq!(code(&tree, 0), 66);
    }
}
