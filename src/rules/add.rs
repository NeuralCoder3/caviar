use crate::trs::ConstantFold;
use crate::trs::Math;
use egg::rewrite as rw;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn add() -> Vec<Rewrite> {
    vec![
        // ADD RULES
        
        rw!("add-assoc"     ; "(+ ?a (+ ?b ?c))"            => "(+ (+ ?a ?b) ?c)"),
        rw!("add-assoc2"     ; "(+ (+ ?a ?b) ?c)" => "(+ ?a (+ ?b ?c))"),
        rw!("add-dist-mul"  ; "(* ?a (+ ?b ?c))"            => "(+ (* ?a ?b) (* ?a ?c))"),
        rw!("add-dist-mul2"  ; "(+ (* ?a ?b) (* ?a ?c))" => "(* ?a (+ ?b ?c))"),
        //FOLD
        rw!("add-const"     ; "( + (* ?x ?a) (* ?y ?b))"    => "( * (+ (* ?x (/ ?a ?b)) ?y) ?b)" if crate::trs::compare_c0_c1("?a", "?b", "%0")),
        rw!("add-const2"     ; "( * (+ (* ?x (/ ?a ?b)) ?y) ?b)" => "( + (* ?x ?a) (* ?y ?b))" if crate::trs::compare_c0_c1("?a", "?b", "%0")),
        // rw!("sub-const-denom-1"; "( - ( / ( + ?x ?y ) ?a ) ( / ( + ?x ?b ) ?a ) )" => "( / ( + ( % ( + ?x ( % ?b ?a ) ) ?a ) ( - ?y ?b ) ) ?a )" if crate::trs::is_not_zero("?a")),
        // rw!("sub-const-denom-2"; "( - ( / ( + ?x ?c1 ) ?c0 ) ( / ( + ?x ?y ) ?c0 ) )" => "( / ( - ( - (- ( + ?c0 ?c1 ) 1 ) ?y ) ( % ( + ?x ( % ?c1 ?c0 ) ) ?c0 ) ) ?c0 )" if crate::trs::is_const_pos("?c0")),

        // CONSISTENT BUT REDUNDANT
        // rw!("add-comm"      ; "(+ ?a ?b)"                   => "(+ ?b ?a)"),
        // rw!("add-zero"      ; "(+ ?a 0)"                    => "?a"),
        // rw!("add-fact-mul"  ; "(+ (* ?a ?b) (* ?a ?c))"     => "(* ?a (+ ?b ?c))"),

        // INCONSISTENT
        // rw!("add-denom-div" ; "(/ (+ ?a (* ?b ?c)) ?b)"     => "(+ (/ ?a ?b) ?c)" if crate::trs::is_not_zero("?b")),
        // rw!("add-denom-mul" ; "(+ (/ ?a ?b) ?c)"            => "(/ (+ ?a (* ?b ?c)) ?b)" if crate::trs::is_not_zero("?b")),
        // rw!("add-div-mod"   ; "( + ( / ?x 2 ) ( % ?x 2 ) )" => "( / ( + ?x 1 ) 2 )"),
    ]
}
