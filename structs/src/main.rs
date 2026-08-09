// Structs
// Structs are used to name and package related values similar to tuples
fn main() {
    //tuple
    let _rect: (i32, i32) = (200, 500);

    // Struct
    struct Book {
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // if we want to mondify, than we have to instanciate with mut
    let mut user1: User = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("someusername@m.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@m.com");
    println!("User email is {}", user1.email);

    // Return a struct from a function
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            email,
            username,
            sign_in_count: 1,
        }
    }

    // Create instances from other instances
    let _user2: User = User {
        email: String::from("anotheremailuser2@m.com"),
        ..user1
    };

    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black: Color = Color(0,0,0);
    let _white: Color = Color(255,255,255);

    // unit-like struck
    struct AlwaysEqual;
    let _subject: AlwaysEqual = AlwaysEqual;
}
