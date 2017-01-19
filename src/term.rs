use std::fmt::{Debug, Formatter, Error};

type Name = String;

enum Term {
    Var(Name),
    App(Box<Term>, Box<Term>),
    Abs(Name, Box<Term>, Box<Term>),
    Pi(Name, Box<Term>, Box<Term>),
    Let(Name, Box<Term>, Box<Term>),   
}

impl Debug for Term {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    use self::Term::*;
    match *self {
      Var(ref var) => write!(fmt, "{:?}", var),
      App(ref f, ref arg) => write!(fmt, "({:?}) ({:?})", f, arg),
      Abs(ref var, ref ty, ref body) => {
        write!(fmt, "\\({:?} : {:?}) -> {:?}", var, ty, body)
      }
      Pi(ref var, ref ty, ref body) => {
        write!(fmt, "\\({:?} : {:?}) -> {:?}", var, ty, body)
      }
      Let(ref var, ref t1, ref t2) => {
        write!(fmt, "let {:?} = {:?} in {:?}", var, t1, t2)
      }
    }
  }
}