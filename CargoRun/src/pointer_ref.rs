pub fn run(){
    let mut arr1: [i32; 5] = [1, 2, 3, 4, 5];
    let mut arr2 = arr1;
    arr1[0] = 100;
    arr2[1] = 999;
    println!("{:?}", arr1);
    println!("{:?}", arr2);

    // Now with vectors

    let vec1 = vec![1,2,3];
    let vec2 = &vec1;
    println!("Values: {:?}", (&vec1, vec2));
}