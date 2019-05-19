pub fn run() {
    // default is i32
    let x = 1;

    // default is f64
    let y = 2.5;

    // add explicit type
    let z: i64 = 454545454545;

    let is_active: bool = true;

    let is_greater = 10 > 5;

    let a1 = 'a';
    let face = '\u{1F600}';

    // find max value
    // println!("max i32: {}", std::i32::MAX);
    // println!("max i64: {}", std::i64::MAX);
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}