// Traditional Struct
struct Color{
    red: u8,
    blue: u8,
    green: u8
}  

// Tuple Struct

struct Color2(u8, u8, u8);

// Person Struct

struct Person{
    fname: String,
    lname: String
}

impl Person{
    // Construct a person
    fn new(first: &str, last: &str) -> Person{
        Person{
            fname: first.to_string(),
            lname: last.to_string()
        }
    }

    fn full_name(&self)->String {
        format!("{} {}", self.fname, self.lname)
    }
}

pub fn run(){
    let mut c = Color {
        red:255,
        blue:0,
        green:0
    };
    c.red = 200;
    println!("Color:{} {} {}", c.red, c.green, c.blue);

    println!("------------");

    let mut c = Color2(255,0,0);
    c.0 = 150;
    println!("Color:{} {} {}", c.0, c.1, c.2);

    println!("------------");

    let mut p = Person::new("John", "Doe");
    println!("Person {} {}", p.fname, p.lname);
    println!("{}", p.full_name());
}