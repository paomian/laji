#[derive(Clone)]
pub enum Expr {
    Add(Box<Expr>,Box<Expr>),
    Num(i64),
    Multiply(Box<Expr>,Box<Expr>),
    Boolean(bool),
    LessThan(Box<Expr>,Box<Expr>),
    //Variable(String)
}

#[derive(Clone)]
pub struct Machine {
    pub ex:Expr,
}

impl Machine {
    fn step(&mut self) {
        self.ex = self.ex.clone().reduce();
    }

    pub fn run(&mut self) {
        while self.ex.is_reducible() {
            self.ex.p();
            self.step();
        }
        self.ex.p();
    }
}

impl Expr {
    fn to_s(&self) -> String {
        match *self {
            Expr::Num(e) => e.to_string(),
            Expr::Add(ref l,ref r) => format!("({} + {})", l.to_s(), r.to_s()),
            Expr::Multiply(ref l,ref r) => format!("({} * {})", l.to_s(), r.to_s()),
            Expr::LessThan(ref l,ref r) => format!("({} < {})", l.to_s(), r.to_s()),
            Expr::Boolean(e) => (if e {String::from("true")} else {String::from("false")}),
            //Expr::Variable
        }
    }
    fn p(&self) { println!("{}", self.to_s()) }
    fn is_reducible(&self) -> bool {
        match *self {
            Expr::Num(_) => false,
            Expr::Add(_,_) => true,
            Expr::Multiply(_,_) => true,
            Expr::LessThan(_,_) => true,
            Expr::Boolean(_) => false,
        }
    }
    fn reduce(self) -> Expr {
        match self {
            Expr::Num(e) => Expr::Num(e),
            Expr::Boolean(e) => Expr::Boolean(e),
            Expr::Add(l,r) => {
                if l.is_reducible() {
                    Expr::Add(box l.reduce(), r)
                } else if r.is_reducible() {
                    Expr::Add(l, box r.reduce())
                }else {
                    if let box Expr::Num(ll) = l {
                        if let box Expr::Num(rr) = r {
                            Expr::Num(ll+rr)
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
            Expr::LessThan(l,r) => {
                if l.is_reducible() {
                    Expr::LessThan(box l.reduce(),r)
                } else if r.is_reducible() {
                    Expr::LessThan(l, box r.reduce())
                } else {
                    if let box Expr::Num(ll) = l {
                        if let box Expr::Num(rr) = r {
                            Expr::Boolean(ll < rr)
                        } else {Expr::Boolean(false)}
                    } else {Expr::Boolean(false)}
                }

            }
        }
    }
}
