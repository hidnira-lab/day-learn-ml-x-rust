// Goal: Learn the fundamentals of Rust programming language
// Last Update: 18 July 2026


// This is how we create a main function in rust
// using main function so we can run the code in rust programming language
// notes: In rust programming language, the main function is the entry point of the program, and it is required in every rust program.
fn main() { 
    setup();
    variables();
    ownership();
}

// First, I'll wrote a setup function to print hello world
fn setup() {
    println!("hello world!");
}

// Second, I'll wrote a variables function to learn about variables in rust programming language
fn variables() {

     println!("===");

    // In rust programming language, we can create a variable using let keyword
    let x = 10;
    println!("{}",x); 
    
    // we can use {} to print the value of a variable in rust programming language
    // because rust programming language is a statically typed language, we don't need to specify the type of the variable
    // rust will infer the type of the variable based on the value assigned to it

      println!("---");
    
    // The difference between let and let mut is that let creates an immutable variable (cannot be changed)
    // while let mut creates a mutable variable (can be changed)
    let mut y = 20;
    y += 10;
    println!("{}",y);

      println!("---");

    // In rust, there's also a concept called Shadowing 
    // which allows us to create a new variable with the same name as a previous variable
    let z = 10; // There will be a warning that z is unused, so we can use symbol _ to ignore the warning later
    let z = "haiii";
    println!("{}",z);
    
      println!("---");

    // -- DATA TYPES
    // Rust use many of types of data types, such as i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, bool, char, and more
    // because rust is a programming language that concerns about performance and memory safety, so it provides many types of data types to choose from.

    // - Integers in rust programming language can be signed or unsigned, and they can be of different sizes (8, 16, 32, 64, and 128 bits)
    let a : i8 = 10; // i8 is a signed 8-bit integer (possible values from -128 to 127)
    let b : u8 = 20; // u8 is an unsigned 8-bit integer (possible values from 0 to 255 only positive values)
    let integers = 30; // by default, integers in rust are i32 (signed 32-bit integer) and u32 (unsigned 32-bit integer)
    println!("{} {} {}",a,b,integers);

      println!("---");

    // - Floating point numbers in rust programming language can be of two types: f32 (32-bit floating point number) and f64 (64-bit floating point number)
    let c = 10.5; // by default, floating point numbers in rust are f64 (64-bit floating point number)
    let as_f32 : f32 = 123456789.123456789; // if we use f32, the value will be rounded to 12345679 because f32 has a precision of 6-7 decimal digits (total 7 digits, including the digits before the decimal point)
    let as_f64 : f64 = 123456789.123456789; // if we use f64, the value will be rounded to 123456789.12345679 because f64 has a precision of 15-16 decimal digits
    println!("{}",c);
    println!("{}",as_f32);
    println!("{}",as_f64);

      println!("---");

    // - Boolean in rust programming language can be either true or false
    let d = true;
    let e : bool = false; // we can also specify the type of the variable
    println!("{} {}",d,e);

      println!("---");

    // - Character in rust programming language is represented by unicode scalar values, which means that it can represent any character in any language, including emojis
    let f : char = 'A'; // in rust char is represented by single quotes, and char is 4 byte in size (32 bits) because unicode have more than 1 million possible characters, so rust take the biggest size to represent char.
    let g : char = '😊';
    println!("{} {}",f,g);
    println!("{}", std::mem::size_of::<char>());

      println!("---");

    // - Strings in rust programming language is very unique, because rust has two types of strings: String and &str (string slice)
    // String is a growable, heap-allocated data structure, while &str is an immutable reference to a string slice, which is usually stored in the stack memory.
    let h : &str = "haii";
    let i : String = String::from("helloo"); //String::from() is a function that creates a new String from a string slice (&str)
    let mut j : String = String::from("hello"); // we can also create a mutable String using let mut
    j.push_str(" world"); // we can use push_str() method to append a string
    println!("{} {} {}",h,i,j);
}

// Third, I'll wrote a ownership function to learn about ownership in rust programming language
fn ownership() {
    println!("===");

}