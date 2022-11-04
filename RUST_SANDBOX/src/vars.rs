pub fn run(){
    let name = "Brad";
    let mut age = 37;
    age = 38;
    println!("My name is {} and i am {}", name, age);

    //Define constant
    const ID: i32 =001;
    println!("ID: {}",ID);

    //Assing multiple variables at once
    let (my_name, my_age) = ("Brad", 37);
    println!("My name is {} and my age is {}", my_name, my_age);

}