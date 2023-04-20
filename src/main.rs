mod machines;
use machines::*;

fn main() {
    let m1 = Machine::<Assembly>::new(10);
    let m2 = Machine::<QualityChecking>::new(9);
    let m3 = Machine::<Soldering>::new(25);
    println!("[m1]: {:?}", m1);
    println!("[m2]: {:?}", m2);
    println!("[m3]: {:?}", m3);
    println!("m1 price: {:?}", m1.map(|m| m.price()));
    println!("m2 price: {:?}", m2.map(|m| m.price()));
    println!("m3 price: {:?}", m3.map(|m| m.price()));
}
