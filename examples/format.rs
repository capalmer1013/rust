fn main() {
    // in general, the {} will be automatically replaced with any
    // arguments. These will be stringified
    println!("{} days", 31);
    
    // without a suffix 21 becimes an i32. this can be changed wiht a suffix
    // positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // named arguments
    println!("{subject} {verb} {object}", 
        object = "the lazy dog",
        subject = "The quick brown fox",
        verb = "jumps over");
    
    // special formatting can be specified after a :
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // right align text with specified width
    println!("{number:>-width$}", number=1, width=6);

    // will check to make sure correct number of args are used
    println!("The name is {0}, {1} {0}", "Bond", "James");

    // create a structure which contains an i32. name it structure
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom typed such as this structure require more complicated handling
    //println!("This struct {} won't print", Structure(3));
}
