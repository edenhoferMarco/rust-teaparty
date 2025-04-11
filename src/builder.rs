use crate::teaparty::{Snack, Tea, TeaParty, TeaSet};

pub trait TeaPartyNameStep {
    fn name(self, name: &str) -> impl TeaPartyTeaStep;
}

pub trait TeaPartyTeaStep {
    fn tea(self, tea: Tea) -> impl TeaPartyTeaSetStep;
}

pub trait TeaPartyTeaSetStep {
    fn tea_set(self, tea_set: TeaSet) -> impl TeaPartySnackStep;
}

pub trait TeaPartySnackStep {
    fn snack(self, snack: Snack) -> impl TeaPartyBuildStep;
}

pub trait TeaPartyBuildStep {
    fn build(self) -> TeaParty;
}

pub struct TeaPartyStepBuilderInternals {
    name: String,
    tea: Option<Tea>,
    tea_set: Option<TeaSet>,
    snack: Option<Snack>,
}

pub struct TeaPartyStepBuilder;

impl TeaPartyNameStep for TeaPartyStepBuilder {
    fn name(self, name: &str) -> impl TeaPartyTeaStep {
        TeaPartyStepBuilderInternals {
            name: name.to_string(),
            tea: None,
            tea_set: None,
            snack: None,
        }
    }
}

impl TeaPartyTeaStep for TeaPartyStepBuilderInternals {
    fn tea(mut self, tea: Tea) -> impl TeaPartyTeaSetStep {
        self.tea = Some(tea);
        self
    }
}

impl TeaPartyTeaSetStep for TeaPartyStepBuilderInternals {
    fn tea_set(mut self, tea_set: TeaSet) -> impl TeaPartySnackStep {
        self.tea_set = Some(tea_set);
        self
    }
}

impl TeaPartySnackStep for TeaPartyStepBuilderInternals {
    fn snack(mut self, snack: Snack) -> impl TeaPartyBuildStep {
        self.snack = Some(snack);
        self
    }
}

impl TeaPartyBuildStep for TeaPartyStepBuilderInternals {
    fn build(self) -> TeaParty {
        TeaParty::new(
            self.name,
            self.tea.expect("Tea must be set"),
            self.tea_set.expect("Tea set must be set"),
            self.snack.expect("Snack must be set"),
        )
    }
}
