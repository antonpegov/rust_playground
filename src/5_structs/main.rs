#[allow(dead_code)]
fn main() {
    {// Basic Struct
        struct User {
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64,
        }

        fn get_user(email: String, username: String) -> User {
            User {
                email, // like in JS
                username, // we can use Field Init Shorthand
                active: true,
                sign_in_count: 1,
            }
        }
        
        let user1 = get_user(
            String::from("user1@test.com"),
            String::from("user1"),
        );

        let user2 = User {
            email: String::from("user2@test.com"),
            username: String::from("user2"),
            ..user1 // copy all fields except email
        };
        
        println!("Email: {}", user1.email);
        println!("Email: {}", user2.email);
    }
    {// Tuple Struct
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let _black = Color(0, 0, 0);
        let _origin = Point(0, 0, 0);
    }
    {// Methods
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            fn area(self: &Self) -> u32 {
                self.width * self.height
            }

            fn width(&mut self, val: u32) {// can use same name as property
                self.width = val;
            }

            fn square(size: u32) -> Rectangle {// constructor
                Rectangle {
                    width: size,
                    height: size,
                }
            }
        }

        let mut rect1 = Rectangle {
            width: 30,
            height: 50,
        };
    
        println!("Area: {}", rect1.area());
        rect1.width(40);
        dbg!(&rect1); // Debug trait

        println!("Square: {:?}", Rectangle::square(33));
    }
}