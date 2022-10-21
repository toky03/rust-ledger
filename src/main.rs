mod model;

use model::read_definition;
use std::fs::File;

fn main() {
    let file =
        File::open("data/account-definition.yaml").expect("account definition cannot be opened");
    let account_definition = serde_yaml::to_string(&read_definition(file))
        .expect("could not serialize Account Definition");

    println!("account definition {account_definition}");
}
