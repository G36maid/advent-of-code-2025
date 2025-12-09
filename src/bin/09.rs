advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

impl Rect {
    fn from_points(p1: Point, p2: Point) -> Self {
        Self {
            x_min: p1.x.min(p2.x),
            x_max: p1.x.max(p2.x),
            y_min: p1.y.min(p2.y),
            y_max: p1.y.max(p2.y),
        }
    }

    fn area(&self) -> u64 {
        ((self.x_max - self.x_min).unsigned_abs() + 1)
            * ((self.y_max - self.y_min).unsigned_abs() + 1)
    }

    // Replaces BoundingBox check
    fn is_inside(&self, other: &Rect) -> bool {
        self.x_min >= other.x_min
            && self.x_max <= other.x_max
            && self.y_min >= other.y_min
            && self.y_max <= other.y_max
    }

    fn intersects_edges(&self, v_edges: &[(i64, i64, i64)], h_edges: &[(i64, i64, i64)]) -> bool {
        // Check vertical edges
        for &(x, y_start, y_end) in v_edges {
            if x > self.x_min && x < self.x_max && y_start.max(self.y_min) < y_end.min(self.y_max) {
                return true;
            }
        }

        // Check horizontal edges
        for &(y, x_start, x_end) in h_edges {
            if y > self.y_min && y < self.y_max && x_start.max(self.x_min) < x_end.min(self.x_max) {
                return true;
            }
        }

        false
    }

    fn center_is_inside(&self, v_edges: &[(i64, i64, i64)]) -> bool {
        let c_x = (self.x_min + self.x_max) / 2;
        let c_y = (self.y_min + self.y_max) / 2;

        let mut intersections = 0;
        for &(x, y_start, y_end) in v_edges {
            if x > c_x && c_y >= y_start && c_y < y_end {
                intersections += 1;
            }
        }
        intersections % 2 == 1
    }
}

struct PolygonData {
    points: Vec<Point>,
    v_edges: Vec<(i64, i64, i64)>, // (x, y_min, y_max)
    h_edges: Vec<(i64, i64, i64)>, // (y, x_min, x_max)
    bounds: Rect,
}

fn parse_input(input: &str) -> PolygonData {
    let points: Vec<Point> = input
        .lines()
        .filter_map(|line| {
            let (x, y) = line.trim().split_once(',')?;
            Some(Point::new(x.parse().ok()?, y.parse().ok()?))
        })
        .collect();

    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    let mut v_edges = Vec::new();
    let mut h_edges = Vec::new();

    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];

        // Update bounding box
        min_x = min_x.min(p1.x);
        max_x = max_x.max(p1.x);
        min_y = min_y.min(p1.y);
        max_y = max_y.max(p1.y);

        if p1.x == p2.x {
            // Vertical
            v_edges.push((p1.x, p1.y.min(p2.y), p1.y.max(p2.y)));
        } else {
            // Horizontal
            h_edges.push((p1.y, p1.x.min(p2.x), p1.x.max(p2.x)));
        }
    }

    PolygonData {
        points,
        v_edges,
        h_edges,
        bounds: Rect {
            x_min: min_x,
            x_max: max_x,
            y_min: min_y,
            y_max: max_y,
        },
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse_input(input);
    let mut max_area = 0;

    for i in 0..data.points.len() {
        for j in (i + 1)..data.points.len() {
            let rect = Rect::from_points(data.points[i], data.points[j]);
            max_area = max_area.max(rect.area());
        }
    }

    Some(max_area)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse_input(input);
    let mut max_area = 0;

    for i in 0..data.points.len() {
        for j in (i + 1)..data.points.len() {
            let rect = Rect::from_points(data.points[i], data.points[j]);
            let area = rect.area();

            // 1. Pruning by area
            if area <= max_area {
                continue;
            }

            // 2. Fast bounds check
            if !rect.is_inside(&data.bounds) {
                continue;
            }

            // 3. Edge intersection check
            if rect.intersects_edges(&data.v_edges, &data.h_edges) {
                continue;
            }

            // 4. Center point check (Ray Casting)
            if rect.center_is_inside(&data.v_edges) {
                max_area = area;
            }
        }
    }

    Some(max_area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
