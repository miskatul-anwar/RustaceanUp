use std::{fs::File, io::Read};

fn main() {
    let mut buf = String::new();

    let mut f = match File::open("/home/miskat/Clone/RustaceanUp/Readfile/src/data.txt") {
        Ok(f) => f,
        Err(e) => {
            panic!("Error opening file: {:?}", e);
        }
    };

    f.read_to_string(&mut buf).unwrap();
    println!("File opened successfully: {}", buf);
}
