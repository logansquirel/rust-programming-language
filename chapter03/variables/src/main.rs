fn main() {
    // mutability/immutability
    let x = 5; // immutable by default
    println!("The value of x is: {}", x);
    //x = 6; // -> compilation error:
    //error[E0384]: cannot assign twice to immutable variable `x`
    //  --> chapter03/variables/src/main.rs:4:5
    //   |
    // 2 |     let x = 5;
    //   |         -
    //   |         |
    //   |         first assignment to `x`
    //   |         help: make this binding mutable: `mut x`
    // 3 |     println!("The value of x is: {}", x);
    // 4 |     x = 6;
    //   |     ^^^^^ cannot assign twice to immutable variable
    // solution:
    let mut x = 5; // mutable with 'mut'
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // constants
    // cannot be mutable, 'mut' forbidden
    const VALUE: u32 = 42;
    //           ---   ^^ constant expression
    //           |
    //           must be annoted
    println!(
        "The 'Answer to the Ultimate Question of Life, the Universe, and Everything' is: {}",
        VALUE
    );

    // shadowing
    let x = 5;
    let x = x + 1;
    let x = x + 2;
    println!("The value of x is: {}", x);

    // shadowing vs mutability
    // special attributes for next variable
    #[allow(unused_mut)] // prevent 'variable does not need to be mutable' warning
    #[allow(unused_variables)] // prevent 'unused variable' warning
    let mut mutable = "string";
    // mutable = mutable.len(); // changing mutable type is not ok: compilation error:
    // error[E0308]: mismatched types
    //   --> chapter03/variables/src/main.rs:44:15
    //    |
    // 44 |     mutable = mutable.len();
    //    |               ^^^^^^^^^^^^^ expected `&str`, found `usize`
    let shadow = "string";
    let shadow = shadow.len(); // changing shadow type is ok
    println!("The value of shadow is: {}", shadow);
}
