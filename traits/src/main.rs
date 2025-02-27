trait Animal {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise())
    }
}

struct Sheep {
    nacked: bool,
    name: &'static str,
}

impl Sheep {
    fn is_nacked(&self) -> bool {
        self.nacked
    }

    fn shear(&mut self) {
        if self.is_nacked() {
            println!("{} is already naked...", self.name)
        } else {
            println!("{} gets a haircut!", self.name)
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Self {
        Self {
            nacked: false,
            name: name,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_nacked() {
            "baaaah?"
        } else {
            "baaaah!"
        }
    }

    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise())
    }
}
fn main() {
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();
}
