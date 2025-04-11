use crate::visitor::{TeaPartyVisitor, Visitable};
pub struct TeaParty {
    name: String,
    tea: Tea,
    tea_set: TeaSet,
    snack: Snack,
}

pub enum TeaType {
    GreenTea,
    BlackTea,
    FruitsTea,
}

pub struct Tea {
    name: String,
    tea_type: TeaType,
}

pub enum SnackType {
    Dango,
    Biscuit,
    Cake,
}

pub struct Snack {
    name: String,
    snack_type: SnackType,
}

pub enum TeaSetType {
    Japanese,
    English,
    European,
}

pub struct TeaSet {
    name: String,
    tea_set_type: TeaSetType,
}

impl TeaParty {
    pub fn new(name: String, tea: Tea, tea_set: TeaSet, snack: Snack) -> Self {
        Self {
            name,
            tea,
            tea_set,
            snack,
        }
    }

    pub fn get_description(&self) -> String {
        format!("A new tea party: {}", self.name)
    }
}
impl Visitable for TeaParty {
    fn accept<T: TeaPartyVisitor>(&self, visitor: &mut T) {
        visitor.visit_tea_party(&self);
        visitor.visit_tea(&self.tea);
        visitor.visit_tea_set(&self.tea_set);
        visitor.visit_snack(&self.snack);
    }
}

impl Tea {
    pub fn new(name: String, tea_type: TeaType) -> Self {
        Self { name, tea_type }
    }
    pub fn get_description(&self) -> String {
        match self.tea_type {
            TeaType::GreenTea => {
                format!("{} - a calming Green Tea", self.name)
            }
            TeaType::BlackTea => {
                format!("{} - a tart Black Tea", self.name)
            }
            TeaType::FruitsTea => {
                format!("{} - a fresh Fruits Tea", self.name)
            }
        }
    }
}

impl Snack {
    pub fn new(name: String, snack_type: SnackType) -> Self {
        Self { name, snack_type }
    }

    pub fn get_description(&self) -> String {
        match self.snack_type {
            SnackType::Dango => format!("{} - a sweet Japanese dango", self.name),
            SnackType::Biscuit => format!("{} - a crispy biscuit", self.name),
            SnackType::Cake => format!("{} - a delicious cake", self.name),
        }
    }
}

impl TeaSet {
    pub fn new(name: String, tea_set_type: TeaSetType) -> Self {
        Self { name, tea_set_type }
    }

    pub fn get_description(&self) -> String {
        match self.tea_set_type {
            TeaSetType::Japanese => format!("{} - a traditional Japanese tea set", self.name),
            TeaSetType::English => format!("{} - a classy English tea set", self.name),
            TeaSetType::European => format!("{} - an elegant European tea set", self.name),
        }
    }
}
