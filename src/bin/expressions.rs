enum Access {
    Admin,
    Manager,
    User,
    Guest
}

fn main() {
    // EXPRESSIONS
    // Rust is an expression-based language
    // this means that most things are evaluated and return some value
    // expresssion value coalesce to a single point and as a result, expressions can be used for nesting loginc

    // secret file: admisn only
    let access_level = Access::Guest;
    let can_access_file: bool = match access_level {
        Access::Admin => true,
        _ => false,
    };

    println!("can acccess: {:?}", can_access_file);
}