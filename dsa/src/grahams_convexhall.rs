use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

// Function to compute the cross product of two vectors (p1 -> p2 and p1 -> p3)
fn cross_product(p1: Point, p2: Point, p3: Point) -> f64 {
    (p2.x - p1.x) * (p3.y - p1.y) - (p2.y - p1.y) * (p3.x - p1.x)
}

// Function to compute the squared distance between two points
fn squared_distance(p1: Point, p2: Point) -> f64 {
    (p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)
}

// Function to compare points for sorting
fn compare_points(p1: &Point, p2: &Point, pivot: Point) -> Ordering {
    let cross = cross_product(pivot, *p1, *p2);
    if cross == 0.0 {
        // If points are collinear, sort by distance from the pivot
        squared_distance(pivot, *p1)
            .partial_cmp(&squared_distance(pivot, *p2))
            .unwrap()
    } else if cross > 0.0 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

// Graham's scan algorithm to compute the convex hull
fn graham_scan(mut points: Vec<Point>) -> Vec<Point> {
    if points.len() <= 3 {
        return points;
    }

    // Find the pivot point (the point with the lowest y-coordinate, and leftmost if tied)
    let pivot_index = points
        .iter()
        .enumerate()
        .min_by(|(_, p1), (_, p2)| {
            p1.y.partial_cmp(&p2.y).unwrap().then(p1.x.partial_cmp(&p2.x).unwrap()
        })
        .map(|(i, _)| i)
        .unwrap();
    let pivot = points.swap_remove(pivot_index);

    // Sort the points based on their polar angle with respect to the pivot
    points.sort_by(|a, b| compare_points(a, b, pivot));

    // Initialize the convex hull with the pivot and the first two sorted points
    let mut hull = Vec::new();
    hull.push(pivot);
    hull.push(points[0]);
    hull.push(points[1]);

    // Iterate through the remaining points
    for &point in points.iter().skip(2) {
        while hull.len() >= 2
            && cross_product(hull[hull.len() - 2], hull[hull.len() - 1], point) <= 0.0
        {
            hull.pop(); // Remove the last point from the hull if it makes a non-left turn
        }
        hull.push(point);
    }

    hull
}

fn main() {
    // Example set of points
    let points = vec![
        Point::new(0.0, 3.0),
        Point::new(1.0, 1.0),
        Point::new(2.0, 2.0),
        Point::new(4.0, 4.0),
        Point::new(0.0, 0.0),
        Point::new(1.0, 2.0),
        Point::new(3.0, 1.0),
        Point::new(3.0, 3.0),
    ];

    // Compute the convex hull
    let convex_hull = graham_scan(points);

    // Print the convex hull
    println!("Convex Hull:");
    for point in convex_hull {
        println!("({}, {})", point.x, point.y);
    }
}

