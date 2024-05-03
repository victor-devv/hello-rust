
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

// So far we have been moving memory

enum Light {
    Bright,
    Dull,
}

fn display_light(light: Light) {
    match light {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}

fn display_borrowed_light(light: &Light) {
    match light {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}

fn main() {
    //EXAMPLE - MOVE

    //the function main owns the variable dull here
    let dull = Light::Dull;

    //calling the function display light MOVES ownership of dull from main to display_light. that means tha tdisplay_light OWNS dull
    //IN RUST, ANY FUNCTION THAT OWNS DATA IS REQUIRED TO DELETE DATA ONCE THE FUNCTION COMPLETES
    //THIS MEANS THAT DULL WILL BE DELETED ONCE DISPLAY_LIGHT COMPLETES
    // ONCE DELETED, ITS NO LONGER AVAILABLE FOR USE IN THE SAME FUNCTION CALL

    display_light(dull);
    //This (below) will throw an error as dull no longer exists
    //display_light(dull);

    //EXAMPLE - BORROW

    // in order to use it twice, you have to borrow the variable
    // & in rust means you're REFERENCING (borrowing) data
    // the owner of bright is still main since the function borrows or references it
    let bright = Light::Bright;
    display_borrowed_light(&bright);
    display_borrowed_light(&bright);

}
