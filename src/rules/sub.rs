use crate::trs::ConstantFold;
use crate::trs::Math;
use egg::rewrite as rw;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn sub() -> Vec<Rewrite> {
    vec![
        // SUB RULES
        rw!("sub-to-add"; "(- ?a ?b)"   => "(+ ?a (* -1 ?b))"),
        rw!("sub-to-add2"; "(+ ?a (* -1 ?b))" => "(- ?a ?b)"),
        // rw!("add-to-sub"; "(+ ?a ?b)"   => "(- ?a (* -1 ?b))"),
    ]
}
