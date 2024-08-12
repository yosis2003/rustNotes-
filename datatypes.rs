fn main(){
    // below we assign x to be a 32-bit signed integer
    // this is also the default too
    let x: i32 = -2;

    /* mutliple data types:
    i8
    i16
    i32
    i64
    i128 (crazy wow)
     */
    /* mutliple unsigned data types:
    u8
    u16
    u32
    u64
    u128 (crazy wow)
     */

    // floats: f32 or f64

    // bool, and char exactly the same as C
    let true_or_false: bool = true;

    let letter:char = 'l';

    // tuples are defined by the types we tell it to have 
    let tup: (i32, bool, char) = (1, true, 's');
    let tup: (i8, bool, char) = (1, true, 's');

    // indexing tuples is stupid here :(

    println!("{}", tup.2);

    // make sure to make tuples mutable!!!

    let mut tup: (i8, bool, char) = (1, true, 's');


    let mut arr: [i32; 5] = [0, 1, 2, 4, 3];





}