fn main() {
    //Types of loop
    // loop - infinite loop
    // while - conditional loop

    //LOOPS
    //Types of loops
    // loop - infinite loop
    // while - conditional loop

    //Infinite loop
    let mut a = 0;
    loop {
        if a == 5 {
            break;
        }

        println!("{:?}", a);
        a = a + 1;
    }

    //while loop
    let mut b = 0;
    while b != 5 {
        println!("{:?}", b);
        b = b + 1;
    }
}
