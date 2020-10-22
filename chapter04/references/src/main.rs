fn main() {
    {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("The length of '{}' is {}.", s1, len);
    }
    {
        let mut s = String::from("hello");
        change(&mut s);
    }
    {
        let mut s = String::from("hello");
        let r1 = &mut s;
        change(r1);
        // let r2 = &mut s;
        // println!("{}, {}", r1, r2);
        //     error[E0499]: cannot borrow `s` as mutable more than once at a time
        //   --> chapter04/references/src/main.rs:14:18
        //    |
        // 13 |         let r1 = &mut s;
        //    |                  ------ first mutable borrow occurs here
        // 14 |         let r2 = &mut s;
        //    |                  ^^^^^^ second mutable borrow occurs here
        // 15 |         println!("{}, {}", r1, r2);
        //    |                            -- first borrow later used here
    }
    {
        let mut s = String::from("hello");
        {
            let r1 = &mut s;
            change(r1);
        } // r1 goes out of scope here, so we can make a new reference with no problems.
        let r2 = &mut s;
        change(r2);
    }
    {
        let s = String::from("hello");
        let r1 = &s; // no problem
        let r2 = &s; // no problem
                     // let r3 = &mut s;
                     //     error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
                     //   --> chapter04/references/src/main.rs:40:18
                     //    |
                     // 38 |         let r1 = &s; // no problem
                     //    |                  -- immutable borrow occurs here
                     // 39 |         let r2 = &s; // no problem
                     // 40 |         let r3 = &mut s; // BIG PROBLEM
                     //    |                  ^^^^^^ mutable borrow occurs here
                     // 41 |         println!("{}, {}, and {}", r1, r2, r3);
                     //    |                                    -- immutable borrow later used here

        println!("r1 = {}, r2 = {}", r1, r2);
    }
    {
        let mut s = String::from("hello");
        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{} and {}", r1, r2);
        // r1 and r2 are no longer used after this point
        let r3 = &mut s; // no problem
        println!("{}", r3);
    }
}

fn calculate_length(s: &str) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
