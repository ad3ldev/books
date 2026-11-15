pub fn interger_types() {
    let signed_8_bits: i8 = 0;
    let signed_16_bits: i16 = 0;
    let signed_32_bits: i32 = 0;
    let signed_64_bits: i64 = 0;
    let signed_128_bits: i128 = 0;

    let unsigned_8_bits: u8 = 0;
    let unsigned_16_bits: u16 = 0;
    let unsigned_32_bits: u32 = 0;
    let unsigned_64_bits: u64 = 0;
    let unsigned_128_bits: u128 = 0;

    let decimal_int = 98_222;
    let hex_int = 0xff;
    let octal_int = 0o77;
    let binary_int = 0b1111_0000;
    let byte_int = b'A';

    let mut overflow: u8 = 255;
    overflow = overflow.wrapping_add(1);
    println!("signed_8_bits is {signed_8_bits}");
    println!("signed_16_bits is {signed_16_bits}");
    println!("signed_32_bits is {signed_32_bits}");
    println!("signed_64_bits is {signed_64_bits}");
    println!("signed_128_bits is {signed_128_bits}");

    println!("unsigned_8_bits is {unsigned_8_bits}");
    println!("unsigned_16_bits is {unsigned_16_bits}");
    println!("unsigned_32_bits is {unsigned_32_bits}");
    println!("unsigned_64_bits is {unsigned_64_bits}");
    println!("unsigned_128_bits is {unsigned_128_bits}");

    println!("decimal_int is {decimal_int}");
    println!("hex_int is {hex_int}");
    println!("octal_int is {octal_int}");
    println!("binary_int is {binary_int}");
    println!("byte_int is {byte_int}");

    println!("overflow is {overflow}");
}

pub fn float_types() {
    let float_32_bits: f32 = 0.0;
    let float_64_bits: f64 = 0.0;
    println!("float_32_bits is {float_32_bits}");
    println!("float_64_bits is {float_64_bits}");
}

pub fn bool_types() {
    let t = true;
    let f: bool = false;
    println!("{t}, {f}")
}

pub fn char_types() {
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("{c}, {z}, {heart_eyed_cat}")
}

pub fn componund_types() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("{five_hundred}, {six_point_four}, {one}")
}

pub fn array_types() {
    let a = [1, 2, 3, 4, 5, 6];
    println!("{}", a[0]);
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", a[0]);
    let a = [3; 5];
    println!("{}", a[0]);

    let first = a[0];
    let second = a[1];
    println!("{}, {}, {first}, {second}", a[0], months[0])
}
