fn do_something() {
    let x = 5; // x comes into scope
    println!("The value of x is: {}", x);
} // x goes out of scope

fn main() {
    do_something();
}
