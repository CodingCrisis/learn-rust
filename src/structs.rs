// Regular struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple struct
struct Colorz(u8, u8, u8);

// Another regular struct
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };
 
    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c = Colorz(255, 0, 0);

    c.0 = 239;

    println!("Colorz: {} {} {}", c.0, c.1, c.2);

    let mut p = Person::new("Coding", "Success");
    println!("Person: {} {}", p.first_name, p.last_name);
    p.set_last_name("Crisis");
    println!("Person: {}", p.full_name());
    println!("Person tuple {:?}", p.to_tuple());
}


