fn main() {
    // initialize
    eval!([1, 2, 3, 4, 5]);
    eval!([3; 5]);
    // access
    let a = [1, 2, 3, 4, 5];
    println!("a = {:?}", a);
    eval!(a[0]);
    eval!(a[1]);
    eval!(a[2]);
    eval!(a[3]);
    eval!(a[4]);
    //illegal access
    //let index = 5;
    //eval!(a[index]);
    //run error: thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 5'
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
