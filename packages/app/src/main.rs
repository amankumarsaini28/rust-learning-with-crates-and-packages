extern crate lib;
extern crate organism;

use lib::create_animal;
use organism::Organism;

fn main() {
    let tiger = create_animal(String::from("Tiger"));

    println!("{:?}", tiger.describe());
}
