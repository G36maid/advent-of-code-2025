advent_of_code::solution!(8);

struct Point {
    x: i64,
    y: i64,
    z: i64,
}

struct Edge {
    u: usize,
    v: usize,
    dist_sq: i64,
}

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
    num_components: usize,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            num_components: n,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            let root = self.find(self.parent[i]);
            self.parent[i] = root;
            root
        }
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
            self.num_components -= 1;
            true
        } else {
            false
        }
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let parts: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
            Point {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            }
        })
        .collect()
}

fn dist_sq(p1: &Point, p2: &Point) -> i64 {
    (p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2) + (p1.z - p2.z).pow(2)
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse_input(input);
    let n = points.len();

    // Generate all edges
    let mut edges = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push(Edge {
                u: i,
                v: j,
                dist_sq: dist_sq(&points[i], &points[j]),
            });
        }
    }

    // Sort edges by distance
    edges.sort_by_key(|e| e.dist_sq);

    // Heuristic for limit: 10 for example (20 lines), 1000 for full input
    let limit = if n < 100 { 10 } else { 1000 };

    let mut dsu = Dsu::new(n);

    // Process first `limit` edges
    for edge in edges.iter().take(limit) {
        dsu.union(edge.u, edge.v);
    }

    // Find circuit sizes
    let mut component_sizes = Vec::new();
    let mut visited_roots = std::collections::HashSet::new();

    for i in 0..n {
        let root = dsu.find(i);
        if !visited_roots.contains(&root) {
            visited_roots.insert(root);
            component_sizes.push(dsu.size[root] as u64);
        }
    }

    component_sizes.sort_unstable_by(|a, b| b.cmp(a)); // Descending

    let result = component_sizes.iter().take(3).product();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse_input(input);
    let n = points.len();

    // Prim's Algorithm
    let mut min_dist = vec![i64::MAX; n];
    let mut parent = vec![None; n];
    let mut visited = vec![false; n];

    // Start from node 0
    min_dist[0] = 0;

    for _ in 0..n {
        // Find vertex u with minimum min_dist among unvisited
        let mut u = None;
        let mut min_val = i64::MAX;

        for i in 0..n {
            if !visited[i] && min_dist[i] < min_val {
                min_val = min_dist[i];
                u = Some(i);
            }
        }

        let u = match u {
            Some(node) => node,
            None => break, // Should not happen for connected graph
        };

        visited[u] = true;

        // Update neighbors
        for v in 0..n {
            if !visited[v] {
                let dist = dist_sq(&points[u], &points[v]);
                if dist < min_dist[v] {
                    min_dist[v] = dist;
                    parent[v] = Some(u);
                }
            }
        }
    }

    // Find the edge with the maximum weight in the MST
    // The MST edges are (v, parent[v]) for all v where parent[v] is Some
    let mut max_edge_dist = -1;
    let mut max_edge_nodes = (0, 0);

    for i in 0..n {
        if let Some(p) = parent[i] {
            let d = dist_sq(&points[i], &points[p]);
            if d > max_edge_dist {
                max_edge_dist = d;
                max_edge_nodes = (i, p);
            }
        }
    }

    let p1 = &points[max_edge_nodes.0];
    let p2 = &points[max_edge_nodes.1];
    Some((p1.x * p2.x) as u64)
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
