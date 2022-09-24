

fn main() {
    const ONE_MIL: i32 = 1_000_000;
    const PI: f32 = 3.14192;
    let age = "47";

    // shadowing
    let mut age: u32 = age.trim().parse().expect("Age is a not a valid number");

    age = age + 1;
    println!("Age: {} ", age);

    //
    println!("Max u32: {} ", u32::MAX );
    println!("Max i8: {} ", i8::MAX );
    println!("Max u8: {} ", u8::MAX );
}
