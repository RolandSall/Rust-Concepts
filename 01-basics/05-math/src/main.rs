use rand::Rng;
use rand::distributions::Uniform;
use rand::distributions::Distribution;

fn main() {

    // print to see the precision
    let num_1: f32 = 1.111111111111111;
    println!("f32: {}", num_1 + 0.111111111111111);

    let num_2: f64 = 1.111111111111111;
    println!("f64: {}", num_2 + 0.111111111111111);

    let num_3: u32 = 5;
    let num_4: u32 = 3;

    println!("5 + 3 = {} ", num_3 + num_4);
    println!("5 * 3 = {} ", num_3 * num_4);
    println!("5 - 3 = {} ", num_3 - num_4);
    println!("5 / 3 = {} ", num_3 / num_4);

    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random number  {} ", random_num);

    // lets role a dice with uniform distribution
    // do not worry about the loops and if --> upcoming examples
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);
    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the dice: {}", throw);
        if throw == 6 {
            break;
        }
    }


}
