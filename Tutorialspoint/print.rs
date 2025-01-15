fn print(arg: &String) -> () {
    println!("{}", arg);
}
fn main() {
    let str: String = String::from("Hello World");
    print(&str);
    print(&str);
}
