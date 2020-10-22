fn main() {
    let v: Vec<i32> = Vec::new();
    println!("v = {:?}", v);
    let v = vec![1, 2, 3];
    println!("v = {:?}", v);
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v = {:?}", v);
    let v = vec![1, 2, 3, 4, 5];
    println!("v = {:?}", v);
    let third = &v[2];
    println!("third = {}", third);
    match v.get(2) {
        Some(x) => println!("The third element is {}", x),
        None => println!("There is no third element"),
    }
    match v.get(6) {
        Some(x) => println!("The sixth element is {}", x),
        None => println!("There is no sixth element"),
    }
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}
