// fn par_match(open: char, close: char) -> bool {
//     let opens = "([{";
//     let closers = ")]}";
//     opens.find(open) == closers.find(close)
// }

// impl Solution {
//     pub fn is_valid(s: String) -> bool {
//         let mut stack = Vec::new();

//         for c in s.chars() {
//             if c == '(' || c == '{' || c == '[' {
//                 stack.push(c);
//             } else if let Some(top) = stack.pop() {
//                 if !par_match(top, c) {
//                     return false; // Not a valid match
//                 }
//             } else {
//                 return false;
//             }
//         }

//         stack.is_empty()
//     }
// }
fn main() {
    println!("Hi")
}
