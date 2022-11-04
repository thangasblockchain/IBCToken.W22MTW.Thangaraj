// vectors also arrays but we can add to and move stuff around

// vectors are resizeable arrays

pub fn run () {
println!("vectors");
let mut arr : Vec<i32> = vec![10,13,15];

//reassign
arr[1]=25;

//adding to vector
arr.push(100);

println!("reassigned {:?}",arr);

//Loop through vectors
for num in arr.iter() {
println!("value {}",num);
}

//Loop and mutate values
for num in arr.iter_mut(){
    *num *= 5;
    // num *= 5
    //try above with num instaed of *num
    //it won't work without deref
}
println!("vectors after mutated {:?}",arr);
}