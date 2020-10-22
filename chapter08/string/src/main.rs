fn main() {
    let s = String::new();
    println!("s = {}", s);

    let data = "initial content";
    let s = data.to_string();
    println!("s = {}", s);
    let s = data.to_owned();
    println!("s = {}", s);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s = {}", s);

    let mut s = String::from("lo");
    s.push('l');
    println!("s = {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s = {}", s);

    // let h = s[0];
    // error[E0277]: the type `String` cannot be indexed by `{integer}`
    //    --> chapter08/string/src/main.rs:27:13
    //     |
    //  27 |     let h = s1[0];
    //     |             ^^^^^ `String` cannot be indexed by `{integer}`
    //     |
    //     = help: the trait `Index<{integer}>` is not implemented for `String`
}
