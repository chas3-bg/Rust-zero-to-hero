use std::io;

static PI :f32 = 3.14159265;

fn circum (x: f32) -> f32 {
    return x*x*PI
}

fn main () {
    let mut input = String::new();
    println!("Enter a radius to get the area:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");



    let  radius: f32;
    let trimmed = input.trim();
    match trimmed.parse::<f32>() {
        Ok(i) => {
             radius = i;
        },
        Err(..) => panic!("Not an integer/float")
    };

    println!("The circuference is {}", circum(radius))

}
