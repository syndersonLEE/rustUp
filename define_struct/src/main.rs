struct User {
    username : String,
    email : String,
    sign_in_count : u64,
    active : bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email : String, username : String) -> User {
    User {
        email,
        username,
        active : true,
        sign_in_count : 1,

    }
}

fn main() {
    let user1  = User {
        email : String::from("someone@example.com"),
        username: String::from("someusername123"),
        active : true,
        sign_in_count : 1,
    };


    

    let mut user2 = build_user(
        String::from("winnery93@gmail.com"), 
        String::from("syndersonLEE")
    );

    

    let user3 = User {
        ..user2
    };

    user2.email = String::from("abc");

    println!("{}", user2.email);
    println!("{}", user3.email);
}


