
pub fn run(){

    let mut count = 0;
    // Infinite loop
    loop {
        count +=1;
        println!("Number: {}", count);

        if count == 30{
            break;
        }
    }

    count = 1;
    // While loop
    while count <=100{
        if count%15==0{
            println!("Fizzbuzz");
        }
        else if count%3==0{
            println!("Fizz");
        }
        else if count%5==0{
            println!("Buzz");
        }
        else {
            println!("{}", count)
        }
        count +=1;
    }

    println!("------------------");
    // For range
    for x in 0..10{
        println!("{}", x);
    } 
}