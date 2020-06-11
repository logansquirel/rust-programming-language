fn main() {
    println!("Hello world!");
    another_function();
    function_with_parameter(5);
    function_with_parameters(5, 6);
    function_statements_and_expression();
}

fn another_function() {
    println!("Another function!");
}

fn function_with_parameter(x: i32) {
    println!("The value of x is {}", x);
}

fn function_with_parameters(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

#[allow(clippy::let_and_return)]
fn function_statements_and_expression() -> i32 {
    // following is a statement - ending with ';'
    let x = 5;
    // following is an expression - without ';'
    x
}
