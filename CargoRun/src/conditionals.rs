pub fn run(){
    let age = 18;

    if age>=21{
        println!("Great");
    }

    // Shorthand if

    let is_of_age = if age >=21 {true} else {false};
    println!("{}", is_of_age);
    
}