#[allow(clippy::nonminimal_bool)]
#[allow(clippy::eq_op)]
fn main() {
    println!("booleans:");
    eval!(true);
    eval!(false);
    println!("AND:");
    eval!(true & true);
    eval!(true & false);
    eval!(false & true);
    eval!(false & false);
    println!("OR:");
    eval!(true | true);
    eval!(true | false);
    eval!(false | true);
    eval!(false | false);
    println!("XOR:");
    eval!(true ^ true);
    eval!(true ^ false);
    eval!(false ^ true);
    eval!(false ^ false);
    println!("NOT:");
    eval!(!true);
    eval!(!false);
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
