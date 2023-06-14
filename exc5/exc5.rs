use std::io;

fn add (a: f32, b: f32) ->f32 {
   return a + b
}
fn substract (a: f32, b: f32) ->f32 {
    return a - b
}

fn multiply (a: f32, b: f32) -> f32 {
    return a * b
}

fn divide (a: f32, b: f32) -> f32 {
    return a / b
}

fn main(){

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let num1 :f32;
    let num2 :f32;
    let operation :char;
    println!("Welcome to simple calculator!");
    println!("Input first number");
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read from stdin");


    match input1.trim().parse::<f32>() {
        Ok(i) => {
            num1 = i;
        }
        Err(..) => panic!("{} is not an intereger", input1.trim())
    };

    println!("Input operation (+, -, *, /");
    io::stdin()
    .read_line(&mut input3)
    .expect("Failed to read from stdin");

    match input3.trim().parse::<char>(){
        Ok(i) => {
            operation = i;
        }
        Err(..)=> panic!("{} is not a valid character", input3)
    };



    println!("Input second number");
    io::stdin()
    .read_line(&mut input2)
    .expect("Failed to read from stdin");

    match input2.trim().parse::<f32>(){
        Ok(i) => {
            num2  = i;
        }
        Err(..) => panic!("{} is not an integer", input2.trim())
    };
    println!("{operation}");
    match operation{
        '+'=> println!("Result: {}",add(num1, num2)),
        '-'=> println!("Result: {}",substract(num1, num2)),
        '*'=> println!("Result: {}",multiply(num1, num2)),
        '/'=> println!("Result: {}",divide(num1, num2)),
        _=> println!("Invalid operation")
    };


}