fn main() {
    {
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;
        println!("four = {:?}, six = {:?}", four, six);
    }
    {
        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));
        println!("home = {:?}", home);
        println!("loopback = {:?}", loopback);
    }
    {
        let quit_msg = Message::Quit;
        println!("quit = {:?}", quit_msg);
        let move_msg = Message::Move { x: 1, y: -1 };
        println!("move = {:?}", move_msg);
        let write_msg = Message::Write("message".to_string());
        println!("write = {:?}", write_msg);
        let change_color_msg = Message::ChangeColor(1, 2, 3);
        println!("change_color = {:?}", change_color_msg);
    }
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
#[derive(Debug)]
enum Message {
    Quit,                       // Unit variant
    Move { x: i32, y: i32 },    // Struct variant
    Write(String),              // Tuple variant (length = 1)
    ChangeColor(i32, i32, i32), // Tuple variant (length = 3)
}
