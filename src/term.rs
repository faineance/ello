
use std::fmt::{Debug, Formatter, Error};

type Name = String;
#[derive(Clone)]
enum Term {
    Var(Name),
    App(Box<Term>, Box<Term>),
    Abs(Name, Box<Term>, Box<Term>),
    Pi(Name, Box<Term>, Box<Term>), // Pi Quantification
    Let(Name, Box<Term>, Box<Term>),
}

impl Debug for Term {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Term::*;
        match *self {
            Var(ref var) => write!(fmt, "{:?}", var),
            App(ref f, ref arg) => write!(fmt, "({:?}) ({:?})", f, arg),
            Abs(ref var, ref ty, ref body) => write!(fmt, "\\({:?} : {:?}) -> {:?}", var, ty, body),
            Pi(ref var, ref ty, ref body) => {
                write!(fmt, "forall({:?} : {:?}) -> {:?}", var, ty, body) // a dependent type
            }
            Let(ref var, ref t1, ref t2) => write!(fmt, "let {:?} = {:?} in {:?}", var, t1, t2),
        }
    }
}



fn substitute(name: Name, with: Term, body: Term) -> Term {
    use self::Term::*;
    match body {
        Var(ref var) => if var == &name { with } else { body.clone() },
        App(f, arg) => {
            App(box substitute(name.clone(), with.clone(), *f),
                box substitute(name, with, *arg))
        }
        Abs(var, ty, body) => {
            if var == name {
                Abs(var, box substitute(name, with, *ty), body)
            } else {
                Abs(var,
                    box substitute(name.clone(), with.clone(), *ty),
                    box substitute(name, with, *body))
            }
        }
        Pi(var, ty, body) => {
            if var == name {
                Pi(var, box substitute(name, with, *ty), body)
            } else {
                Pi(var,
                   box substitute(name.clone(), with.clone(), *ty),
                   box substitute(name, with, *body))
            }
        }
        Let(var, t1, t2) => {
            if var == name {
                Let(var, box substitute(name, with, *t1), t2)
            } else {
                Let(var,
                   box substitute(name.clone(), with.clone(), *t1),
                   box substitute(name, with, *t2))
            }
        }
    }
} 


fn alpha_rename(term: Term) -> Term {
    unimplemented!();
}