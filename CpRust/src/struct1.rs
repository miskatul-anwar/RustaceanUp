struct Employee {
    _name: String,
    _company: String,
    _age: u32,
}

fn display(emp: Employee) {
    println!("{}\n{}\n{}", emp._name, emp._company, emp._age);
}

fn main() {
    let mut empl1 = Employee {
        _name: String::from("Miskatul Anwar"),
        _company: String::from("Facebook"),
        _age: 22,
    };

    empl1._age = 5;
    display(empl1);
}
