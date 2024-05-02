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