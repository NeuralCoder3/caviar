use crate::trs::ConstantFold;
use crate::trs::Math;
use crate::trs::ConditionRewrite;
use crate::rewrite2 as rw;
pub type Rewrite = ConditionRewrite<Math, ConstantFold>;
pub fn custom() -> Vec<Rewrite> {
    vec![
        // new rules
        rw!("lt-mul"     ; "(< ?a (* ?b ( min ?c ?d)))" => "(&& (< ?a (* ?b ?c)) (< ?a (* ?b ?d)) )"  if crate::trs::is_const_pos("?b")),
        rw!("mod-min"     ; "(% ?a ?b)" => "(min (+ ?b -1 ) (% ?a ?b))"  if crate::trs::is_const_pos("?b")),
        rw!("mod-max"     ; "(% ?a ?b)" => "(max (- 1 ?b) (% ?a ?b))"  if crate::trs::is_const_pos("?b")),

        // range same, range congruence div/add exists

        rw!("div-mul-reduce"; "(/ (* ?a ?x) ?b)" => "(+ (* (/ ?a ?b) ?x) (/ (* (% ?a ?b) ?x) ?b))" if crate::trs::CompareCondition::new(vec!["?a", "?b"], |vals| vals["?a"].abs() >= vals["?b"].abs() && vals["?b"] != 0)),

        rw!("eq-div-to-ineq-pos"; "(== (/ ?x ?c) ?k)" => "(&& (<= (* ?k ?c) ?x) (< ?x (+ (* ?k ?c) ?c)))" if crate::trs::is_const_pos("?c") if crate::trs::is_const_pos("?k")),

        // not needed
        // rw!("eq-div-to-ineq-neg"; "(== (/ ?x ?c) ?k)" => "(&& (< (- (* ?k ?c) ?c) ?x) (<= ?x (* ?k ?c)))" if crate::trs::CompareCondition::new(vec!["?c", "?k"], |vals| vals["?c"] > 0 && vals["?k"] < 0)),
        // rw!("eq-div-to-ineq-neg"; "(== (/ ?x ?c) ?k)" => "(&& (< (+ (* ?k ?c) ?c) ?x) (<= ?x (* ?k ?c)))" if crate::trs::CompareCondition::new(vec!["?c", "?k"], |vals| vals["?c"] < 0 && vals["?k"] > 0)),


        rw!("lt-mul-const-pos"; "(< (* ?c ?x) ?k)" => "(<= ?x (/ (- ?k 1) ?c))" if crate::trs::CompareCondition::new(vec!["?c", "?k"], |vals| vals["?c"] > 0 && vals["?k"] > 0)),

        // not implied => needed
        rw!("lt-mul-const-neg"; "(< (* ?c ?x) ?k)" => "(<= ?x (/ (- ?k ?c) ?c))" if crate::trs::CompareCondition::new(vec!["?c", "?k"], |vals| vals["?c"] > 0 && vals["?k"] <= 0)),

        // rw!("lt-mul-const-pos"; "(< (* ?c ?x) ?k)" => "(< ?x (+ (/ ?k ?c) 1))" if crate::trs::is_const_pos("?c")), 
    ]
}

