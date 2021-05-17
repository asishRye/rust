pub fn run(){
    //
    println!("Hello from print.rs file");
    let mut age = 37;
    let name = "John";
    age = 38;
    println!("My name is {} and my age is {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let ( my_name, my_age) = ("Asish", 23);

    println!("My name is {}", my_name)
}