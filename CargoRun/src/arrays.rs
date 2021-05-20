// Fixed list
pub fn run(){
    let mut numbers: [i32; 5] = [1,2,3,4,5];
    numbers[2] = 20;
    
    // Print everything
    println!("{:?}", numbers);

    // Get single value
    println!("{}", numbers[0]);
    
    // numbers[6]=90; Not possible

    // Get array length
    println!("Array length is {}", numbers.len());

    // Arrays are stack allocated
    println!("Arrays occupied {} bytes", std::mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}