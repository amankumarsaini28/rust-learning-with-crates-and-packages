extern crate organism;

use organism::Organism;
pub struct Animal {
    name: String,
}

pub fn create_animal (name: String) -> Animal {
    Animal { name }
}

impl Organism for Animal {
    fn describe(&self) -> String {
        format!("Animal {}", self.name)
    }
}
