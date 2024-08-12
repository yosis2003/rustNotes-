// this allows us to use input/output crate
use std::io;

fn main(){
    // USER INPUT NOT INCLUDED IN PRELUDE
    println!("hello world!");

    let mut input = String::new();

    // the reason we have the .expect here is because rust returns some error handling objects
    // the .expect essentially just receives that error object and we go ahead and print out an error statement in that case
    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("{}", input);
}