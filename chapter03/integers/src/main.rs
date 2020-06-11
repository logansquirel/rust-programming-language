fn main() {
    println!("Data type: integers");
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
    println!("Integer types:");
    println!("{:8} | {:8}", "signed", "unsigned");
    println!("{0:-<8} | {0:-<8}", "");
    for i in &[8, 16, 32, 64, 128] {
        println!("i{0:<7} | u{0:<7}", i)
    }
    println!("{:<8} | {:<8}", "isize", "usize");
}

fn literals() {
    println!("Integer literals:");
    println!("separators '_':");
    eval!(1_234);
    println!("type suffix:");
    eval!(-1234i32);
    println!("both:");
    eval!(1_234_i32);
    println!("base/format:");
    eval!(98_222);
    eval!(0xff);
    eval!(0o77);
    eval!(0b1111_0000);
}

fn constants() {
    println!("constants:");
    eval!(u32::MIN);
    eval!(u32::MAX);
    eval!(i32::MIN);
    eval!(i32::MAX);
}

fn methods() {
    println!("methods/associated functions:");
    eval!(i32::from_str_radix("A", 16).unwrap());
    eval!(0b1111_0000_u8.count_ones());
    eval!(0b1111_0000_u8.count_zeros());
    eval!(0b0000_1111_u8.leading_zeros());
    eval!(0b1111_0000_u8.trailing_zeros());
    eval_fmt!(0b1000_1111_u8.rotate_left(2), "{:#010b}");
    eval_fmt!(0b1000_1111_u8 << 2, "{:#010b}");
    eval_fmt!(0b1000_1111_u8.rotate_right(2), "{:#010b}");
    eval_fmt!(0b1000_1111_u8 >> 2, "{:#010b}");
    eval_fmt!(0b1111_0000_0000_1111_u16.swap_bytes(), "{:#018b}");
    eval_fmt!(0b1111_0000_u8.reverse_bits(), "{:#010b}");
    println!("checked operation available for:");
    for op in &[
        "add",
        "sub",
        "mul",
        "div",
        "div_euclid",
        "rem",
        "rem_euclid",
        "neg",
        "shl",
        "shr",
        "pow",
    ] {
        println!("- {}", op);
    }
    eval!((i32::MAX - 1).checked_add(1));
    eval!(i32::MAX.checked_add(1));
    println!("saturating operation available for:");
    for op in &["add", "sub", "mul", "pow"] {
        println!("- {}", op);
    }
    eval!(i32::MIN);
    eval!(i32::MIN.saturating_sub(1));
    println!("wrapping operation for:");
    for op in &[
        "add",
        "sub",
        "mul",
        "div",
        "div_euclid",
        "rem",
        "rem_euclid",
        "neg",
        "shl",
        "shr",
        "pow",
    ] {
        println!("- {}", op);
    }
    eval!(i32::MIN);
    eval!(i32::MIN.wrapping_mul(-1));
    println!("overflowing operation for:");
    for op in &[
        "add",
        "sub",
        "mul",
        "div",
        "div_euclid",
        "rem",
        "rem_euclid",
        "neg",
        "shl",
        "shr",
        "pow",
    ] {
        println!("- {}", op);
    }
    eval!(i32::MIN.overflowing_div(-1));
    eval!(42_i32.overflowing_div(2));
    eval!(2_i32.pow(3));
    eval!(1_i32.abs());
    eval!((-1_i32).abs());
    eval!(0_i32.signum());
    eval!(1_i32.signum());
    eval!((-1_i32).signum());
    eval!(1_i32.is_positive());
    eval!((-1_i32).is_negative());
}

fn operators() {
    println!("operators:");
    eval!(42 + 294);
    eval!(42 - 294);
    eval!(42 * 7);
    eval!(295 / 42);
    eval!(295 % 42);
    eval!(-(123));
    eval_fmt!(0b1000_1111_u8 << 2, "{:#010b}");
    eval_fmt!(0b1000_1111_u8 >> 2, "{:#010b}");
    eval_fmt!(0b0011_1100_u8 & 0b_0000_1111_u8, "{:#010b}");
    eval_fmt!(0b1100_1100_u8 | 0b_0000_1111_u8, "{:#010b}");
    eval_fmt!(!0b1100_1100_u8, "{:#010b}");
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
