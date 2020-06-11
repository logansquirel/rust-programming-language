fn main() {
    println!("Chars");
    println!("literals");
    eval!('a');
    eval!('\u{0061}');
    println!("methods/associated functions:");
    eval!('1'.is_digit(10));
    eval!('a'.is_digit(10));
    eval!('a'.is_alphabetic());
    eval!('a'.is_lowercase());
    eval!('A'.is_uppercase());
    eval!(' '.is_whitespace());
    eval!('\u{009c}'.is_control());
    eval!('7'.is_numeric());
    eval!('a'.to_uppercase().to_string());
    eval!('A'.to_uppercase().to_string());
    eval!('a'.is_ascii());
    eval!('µ'.is_ascii());
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
