fn main() {
    println!("Convert temperature from Fahrenheit to Celsius");
    let fahr: f64 = loop {
        println!("Please input your temperature in Fahrenheit");
        let mut fahr = String::new();
        std::io::stdin()
            .read_line(&mut fahr)
            .expect("failed to read line");
        match fahr.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };
    println!("Temperature: {}°F", fahr);
    let cels = (fahr - 32.0) * 5.0 / 9.0;
    println!("Temperature: {}°C", cels);
}
