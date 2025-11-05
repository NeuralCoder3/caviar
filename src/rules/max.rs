use crate::trs::ConstantFold;
use crate::trs::Math;
use egg::rewrite as rw;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn max() -> Vec<Rewrite> { vec![
    // MAX RULES
    rw!("max-to-min"; "(* -1 (min (* -1 ?a) (* -1 ?b)))" => "(max ?a ?b)"),
    // rw!("min-to-max"; "(min ?a ?b)" => "(* -1 (max (* -1 ?a) (* -1 ?b)))"),
    
]}