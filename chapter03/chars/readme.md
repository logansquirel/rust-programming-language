# chapter03/chars

Primitive char type literals and methods.

## Run

```console
$ cargo run --quiet --release
Chars
literals
'a' = 'a'
'\u{0061}' = 'a'
methods/associated functions:
'1'.is_digit(10) = true
'a'.is_digit(10) = false
'a'.is_alphabetic() = true
'a'.is_lowercase() = true
'A'.is_uppercase() = true
' '.is_whitespace() = true
'\u{009c}'.is_control() = true
'7'.is_numeric() = true
'a'.to_uppercase().to_string() = "A"
'A'.to_uppercase().to_string() = "A"
'a'.is_ascii() = true
'µ'.is_ascii() = false
```
