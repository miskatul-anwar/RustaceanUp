struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new() -> Self {
        Point { x: 0, y: 0 }
    }

    fn setPoint(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    fn dist_origin(&self) -> f64 {
        let temp_x = self.x as f64;
        let temp_y = self.y as f64;
        (temp_x.powi(2) + temp_y.powi(2)).sqrt()
    }
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    *largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);

    println!("{}", result);

    let mut point = Point::new();
    point.setPoint(5, 5);

    println!(
        "Distance from origin, ({}, {}) {}",
        point.x,
        point.y,
        point.dist_origin()
    );
}
