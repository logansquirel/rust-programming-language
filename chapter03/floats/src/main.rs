fn main() {
    println!("Data type: floats");
    println!("{:-<80}", "");
    types();
    println!("{:-<80}", "");
    literals();
    println!("{:-<80}", "");
    println!("Associated functions/constants/methods/operators:");
    println!("{:-<3}", "");
    constants();
    println!("{:-<3}", "");
    methods();
    println!("{:-<3}", "");
    operators();
}

fn types() {
    println!("float types:");
    for size in &[32, 64] {
        println!("f{0} -> {0} bits", size);
    }
}

fn literals() {
    println!("float literals");
    println!("separator '_':");
    eval!(1.234_567);
    println!("type suffix:");
    eval!(1.234f32);
    eval!(1.234f64);
    println!("both:");
    eval!(1.234_567_f64);
}

fn constants() {
    println!("constants:");
    eval!(f64::RADIX);
    eval!(f64::MANTISSA_DIGITS);
    eval!(f64::DIGITS);
    eval!(f64::EPSILON);
    eval_fmt!(f64::MIN, "{:.16e}");
    eval_fmt!(f64::MIN_POSITIVE, "{:.16e}");
    eval_fmt!(f64::MAX, "{:.16e}");
    eval!(f64::MIN_EXP);
    eval!(f64::MAX_EXP);
    eval!(f64::MIN_10_EXP);
    eval!(f64::MAX_10_EXP);
    eval!(f64::NAN);
    eval!(f64::INFINITY);
    eval!(f64::NEG_INFINITY);
    eval!(std::f64::consts::E);
    eval!(std::f64::consts::FRAC_1_PI);
    eval!(std::f64::consts::FRAC_2_PI);
    eval!(std::f64::consts::FRAC_2_SQRT_PI);
    eval!(std::f64::consts::FRAC_1_SQRT_2);
    eval!(std::f64::consts::FRAC_PI_2);
    eval!(std::f64::consts::FRAC_PI_3);
    eval!(std::f64::consts::FRAC_PI_4);
    eval!(std::f64::consts::FRAC_PI_6);
    eval!(std::f64::consts::FRAC_PI_8);
    eval!(std::f64::consts::LN_2);
    eval!(std::f64::consts::LN_10);
    eval!(std::f64::consts::LOG2_E);
    eval!(std::f64::consts::LOG10_E);
    eval!(std::f64::consts::LOG10_2);
    eval!(std::f64::consts::PI);
    eval!(std::f64::consts::SQRT_2);
}

fn methods() {
    println!("methods/associated functions:");
    println!("rouding methods:");
    eval!(4.2_f64.floor());
    eval!((-4.2_f64).floor());
    eval!(4.2_f64.ceil());
    eval!((-4.2_f64).ceil());
    eval!(4.2_f64.round());
    eval!((-4.2_f64).round());
    eval!(4.2_f64.trunc());
    eval!((-4.2_f64).trunc());
    eval!(4.2_f64.fract());
    eval!(4.2_f64.abs());
    eval!((-4.2_f64).abs());
    println!();
    println!("math functions:");
    eval!(0.0_f64.signum());
    eval!((-0.0_f64).signum());
    eval!(4.2_f64.signum());
    eval!((-4.2_f64).signum());
    eval!(4.2_f64.copysign(-1.0));
    eval!((-4.2_f64).copysign(1.0));
    eval!(0.8_f64.mul_add(7.0, -1.4));
    eval!(4.2_f64.div_euclid(0.5));
    eval!(4.2_f64.rem_euclid(0.5));
    eval!(std::f64::consts::E.powi(2));
    eval!(2.0_f64.powf(0.5));
    eval!(2.0_f64.sqrt());
    eval!(1.0_f64.exp());
    eval!(2.0_f64.ln());
    eval!(10.0_f64.log(2f64));
    eval!(2.0_f64.log(10f64));
    eval!(10.0_f64.log2());
    eval!(2.0_f64.log10());
    eval!(8.0_f64.cbrt());
    eval!(1.0_f64.exp_m1());
    eval!(1.0_f64.ln_1p());
    println!();
    println!("trigonometry:");
    eval!(3.0_f64.hypot(4.0));
    eval!(std::f64::consts::FRAC_PI_2.sin());
    eval!(std::f64::consts::PI.cos());
    eval!(std::f64::consts::FRAC_PI_4.tan());
    eval!(1.0f64.asin());
    eval!(1.0f64.acos());
    eval!(1.0f64.atan());
    eval!(1.0f64.atan2(1.0));
    eval!(std::f64::consts::FRAC_PI_4.sin_cos());
    eval!(1.0f64.sinh());
    eval!(1.0f64.cosh());
    eval!(1.0f64.tanh());
    eval!(1.0f64.asinh());
    eval!(1.0f64.acosh());
    eval!(1.0f64.atanh());
    eval!(std::f64::consts::PI.to_degrees());
    eval!(180.0f64.to_radians());
    println!();
    println!("testing:");
    eval!(f64::NAN.is_nan());
    eval!(f64::NEG_INFINITY.is_infinite());
    eval!(f64::INFINITY.is_infinite());
    eval!(1.0f64.is_normal());
    eval!(1.0f64.is_sign_positive());
    eval!((-1.0f64).is_sign_negative());
    eval!(1.1f64.max(2.2));
    eval!(1.1f64.min(2.2));
}

fn operators() {
    println!("operators:");
    eval!(-1.1);
    eval!(1.1 + 2.2);
    eval!(2.2 - 1.1);
    eval!(1.1 * 2.2);
    eval!(2.2 / 1.1);
    eval!(2.3 % 1.1);
}

#[macro_export]
macro_rules! stringify {
    ($x:expr) => {
        std::stringify!($x)
    };
}

#[macro_export]
macro_rules! eval {
    ($x:expr) => {
        let string = stringify!($x);
        let value = $x;
        println!("{} = {:?}", string, value);
    };
}

#[macro_export]
macro_rules! eval_fmt {
    ($x:expr, $fmt: tt) => {
        let string = stringify!($x);
        let value = $x;
        print!("{} = ", string);
        println!($fmt, value);
    };
}
