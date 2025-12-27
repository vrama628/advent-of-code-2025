use std::{
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

use itertools::Itertools;

struct Point {
    x: usize,
    y: usize,
    z: usize,
}

/// ignores element value for comparison
struct Distanced<T> {
    distance: f64,
    element: T,
}

impl<T> PartialEq for Distanced<T> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<T> Eq for Distanced<T> {}

impl<T> PartialOrd for Distanced<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl<T> Ord for Distanced<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // reverse order so that the max heap will put minimum distances first
        other.distance.total_cmp(&self.distance)
    }
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str, z_str) = s.split(',').collect_tuple().unwrap();
        let x = x_str.parse().unwrap();
        let y = y_str.parse().unwrap();
        let z = z_str.parse().unwrap();
        Ok(Self { x, y, z })
    }
}

impl Point {
    fn distance(&self, other: &Self) -> f64 {
        let x_diff = self.x.abs_diff(other.x) as f64;
        let y_diff = self.y.abs_diff(other.y) as f64;
        let z_diff = self.z.abs_diff(other.z) as f64;
        (x_diff.powi(2) + y_diff.powi(2) + z_diff.powi(2)).sqrt()
    }
}

struct UnionFind(HashMap<usize, usize>);

impl UnionFind {
    fn new(size: usize) -> Self {
        Self((0..size).map(|i| (i, i)).collect())
    }

    /// also compacts, as an (unnecessary) optimization
    fn peek(&mut self, x: usize) -> usize {
        let next = self.0[&x];
        if next == x {
            x
        } else {
            let peeked = self.peek(next);
            // compaction for optimization
            self.0.insert(x, peeked);
            peeked
        }
    }

    fn unify(&mut self, a: usize, b: usize) {
        let peeked_a = self.peek(a);
        let peeked_b = self.peek(b);
        debug_assert_eq!(self.0[&peeked_a], peeked_a);
        debug_assert_eq!(self.0[&peeked_b], peeked_b);
        self.0.insert(peeked_a, peeked_b);
    }

    fn classes(&mut self) -> Vec<usize> {
        let mut classes: HashMap<usize, usize> = HashMap::new();
        for i in 0..self.0.len() {
            let peeked = self.peek(i);
            *classes.entry(peeked).or_default() += 1
        }
        classes.values().copied().collect()
    }
}

const CONNECTIONS: usize = 1000;
const TOP_CIRCUITS: usize = 3;

fn main() {
    let points = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().parse::<Point>().unwrap())
        .collect_vec();
    let mut distances: BinaryHeap<Distanced<(usize, usize)>> = (0..points.len())
        .cartesian_product(0..points.len())
        .filter(|(a, b)| a < b)
        .map(|(a, b)| Distanced {
            distance: points[a].distance(&points[b]),
            element: (a, b),
        })
        .collect();
    let mut union_find = UnionFind::new(points.len());
    for _ in 0..CONNECTIONS {
        let Distanced {
            distance: _,
            element: (a, b),
        } = distances.pop().expect("more pairs than connections");
        union_find.unify(a, b)
    }
    let mut classes: BinaryHeap<usize> = union_find.classes().into_iter().collect();
    let res = (0..TOP_CIRCUITS)
        .map(|_| classes.pop().expect("at least {TOP_CIRCUITS} classes"))
        .product::<usize>();
    println!("{res}");
}
