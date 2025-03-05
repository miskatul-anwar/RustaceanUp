use plotters::prelude::*;
use std::{error::Error, f64, process::Command};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

static mut REFF: Point = Point::new(0, 0);

fn find_ref(points: &Vec<Point>) -> Point {
    let mut min_point = points[0];

    for i in 1..points.len() {
        let condition_met =
            points[i].y < min_point.y || points[i].y == min_point.y && points[i].x < min_point.x;
        if condition_met {
            min_point = points[i]
        }
    }

    min_point
}

fn next_to_top(s: &mut Vec<Point>) -> Point {
    let mut res = Point::new(0, 0);
    if let Some(p) = s.pop() {
        res = *s.last().unwrap();
        s.push(p);
    }

    res
}

fn squared_dist(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2)
}

fn get_angle(p: &Point) -> f64 {
    let y = (p.y - unsafe { REFF.y }) as f64;
    let x = (p.x - unsafe { REFF.x }) as f64;
    y.atan2(x)
}

fn compare(p1: &Point, p2: &Point) -> std::cmp::Ordering {
    let angle1 = get_angle(p1);
    let angle2 = get_angle(p2);

    if angle1 == angle2 {
        let reff;
        unsafe { reff = REFF };
        squared_dist(&reff, p1).cmp(&squared_dist(&reff, p2))
    } else {
        angle1.partial_cmp(&angle2).unwrap()
    }
}

fn graham_scan(points: &mut Vec<Point>) -> Vec<Point> {
    unsafe { REFF = find_ref(points) }
    points.sort_by(|p1, p2| compare(p1, p2));

    let mut s = Vec::new();
    s.push(points[0]);
    s.push(points[1]);

    for i in 2..points.len() {
        let condition_met = s.len() > 1
            && (s.last().unwrap().x - next_to_top(&mut s).x)
                * (points[i].y - next_to_top(&mut s).y)
                - (s.last().unwrap().y - next_to_top(&mut s).y)
                    * (points[i].x - next_to_top(&mut s).x)
                < 0;

        while condition_met {
            s.pop();
        }

        s.push(points[i]);
    }

    s
}

fn plot_points(points: &[Point], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_min = points.iter().map(|p| p.x).min().unwrap_or(0) - 1;
    let x_max = points.iter().map(|p| p.x).max().unwrap_or(10) + 1;
    let y_min = points.iter().map(|p| p.y).min().unwrap_or(0) - 1;
    let y_max = points.iter().map(|p| p.y).max().unwrap_or(10) + 1;

    let mut chart = ChartBuilder::on(&root)
        .caption("Convex Hull", ("CaskaydiaCove Nerd font Mono", 30))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh().draw()?;

    // Draw points as red circles
    chart.draw_series(
        points
            .iter()
            .map(|p| Circle::new((p.x, p.y), 5, RED.filled())),
    )?;

    // Connect points with a blue line, ensuring a cycle is formed
    let mut cycle_points = points.to_vec();
    if let Some(&first) = points.first() {
        cycle_points.push(first); // Append the first point at the end to complete the cycle
    }

    chart.draw_series(LineSeries::new(
        cycle_points.iter().map(|p| (p.x, p.y)),
        &BLUE,
    ))?;

    root.present()?;
    println!("Plot saved to {}", filename);

    let output = Command::new("eog")
        .arg(filename)
        .output()
        .expect("Failed to run the command!");

    println!("{}", String::from_utf8_lossy(&output.stdout)); // Print the output

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut points: Vec<Point> = vec![
        Point::new(0, 0),
        Point::new(4, 0),
        Point::new(4, 3),
        Point::new(2, 5),
        Point::new(0, 3),
    ];

    let mut s: Vec<Point> = graham_scan(&mut points);
    plot_points(&s, "scatter.png")?;
    println!("Points in anti-clockwise order");
    for &p in points.iter() {
        println!("{} {}", p.x, p.y)
    }

    let mut t = Vec::new();

    while let Some(top) = s.pop() {
        t.push(top);
    }

    println!("Points in Convex Hull");

    while let Some(top) = t.pop() {
        println!("{} {}", top.x, top.y)
    }
    Ok(())
}
