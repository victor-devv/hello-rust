//STRINGS
// strings are used to store texts and words
// there are multiple types of strings

// two most commonly used are:
// String - owned data type
// &str - borrowed String slice

// To store string data in a struct, you must use an owned string
// Uses &str when passing to a function (more efficient)

struct Employee {
    //use String not &str
    name: String,
}

fn print_it(data: &str) {
    println!("{:?}", data);
}

struct LineItem {
    name: String,
    count: i32,
}

fn print_name(name: &str) {
    println!("name: {:?}", name);
}

fn main() {
    print_it("a string slice"); //automatically borrowed
    
    //owned string
    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another");

    print_it(&owned_string);
    print_it(&another_owned);

    let emp_name = "Victor".to_owned();
    let emp = Employee { name: emp_name };

    let receipt = vec![
        LineItem {
            name: "cereal".to_owned(),
            count: 1,
        },
        LineItem {
            name: String::from("fruit"),
            count: 3,
        }
    ];

    for item in receipt {
        //you must pass a reference of the string to functions
        print_name(&item.name);
        println!("count: {:?}", item.count);
    }
}