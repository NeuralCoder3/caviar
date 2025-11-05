use crate::trs::ConstantFold;
use crate::trs::Math;
use egg::rewrite as rw;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn modulo() -> Vec<Rewrite> {
    vec![
        //MOD RULES
        rw!("mod-zero"      ; "(% num0 ?x)"             => "num0" if crate::trs::is_not_zero("?x")),
        rw!("mod-x-x"       ; "(% ?x ?x)"            => "num0" if crate::trs::is_not_zero("?x")),
        rw!("mod-one"       ; "(% ?x num1)"             => "num0"),
        rw!("mod-minus-out" ; "(% (* ?x numneg1) ?c)"     => "(* numneg1 (% ?x ?c))" if crate::trs::is_not_zero("?c")),
        rw!("mod-minus-in"  ; "(* numneg1 (% ?x ?c))"     => "(% (* ?x numneg1) ?c)" if crate::trs::is_not_zero("?c")),
        //FOLD
        rw!("mod-consts"    ; "( % ( + ( * ?x ?c0 ) ?y ) ?c1 )" => "( % ?y ?c1 )" if crate::trs::compare_c0_c1("?c0", "?c1", "%0")),
        rw!("mod-multiple";"(% (* ?c0 ?x) ?c1)" => "num0" if crate::trs::compare_c0_c1("?c0", "?c1", "%0")),

        // INCONSISTENT
        // rw!("mod-two"       ; "(% (- ?x ?y) num2)"      => "(% (+ ?x ?y) num2)"),
        // rw!("mod-const-add" ; "(% ?x ?c1)"           => "(% (+ ?x ?c1) ?c1)" if crate::trs::compare_c0_c1("?c1","?x","<=a")),
        // rw!("mod-const-sub" ; "(% ?x ?c1)"           => "(% (- ?x ?c1) ?c1)" if crate::trs::compare_c0_c1("?c1","?x","<=a")),
    ]
}
