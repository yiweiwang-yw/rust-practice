fn main() {
    another_function();

    let x = plus_one(5);
    print!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function.");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}