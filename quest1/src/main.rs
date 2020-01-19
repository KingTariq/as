mod meal{
   pub mod lunch{
       pub fn menu() {
            println!("What would you like to have in lunch Sir");
        }
    }
}

fn main() {
    meal::lunch::menu();
}