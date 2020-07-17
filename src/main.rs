fn main() {
    another_function(5, "What does the fox say?");
    let x = five();
    plus_one(x);
    println!("{}", plus_one(x))
}

// in impure functions, you MUST declare the type that the function should accept.
fn another_function(x: i32, y: &str) {
    println!("The value of x is {}", x);
    println!("{}", y)
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
