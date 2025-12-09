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

// Rectangle structure
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

    // Check if the rectangle is intersected by any polygon edge
    fn is_intersected_by_edges(&self, edges: &[(Point, Point)]) -> bool {
        for &(p1, p2) in edges {
            // Edge is vertical
            if p1.x == p2.x {
                let x = p1.x;
                let y_start = p1.y.min(p2.y);
                let y_end = p1.y.max(p2.y);

                // Check if vertical edge strictly cuts through the rectangle horizontally
                // x must be strictly between x_min and x_max
                // y range must overlap with rect y range
                if x > self.x_min
                    && x < self.x_max
                    && y_start.max(self.y_min) < y_end.min(self.y_max)
                {
                    return true;
                }
            } else {
                // Edge is horizontal
                let y = p1.y;
                let x_start = p1.x.min(p2.x);
                let x_end = p1.x.max(p2.x);

                // Check if horizontal edge strictly cuts through the rectangle vertically
                if y > self.y_min
                    && y < self.y_max
                    && x_start.max(self.x_min) < x_end.min(self.x_max)
                {
                    return true;
                }
            }
        }
        false
    }

    // Use Ray Casting to check if the center point is inside the polygon
    fn center_is_inside(&self, edges: &[(Point, Point)]) -> bool {
        // Use floating point center to avoid integer boundary issues
        let c_x = (self.x_min as f64 + self.x_max as f64) / 2.0;
        let c_y = (self.y_min as f64 + self.y_max as f64) / 2.0;

        // Simple check: if the center is exactly on an edge, treat as inside (problem says boundary is green)
        // But floating point comparison is tricky, so we just use Ray Casting; if the point is on the edge, it's usually counted as correct or needs epsilon
        // Here we use standard Ray Casting: shoot a ray to the right
        let mut intersections = 0;

        for &(p1, p2) in edges {
            // Only consider vertical edges
            if p1.x == p2.x {
                let x = p1.x as f64;
                let y1 = p1.y.min(p2.y) as f64;
                let y2 = p1.y.max(p2.y) as f64;

                // Edge is to the right of the point, and the point's Y coordinate is within the edge's range
                if x > c_x && c_y >= y1 && c_y < y2 {
                    intersections += 1;
                }
            }
        }

        intersections % 2 == 1
    }
}

fn parse_input(input: &str) -> (Vec<Point>, Vec<(Point, Point)>) {
    let points: Vec<Point> = input
        .lines()
        .filter_map(|line| {
            let (x, y) = line.trim().split_once(',')?;
            Some(Point::new(x.parse().ok()?, y.parse().ok()?))
        })
        .collect();

    let mut edges = Vec::new();
    if !points.is_empty() {
        for i in 0..points.len() {
            edges.push((points[i], points[(i + 1) % points.len()]));
        }
    }
    (points, edges)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (points, _) = parse_input(input);
    let mut max_area = 0;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let rect = Rect::from_points(points[i], points[j]);
            max_area = max_area.max(rect.area());
        }
    }

    Some(max_area)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (points, edges) = parse_input(input);
    let mut max_area = 0;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let rect = Rect::from_points(points[i], points[j]);
            let area = rect.area();

            // Pruning: if the area is already less than the current maximum, skip checking
            if area <= max_area {
                continue;
            }

            // 1. Check if any edge crosses the rectangle
            if rect.is_intersected_by_edges(&edges) {
                continue;
            }

            // 2. Check if the rectangle is inside (Ray Casting)
            if rect.center_is_inside(&edges) {
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
