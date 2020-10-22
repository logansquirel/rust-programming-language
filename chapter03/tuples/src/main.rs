fn main() {
    // create a tuple
    println!("initialize tuple");
    let tup: (i32, f64, &str) = (42, 4.2, "42!");
    println!("tup = {:?}", tup);
    // destructuring a tuple
    println!("destructure tuple");
    let (x, y, z) = tup;
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
    // accessing tuple elements
    println!("accessing tuple elements");
    eval!(tup.0);
    eval!(tup.1);
    eval!(tup.2);
}

#[macro_export]
macro_rules! eval {
    ($x:expr) => {
        let string = stringify!($x);
        let value = $x;
        println!("{} = {:?}", string, value);
    };
}
