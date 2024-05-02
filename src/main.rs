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

    // STRUCTS (structure)
    //structs contains multiple pieces of data
    // they are all or nothing; each piece of data within the struct must be populated
    // each piece of data is called a field
    // makes working with data easier
    // similar data can be grouped together

    // let box1 = ShippingBox {
    //     depth: 3,
    //     width: 2,
    //     height: 5,
    // };

    // let tall = box1.height;
    // println!("the box is {:?} units tall", tall);

    // //TUPLES
    // //a tuple is a type of record
    // //it stores data anonymously, no need to name fields unlike structs and enums
    // // useful when returning pairs of data from functions
    // // can be destructured easily into variables
    // // tuples are wrapped in ()

    // enum Access {
    //     Full,
    // }

    // fn one_two_three() -> (i32, i32, i32) {
    //     (1, 2, 3)
    // }

    // let numbers = one_two_three();

    // //destructuring
    // let (x, y, z) = one_two_three();
    
    // println!("{:?}, {:?}", x, numbers.0);
    // println!("{:?}, {:?}", y, numbers.1);
    // println!("{:?}, {:?}", z, numbers.2);

    // let (employee, access) = ("Victor", Access::Full);

    let coord = (2, 3);
    let (x, y) = (5, 6);

    println!("{:?}, {:?}", coord.0, coord.1);
    println!("{:?}, {:?}", x, y);

}

