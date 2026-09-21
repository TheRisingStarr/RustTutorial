//main function
fn main(){ 
    //println! and print! are macros
    println!("Hello Rust"); 
    print!("Hello Rust1");

//Mutable variables
    let mut x=23;
    println!("Before: {}",x);
    x=90;
    print!("After: {}",x);

//Data Types
    let num = 90;         // integer
    let double = 0.9;     // float
    let letter = 'P';     // character
    let boole = false;    // boolean
    let text = "Rust";    // string

// Define Explicitly
    let no: i32= 90;
    let f: f64= 2.3;
    let l: char= 'IO';
    let b: bool= true;
    let s: &str = "Hi";

/*
Data Types Groups:
    Numbers- whole no and decimals (i32, f64)
    Strings- Sequence of characters (&str)
    Character- Single symbols or letters (char)
    Boolean- True or False (bool)
*/
}


