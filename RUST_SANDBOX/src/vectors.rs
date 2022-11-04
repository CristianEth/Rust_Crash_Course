use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1, 2, 30, 4];
    
    // Re-assing value
    numbers[2] = 3;

    // Add to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value 
    numbers.pop();
    
    println!("{:?}", numbers);
    
    // Get single val
    println!("Index 0: {}", numbers[0]);

    // Get array length
    println!("Array length: {}", numbers.len());

    // Arrary are stack allocated
    println!("Array occupies {} bytes" , mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop through vectors values
    for x in numbers.iter() {
        println!("{}", x);
    }

    // Loop and mutete values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers in Vec: {:?}", numbers);

}