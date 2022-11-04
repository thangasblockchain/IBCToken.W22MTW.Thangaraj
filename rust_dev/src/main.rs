
// from task 1

// import the print.rs

mod print;
mod vars;
mod types;
mod strings;
fn main() {
    println!("Hello, Harshiv!");
    print::run();
    println!("below is workspace of variables");
    vars::run();
    println!("below is workspace of primitive data types");
    types::run();
    println!("below is workspace of strings");
    strings::run();

}
