// for printline and formatter

pub fn run(){
    // print to console
    println!("hello from print.rs file !");

    // basic formatting
    println!("we can't print without a placeholder {} ","Ok ?");
    println!("If you are adding a place holder {} we can {} more","like this","add");

    //positional formatting
    println!("placeholder as position {0} {1} {0}","im zero","im one");

    //named formatting
    println!(
        "{name} is my son",
        name="harshiv");

    // placeholder traits
    println!("Hex value is : {:x} Binary value is : {:b} Octal value is : {:o}",10,10,10);    
}