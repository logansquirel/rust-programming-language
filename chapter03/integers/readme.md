# chapter03/integers

Integers types, constants, functions, methods and operators.

## Run

```console
$ cargo run --quiet --release
Data type: integers
--------------------------------------------------------------------------------
Integer types:
signed   | unsigned
-------- | --------
i8       | u8
i16      | u16
i32      | u32
i64      | u64
i128     | u128
isize    | usize
--------------------------------------------------------------------------------
Integer literals:
separators '_':
1_234 = 1234
type suffix:
-1234i32 = -1234
both:
1_234_i32 = 1234
base/format:
98_222 = 98222
0xff = 255
0o77 = 63
0b1111_0000 = 240
--------------------------------------------------------------------------------
Associated functions/constants/methods/operators:
---
constants:
u32::MIN = 0
u32::MAX = 4294967295
i32::MIN = -2147483648
i32::MAX = 2147483647
---
methods/associated functions:
i32::from_str_radix("A", 16).unwrap() = 10
0b1111_0000_u8.count_ones() = 4
0b1111_0000_u8.count_zeros() = 4
0b0000_1111_u8.leading_zeros() = 4
0b1111_0000_u8.trailing_zeros() = 4
0b1000_1111_u8.rotate_left(2) = 0b00111110
0b1000_1111_u8 << 2 = 0b00111100
0b1000_1111_u8.rotate_right(2) = 0b11100011
0b1000_1111_u8 >> 2 = 0b00100011
0b1111_0000_0000_1111_u16.swap_bytes() = 0b0000111111110000
0b1111_0000_u8.reverse_bits() = 0b00001111
checked operation available for:
- add
- sub
- mul
- div
- div_euclid
- rem
- rem_euclid
- neg
- shl
- shr
- pow
(i32::MAX - 1).checked_add(1) = Some(2147483647)
i32::MAX.checked_add(1) = None
saturating operation available for:
- add
- sub
- mul
- pow
i32::MIN = -2147483648
i32::MIN.saturating_sub(1) = -2147483648
wrapping operation for:
- add
- sub
- mul
- div
- div_euclid
- rem
- rem_euclid
- neg
- shl
- shr
- pow
i32::MIN = -2147483648
i32::MIN.wrapping_mul(-1) = -2147483648
overflowing operation for:
- add
- sub
- mul
- div
- div_euclid
- rem
- rem_euclid
- neg
- shl
- shr
- pow
i32::MIN.overflowing_div(-1) = (-2147483648, true)
42_i32.overflowing_div(2) = (21, false)
2_i32.pow(3) = 8
1_i32.abs() = 1
(-1_i32).abs() = 1
0_i32.signum() = 0
1_i32.signum() = 1
-1_i32.signum() = -1
1_i32.is_positive() = true
(-1_i32).is_negative() = true
---
operators:
42 + 294 = 336
42 - 294 = -252
42 * 7 = 294
295 / 42 = 7
295 % 42 = 1
-(123) = -123
0b1000_1111_u8 << 2 = 0b00111100
0b1000_1111_u8 >> 2 = 0b00100011
0b0011_1100_u8 & 0b_0000_1111_u8 = 0b00001100
0b1100_1100_u8 | 0b_0000_1111_u8 = 0b11001111
!0b1100_1100_u8 = 0b00110011
```
