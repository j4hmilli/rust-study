fn main() {
    let x = 5;
    println!("The value of x is {}", x);
    x = 5; // Err: cannot assign twice to immutable variable
    println!("The value of x is {}", x);
}
