use std::io;

fn main () {
    let mut user_string = String::new();
    let mut user_num = String::new();

    println!("Please input a string:");
    io::stdin()
        .read_line(&mut user_string)
        .expect("Failed to read from stdin");

    println!("Please input a number:");
    io::stdin()
        .read_line(&mut user_num)
        .expect("Failed to read from stdin");

    println!("The string is\n{}", user_string);
    match user_num.trim().parse::<i32>() {
        Ok(i) => println!("The number is \n{}", i),
        Err(..) => println!("This is no integer!")
    };

}