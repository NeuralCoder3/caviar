use crate::trs::{ConditionRewrite, ConstantFold, Math};

type Rewrite = ConditionRewrite<Math, ConstantFold>;

macro_rules! register_rules {
    ($($module:ident),*) => {
        $(pub mod $module;)*

        pub fn all() -> Vec<Rewrite> {
            let mut rules = Vec::new();
            $(
                rules.extend($module::$module());
            )*
            rules
        }
    };
}

register_rules!(
    add,
    and,
    andor,
    div,
    eq,
    ineq,
    lt,
    max,
    min,
    modulo,
    mul,
    not,
    or,
    sub
    // custom
);

pub fn arithmetic() -> Vec<Rewrite> {
    vec![
        add::add(),
        div::div(),
        modulo::modulo(),
        mul::mul(),
        sub::sub(),
    ]
    .concat()
}