fn main() {
    /* #region  Shadowing Unmutable*/
    let x = 5;

    println!("X = {x}");

    let x = 6;

    println!("X = {x}");

    let spaces = "   ";
    let spaces = spaces.len();

    println!("spaces = {spaces}");
    /* #endregion */

    /* #region  Mutable */
    let mut x = 5;

    println!("X = {x}");

    x = 6;

    println!("X = {x}");

    let spaces = "   ";
    // spaces = spaces.len(); // error: cannot change type of even muttable variable

    println!("spaces = {spaces}");
    /* #endregion */

    /* #region  Basic Types */
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, ..) = tup;

    println!("The value of y is: {}", y);

    let arr: [u8; 5] = [1, 2, 3, 4, 5];

    let [first, .., last] = arr;

    println!("The value of first element is: {}", first);
    println!("The value of last element is: {}", last);
    /* #endregion */

    /* #region  Statements and Expressions */
    // Statements are instructions that perform some action and do not return a value
    // Expressions are instructions that return a value
    #[allow(unused_variables)]
    let y = 5; // statement

    let y = {
        let x = 3;
        x + 1
    }; // expression

    println!("The value of y is: {}", y);
    /* #endregion */

    /* #region  Functions */
    fn five() -> i32 {
        5
    }

    println!("The value of five is: {}", five());

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    println!("The value of add_one is: {}", add_one(5));

    fn add_one_ref(x: &i32) -> i32 {
        x + 1
    }

    println!("The value of add_one_ref is: {}", add_one_ref(&5));
    /* #endregion */

    /* #region  Loops */
    let mut count = 0;

    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
    /* #endregion */

    /* #region  For Loop */
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
    /* #endregion */
}
