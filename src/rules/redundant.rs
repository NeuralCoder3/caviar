use crate::trs::ConstantFold;
use crate::trs::Math;
use egg::rewrite as rw;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;
pub fn rules() -> Vec<Rewrite> {
    // check env variable RULE_COUNT if exist, only take the first n rules
    let rules = vec![
        rw!("add-comm"      ; "(+ ?a ?b)"                   => "(+ ?b ?a)"),
        rw!("add-fact-mul"  ; "(+ (* ?a ?b) (* ?a ?c))"     => "(* ?a (+ ?b ?c))"),

        rw!("and-comm"          ;  "(&& ?y ?x)"                         => "(&& ?x ?y)"),
        rw!("and-assoc"         ;  "(&& ?a (&& ?b ?c))"                 => "(&& (&& ?a ?b) ?c)"),
        rw!("and-x-x"           ;  "(&& ?x ?x)"                         => "?x"),
        rw!("and-min-to-lt"     ;  "(< ?x (min ?y ?z))"                 => "(&& (< ?x ?y) (< ?x ?z))"),
        rw!("and-min-to-eqlt"   ;  "(<= ?x (min ?y ?z))"                => "(&& (<= ?x ?y) (<= ?x ?z))"),
        rw!("and-max-to-lt"     ;  "(> ?x (max ?y ?z))"                 => "(&& (< ?z ?x) (< ?y ?x))"),
        rw!("and-max-to-eqlt"   ;  "(>= ?x (max ?y ?z))"                => "(&& (<= ?z ?x) (<= ?y ?x))"),

        rw!("and-over-or"   ;  "(&& ?a (|| ?b ?c))"        => "(|| (&& ?a ?b) (&& ?a ?c))"),
        rw!("or-x-and-x-y"  ;  "(|| ?x (&& ?x ?y))"        => "?x"),

        rw!("div-zero"      ; "(/ 0 ?x)"            => "(0)"),
        rw!("div-cancel"    ; "(/ ?a ?a)"           => "1" if crate::trs::is_not_zero("?a")),
        rw!("div-minus-up"  ; "(/ ?a (* -1 ?b))"    => "(/ (* -1 ?a) ?b)"),
        rw!("div-minus-out" ; "(/ (* -1 ?a) ?b)"    => "(* -1 (/ ?a ?b))"),

        rw!("eq-comm"       ; "(== ?x ?y)"           => "(== ?y ?x)"),
        rw!("eq-swap"       ; "(== (+ ?x ?y) ?z)"    => "(== ?x (- ?z ?y))"),
        rw!("eq-max-lt"     ; "( == (max ?x ?y) ?y)" => "(<= ?x ?y)"),
        rw!("Eq-min-lt"     ; "( == (min ?x ?y) ?y)" => "(<= ?y ?x)"),

        rw!("lt-swap"      ;  "(< ?x ?y)"              => "(< (* -1 ?y) (* -1 ?x))"),
        rw!("lt-to-zero"    ;  "(< ?a ?a)"              => "0"),
        rw!("lt-swap-in"    ;  "(< (+ ?x ?y) ?z)"       => "(< ?x (- ?z ?y))" ),
        rw!("lt-swap-out"   ;  "(< ?z (+ ?x ?y))"       => "(< (- ?z ?y) ?x)" ),
        rw!("lt-x-x-sub-a"  ;  "(< (- ?a ?y) ?a )"      => "1" if crate::trs::is_const_pos("?y")),
        rw!("lt-const-pos"  ;  "(< 0 ?y )"              => "1" if crate::trs::is_const_pos("?y")),
        rw!("min-lt-cancel" ;  "( < ( min ?x ?y ) ?x )" => "( < ?y ?x )"),
        rw!("lt-min-mutual-term"    ; "( < ( min ?z ?y ) ( min ?x ?y ) )"           => "( < ?z ( min ?x ?y ) )"),
        rw!("lt-max-mutual-term"    ; "( < ( max ?z ?y ) ( max ?x ?y ) )"           => "( < ( max ?z ?y ) ?x )"),
        rw!("lt-min-max-cancel"     ; "(< (max ?a ?c) (min ?a ?b))"                 => "0"),

        rw!("min-max"       ; "(min (max ?x ?y) ?x)"                => "?x"),
        rw!("min-max-max-x" ; "(min (max ?x ?y) (max ?x ?z))"       => "(max (min ?y ?z) ?x)"),
        rw!("min-max-min-y" ; "(min (max (min ?x ?y) ?z) ?y)"       => "(min (max ?x ?z) ?y)"),
        rw!("min-add-both"  ; "(+ (min ?x ?y) ?z)"                  => "(min (+ ?x ?z) (+ ?y ?z))"),
        rw!("min-consts-or"          ; "( < ( min ?y ?c0 ) ?c1 )" => "( || ( < ?y ?c1 ) ( < ?c0 ?c1 ) )"),
        rw!("max-consts-and"         ; "( < ( max ?y ?c0 ) ?c1 )" => "( && ( < ?y ?c1 ) ( < ?c0 ?c1 ) )"),
        rw!("max-consts-or"          ; "( < ?c1 ( max ?y ?c0 ) )" => "( || ( < ?c1 ?y ) ( < ?c1 ?c0 ) )"),

        rw!("mod-zero"      ; "(% 0 ?x)"             => "0"),
        rw!("mod-minus-out" ; "(% (* ?x -1) ?c)"     => "(* -1 (% ?x ?c))"),

        rw!("mul-comm"      ; "(* ?a ?b)"                   => "(* ?b ?a)"),
        rw!("mul-zero"      ; "(* ?a 0)"                    => "0"),

        rw!("not-gt-to-eqlt";  "(! (< ?y ?x))"  => "(<= ?x ?y)" ),
        rw!("not-eq-to-ineq";  "(! (== ?x ?y))" => "(!= ?x ?y)" ),

    ];
    if let Ok(rule_count_str) = std::env::var("RULE_COUNT") {
        if let Ok(rule_count) = rule_count_str.parse::<usize>() {
            println!("Using {} redundant rules", rule_count);
            return rules.into_iter().take(rule_count).collect();
        }
    }
    rules
}