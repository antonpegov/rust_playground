fn main() {
    /* #region VECTORS */
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; // can panic if out of bounds

    println!("The third element is {}", third);

    match v.get(2) {
        // will not panic if out of bounds (returns None)
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    /*
    v.push(6); // cannot borrow as mutable
    */

    let mut v1 = vec![100, 32, 57];

    for i in &mut v1 {
        *i += 50; // dereference and mutate
    }
    println!("{:?}", v1);

    enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // Rust needs to know what types will be in the vector at compile time
    // so it knows exactly how much memory on the heap will be needed to store each element
    let row = vec![
        Cell::Int(3),
        Cell::Text(String::from("blue")),
        Cell::Float(10.12),
    ];
    /* #endregion */
    /* #region STRING (growable, mutable, owned, UTF-8 encoded string type) */

    let mut s = String::new();
    let data = "initial contents";

    s = data.to_string();

    println!("{}", s);

    let hello = String::from("안녕하세요"); // utf-8 encoded string

    println!("{}", hello);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{}", s);

    // A String is a wrapper over a Vec<u8>
    // English characters are encoded in 1 byte UTF-8
    // Russian characters are encoded in 2 bytes UTF-8
    // Korean characters are encoded in 3 bytes UTF-8
    // Chinese characters are encoded in 4 bytes UTF-8
    // etc.

    let hello1 = "Здравствуйте";
    let hello2 = "Hello";

    let s1 = &hello1[0..4]; // => "Зд"
    let s2 = &hello2[0..1]; // => "H"
                            /*
                            let s3 = &hello2[0..8]; // => Panics!
                            */

    // Safe way to get a substring

    let s = "Здравствуйте";
    let s_len = s.len(); // s.len() returns the length of the string in bytes

    for c in "Hello".chars() {
        println!("{}", c);
    }

    println!("{} != {}", s.chars().count(), s_len);

    /* #endregion */
    /* #region HASH MAPS */

    let mut scores = std::collections::HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let hashmap = std::collections::HashMap::from([("foo", 1), ("bar", 2)]);

    println!("{:?} {:?}", scores, hashmap);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.entry(String::from("Green")).or_insert(60);
    scores.remove(&String::from("Blue"));
    scores.entry(String::from("Blue")).or_insert(70);

    println!(
        "Blue has score {:?} and Green has score {:?}",
        scores.get(&String::from("Blue")).unwrap(),
        scores.get(&String::from("Green")).unwrap()
    );

    /* #endregion */
}
