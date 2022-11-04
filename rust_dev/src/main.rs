
// from task 1

// import the print.rs

mod print;
mod vars;
mod types;
fn main() {
    println!("Hello, Harshiv!");
    print::run();
    println!("below is workspace of variables");
    vars::run();
    println!("below is workspace of data types");
    types::run();

}
