//arrays are fixed length

use std::mem;

pub fn run(){
    let arr : [i32;3] = [10,13,15];

    // entire array
    println!("arrays {:?}",arr);

    //specific value in an array
    println!("value 1 {}",arr[0]);

    //length of an array 
    println!("length {}",arr.len());

    //arrays are stack allocated
    println!("array occupies {} bytes",mem::size_of_val(&arr));

    //getting slices from array
    let slice : &[i32] = &arr[1..3];
    println!("slices {:?}",slice);

    //length of a slice
    println!("length of slice {}",slice.len());

}