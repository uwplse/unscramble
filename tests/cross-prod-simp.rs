use egg::*;
use unscramble::*;

define_language! {
    enum Prop {
        Num(i32),
        "*" = Times([Id; 2]),
        "x" = X,
        Symbol(Symbol),
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

rule! {times_to_shift, "(* ?x 2)",  "(<< ?x 1)"}
rule! {mul_comm, "(* ?x ?y)", "(* ?y ?x)"}
rule! {input2, "2", "x"}
rule! {input3, "3", "x"}

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

  let intersection = intersect(&egraph, &egraph);
  intersection
    .dot()
    .to_dot(format!("tests/{}-intersect.dot", name))
    .unwrap();

  for (i, (goal_expr, goal_str)) in goal_exprs.iter().zip(goals).enumerate() {
    println!("Trying to prove goal {}: {}", i, goal_str);
    let equivs = egraph.equivs(&start_expr, &goal_expr);
    if equivs.is_empty() {
      panic!("Couldn't prove goal {}: {}", i, goal_str);
    }
  }
}

fn get_egraph(start: &str, rewrites: &[Rewrite]) -> EGraph {
  let start_expr: RecExpr<_> = start.parse().unwrap();

  Runner::default()
    .with_iter_limit(20)
    .with_node_limit(5_000)
    .with_expr(&start_expr)
    .run(rewrites)
    .egraph
}

#[test]
fn prove_two_simp() {
  let _ = env_logger::builder().is_test(true).try_init();
  let rules = &[mul_comm(), input2()];
  prove_something("two_simp", "(* a 2)", rules, &[]);
}

#[test]
fn prove_three_simp() {
  let _ = env_logger::builder().is_test(true).try_init();
  let rules = &[mul_comm(), input3()];
  prove_something("three_simp", "(* a 3)", rules, &[]);
}

fn intersect_and_dump(name: &str, egg1: &EGraph, egg2: &EGraph) {
  let intersection = intersect(&egg1, &egg2);
  intersection
    .dot()
    .to_dot(format!("tests/{}-intersect.dot", name))
    .unwrap();
}

#[test]
fn prove_two_three_simp() {
  let _ = env_logger::builder().is_test(true).try_init();
  let rules1 = &[mul_comm(), input2()];
  let rules2 = &[mul_comm(), input3()];
  let egg1 = get_egraph("(* a 2)", rules1);
  let egg2 = get_egraph("(* a 3)", rules2);

  intersect_and_dump("two_three_simp-intersect", &egg1, &egg2);
}
