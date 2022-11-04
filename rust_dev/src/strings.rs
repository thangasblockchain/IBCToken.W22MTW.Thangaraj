
//strings
// primitive strings - Immutable / fixed memory
// growable strings - heap allocated data structure
pub fn run(){
    println!("all about strings");

    //immuatbel strings
    let immutablehello = "Hello";

    //below throws error as its immutable string
   //immutablehello.push('A');
    
   let mut growablehello = String::from("Hello");
   
   
   growablehello.push('A');
   //above will throw error if mut is not added at growablehello
   println!("growable {}",growablehello);

   //growablehello.push('ch');
   //above will throw error if we try to add more than one character

   growablehello.push_str("added string");



   println!("immutable {}",immutablehello);
   println!("growable {}",growablehello);
   println!("contains {}",growablehello.contains("llo"));
   println!("replace {}",growablehello.replace("llo", "ool"));

   //loop through whitespace
   for word in growablehello.split_whitespace(){
    println!("{}",word);
   }

   // string with capacity
   let mut s = String::with_capacity(5);
   s.push('q');
   s.push('q');

   //assert is used to validate 
   // if it passes it returns nothing
   // if it fails, will throe an error
   // it takes two paramaeters and matches left to right
   assert_eq!(2,s.len());
   //above won't do anything

   assert_eq!(3,s.len());
   //above will return an assertion failed

   println!("capacity {}",s);
}