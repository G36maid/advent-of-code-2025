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

    fn within_bounds(&self, bbox: &BoundingBox) -> bool {
        self.x_min >= bbox.min_x
            && self.x_max <= bbox.max_x
            && self.y_min >= bbox.min_y
            && self.y_max <= bbox.max_y
    }

    fn is_intersected_by_edges(&self, edges: &PolygonEdges) -> bool {
        // Check vertical edges
        for &(x, y_start, y_end) in &edges.vertical {
            if x > self.x_min && x < self.x_max && y_start.max(self.y_min) < y_end.min(self.y_max) {
                return true;
            }
        }

        // Check horizontal edges
        for &(y, x_start, x_end) in &edges.horizontal {
            if y > self.y_min && y < self.y_max && x_start.max(self.x_min) < x_end.min(self.x_max) {
                return true;
            }
        }

        false
    }

    fn center_is_inside(&self, vertical_edges: &[(i64, i64, i64)]) -> bool {
        // Use integer arithmetic for center point
        let c_x = (self.x_min + self.x_max) / 2;
        let c_y = (self.y_min + self.y_max) / 2;

        // Ray casting: count intersections with vertical edges
        let mut intersections = 0;

        for &(x, y_start, y_end) in vertical_edges {
            // Ray goes to the right, so edge must be to the right of center
            // Center's Y must be in the edge's Y range (half-open interval)
            if x > c_x && c_y >= y_start && c_y < y_end {
                intersections += 1;
            }
        }

        intersections % 2 == 1
    }
}

struct BoundingBox {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

struct PolygonEdges {
    vertical: Vec<(i64, i64, i64)>,   // (x, y_min, y_max)
    horizontal: Vec<(i64, i64, i64)>, // (y, x_min, x_max)
}

struct PolygonData {
    points: Vec<Point>,
    edges: PolygonEdges,
    bbox: BoundingBox,
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

    let mut vertical = Vec::new();
    let mut horizontal = Vec::new();

    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];

        // Update bounding box
        min_x = min_x.min(p1.x);
        max_x = max_x.max(p1.x);
        min_y = min_y.min(p1.y);
        max_y = max_y.max(p1.y);

        // Categorize edge by orientation
        if p1.x == p2.x {
            // Vertical edge
            let y_start = p1.y.min(p2.y);
            let y_end = p1.y.max(p2.y);
            vertical.push((p1.x, y_start, y_end));
        } else {
            // Horizontal edge
            let x_start = p1.x.min(p2.x);
            let x_end = p1.x.max(p2.x);
            horizontal.push((p1.y, x_start, x_end));
        }
    }

    PolygonData {
        points,
        edges: PolygonEdges {
            vertical,
            horizontal,
        },
        bbox: BoundingBox {
            min_x,
            max_x,
            min_y,
            max_y,
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

            // Early pruning: skip if area can't beat current maximum
            if area <= max_area {
                continue;
            }

            // Fast rejection: check if rectangle is within polygon bounds
            if !rect.within_bounds(&data.bbox) {
                continue;
            }

            // Check if any polygon edge cuts through the rectangle interior
            if rect.is_intersected_by_edges(&data.edges) {
                continue;
            }

            // Check if rectangle center is inside polygon (ray casting)
            if rect.center_is_inside(&data.edges.vertical) {
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
