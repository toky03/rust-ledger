mod model;
use model::read_ledger;
use std::fs::File;

fn main() {
    let file = File::open("data/ledger.yaml").expect("account definition cannot be opened");
    let account_definition =
        serde_yaml::to_string(&read_ledger(file)).expect("could not serialize Account Definition");

    println!("entity {account_definition}");
}

#[cfg(test)]
mod tests {
    use arch_test_core::access_rules::{
        MayNotAccess, MayNotBeAccessedBy, NoLayerCyclicDependencies, NoModuleCyclicDependencies,
        NoParentAccess,
    };
    use arch_test_core::{hash_set, Architecture, ModuleTree};

    #[test]
    fn arch_tests() {
        let architecture = Architecture::new(hash_set![
            "model".to_owned(),
            "entity".to_owned(),
            "core".to_owned()
        ])
        .with_access_rule(NoParentAccess)
        .with_access_rule(NoModuleCyclicDependencies)
        .with_access_rule(NoLayerCyclicDependencies)
        .with_access_rule(MayNotBeAccessedBy::new(
            "core".to_owned(),
            hash_set!["entity".to_owned()],
            false,
        ))
        .with_access_rule(MayNotAccess::new(
            "entity".to_owned(),
            hash_set!["core".to_owned()],
            false,
        ));
        let module_tree = ModuleTree::new("src/model.rs");
        assert!(architecture.validate_access_rules().is_ok());
        assert!(architecture.check_access_rules(&module_tree).is_ok());
    }
}
