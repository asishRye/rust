fn main(){
    let logical: bool = false;
    println!("{}",logical);
    let a_var = 5i32;
    println!("This variable has been annotated with suffix->{}", a_var);
    let b_var: f64 = 1.5;
    println!("This variable has been annotated using regular annotation->{}", b_var);
    let mut mutable = 12;
    println!("{}",mutable);
    mutable = 5;
    // a_var = "meh";
    // mutable = true;  //Throws error
    let mutable = true;
}