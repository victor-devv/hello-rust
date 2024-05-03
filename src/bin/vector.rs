fn main() {
    //VECTOR
    // A vector is a structure that allows you to store multiple piececs of data
    //They are used for lists of information
    // you can add, remove and traverse the entries

    let my_numbers: Vec<i32> = vec![1, 2, 3];

    let mut my_numbers_sec: Vec<i32> = Vec::new();

    my_numbers_sec.push(1);
    my_numbers_sec.push(2);
    my_numbers_sec.push(3);

    //remove last item
    my_numbers_sec.pop();

    //get length
    my_numbers_sec.len();

    let two = my_numbers[1];

    //itereating through vector elements

    for num in my_numbers {
        println!("{:?}", num);
    }
}
