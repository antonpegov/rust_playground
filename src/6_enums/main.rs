fn main() {
    // The `derive` attribute automatically creates the implementation
    // required to make this `enum` printable with `fmt::Debug`.
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6,
    }

    let four = IpAddr::V4(String::from("127.0.0.1"));
    let _six = IpAddr::V6;

    println!("{:?}", four);

    // RUST has no Nullable Types, but we can use standard enum Option<T>
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    let mut some_number: Option<i8> = Some(5);
    let some_string = Some("a string");

    println!("{:?}", some_number);
    some_number = None;
    println!("{:?}", some_number);
    println!("{:?}", some_number.unwrap_or(0) + Some::<i8>(5).unwrap());
}
