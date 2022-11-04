//loops
pub fn run(){
    println!("loop");
    let mut test = 1;

    loop {
        test += 1;
        println!("value {}",test);
        if test == 15{break;}
    }

    //fizzbuzz and while
    while test <= 100 {

        if test % 15 == 0 {
            println!("fizzbuzz values {}",test)
        }else if test % 3 == 0{
            println!("fizz values {}",test)
        }else if test % 5 == 0 {
            println!("buzz values {}",test)
        }else {
            println!("nothing {}",test);
        }
        test += 1;       
    }

    // for range
    for test in 0..100{

        if test % 15 == 0 {
            println!("fizzbuzz values {}",test)
        }else if test % 3 == 0{
            println!("fizz values {}",test)
        }else if test % 5 == 0 {
            println!("buzz values {}",test)
        }else {
            println!("nothing {}",test);
        }

    }


}