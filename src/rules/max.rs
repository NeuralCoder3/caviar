use crate::trs::ConstantFold;
use crate::trs::Math;
use egg::rewrite as rw;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn max() -> Vec<Rewrite> { vec![
    // MAX RULES
    rw!("max-to-min"; "(max ?a ?b)" => "(* numneg1 (min (* numneg1 ?a) (* numneg1 ?b)))"),
    // rw!("min-to-max"; "(min ?a ?b)" => "(* numneg1 (max (* numneg1 ?a) (* numneg1 ?b)))"),
    
]}