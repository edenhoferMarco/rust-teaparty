use crate::teaparty::{Snack, Tea, TeaParty, TeaSet};

pub trait TeaPartyVisitor {
    fn visit_tea_party(&mut self, tea: &TeaParty);
    fn visit_tea(&mut self, tea: &Tea);
    fn visit_tea_set(&mut self, tea_set: &TeaSet);
    fn visit_snack(&mut self, snack: &Snack);
}

pub trait Visitable {
    fn accept<T: TeaPartyVisitor>(&self, visitor: &mut T);
}

pub struct DefaultVerbosityVisitor {
}

impl DefaultVerbosityVisitor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write(&self, message: String) {
        println!("{}", message);
    }
}

impl TeaPartyVisitor for DefaultVerbosityVisitor {
    fn visit_tea_party(&mut self, tea_party: &TeaParty) {
        self.write(tea_party.get_description());
    }

    fn visit_tea(&mut self, tea: &Tea) {
        self.write(tea.get_description());
    }

    fn visit_tea_set(&mut self, tea_set: &TeaSet) {
        self.write(tea_set.get_description());
    }

    fn visit_snack(&mut self, snack: &Snack) {
        self.write(snack.get_description());
    }
}
