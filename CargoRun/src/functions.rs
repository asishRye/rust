pub fn run(){
    // greetings("hello", "Jane");
    println!("{}", add(3,2));

    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("Closure sum is: {}", add_nums(10,100));
}

fn greetings(greet: &str, name: &str){
    println!("{} {}",greet, name);
}

fn add(n1: i32, n2: i32)-> i32 {
    n1 + n2
}