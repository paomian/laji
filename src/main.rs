#![feature(box_syntax, box_patterns)]

#[macro_use]
extern crate nom;

use std::collections::BTreeMap;

mod Expr;
use Expr::{Expr as expr,Machine};

fn main() {
    let mut e = Machine {ex:expr::Add (
        Box::new(expr::Multiply(Box::new(expr::Num(1)),Box::new(expr::Num(2)))),
        Box::new(expr::Multiply(Box::new(expr::Num(3)),Box::new(expr::Num(4)))),
    )};
    let mut env = BTreeMap::new();
    e.run(env);

    let mut e1 = Machine {ex:expr::Add (
        Box::new(expr::Multiply(Box::new(expr::Num(1)),Box::new(expr::Num(2)))),
        Box::new(expr::Multiply(Box::new(expr::Num(3)),Box::new(expr::Num(4)))),
    )};
}
