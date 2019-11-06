fn do_something() {
    let x = 5; // x comes into scope
    println!("The value of x is: {}", x); // 5
} // x goes out of scope

fn do_something_else() {
    let s = String::from("hello"); // s comes into scope
    println!("The value of s is: {}", s); // hello
} // s goes out of scope

fn do_even_more() {
    let s = String::from("hello");
    // s.push_str(", world!"); // CANNOT BORROW AS MUTABLE
    println!("The value of s is: {}", s); // hello
    let mut t = String::from("hello");
    t.push_str(", world!"); // CANNOT BORROW AS MUTABLE
    println!("The value of t is: {}", t); // hello, world!
}

fn do_something_crazy() {
    // COPY
    let x = 5;
    println!("The value of x is: {}", x); // 5
    let y = x;
    println!("The value of y is: {}", y); // 5
    println!("The value of x is: {}", x); // 5
    
    // MOVE
    let s = String::from("hello");
    println!("The value of s is: {}", s); // hello
    let t = s;
    println!("The value of t is: {}", t); // hello
    // println!("The value of s is: {}", s); // BORROW OF MOVED VALUE
}

fn main() {
    do_something();
    do_something_else();
    do_even_more();
    do_something_crazy();
}
