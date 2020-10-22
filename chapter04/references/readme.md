# chapter04/references

## References and Borrowing

```rust
let mut variable = 5;
let immutable_reference = &variable;
let mutable_reference = &mut variable;
```

## Rules of references

- At any given time, you can have either one mutable reference or any number of
  immutable references.
- References must always be valid.
