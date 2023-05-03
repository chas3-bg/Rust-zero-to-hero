fn main () {
    //primitives
    let signed_num: i32 = 42;
    let unsigned_num: u8 = 42;
    let deciman: f32 = 4.2;
    let letter: char = "a";
    let phrase: String = "Don't panic";

    //tuple
    let sample_tuple = (4u8, 2i8, "a", 5, 3.14);

    //array
    let sample_array = [1,2,3,4,5];
    let sample_array2 = ["a", "b", "c"]

    //slice
    let slice1 = [0..2]

    println!("This is a signed number {}, this is unsigned: {}", signed_num, unsigned_num);
    println!("This is a float {}  and this is a character {}", deciman, letter);
    println!("Remember : {}", phrase);

    println!("This is the first member of a tuple {} and this is the last: {}", 
                sample_tuple.0, sample_tuple.4);
    println!("This is an array {}", sample_array[0..4]);
    println!("This is a slice {}", slice1);

}