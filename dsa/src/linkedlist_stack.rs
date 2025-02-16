/*~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~*
 * THIS TEMPLATE IS MADE BY MISKATUL ANWAR<miskatul.anwar.csecu@gmail.com> *
 *               GITHUB : https://github.com/miskatul-anwar                *
 *                                THANK YOU                                *
 *~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~*/

#![allow(unused)]

use std::io::{stdin, BufRead};

macro_rules! read_line {
    () => {{
        let mut input = String::new();
        stdin().lock().read_line(&mut input).unwrap();
        input
    }};
}

macro_rules! rin {
    ($t:ty) => {{
        let mut input = String::new();
        stdin().lock().read_line(&mut input).unwrap();
        input.trim().parse::<$t>().unwrap()
    }};
}

macro_rules! rin_vec {
    ($t:ty) => {{
        let mut input = String::new();
        stdin().lock().read_line(&mut input).unwrap();
        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<$t>().unwrap())
            .collect::<Vec<$t>>()
    }};
}

macro_rules! rin_vec_char {
    () => {{
        let mut input = String::new();
        stdin().lock().read_line(&mut input).unwrap();
        input.trim().chars().collect::<Vec<char>>()
    }};
}

macro_rules! xor_swap {
    ($x:expr, $y:expr) => {
        if $x != $y {
            $x ^= $y;
            $y ^= $x;
            $x ^= $y;
        }
    };
}

macro_rules! prefix_sum {
    ($vec:expr) => {
        $vec.iter()
            .scan(0, |sum, &x| {
                *sum += x;
                Some(*sum)
            })
            .collect::<Vec<i32>>()
    };
}

macro_rules! max {
    ($vec:expr) => {
        *$vec.iter().max().unwrap()
    };
}

macro_rules! min {
    (vec:expr) => {
        *vec.iter().min().unwrap()
    };
}

macro_rules! sort_des {
    ($vec:expr) => {
        $vec.sort_unstable_by(|a, b| b.cmp(a));
    };
}

macro_rules! vec_to_string {
    ($vec:expr) => {
        $vec.iter().collect::<String>()
    };
}

macro_rules! string_to_vec {
    ($string:expr) => {
        $string.chars().collect::<Vec<char>>()
    };
}

macro_rules! stot {
    ($type:ty, $value:expr) => {
        $value.parse::<$type>().expect("Conversion failed")
    };
}

macro_rules! rout_vec {
    ($vec:expr) => {{
        for i in $vec {
            print!("{} ", i)
        }
        println!()
    }};
}

macro_rules! rout {
    ($x:expr) => {
        print!("{} ", $x)
    };
}

macro_rules! routln {
    ($x:expr) => {
        println!("{}", $x)
    };
}

macro_rules! nl {
    () => {
        println()
    };
}

/*~~~~~~~~~~~~~*
 * CODE BELOW: *
 *~~~~~~~~~~~~~*/
type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Clone)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self {
            data: data,
            next: None,
        }
    }
}

#[derive(Debug, Clone)]

struct LStack<T> {
    size: usize,
    top: Link<T>,
}

impl<T: Clone> LStack<T> {
    fn new() -> Self {
        Self { size: 0, top: None }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.size = 0;
        self.top = None;
    }

    fn push(&mut self, val: T) {
        let mut node = Node::new(val);
        node.next = self.top.take();
        self.top = Some(Box::new(node));
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| {
            let node = *node;
            self.top = node.next;
            self.size -= 1;
            node.data
        })
    }

    fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| &node.data)
    }
}

fn solve() {
    let stack = rin_vec!(i32);
    let mut lstack = LStack::new();

    for s in stack {
        lstack.push(s);
    }

    rout!("Top:");
    if let Some(top) = lstack.peek() {
        routln!(top)
    }
}

fn main() {
    solve()
}
