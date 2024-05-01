fn main() {
    //beautiful comment baby
    println!("Hello, world!");

    //let my_favorite_color = "red";

    // let life = 42;

    // bang ! means macro
    // {:?} means debug print
    //without :?  means end user display
    
    // println!("hello");
    // println!("{:?}", life); //debug print
    // println!("{life}"); //end user print

    //CONTROL FLOW
    // let a = 99;

    // if a > 99 {
    //     if a > 200 {
    //         println!("Huge number");
    //     } else {
    //         println!("Big number")
    //     }
    // } else {
    //     println!("Small number")
    // }

    //LOOPS
    //Types of loops
    // loop - infinite loop
    // while - conditional loop

    //Infinite loop
    // let mut a = 0;
    // loop {
    //     if a == 5 {
    //         break;
    //     }

    //     println!("{:?}", a);
    //     a = a + 1;
    // }

    // //while loop
    // let mut b = 0;
    // while b != 5 {
    //     println!("{:?}", b);
    //     b = b + 1;
    // }

    //Match expressions are similar to if else and else if chaining
    // main difference is that match must be exhaustive, which means that every possible option must be exhausted in your code
    //in match new possibilities are checked by the compiler, so if you don't add that case to the block, youre informed by the compiler. this doesnt happen in if else
    // let some_bool: bool = true;

    // match some_bool {
    //     true => println!("its true"),
    //     false=> println!("its false"),
    // }

    // let some_int: i32 = 5;

    // match some_int {
    //     1 => println!("its 1"),
    //     2 => println!("its 2"),
    //     3 => println!("its 3"),
    //     _ => println!("its something else"), //_ means wildcard
    // }

    // ENUM (ENUMERATION)
    // An enum is a piece data that can be one of multiple different possibilities
    //each possibility is called a variant
    // enums provide info about your program to the compiler resulting in more robust programms

    // enum Direction {
    //     Up,
    //     Down,
    //     Left,
    //     Right
    // }

    // fn which_way(go: Direction) {
    //     match go {
    //         Direction::Up => "up",
    //         Direction::Down => "down",
    //         Direction::Left => "left",
    //         Direction::Right => "right",
    //     }
    // }

}

