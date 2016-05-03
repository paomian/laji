#![feature(box_syntax, box_patterns)]

mod Expr;
use Expr::{Expr as expr,Machine};
fn main() {
    let mut e = Machine {ex:expr::Add (
        Box::new(expr::Multiply(Box::new(expr::Num(1)),Box::new(expr::Num(2)))),
        Box::new(expr::Multiply(Box::new(expr::Num(3)),Box::new(expr::Num(4)))),
    )};
    e.run();
    //e.step();
    //e.ex.p();
}
