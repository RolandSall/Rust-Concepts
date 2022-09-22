/***

    to declare a variable you will use the let keyword
    to print on screen we can use println! that takes the below arguments

    Scalar types:
        - Integer by default i32
        - Floating-point
        - Booleans
        - Characters
 */

fn main(){
    let _company_name = "This is a string";
    let _float_number = 4.5;
    let _is_flag = false;



    println!("company name is:{}-{}",_company_name, _is_flag);
    println!("rating:{}",_float_number);


    // ------------Integer examples--------- //
    let age:u32 = 20;
    let sum:i32 = 5-15;
    let mark:isize = 10;
    let count:usize = 30;

    // ----------- Overflow ----------- //
    let _age:u8 = 255;
    // 0 to 255 only allowed for u8
    //let _weight:u8 = 256;   //overflow value is 0

    println!("age is {} ", _age);
    println!("age is {} ",_weight);
}