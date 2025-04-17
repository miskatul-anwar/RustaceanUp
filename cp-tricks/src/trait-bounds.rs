fn add<'a, 'b, T: std::ops::Add<Output = T>>(i: &'a T, j: &'b T) -> T
where
    T: std::ops::Add<Output = T> + Copy,
{
    *i + *j
}

fn mul<'a, 'b, T: std::ops::Mul<Output = T>>(i: &'a T, j: &'b T) -> T
where
    T: std::ops::Mul<Output = T> + Copy,
{
    *i * *j
}

fn main() {
    let x: usize = 2;
    let y: usize = 2;

    'outer: for i in 1..=x {
        for j in 1..=y {
            if add(&i, &j) == 4 && mul(&i, &j) == 4 {
                println!("Pookie!!!");
                break 'outer;
            }
        }
    }
}
