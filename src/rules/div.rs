use crate::trs::ConstantFold;
use crate::trs::Math;
use egg::rewrite as rw;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn div() -> Vec<Rewrite> {
    vec![
        //DIV RULES
        rw!("div-zero"      ; "(/ num0 ?x)"            => "(num0)" if crate::trs::is_not_zero("?x")),
        rw!("div-cancel"    ; "(/ ?a ?a)"           => "num1" if crate::trs::is_not_zero("?a")),
        rw!("div-minus-down"; "(/ (* numneg1 ?a) ?b)"    => "(/ ?a (* numneg1 ?b))" if crate::trs::is_not_zero("?b")),
        rw!("div-minus-up"  ; "(/ ?a (* numneg1 ?b))"    => "(/ (* numneg1 ?a) ?b)" if crate::trs::is_not_zero("?b")),
        rw!("div-minus-in"  ; "(* numneg1 (/ ?a ?b))"    => "(/ (* numneg1 ?a) ?b)" if crate::trs::is_not_zero("?b")),
        rw!("div-minus-out" ; "(/ (* numneg1 ?a) ?b)"    => "(* numneg1 (/ ?a ?b))" if crate::trs::is_not_zero("?b")),
        //FOLD
        rw!("div-consts-div"; "( / ( * ?x ?a ) ?b )" => "( / ?x ( / ?b ?a ) )" if crate::trs::compare_c0_c1("?b", "?a", "%0<0")),
        rw!("div-consts-mul"; "( / ( * ?x ?a ) ?b )" => "( * ?x ( / ?a ?b ) )" if crate::trs::compare_c0_c1("?a", "?b", "%0<")),


        // INCONSISTENT
        // rw!("div-consts-add"; "( / ( + ( * ?x ?a ) ?y ) ?b )" => "( + ( * ?x ( / ?a ?b ) ) ( / ?y ?b ) )" if crate::trs::compare_c0_c1("?a", "?b", "%0<")),
        // rw!("div-separate"  ; "( / ( + ?x ?a ) ?b )" => "( + ( / ?x ?b ) ( / ?a ?b ) )" if crate::trs::compare_c0_c1("?a", "?b", "%0<")),
    ]
}
