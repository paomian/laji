use std::collections::BTreeMap;

#[derive(Clone)]
pub enum Expr {
    Add(Box<Expr>,Box<Expr>),
    Num(i64),
    Multiply(Box<Expr>,Box<Expr>),
    Boolean(bool),
    LessThan(Box<Expr>,Box<Expr>),
    Variable(String),
    Assign(String,Box<Expr>),
    DoNothing,
    Statement(Box<Expr>,BTreeMap<String,Expr>),
}

#[derive(Clone)]
pub struct Machine {
    pub ex:Expr,
}

impl Machine {
    fn step(&mut self,env:BTreeMap<String,Expr>) {
        self.ex = self.ex.clone().reduce(env);
    }

    pub fn run(&mut self,env:BTreeMap<String,Expr>) {
        while self.ex.is_reducible() {
            self.ex.p();
            self.step(env);
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
            Expr::Assign(ref s, ref e) => format!("({} < {})", s.clone(), e.to_s()),
            Expr::Variable(ref e) => e.clone(),
            Expr::DoNothing => String::from("DoNothing"),
            Statement(Box<Expr>,BTreeMap<String,Expr>),
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
            Expr::Variable(_) => true,
            Expr::DoNothing => false,
            Expr::Assign(_,_) => true,
        }
    }
    fn reduce(self,env:BTreeMap<String,Expr>) -> Expr {
        match self {
            Expr::Num(e) => Expr::Num(e),
            Expr::Boolean(e) => Expr::Boolean(e),
            Expr::DoNothing => Expr::DoNothing,
            Expr::Add(l,r) => {
                if l.is_reducible() {
                    Expr::Add(box l.reduce(env), r)
                } else if r.is_reducible() {
                    Expr::Add(l, box r.reduce(env))
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
                    Expr::Multiply(box l.reduce(env), r)
                } else if r.is_reducible() {
                    Expr::Multiply(l, box r.reduce(env))
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
                    Expr::LessThan(box l.reduce(env),r)
                } else if r.is_reducible() {
                    Expr::LessThan(l, box r.reduce(env))
                } else {
                    if let box Expr::Num(ll) = l {
                        if let box Expr::Num(rr) = r {
                            Expr::Boolean(ll < rr)
                        } else {Expr::Boolean(false)}
                    } else {Expr::Boolean(false)}
                }

            }
            Expr::Variable(n) => {
                if let Some(s) = env.get(&n) {
                    (*s).clone()
                } else {Expr::Boolean(false)}
            }
            Expr::Assign(n,e) => {
                if e.is_reducible() {
                    Expr::Statement(box Expr::Assign(n,box e.reduce(env)),env)
                } else {
                    env.insert(n,*e);
                    Expr::Statement(box Expr::DoNothing,env)
                }
            }
        }
    }
}
