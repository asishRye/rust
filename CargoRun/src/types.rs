pub fn run(){
    println!("Running from types");
    let y:i64 = 77678998;
    println!("Max size of i64: {}", std::i64::MAX);


    //  Boolean
    let is_active: bool = true;

    let is_greater = 10 > 5;
    println!("Is it greater? {}", is_greater);

    // println!("Type is:", )
    println!("---------Look from here------------");

    let mut hello = String::from("hello");
    println!("Length: {}", hello.len());
    // Push a single char
    hello.push('W');
    // Push string
    hello.push_str("orld!");
    println!("{}", hello);

    /*
    hello.replace("this", "withThis")
    hello.split_whitespace()
    hello.contains()
    hello.capacity()
    */

    // Assertions
    assert_eq!(11, hello.len());
}