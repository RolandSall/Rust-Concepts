fn main() {

    let my_tuple: (u8, String, f64) = (47, String::from("Derek"), 50_000.00 );
    println!("{}",my_tuple.0);
    let (_v1,_v2,_v3) = my_tuple;
    println!("{},{}",_v2,_v3);


    //************String********************//
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");

    for word in st1.split_whitespace(){
        print!("{} ",word)
    }

    let st2 = st1.replace("A", "Another");
    println!("{}",st2);

    let mut v1: Vec<char> = st1.chars().collect();
    v1.sort();
    // remove duplicates
    v1.dedup();

    for char in v1 {
        print!("{}" ,char);
    }


    let st6:String = String::from("Just some");
    let st7:String = String::from(" words");
    let st8:String = st6 + &st7;


}
