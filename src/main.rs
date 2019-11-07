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

fn makes_copy(some_integer: i32) { 
    println!("The value of some_integer is: {}", some_integer); // 5
} 

fn takes_ownership(some_string: String) { 
    println!("{}", some_string); // hello
} 

fn gives_ownership() -> String { 
    let some_string = String::from("hello");
    some_string
}

fn main() {
    do_something();
    do_something_else();
    do_even_more();
    do_something_crazy();
    
    // OWNERSHIP AND FUNCTION PARAMETERS
    let x = 5;
    makes_copy(x);
    println!("The value of x is: {}", x); // 5

    let s = String::from("hello");
    takes_ownership(s);
    // println!("The value of s is: {}", s); // BORROW OF MOVED VALUE

    // OWNERSHIP AND FUNCTION RETURNS
    let s1 = gives_ownership();
    println!("The value of s1 is: {}", s1); // hello
}
