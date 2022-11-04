//conditionals

pub fn run(){
    println!("conditionals");

    let age = 18;
    let check: bool = true;

    if age > 21 && check {
        println!("what you would like to drink ?");
    }else if age < 21 && check {
        println!("you have to leave sir");
    } else {
        println!("nothing");
    }
}