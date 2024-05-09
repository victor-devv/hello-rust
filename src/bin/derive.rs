// DERIVE
// This can be used to automatically implement funcitonality on enums and structs
// Debug allows you to print an enum or struct. On structs, ensure all enums implement that derive too
// Clone informs the compiler that it's allowed to automatically make a copy when storing in a struct or enum. This means that ownership no longer changes because a copy is always made
// Copy informs the compiler that it's allowed to automatically make a copy when storing in a struct or enum. This means that ownership no longer changes because a copy is always made
// Ensure to only use Clone and Copy for structs that are small in size, as this can be expensive

#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i64
}

fn print_employee(emp: Employee) {
    println!("{:?}", emp)
}

fn main () {
    let me  = Employee {
        position: Position::Worker,
        work_hours: 40
    };

    println!("{:?}", me.position);
    println!("{:?}", me);

    //no need to pass copy/reference since we have Clone and Copy derive on the struct
    print_employee(me);
    print_employee(me)
}