use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    if args.len() != 4 {
        println!("Usage: <number> <+-*/> <number>");
    } else {
        let first = args.nth(1).unwrap();
        let operator = args.nth(0).unwrap().chars().next().unwrap(); //turn string into char type
        let second = args.nth(0).unwrap();
        //println!("{:?}", first)

        let first_num = first.parse::<f32>().unwrap(); //both are valid methods to
                                                       //select a type, teh first one is called
                                                       //TURBOFISH ::<type>
        let second_num: f32 = second.parse().unwrap();
        let result: f32 = calculate(operator, first_num, second_num);
        println!("{}", output(first_num, second_num, operator, result));
    }
    //test
    assert_eq!(calculate('+', 5.0, 6.0), 11.0);
}

//implicit return is :
//no need for -> type
//no need for 'return' keyword
//no need for ; at the end of statement
fn calculate(op: char, a: f32, b: f32) -> f32 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '/' => a / b,
        '*' | 'x' | 'X' => a * b,
        _ => panic!("Invalid operator"),
    }
}

fn output(a: f32, b: f32, op: char, result: f32) -> String {
    format!("{} {} {} = {}", a, op, b, result)
}
