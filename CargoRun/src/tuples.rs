//  Tuples MAX 12 elements

pub fn run(){
    let person: (&str, &str, i8) =("Asish", "India", 23);
    println!("{} is from {} and is {}", person.0, person.1, person.2);
}