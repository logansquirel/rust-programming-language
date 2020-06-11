# chapter03/booleans

Primitive boolean type and operators.

## Run

```console
$ cargo run --quiet --release
booleans:
true = true
false = false
AND:
true & true = true
true & false = false
false & true = false
false & false = false
OR:
true | true = true
true | false = true
false | true = true
false | false = false
XOR:
true ^ true = false
true ^ false = true
false ^ true = true
false ^ false = false
NOT:
!true = false
!false = true
```
