enum Access {
    Admin,
    Manager,
    User,
    Guest
}

fn main() {
    // secret file: admisn only
    let access_level = Access::Guest;
    let can_access_file: bool = match access_level {
        Access::Admin => true,
        _ => false,
    };

    println!("can acccess: {:?}", can_access_file);
}