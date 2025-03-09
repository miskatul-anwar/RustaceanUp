macro_rules! gcd {
    ($a:expr, $b:expr) => {{
        let (mut a, mut b) = ($a, $b);
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }};
}
/*  Leet code problem solved */
pub fn gcd_of_strings(str1: String, str2: String) -> String {
    if str1.clone() + &str2 != str2.clone() + &str1 {
        String::from("")
    } else {
        let (m, n) = (str1.len().max(str2.len()), str1.len().min(str2.len()));
        str1[..gcd!(m, n)].to_string()
    }
}
fn main() {}
