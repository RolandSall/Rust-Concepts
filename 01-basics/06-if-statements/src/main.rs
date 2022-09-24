use std::cmp::Ordering;

fn main() {
    let car_model:i32 = 2000;
    if (car_model > 2018) {
        println!("New Model \n")
    } else if(car_model >= 2000 && car_model < 2018)  {
        print!("Still Almost new \n");
    } else {
        println!("Old car \n")
    }

    // ternary operator

    let my_age = 21;
    let can_vote = if my_age >= 18{
        true
    } else {
        false
    };
    println!("Can vote: {}", can_vote);

    // Match

    let car_model = 1998;
    match car_model {
        2017..= 2020 => println!("New Model"),
        2000 => println!("The 2000s Models!"),
        i32::MIN ..=1999 => println!("Oldies models"),
        _ => {}
    }

    // Match example 2
    let license_age = 18;
    match my_age.cmp(&license_age) {
        Ordering::Less => println!("Cannot Drive"),
        Ordering::Greater | Ordering::Equal => println!("Can Drive"),
        _ => {}
    }

    // Match example 3
    let food = "apple";
    match food {
        "apple" => println!("Fruits!"),
         &_ => {}
    }
}
