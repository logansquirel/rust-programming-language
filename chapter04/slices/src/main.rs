fn main() {
    {
        let s = String::from("hello");
        println!("s = {}", s);
        eval!(&s[0..2]);
        eval!(&s[..2]);
        let len = s.len();
        eval!(&s[3..len]);
        eval!(&s[..len]);
        eval!(&s[0..len]);
        eval!(&s[..]);
    }
    {
        #[allow(unused_mut)]
        let mut s = String::from("hello world");
        let word = first_word(&s);
        // s.clear();
        // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
        //   --> chapter04/slices/src/main.rs:16:9
        //    |
        // 15 |         let word = first_word(&s);
        //    |                               -- immutable borrow occurs here
        // 16 |         s.clear();
        //    |         ^^^^^^^^^ mutable borrow occurs here
        // 17 |
        // 18 |         println!("the first word is: {}", word);
        //    |                                           ---- immutable borrow later used here
        println!("the first word is: {}", word);
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

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
