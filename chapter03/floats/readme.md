# chapter03/floats

floats types, constants, functions, methods and operators.

## Run

```console
$ cargo run --quiet --release
Data type: floats
--------------------------------------------------------------------------------
float types:
f32 -> 32 bits
f64 -> 64 bits
--------------------------------------------------------------------------------
float literals
separator '_':
1.234_567 = 1.234567
type suffix:
1.234f32 = 1.234
1.234f64 = 1.234
both:
1.234_567_f64 = 1.234567
--------------------------------------------------------------------------------
Associated functions/constants/methods/operators:
---
constants:
f64::RADIX = 2
f64::MANTISSA_DIGITS = 53
f64::DIGITS = 15
f64::EPSILON = 0.0000000000000002220446049250313
f64::MIN = -1.7976931348623157e308
f64::MIN_POSITIVE = 2.2250738585072014e-308
f64::MAX = 1.7976931348623157e308
f64::MIN_EXP = -1021
f64::MAX_EXP = 1024
f64::MIN_10_EXP = -307
f64::MAX_10_EXP = 308
f64::NAN = NaN
f64::INFINITY = inf
f64::NEG_INFINITY = -inf
std::f64::consts::E = 2.718281828459045
std::f64::consts::FRAC_1_PI = 0.3183098861837907
std::f64::consts::FRAC_2_PI = 0.6366197723675814
std::f64::consts::FRAC_2_SQRT_PI = 1.1283791670955126
std::f64::consts::FRAC_1_SQRT_2 = 0.7071067811865476
std::f64::consts::FRAC_PI_2 = 1.5707963267948966
std::f64::consts::FRAC_PI_3 = 1.0471975511965979
std::f64::consts::FRAC_PI_4 = 0.7853981633974483
std::f64::consts::FRAC_PI_6 = 0.5235987755982989
std::f64::consts::FRAC_PI_8 = 0.39269908169872414
std::f64::consts::LN_2 = 0.6931471805599453
std::f64::consts::LN_10 = 2.302585092994046
std::f64::consts::LOG2_E = 1.4426950408889634
std::f64::consts::LOG10_E = 0.4342944819032518
std::f64::consts::LOG10_2 = 0.3010299956639812
std::f64::consts::PI = 3.141592653589793
std::f64::consts::SQRT_2 = 1.4142135623730951
---
methods/associated functions:
rouding methods:
4.2_f64.floor() = 4.0
(-4.2_f64).floor() = -5.0
4.2_f64.ceil() = 5.0
(-4.2_f64).ceil() = -4.0
4.2_f64.round() = 4.0
(-4.2_f64).round() = -4.0
4.2_f64.trunc() = 4.0
(-4.2_f64).trunc() = -4.0
4.2_f64.fract() = 0.20000000000000018
4.2_f64.abs() = 4.2
(-4.2_f64).abs() = 4.2

math functions:
0.0_f64.signum() = 1.0
(-0.0_f64).signum() = -1.0
4.2_f64.signum() = 1.0
(-4.2_f64).signum() = -1.0
4.2_f64.copysign(-1.0) = -4.2
(-4.2_f64).copysign(1.0) = 4.2
0.8_f64.mul_add(7.0, -1.4) = 4.2
4.2_f64.div_euclid(0.5) = 8.0
4.2_f64.rem_euclid(0.5) = 0.20000000000000018
std::f64::consts::E.powi(2) = 7.3890560989306495
2.0_f64.powf(0.5) = 1.4142135623730951
2.0_f64.sqrt() = 1.4142135623730951
1.0_f64.exp() = 2.718281828459045
2.0_f64.ln() = 0.6931471805599453
10.0_f64.log(2f64) = 3.3219280948873626
2.0_f64.log(10f64) = 0.30102999566398114
10.0_f64.log2() = 3.321928094887362
2.0_f64.log10() = 0.3010299956639812
8.0_f64.cbrt() = 2.0
1.0_f64.exp_m1() = 1.718281828459045
1.0_f64.ln_1p() = 0.6931471805599453

trigonometry:
3.0_f64.hypot(4.0) = 5.0
std::f64::consts::FRAC_PI_2.sin() = 1.0
std::f64::consts::PI.cos() = -1.0
std::f64::consts::FRAC_PI_4.tan() = 0.9999999999999999
1.0f64.asin() = 1.5707963267948966
1.0f64.acos() = 0.0
1.0f64.atan() = 0.7853981633974483
1.0f64.atan2(1.0) = 0.7853981633974483
std::f64::consts::FRAC_PI_4.sin_cos() = (0.7071067811865475, 0.7071067811865476)
1.0f64.sinh() = 1.1752011936438014
1.0f64.cosh() = 1.5430806348152437
1.0f64.tanh() = 0.7615941559557649
1.0f64.asinh() = 0.8813735870195429
1.0f64.acosh() = 0.0
1.0f64.atanh() = inf
std::f64::consts::PI.to_degrees() = 180.0
180.0f64.to_radians() = 3.141592653589793

testing:
f64::NAN.is_nan() = true
f64::NEG_INFINITY.is_infinite() = true
f64::INFINITY.is_infinite() = true
1.0f64.is_normal() = true
1.0f64.is_sign_positive() = true
(-1.0f64).is_sign_negative() = true
1.1f64.max(2.2) = 2.2
1.1f64.min(2.2) = 1.1
---
operators:
-1.1 = -1.1
1.1 + 2.2 = 3.3000000000000003
2.2 - 1.1 = 1.1
1.1 * 2.2 = 2.4200000000000004
2.2 / 1.1 = 2.0
2.3 % 1.1 = 0.09999999999999964
```
