fn main() {
    //Match expressions are similar to if else and else if chaining
    // main difference is that match must be exhaustive, which means that every possible option must be exhausted in your code
    let some_bool: bool = true;
    
    match some_bool {
        true => println!("its true"),
        false=> println!("its false"),
    }

    //in match new possibilities are checked by the compiler, so if you don't add that case to the block, youre informed by the compiler. this doesnt happen in if else

    let some_int: i32 = 5;
    
    match some_int {
        1 => println!("its 1"),
        2 => println!("its 2"),
        3 => println!("its 3"),
        _ => println!("its something else"), //_ means wildcard
    }
}
