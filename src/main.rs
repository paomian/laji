#![feature(box_syntax, box_patterns)]

enum Expr {
    Add(Box<Expr>,Box<Expr>),
    Num(i64),
    Multiply(Box<Expr>,Box<Expr>),
}


impl Expr {
    fn to_s(&self) -> String {
        match *self {
            Expr::Num(e) => e.to_string(),
            Expr::Add(ref l,ref r) => format!("({} + {})", l.to_s(), r.to_s()),
            Expr::Multiply(ref l,ref r) => format!("({} * {})", l.to_s(), r.to_s()),
        }
    }
    fn p(&self) { print!("{}", self.to_s()) }
    fn is_reducible(&self) -> bool {
        match *self {
            Expr::Num(_) => false,
            Expr::Add(_,_) => true,
            Expr::Multiply(_,_) => true,
        }
    }
    fn reduce(self) -> Expr {
        match self {
            Expr::Num(e) => Expr::Num(e),
            Expr::Add(l,r) => {
                if l.is_reducible() {
                    Expr::Add(box l.reduce(), r)
                } else if r.is_reducible() {
                    Expr::Add(l, box r.reduce())
                } else {
                    if let box Expr::Num(ll) = l {
                        if let box Expr::Num(rr) = r {
                            Expr::Num(ll*rr)
                        } else {Expr::Num(0)}
                    } else {Expr::Num(0)}
                }
            },
            Expr::Multiply(l,r) => {
                if l.is_reducible() {
                    Expr::Multiply(box l.reduce(), r)
                } else if r.is_reducible() {
                    Expr::Multiply(l, box r.reduce())
                } else {
                    if let box Expr::Num(ll) = l {
                        if let box Expr::Num(rr) = r {
                            Expr::Num(ll*rr)
                        } else {Expr::Num(0)}
                    } else {Expr::Num(0)}
                }
            }
        }
    }
}

fn main() {
    let e = Expr::Add (
        Box::new(Expr::Multiply(Box::new(Expr::Num(1)),Box::new(Expr::Num(2)))),
        Box::new(Expr::Multiply(Box::new(Expr::Num(3)),Box::new(Expr::Num(4)))),
    );
    e.p();
    /*
    let e1 = Num{value:1};
    let e2 = Add{left:Num{value:1},right:Num{value:2},};
    print!("{:?} {:?}",e1.is_reducible(),e2.is_reducible());
     */
}
