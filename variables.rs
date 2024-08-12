fn main() {
    let x = 4;
    // this is an implicit type assignment
    // rust is statically and strongly typed. Like C/C++

    println!("x is : {}", x);

    // all variables are BY DEFAULT, immutable. You need to
    // redeclare the variable, or declare it as MUT

    let mut y = 4;

    y = 5;

    println!("y is : {}", y);

    // you can also do this, it saves the state :woozy:

    let x = x + 1; // this makes the variable x = 5


    // by the way variable scopes here work just like Go :(

    // if I do this for instance
    {
        let x = x + 2;
        println!("x is : {}", x);
    // the value becomes 7
    }

    // but outside of here, it stays as 5
    println!("x is : {}", x);

    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{}", SECONDS_IN_MINUTE);


}
