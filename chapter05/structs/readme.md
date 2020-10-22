# chapter05/structs

## Defining a struct

```rust
// Standard struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple struct
struct Color(i32, i32, i32);

// Unit struct
struct Unit;
```

## Instantiating a struct

Standard struct:

```rust
let user =  User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
}
```

Tuple struct:

```rust
let color =  Color(0, 0, 0);
```

Unit struct:

```rust
let unit = Unit;
```

## Field Init Shorthand

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,              //equivalent to email: email
        username,           //equivalent to username: username
        active: true,
        sign_in_count: 1,
    }
}
```

## Struct update syntax

```rust
let user1 =  User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
}

let user2 =  User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1, // equivalent to sign_in_count: user1.sign_in_count, active: user1.active,
}
```

## Methods

Methods are defined in an implementation block and take ownership or reference
self (immutable or mutable reference):

```rust
impl User {
    fn email(&self) -> &str {
        &self.email
    }

    fn update_email(&mut self, new_email: String) {
        self.email = new_email;
    }

    fn into_email(self) -> String {
        self.email
    }
}
```

## Associated functions

Associated functions are also defined in an implementation block without self
reference/borrowing:

```rust
impl User {
    fn new(username: String, email: String) -> Self {
        User {
            username,
            email,
            sign_in_count: 1,
            active: true,
        }
    }
}
```
