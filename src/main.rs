mod builder;
mod teaparty;
mod visitor;

use crate::builder::*;
use crate::teaparty::{Snack, SnackType, Tea, TeaSet, TeaSetType, TeaType};
use visitor::*;

fn main() {
    let mut visitor = DefaultVerbosityVisitor::new();
    let tea_party = TeaPartyStepBuilder
        .name("My awesome tea party")
        .tea(Tea::new("Grandmas finest".to_string(), TeaType::BlackTea))
        .tea_set(TeaSet::new("Classic Tea Set".to_string(), TeaSetType::European))
        .snack(Snack::new("Chocolate tart".to_string(), SnackType::Cake))
        .build();
    tea_party.accept(&mut visitor);
}
