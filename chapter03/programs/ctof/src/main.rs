fn main() {
    println!("Convert temperature from Celsius to Fahrenheit");
    let cels: f64 = loop {
        println!("Please input your temperature in Celsius");
        let mut cels = String::new();
        std::io::stdin()
            .read_line(&mut cels)
            .expect("failed to read line");
        match cels.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };
    println!("Temperature: {}°C", cels);
    let fahr = cels * 9.0 / 5.0 + 32.0;
    println!("Temperature: {}°F", fahr);
}
