//var are immutable by default
//variables hold primitve data or references to the data
//rust is a block-scoped language

pub fn run(){
    let name = "harshiv thangaraj";
    let mut myage = 27;
    println!("my son name is {} and my age is {} ",name,myage);

    myage=28;

    println!("my son name is {} and my age is {} ",name,myage);


    //contant variables
    const ID:i32 = 2;
    println!("{}",ID);

    //throws error
    //ID:i64=3;
    //ID:i32=2;

    let (myname,myage) = ("thangaraj",27);
    println!("{} is {}",myname,myage);
}