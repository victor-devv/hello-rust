// STRUCTS (structure)
//structs contains multiple pieces of data
// they are all or nothing; each piece of data within the struct must be populated
// each piece of data is called a field
// makes working with data easier
// similar data can be grouped together

struct Grocery {
    stock: i32,
    price: f64, //f64 for decimal points

}

fn main() {
    let cereal = Grocery {
        stock: 10,
        price: 2.99,
    };

    println!("stock: {:?}", cereal.stock);
    println!("price: {:?}", cereal.price);
}