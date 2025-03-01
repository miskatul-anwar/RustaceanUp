const DAYS_IN_MONTH: [i32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

pub fn day_of_year(date: String) -> i32 {
    let args: Vec<i32> = date.split('-').map(|i| i.parse().unwrap()).collect();
    let (year, month, day) = (args[0], args[1], args[2]);

    let is_leap = (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0);

    let mut n = 0;
    for i in 1..month {
        n += if is_leap && i == 2 {
            29
        } else {
            DAYS_IN_MONTH[i as usize]
        };
    }

    n + day
}

fn main() {
    println!("{}", day_of_year("2019-02-10".to_string()))
}
