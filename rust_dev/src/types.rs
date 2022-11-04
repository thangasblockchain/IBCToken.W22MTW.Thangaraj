//datatypes

//rust is a statistically types lang. it must know the type of variables at compile time
//compiler can infer what type and how we use it.
pub fn run(){
    println!("all about data types");

    //default i32
    let a = 5;

    //default i64
    let b = 10.8;

    //adding type explicitly
    let c:i128=666666666;

    //boolean
    let istrue = a > c;

    //char
    let Char = 'c';
    let Uni = '\u{1F602}';

    // to know the max size of a type
    println!("Max of i32 : {}",std::i32::MAX);
    println!("Max of i64 : {}",std::i64::MAX);
    println!("istrue: {}",istrue);
    println!("char {}",Char);
    println!("UNICODE {}",Uni);
}