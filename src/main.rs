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
    // each possibility is called a variant
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

    // let coord = (2, 3);
    // let (x, y) = (5, 6);

    // println!("{:?}, {:?}", coord.0, coord.1);
    // println!("{:?}, {:?}", x, y);

    // EXPRESSIONS
    // Rust is an expression-based language
    // this means that most things are evaluated and return some value
    // expresssion value coalesce to a single point and as a result, expressions can be used for nesting loginc
    

    //MEMORY
    //memory is stored using binary (bits: 0 or 1)
    // a bit is the smallest unit of memory that can be stored
    // computer hardware is optimized to work with bytes
    //1 byte == 8 contiguous bits
    // THE ENTIRE COMPUTER MEMORY CAN BE THOUGHT OF AS ONE CONTIGUOUG STRING OF BITS
    // All data in memory has memory address. Address are always the same; only data changes
    // you won't usually utilize addresses directly; instead variables will do all the heavy lifting for you
    //Memory offset can be used to locate items at a specific address
    //offsets begin at 0 and they represent theh no of bytes away from the original address
    // you usually dont dal with offsets directly but with indexes; the compiler will calculate how many bytes away from the original address
    // ADDRESS -> VARIABLE
    // OFFSET -> INDEX

    // OWNERSHIP
    // Ownership allows rust to execute code in a performant manner and helps ensures compiled code executes correctly under various circumstances
    // All programs must track their memory usage. If they fail to do so, a memory leak occurs
    // a memory leak is when a program fails to track which memory is being used and then and has to reserve new pieces of memory
    // All programming languages have their ways of tracking memory
    // Rust uses OWNERSHIP MODEL to manage memory
    // The owner of the memory is responsible for cleaninng up. The owner is simply a function
    // Memory can either be moved or borrowed from the owner


    // IMPL
    // this allows you to implement functionality on specific enumerations and structs. This greattly enhances code organization and makes it eaesy to follow

    // struct Temperature {
    //     degrees_c: f64,
    // }

    // fn show_temp(temp: Temperature) {
    //     println!("{:?} degrees C", temp.degrees_c);
    // }

    // fn main() {
    //     let hot  = Temperature { degrees_c: 38.0 };
    //     show_temp(hot);
    // }

    // // To improve the above, we can move shw temp into Temperature since it's only gonna be part of temperature
    // struct Temperature {
    //     degrees_c: f64,
    // }

    // impl Temperature {
    //     fn show_temp(temp: Temperature) {
    //         println!("{:?} degrees C", temp.degrees_c);
    //     }
    // }

    // fn main() {
    //     // see this as OOP 
    //     // to me oo, this is better than Go's receiver structs
    //     let hot  = Temperature { degrees_c: 38.0 };
    //     Temperature::show_temp(hot);
    // }

    //We can improve the above by taking a reference to self
    // struct Temperature {
    //     degrees_c: f64,
    // }

    // impl Temperature {
    //     fn show_temp(&self) {
    //         println!("{:?} degrees C", self.degrees_c);
    //     }
    // }

    // fn main() {
    //     // see this as OOP 
    //     // to me oo, this is better than Go's receiver structs
    //     let hot  = Temperature { degrees_c: 38.0 };
    //     hot.show_temp();
    // }


    //VECTOR
    // A vector is a structure that allows you to store multiple piececs of data
    //They are used for lists of information
    // you can add, remove and traverse the entries

    // let my_numbers: Vec<i32> = vec![1, 2, 3];

    // let mut my_numbers_sec: Vec<i32> = Vec::new();

    // my_numbers_sec.push(1);
    // my_numbers_sec.push(2);
    // my_numbers_sec.push(3);

    // //remove last item
    // my_numbers_sec.pop();

    // //get length
    // my_numbers_sec.len();

    // let two = my_numbers[1];

    // //itereating through vector elements

    // for num in my_numbers {
    //     println!("{:?}", num);
    // }


    //STRINGS
    // strings are used to store texts and words
    // there are multiple types of strings

    // two most commonly used are:
    // String - owned data type
    // &str - borrowed String slice

    // To store string data in a struct, you must use an owned string
    // Uses &str when passing to a function (more efficient)

    // fn print_it(data: &str) {
    //     println!("{:?}", data);
    // }

    // fn main() {
    //     print_it("a string slice"); //automatically borrowed
        
    //     //owned string
    //     let owned_string = "owned string".to_owned();
    //     let another_owned = String::from("another");

    //     print_it(&owned_string);
    //     print_it(&another_owned);
    // }

    // DERIVE
    // This can be used to automatically implement funcitonality on enums and structs
    // Debug allows you to print an enum or struct. On structs, ensure all enums implement that derive too
    // Clone
    // Copy


    // #[derive(Debug)]
    // enum Position {
    //     Manager,
    //     Supervisor,
    //     Worker
    // }

    // struct Employee {
    //     position: Position,
    //     work_hours: i64
    // }

    // fn main () {
    //     let me  = Employee {
    //         position: Position::Worker,
    //         work_hours: 40
    //     };

    //     println!("{:?}", me.position);
    // }

    // Type Annotations
    // This is required for function signatures. Types on variables are usually inferred by the compiler, but it can also be specified in code by you.
    // Such is called Explicit Type Annotations

    // fn print_many(msg: &str, count: i32) {
    //     // do something
    // }

    // enum Mouse {
    //     LeftClick,
    //     RightClick,
    //     MiddleClick,
    // }

    // fn main() {
    //     let num: i32 = 15;
    //     let a: char = 'a';
    //     let left_click: MouseClicks = MouseClicks::LeftClick;
        
    //     //Generics
        
    //     let numbers: Vec<i32> = vec![1, 2, 3];
    //     let letters: Vec<char> = vec!['a', 'b', 'c'];
    //     let clicks: Vec<MouseClicks> = vec![
    //         MouseClicks::LeftClick,
    //         MouseClicks::RightClick,
    //         MouseClicks::MiddleClick,
    //     ];
    // }

    // ENUMS: WORKING WITH DATA
    // RECAP
    // An enum is a type that can represent one item at a time
    // Each item is called a variant
    // Each variant can optionally contain additional data

    // enum Mouse {
    //     LeftClick,
    //     RightClick,
    //     MiddleClick,
    //     Scroll(i32),
    //     Move(i32, i32),
    // }

    // enum PromoDiscount {
    //     NewUser,
    //     Holiday(String)
    // }

    // enum Discount {
    //     Percent(f64),
    //     Flat(i32),
    //     Promo(PromoDiscount), // enum in enum
    //     Custom(String), //special instructions to the cashier
    // }


    // // ADVANCED MATCH
    // enum Discount {
    //     Percent(i32),
    //     Flat(i32),
    // }

    // struct Ticket {
    //     event: String,
    //     price: i32,
    // }

    // fn main() {
    //     let n = 3;

    //     match n {
    //         3 => println!("three"),
    //         other => println!("number: {:?}", other),
    //     }

    //     let flat = Discount::Flat(2);

    //     match flat {
    //         Discount::Flat(2) => println!("flat 2"),
    //         Discount::Flat(amount) => println!("flat discount of: {:?}", amount),
    //         _ => (), // () means return nothing. here, we are ignoring percent discounts in this match
    //     }

    //     let concert = Ticket {
    //         event: "concert".to_owned(),
    //         price: 50.0
    //     };

    //     match concert {
    //         Ticket { price: 50, event } => println!("event @ 50 = {:?}", event),
    //         Ticket { price, .. } => println!("price: {:?}", price), // .. means ignore other fields in the struct
    //     }
    // }
    
}

