use std::io::{self, BufRead};

fn rin() -> String {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let t: i8 = rin().parse().unwrap(); // Read the number of test cases
    let mut output = String::new(); // Collect output to print in one go

    for _ in 0..t {
        let (n, k, x): (i8, i8, i8) = {
            let values: Vec<i8> = rin()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            (values[0], values[1], values[2])
        };

        if x != 1 {
            output.push_str(&format!("YES\n{}\n", n));
            output.push_str(&vec!["1"; n as usize].join(" "));
            output.push('\n');
        } else if k >= 2 && n % 2 == 0 {
            output.push_str(&format!("YES\n{}\n", n / 2));
            output.push_str(&vec!["2"; (n / 2) as usize].join(" "));
            output.push('\n');
        } else if k >= 3 {
            output.push_str(&format!("YES\n{}\n", n / 2));
            let mut sequence = vec!["2"; (n / 2) as usize];
            sequence[0] = "3"; // Replace the first element with "3"
            output.push_str(&sequence.join(" "));
            output.push('\n');
        } else {
            output.push_str("NO\n");
        }
    }

    // Print the entire output at once
    print!("{}", output);
}
