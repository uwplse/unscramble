#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
  Num(i32),
  Add(Box<Expr>, Box<Expr>),
  Mul(Box<Expr>, Box<Expr>),
}

impl Expr {
  fn eval(&self) -> i32 {
    match self {
      Expr::Num(i) => *i,
      Expr::Add(l, r) => Expr::eval(l) + Expr::eval(r),
      Expr::Mul(l, r) => l.eval() * r.eval(),
    }
  }
}

/* impl Clone for Expr {
  fn clone(&self) -> Self {
    match self {
      Expr::Num(i) => Expr::Num(*i),
      Expr::Add(l, r) => Expr::Add(l.clone(), r.clone()),
      Expr::Mul(l, r) => Expr::Mul(l.clone(), r.clone()),
    }
  }
} */

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn take_borrow_twice() {
    let e = Expr::Add(Box::new(Expr::Num(1)), Box::new(Expr::Num(3)));
    assert_eq!(Expr::eval(&e), 4);
    assert_eq!(e.eval(), 4);
  }

  #[test]
  fn clone_it() {
    let two = Box::new(Expr::Num(2));
    let e = Expr::Add(two.clone(), two);
    assert_eq!(e.eval(), 4);
  }
}
