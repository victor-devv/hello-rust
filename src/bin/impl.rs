// IMPL (implementation)
// this allows you to implement functionality on specific enumerations and structs. This greattly enhances code organization and makes it eaesy to follow

struct Temperature {
    degrees_c: f64,
}

impl Temperature {
    // Self vs self => 
    // self means that entity has already been created or initialized somewhere else
    // Self means we will be creating a new one
    // fn freezing() -> Temperature { can also work

    fn freezing() -> Self {
        Self { degrees_c: 0.0 }
    }

    fn boiling() -> Self {
        Self { degrees_c: 100.0 }
    }

    fn show_temp(&self) {
        println!("{:?} degrees C", self.degrees_c);
    }
}

fn main() {
    // see this as OOP 
    // to me oo, this is better than Go's receiver structs
    let hot  = Temperature { degrees_c: 38.0 };
    hot.show_temp();

    let cold = Temperature::freezing();
    cold.show_temp();

    let boiling = Temperature::boiling();
    boiling.show_temp();
}
