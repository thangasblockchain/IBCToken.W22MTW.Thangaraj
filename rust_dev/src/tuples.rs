// tuples
// group together values of different elements
// max 12 elements

pub fn run(){
    println!("tuples");
    let test : (&str,&str,i32,i64,f64) = ("thanga","raj",0,1,1.10);

    println!("test tuple 0 is {} and tuple 4 is {} ",test.0,test.4);
}