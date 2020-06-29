use egg::*;

define_language! {
    enum Prop {
        Num(i32),
        "*" = Times([Id; 1]),
        "<<" = LShift([Id; 1]),
        "/" = Div([Id; 1]),
        "x" = X,
        Symbol(Symbol),
        "pr" = Pr([Id; 2]),
        "vec" = PVec([Id; 2]),
        "pi1" = Pi1(Id),
        "pi2" = Pi2(Id),
    }
}

type EGraph = egg::EGraph<Prop, ()>;
type Rewrite = egg::Rewrite<Prop, ()>;

macro_rules! rule {
    ($name:ident, $left:literal, $right:literal) => {
        #[allow(dead_code)]
        fn $name() -> Rewrite {
            rewrite!(stringify!($name); $left => $right)
        }
    };
    ($name:ident, $name2:ident, $left:literal, $right:literal) => {
        rule!($name, $left, $right);
        rule!($name2, $right, $left);
    };
}

macro_rules! rev_rule {
  ($name:ident, $left:literal, $right:literal) => {
      #[allow(dead_code)]
      fn $name() -> Rewrite {
          rewrite!(stringify!($name); $right => $left)
      }
  };
}

rule! {times_to_shift, "(* (vec (pr ?x 2) (pr ?y 2)))",  "(<< (vec (pr ?x 1) (pr ?y 1)))"}
rule! {mul_comm, "(* (vec (pr ?x1 ?y1) (pr ?x2 ?y2)))", "(* (vec (pr ?y1 ?x1) (pr ?y2 ?x2)))"}
rule! {pi1, "(pi1 (vec (pr ?x1 ?y1) (pr ?x2 ?y2)))", "(vec ?x1 ?x2)"}
rule! {pi2, "(pi2 (vec (pr ?x1 ?y1) (pr ?x2 ?y2)))", "(vec ?y1 ?y2)"}
rev_rule! {pi1r, "(pi1 (vec (pr ?x1 ?x2) (pr ?x2 ?x1)))", "(vec ?x1 ?x2)"}
rev_rule! {pi2r, "(pi2 (vec (pr ?y2 ?y1) (pr ?y1 ?y2)))", "(vec ?y1 ?y2)"}
rule! {input, "(vec 2 3)", "x"}

fn prove_something(name: &str, start: &str, rewrites: &[Rewrite], goals: &[&str]) {
  let _ = env_logger::builder().is_test(true).try_init();
  println!("Proving {}", name);

  let start_expr: RecExpr<_> = start.parse().unwrap();
  let goal_exprs: Vec<RecExpr<_>> = goals.iter().map(|g| g.parse().unwrap()).collect();

  let egraph = Runner::default()
    .with_iter_limit(20)
    .with_node_limit(5_000)
    .with_expr(&start_expr)
    .run(rewrites)
    .egraph;

  egraph.dot().to_dot(format!("tests/{}.dot", name)).unwrap();

  for (i, (goal_expr, goal_str)) in goal_exprs.iter().zip(goals).enumerate() {
    println!("Trying to prove goal {}: {}", i, goal_str);
    let equivs = egraph.equivs(&start_expr, &goal_expr);
    if equivs.is_empty() {
      panic!("Couldn't prove goal {}: {}", i, goal_str);
    }
  }
}

#[test]
fn prove_two_and_three() {
  let _ = env_logger::builder().is_test(true).try_init();
  let rules = &[
    times_to_shift(),
    mul_comm(),
    pi1(),
    pi2(),
    pi1r(),
    pi2r(),
    input(),
  ];
  prove_something("two_and_three", "(* (vec (pr a 2) (pr a 3)))", rules, &[]);
}
