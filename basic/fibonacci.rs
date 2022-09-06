use std::f32::INFINITY;

fn main() {

    let x: i32 = 5;

    println!("{}", fibonacci(x));
}


fn fibonacci(number: i32) -> i32 {
    if number == 0 {
        return 0
    } else if number == 1 {
        return 1
    } else {
        return fibonacci(number - 1) + fibonacci(number - 2)
    }
}

