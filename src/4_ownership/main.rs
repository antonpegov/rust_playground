#[allow(dead_code)]
fn main() {
    // Ownership is a set of rules that governs how a Rust program manages memory.
    // Rust has two kinds of ownership:
    // 1. Owned: A value that is owned by a particular owner.
    // 2. Borrowed: A value that is owned by a context that grants you permission to use it.

    // Stack and Heap
    // Stack: A stack is a data structure that stores a collection of FIXED SIZE data.
    // Heap: A heap is a data structure that stores a collection of VARIABLE SIZE data. It is slower than a stack.

    // Ownership Rules
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // Reference rules
    // 1. At any given time, you can have either one mutable reference or any number of immutable references.
    // 2. References must always be valid.

    /* #region Variable Scope */
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward

        // let s = s + ", world!"; // error: s has fixed size
        // s = s.to_string() + ", world!"; // error: s has &str type but got String type

        let s = s.to_string() + ", world!"; // s still valid here, no problem

        println!("The value of s is: {s}");
    } // this scope is now over, and s is no longer valid

    /* #endregion */
    /* #region  Move */
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); // Error: s1 is invalid because it was moved
    println!("{}, world!", s2); // OK: s2 is still valid because it was moved here

    /* #endregion */
    /* #region  Clone */
    let s1 = String::from("hello");
    let s2 = s1.clone(); // s2 is a deep copy of s1, no problem

    println!("s1 = {}, s2 = {}", s1, s2);

    /* #endregion */
    /* #region  Ownership */
    #[allow(unused)]
    {
        fn takes_ownership(some_string: String) {
            println!("{some_string}");
        } // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

        fn makes_copy(some_integer: i32) {
            println!("{some_integer}");
        } // Here, some_integer goes out of scope. Nothing special happens.

        let s = String::from("hello");

        /*
        takes_ownership(s); // s's value moves into the function and so is no longer valid here
        println!("{}", s); // Error: s no longer valid
        */
        takes_ownership(s.clone());
        println!("{s}");

        let x = 5;

        makes_copy(x); // x would move into the function, but i32 is Copy, so it's okay to still
        println!("{x}");
    }
    /* #endregion */
    /* #region  References and Borrowing */
    // Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type
    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);


    //fn change(some_string: &String) { // references are immutable by default
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
    
    // let s = String::from("hello"); // references are immutable by default
    let mut s = String::from("hello");
    /*
    let r1 = &s; // no problem
    let _r2 = &s; // no problem
    */
    let mut r1 = &mut s; // no problem
    // let _r2 = &mut s; // problem: cannot have multiple mutable references to the same value


    // change(&s); // references are immutable by default
    change(&mut r1);
    println!("{}", r1);
    /* #endregion */
    /* #region  Slice Type */
    let mut s = String::from("hello world");
    {
        fn first_word(s: &String) -> usize {
            let bytes = s.as_bytes();
            
            println!("Bytes of '{s}': {:?}", bytes);

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return i;
                }
            }

            s.len()
        }

        let word = first_word(&s); // word will get the value 5

        println!("Length of '{}' is {}", &s[..word], word);
        s.clear();
        println!("--> Length of {:?} is {}", vec!(&s).get(word), word); // word is no longer valid!
    }
    {
        // Rather than a reference to the entire String, hello is a reference to a portion of the String,
        // specified in the extra [0..5] bit. We create slices using a range within brackets
        // by specifying [starting_index..ending_index], where starting_index is the first position
        // in the slice and ending_index is one more than the last position in the slice. 
        // Internally, the slice data structure stores the starting position and the length of the slice,
        // which corresponds to ending_index minus starting_index. So in the case of let world = &s[6..11];,
        // world would be a slice that contains a pointer to the byte at index 6 of s with a length value of 5.
        
        let _hello = &s[..5];
        let _world = &s[6..];

        fn first_word(s: &String) -> &str {
            let bytes = s.as_bytes();
        
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }
        
            &s[..]
        }
    }
    /* #endregion */
}
