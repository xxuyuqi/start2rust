fn main() {
    // integers
    let _a: i32 = 100_000_000;
    let _a1 = 1_i32; // i8 i16 i64 i128 isize
    // -2^(n-1)~2^(n-1)-1

    let _b : u8 = 1; // u16 u32 u64 u128 usize
    // 0~2^n-1
    // isize and usize depend on the type of CPU, they are intended to be the index of a set.

    // int_overflow()

    // floats
    let _c = 2.0_f64; // f32

    // Determines that two floating-point numbers are equal
    // assert!(0.1_f64 + 0.2_f64 == 0.3_f64);
    // assert!(0.1_f32 + 0.2_f32 == 0.3_f32);
    // The more recommand approach
    assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.00001);

    // NAN
    let x = (-1.0_f32).sqrt();
    if x.is_nan() {
        println!("nan");
    
    // operator
    // + - * / %  & | ^  ! << >>
    
    // Range
    for i in 1..=5 {
        println!("{}",i);
    }
    for i in 'a'..='z' {
        println!("{}",i);
    }

    // char
    let z = 'ℤ'; // unicode
    println!("{} bytes",std::mem::size_of_val(&z));

    // bool
    let _t = true | false;

    // element
    () // returned by main and element type has a unique value "()
    }


}

#[allow(dead_code)]
fn int_overflow() -> ()
{
    // can be compiled in release mode.
    let a: u8 = 1_u8<<9-1;
    print!("a : {}, ", a);
    println!("a + 1: {}", a+1);
}
