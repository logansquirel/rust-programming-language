fn main() {
    // create a tuple
    let tup: (i32, f64, &str) = (42, 4.2, "42!");
    println!("{:?}", tup);
    // destructuring a tuple
    let (x, y, z) = tup;
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);
    // accessing tuple elements
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);
}
