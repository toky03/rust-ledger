mod model;

use model::read_ledger;
use std::fs::File;

fn main() {
    let file =
        File::open("data/ledger.yaml").expect("account definition cannot be opened");
    let account_definition = serde_yaml::to_string(&read_ledger(file))
        .expect("could not serialize Account Definition");

    println!("ledger {account_definition}");
}
