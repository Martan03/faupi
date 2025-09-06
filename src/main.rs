use crate::specs::Specs;

pub mod error;
pub mod specs;

fn main() {
    println!("Hello, faupi!");
    let specs = Specs::load("test.yaml");
    println!("{:?}", specs);
}
