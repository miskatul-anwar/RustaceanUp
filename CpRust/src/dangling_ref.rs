// fn main() {
//     let oop;
//     {
//         let miskat = 5;
//         oop = &miskat;
//     }
//     println!("cse212: {oop}")
// } // won't run due to lifetime

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
// # Here, 'a ensures that the returned reference lives at least as long as both s1 and s2.
//
// fn longest_str<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

fn main() {
    let oop;
    let miskat = 6;
    oop = &miskat;
    println!("{oop}");
}
