static GREET :&str =  "Hello world!"; //global variables are declared with static or consts, not let

fn main () {
    println!("{GREET}");

    let  greet2 : &str; //default is imutable
    greet2 = "Hello again!";
    println!("{greet2}")
}