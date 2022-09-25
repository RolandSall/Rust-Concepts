fn main() {
   // Elements in an array should be the same type and have fixed size

    let arr_1 = [1,2,3,4];
    println!("1st: {}", arr_1[0]);
    println!("Length of the array: {}", arr_1.len());

    // Loop  1st way
    let mut loop_index = 0;
    loop {
        if loop_index >= arr_1.len() {
            break;
        }

        if arr_1[loop_index] % 2 == 0 {
            println!("{} ", arr_1[loop_index])
        }
        loop_index +=1;
    }

    // while
    let mut loop_index_2 = 0;

    while loop_index_2 < arr_1.len() {
        print!("{} ", arr_1[loop_index_2]);
        loop_index_2 += 1;

    }


    println!();
    // for loop
    let v = &["apples", "cake", "coffee"];

    for text in v {
        println!("I like {}.", text);
    }


    // for loop

    for text in arr_1.iter() {
        println!("I like {}.", text);
    }



}
