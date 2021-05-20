// Vectors are resizable vectors

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    numbers[2] = 20;
    

    // Get single value
    println!("{}", numbers[0]);
    
    // numbers[6]=90; // Not possible
    numbers.push(55);
    numbers.push(101);

    // Print everything
    println!("{:?}", numbers);

    numbers.pop();

    // Get vector length
    println!("vector length is {}", numbers.len());

    // vectors are stack allocated
    println!("vectors occupied {} bytes", std::mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);


    // Print everything
    println!("{:?}", numbers);


    println!("------------");
    // Loops
    for x in numbers.iter(){
        println!("Number is: {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    }

    println!("Number vec: {:?}", numbers);
}