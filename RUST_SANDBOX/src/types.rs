pub fn run(){
    // Default is "i32" 
    let x = 1;

    // Default is "f64" 
    let y = 5.5;

    // Add explicit types
    let z: i64 = 4545454545;

    // Find max_size
    println!("Max i32:  {}", std::i32::MAX);
    println!("Max i64:  {}", std::i64::MAX);

    // Boolean
    let is_active = true;
    
    //Get boolean from expression
    let is_greater = 10 > 11;

    let a1 = 'a';
    let face = '\u{1F600}';
    
    println!("{:?}", (x,y,z, is_active, is_greater, a1, face));
}