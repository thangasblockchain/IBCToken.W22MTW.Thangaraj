
// from task 1

// import the print.rs

mod print;
mod vars;
fn main() {
    println!("Hello, Harshiv!");
    print::run();
    println!("below is workspace of variables");
    vars::run();

}
