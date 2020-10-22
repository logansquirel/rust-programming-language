fn main() {
    println!("Generate the nth Fibonacci number");
    let n: u32 = loop {
        println!("Please input your desired fibonacci number index");
        let mut n = String::new();
        std::io::stdin()
            .read_line(&mut n)
            .expect("failed to read line");
        match n.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        }
    };
    println!("n = {}", n);
    println!("fib[n] = {}", fibonacci(n));
}

fn fibonacci(n: u32) -> u32 {
    match n {
        1 | 2 => 1,
        x => fibonacci(x - 1) + fibonacci(x - 2),
    }
}
