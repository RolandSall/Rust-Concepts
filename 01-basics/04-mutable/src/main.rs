// defining some imports that might be used
#[warn(unused_imports)]
use std::io;
use rand::Rng;
use std::io::{Write, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("What is your name");
    // define a mutable variable * a variable that can change by default everything is immutable
    // Rust use immutable this will allow rust to eliminate the need how variables will change in you code
    let mut  name:String = String::new();
    let greeting = "Nice to meet you";

    // & define this is a reference to the variable
    /*
    pub fn expect(self, msg: &str) -> T
    where
        E: fmt::Debug,
    {
        match self {
            Ok(t) => t,
            Err(e) => unwrap_failed(msg, &e),
        }
    }
     The expect method signature above
     */
    io::stdin().read_line( &mut name).expect("Did not receive an input");

    // we will see how to handels the error later

    println!("Hello {}! {} ", name.trim_end(),greeting);

}
