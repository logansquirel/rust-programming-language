# chapter06/enums

## Defining an enum

```rust
enum Message {
    Quit,                       // Unit variant
    Move { x: i32, y: i32 },    // Struct variant
    Write(String),              // Tuple variant (length = 1)
    ChangeColor(i32, i32, i32), // Tuple variant (length = 3)
}
```

## Matching on enum

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```
