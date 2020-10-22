# chapter03/control_flow

Rust control flow includes following keywords:

- `if`
- `else`
- `loop`
- `while`
- `for`
- `break`
- `continue`
- `match`

## Conditional `if, else, else if`

```rust
if condition1 {
    // snip
} else if condition2 {
    // snip
} else {
    //snip
}
```

## Loops

```rust
// infinite loop
loop {
    // snip
    break;
    // snip
    break result;
    // snip
    continue;
}

// conditional loop
while condition {

}

// iteration over a collection
let numbers = [1, 2, 3, 4, 5];
for number in numbers {
    // snip
}
```

## Pattern matching

Match is an exhaustive control flow keyword:

```rust
let integer = 2;
match integer {
    1 => {
        // snip
    },
    2 => {
        // snip
    }
    // placeholder
    _ => {
        // snip
    }
}

let option = Some(1);
match option {
    Some(x) => { assert_eq!(x, 1); }
    None => {
        // snip
    }
}
```

The `_` placeholder covers all not already encountered possible cases.

## Conditional assign / single match pattern

```rust
let some = Some(1);
if let Some(x) = some {
    assert_eq!(x, 1);
}

let vec = vec![1, 2, 3];
let iter = vec.iter();
while let Some(x) = iter.next() {
    // snip
}
```

## Run

```console
$ cargo run --quiet --release
number is divisible by 3
number is 3
3!
2!
1!
3!
2!
1!
3!
2!
1!
```
