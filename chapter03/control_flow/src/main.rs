fn main() {
    if_expression();
    infinite_loop();
    while_loop();
    for_loop();
}

fn if_expression() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 2, 3 and 4")
    }
    // if is an expression we can assign its result
    let number = if number % 2 == 0 { number / 2 } else { 0 };
    println!("number is {}", number);
}

fn infinite_loop() {
    let mut number = 3;
    loop {
        if number == 0 {
            break;
        }
        println!("{}!", number);
        number -= 1;
    }
}

fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
}

fn for_loop() {
    let a = [1, 2, 3];
    for number in a.iter().rev() {
        println!("{}!", number);
    }
}
