fn main() {
    {
        // immutable user
        // fields do not need to respect struct definition order
        let user = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        println!("user = {:?}", user);
        // query
        eval!(user.username);
        eval!(user.email);
        eval!(user.sign_in_count);
        eval!(user.active);
    }
    {
        // mutable user
        // only the full struct can be mutable not part of it
        let mut user = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        println!("user = {:?}", user);
        // change field
        println!("changing user.email");
        user.email = String::from("anotheremail@example.com");
        println!("user = {:?}", user);
    }
    {
        let user = build_user(
            "someone@example.com".to_string(),
            "someusername123".to_string(),
        );
        println!("user = {:?}", user)
    }
    {
        // struct update syntax
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        let user2 = User {
            email: String::from("another@example.com"),
            username: String::from("anotherusername567"),
            ..user1 // equivalent to sign_in_count: user1.sign_in_count, active: user1.active
        };
        println!("user 2 = {:?}", user2)
    }
    {
        // tuple struct
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
        println!("black = {:?}", black);
        println!("origin = {:?}", origin);
    }
    {
        // unit struct
        let unit = Unit;
        println!("unit = {:?}", unit);
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        // use of field init shorthand
        email,    //equivalent to email: email
        username, //equivalent to username: username
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Unit;

#[macro_export]
macro_rules! stringify {
    ($x:expr) => {
        std::stringify!($x)
    };
}

#[macro_export]
macro_rules! eval {
    ($x:expr) => {
        let string = stringify!($x);
        let value = $x;
        println!("{} = {:?}", string, value);
    };
}
