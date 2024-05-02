fn main() {
    //TUPLES
    //a tuple is a type of record
    //it stores data anonymously, no need to name fields unlike structs and enums
    // useful when returning pairs of data from functions
    // can be destructured easily into variables
    // tuples are wrapped in ()

    enum Access {
        Full,
    }

    fn one_two_three() -> (i32, i32, i32) {
        (1, 2, 3)
    }

    let numbers = one_two_three();

    //destructuring
    let (x, y, z) = one_two_three();
    
    println!("{:?}, {:?}", x, numbers.0);
    println!("{:?}, {:?}", y, numbers.1);
    println!("{:?}, {:?}", z, numbers.2);

    let (employee, access) = ("Victor", Access::Full);
}