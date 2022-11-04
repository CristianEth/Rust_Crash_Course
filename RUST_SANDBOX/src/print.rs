pub fn run(){
    //print console
    println!("Hello world from the print.rs file!");

    //Basic formatting
    println!("Number: {}", 1);
    println!("{} is from {}", "Brad", "Mass");

    //positional formatting
    println!("{0} is from {1} and {0} like to {2}", "Brad", "Mass", "code");

    //Named arguments
    println!("{name} likes to play {activity}", name = "John", activity = "baseball");

    //Placeholder traits
    println!("Binary:{:b} Hex:{:x} Octal:{:o}", 10, 10, 10 );

    //Placeholder debug traits
    println!("{:?}", (12,true,"hello"));

    //Basic math
    println!("10 + 10 = {}", 10+10);

}