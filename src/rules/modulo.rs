use crate::trs::ConstantFold;
use crate::trs::Math;
use egg::rewrite as rw;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn modulo() -> Vec<Rewrite> {
    vec![
        //MOD RULES
        rw!("mod-zero"      ; "(% 0 ?x)"             => "0" if crate::trs::is_not_zero("?x")),
        rw!("mod-x-x"       ; "(% ?x ?x)"            => "0" if crate::trs::is_not_zero("?x")),
        rw!("mod-one"       ; "(% ?x 1)"             => "0"),
        rw!("mod-const-add" ; "(% ?x ?c1)"           => "(% (+ ?x ?c1) ?c1)" if crate::trs::compare_c0_c1("?c1","?x","<=a")),
        rw!("mod-const-sub" ; "(% ?x ?c1)"           => "(% (- ?x ?c1) ?c1)" if crate::trs::compare_c0_c1("?c1","?x","<=a")),
        rw!("mod-minus-out" ; "(% (* ?x -1) ?c)"     => "(* -1 (% ?x ?c))" if crate::trs::is_not_zero("?c")),
        rw!("mod-minus-in"  ; "(* -1 (% ?x ?c))"     => "(% (* ?x -1) ?c)" if crate::trs::is_not_zero("?c")),
        rw!("mod-two"       ; "(% (- ?x ?y) 2)"      => "(% (+ ?x ?y) 2)"),
        //FOLD
        rw!("mod-consts"    ; "( % ( + ( * ?x ?c0 ) ?y ) ?c1 )" => "( % ?y ?c1 )" if crate::trs::compare_c0_c1("?c0", "?c1", "%0")),
        rw!("mod-multiple";"(% (* ?c0 ?x) ?c1)" => "0" if crate::trs::compare_c0_c1("?c0", "?c1", "%0")),
    ]
}
