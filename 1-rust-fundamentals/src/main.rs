// Goal: Learn the fundamentals of Rust programming language
// Last Update: 18 July 2026


// This is how we create a main function in rust
// using main function so we can run the code in rust programming language
// notes: In rust programming language, the main function is the entry point of the program, and it is required in every rust program.
fn main() { 
    setup();
    variables();
}

// First, I'll wrote a setup function to print hello world
fn setup() {
    println!("hello world!");
}

// Second, I'll wrote a variables function to learn about variables in rust programming language
fn variables() {

    // In rust programming language, we can create a variable using let keyword
    let x = 10;
    println!("{}",x); 
    
    // we can use {} to print the value of a variable in rust programming language
    // because rust programming language is a statically typed language, we don't need to specify the type of the variable
    // rust will infer the type of the variable based on the value assigned to it

    // The difference between let and let mut is that let creates an immutable variable (cannot be changed)
    // while let mut creates a mutable variable (can be changed)
    let mut y = 20;
    y += 10;
    println!("{}",y);

    // In rust, there's also a concept called Shadowing 
    // which allows us to create a new variable with the same name as a previous variable
    let z = 10; // There will be a warning that z is unused, so we can use symbol _ to ignore the warning later
    let z = "haiii";
    println!("{}",z);

    let a : i8 = 10; // i8 is a signed 8-bit integer (possible values from -128 to 127)
    let b : u8 = 20; // u8 is an unsigned 8-bit integer (possible values from 0 to 255 only positive values)
    // by default, integers in rust are i32 (signed 32-bit integer) and u32 (unsigned 32-bit integer)
    println!("{} {}",a,b);

    // Rust use many of types of data types, such as i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, bool, char, and more until 128 bits
    // because rust is a programming language that concerns about performance and memory safety, so it provides many types of data types to choose from.
}
