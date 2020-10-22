fn main() {
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward
        println!("{}", s); // do stuff with s
    } // this scope is now over, and s is no longer valid
      // println!("{}", s);
      //     error[E0425]: cannot find value `s` in this scope
      //  --> chapter04/ownership/src/main.rs:6:20
      //   |
      // 6 |     println!("{}", s);
      //   |                    ^ not found in this scope

    // On the stack
    println!("copy x (i32) to y (on the stack)");
    let x = 5;
    let y = x; // x is copied to y because i32 is Copy
    println!("x = {}", x);
    println!("y = {}", y);

    // On the heap
    println!("move s1 (String) to s2 (on the heap)");
    let s1 = String::from("Hello");
    let s2 = s1; // s1 is moved to s2
    println!("s2 = {}", s2);
    // println!("{}, world!", s1);
    // error[E0382]: borrow of moved value: `s1`
    //  --> chapter04/ownership/src/main.rs:5:28
    //   |
    // 2 |     let s1 = String::from("Hello");
    //   |         -- move occurs because `s1` has type `std::string::String`, which does not implement the `Copy` trait
    // 3 |     let s2 = s1;
    //   |              -- value moved here
    // 4 |
    // 5 |     println!("{}, world!", s1);

    println!("clone s1 (String) to s2 (on the heap)");
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    // Functions and ownership
    {
        let s = String::from("hello"); // s comes into scope
        println!("s = {}", s);
        takes_ownership(s); // s's value moves into the function and so is no longer valid here

        // println!("s = {}", s);
        //    error[E0382]: borrow of moved value: `s`
        //    --> chapter04/ownership/src/main.rs:47:24
        //     |
        //  43 |     let s = String::from("hello");
        //     |         - move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
        //  44 |     println!("s = {}", s);
        //  45 |     takes_ownership(s);
        //     |                     - value moved here
        //  47 |     println!("s = {}", s);
        //     |                        ^ value borrowed here after move

        let x = 5; // x comes into scope
        println!("x = {}", x);
        makes_copy(x); // x would move into the function,
                       // but i32 is Copy, so it’s okay to still
                       // use x afterward
        println!("x = {}", x);
    }
    {
        let s1 = gives_ownership(); // gives_ownership moves its return value into s1
        println!("s1 = {}", s1);
        let s2 = String::from("hello"); // s2 comes into scope
        println!("s2 = {}", s2);
        let s3 = takes_and_gives_back(s2); // s2 is moved into
                                           // takes_and_gives_back, which also
                                           // moves its return value into s3
        println!("s3 = {}", s3);
    } // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
      // moved, so nothing happens. s1 goes out of scope and is dropped.
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

#[allow(clippy::let_and_return)]
fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
