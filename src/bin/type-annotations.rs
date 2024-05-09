// Type Annotations
// This is required for function signatures. Types on variables are usually inferred by the compiler, but it can also be specified in code by you.
// Such is called Explicit Type Annotations
// usually, the compiler is able to infer the ype, but when it can't you need to explicitly state it
// always state it when working with a struct in a vector for example

fn print_many(msg: &str, count: i32) {
    // do something
}

enum MouseClicks {
    LeftClick,
    RightClick,
    MiddleClick,
}

fn main() {
    let num: i32 = 15;
    let a: char = 'a';
    let left_click: MouseClicks = MouseClicks::LeftClick;
    
    //Generics
    
    let numbers: Vec<i32> = vec![1, 2, 3];
    let letters: Vec<char> = vec!['a', 'b', 'c'];
    let clicks: Vec<MouseClicks> = vec![
        MouseClicks::LeftClick,
        MouseClicks::RightClick,
        MouseClicks::MiddleClick,
    ];
}
