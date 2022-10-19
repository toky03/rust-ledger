mod model;

use std::fs::File;
use model::read_definition;

fn main() {
    let file = File::open("data/account-definition.yaml").expect("account definition cannot be opened");
    let account_definition = read_definition(file);

    println!("account definition {:?}", account_definition);
}
