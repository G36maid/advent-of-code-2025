advent_of_code::solution!(8);

use std::cmp::Reverse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn dist_sq(&self, other: &Self) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<u64>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
            true
        } else {
            false
        }
    }

    fn num_components(&mut self) -> usize {
        (0..self.parent.len())
            .filter(|&i| self.find(i) == i)
            .count()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let points: Vec<Point> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut parts = line.split(',');
            Point {
                x: parts.next().unwrap().parse().unwrap(),
                y: parts.next().unwrap().parse().unwrap(),
                z: parts.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    let n = points.len();
    // Heuristic to distinguish between example and real input
    let limit = if n < 50 { 10 } else { 1000 };

    let mut edges = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push((points[i].dist_sq(&points[j]), i, j));
        }
    }

    edges.sort_unstable_by_key(|e| e.0);

    let mut dsu = DSU::new(n);

    for &(_, u, v) in edges.iter().take(limit) {
        dsu.union(u, v);
    }

    let mut component_sizes = Vec::new();
    for i in 0..n {
        if dsu.parent[i] == i {
            component_sizes.push(dsu.size[i]);
        }
    }

    component_sizes.sort_unstable_by_key(|&s| Reverse(s));

    let result = component_sizes.iter().take(3).product();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points: Vec<Point> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut parts = line.split(',');
            Point {
                x: parts.next().unwrap().parse().unwrap(),
                y: parts.next().unwrap().parse().unwrap(),
                z: parts.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    let n = points.len();

    let mut edges = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push((points[i].dist_sq(&points[j]), i, j));
        }
    }

    edges.sort_unstable_by_key(|e| e.0);

    let mut dsu = DSU::new(n);

    for &(_, u, v) in &edges {
        if dsu.union(u, v) && dsu.num_components() == 1 {
            let result = points[u].x * points[v].x;
            return Some(result as u64);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
