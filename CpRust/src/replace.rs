use std::io::stdin;

fn _rin() -> Vec<char> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().chars().collect()
}

fn replace(orgin: &mut Vec<char>, target: Vec<char>, rep: Vec<char>) {
    let n = orgin.len();
    let t_len = target.len();
    let r_len = rep.len();

    let mut i = 0;
    while i <= n - t_len {
        if orgin[i..i + t_len] == target[..] {
            orgin.splice(i..i + t_len, rep.iter().cloned());
            i += r_len;
        } else {
            i += 1;
        }
    }
}

fn main() {
    let mut orgin = _rin();
    let target = _rin();
    let rep = _rin();
    replace(&mut orgin, target, rep);
    println!("{}", orgin.iter().collect::<String>());
}
