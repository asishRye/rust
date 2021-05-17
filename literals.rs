fn main(){
    println!("1+2={}",1u32 +2);
    println!("1-2={}",1i32-2);

    // Boolean logic
    println!("true and false is {}", true&&false);
    println!("true and true is {}", true&&true);
    println!("false or false is {}", true||true);
    println!("NOT false is {}", !false);

    //bitwise operations

    println!("0011 AND 0101 is {:04b}", 0b0011 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
}