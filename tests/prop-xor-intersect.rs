use egg::*;
use unscramble::*;

define_language! {
    enum Prop {
        Bool(bool),
        "&" = And([Id; 2]),
        "~" = Not(Id),
        "x" = X,
        "y" = Y,
        "|" = Or([Id; 2]),
        // "->" = Implies([Id; 2]),
        // Symbol(Symbol),
    }
}

type EGraph = egg::EGraph<Prop, ()>;
type RootedEGraph = unscramble::RootedEGraph<Prop, ()>;
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

// "less complex" -> "more complex"

rule! {not_true, "false", "(~ true)"}
rule! {not_false, "true", "(~ false)"}

rule! {and_ff, "false", "(& false false)"}
rule! {and_ft, "false", "(& false true)"}
rule! {and_tf, "false", "(& true false)"}
rule! {and_tt, "true", "(& true true)"}

rule! {or_ff, "false", "(| false false)"}
rule! {or_ft, "true", "(| false true)"}
rule! {or_tf, "true", "(| true false)"}
rule! {or_tt, "true", "(| true true)"}

fn bool_inv_semantics() -> Vec<Rewrite> {
  vec![
    not_true(),
    not_false(),
    and_ff(),
    and_ft(),
    and_tf(),
    and_tt(),
    or_ff(),
    or_ft(),
    or_tf(),
    or_tt(),
  ]
}

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

  egraph.dot().to_dot("target/foo.dot").unwrap();

  for (i, (goal_expr, goal_str)) in goal_exprs.iter().zip(goals).enumerate() {
    println!("Trying to prove goal {}: {}", i, goal_str);
    let equivs = egraph.equivs(&start_expr, &goal_expr);
    if equivs.is_empty() {
      panic!("Couldn't prove goal {}: {}", i, goal_str);
    }
  }
}

#[test]
fn prove_simple() {
  let _ = env_logger::builder().is_test(true).try_init();
  let rules = &[
    not_true(),
    not_false(),
    and_ff(),
    and_ft(),
    and_tf(),
    and_tt(),
  ];
  prove_something("simple", "true", rules, &["(& true (~ false))"]);
}

fn get_rooted_egraph(start: &str, rewrites: &[Rewrite]) -> RootedEGraph {
  let start_expr: RecExpr<_> = start.parse().unwrap();

  let runner = Runner::default()
    .with_iter_limit(20)
    .with_node_limit(5_000)
    .with_expr(&start_expr)
    .run(rewrites);

  (runner.egraph, vec![runner.roots[0]])
}

fn intersect_and_dump(name: &str, egg1: &RootedEGraph, egg2: &RootedEGraph) -> RootedEGraph {
  let intersection = intersect(&egg1, &egg2);
  intersection
    .0
    .dot()
    .to_dot(format!("tests/{}-intersect.dot", name))
    .unwrap();
  intersection
}

#[test]
fn synthesize_xor() {
  let _ = env_logger::builder().is_test(true).try_init();
  // (x, y)
  let inputs = &[
    ("false", "false"),
    ("false", "true"),
    ("true", "false"),
    ("true", "true"),
  ];
  let outputs = &["false", "true", "true", "false"];

  let mut eggs = vec![];
  for (i, (x_val, y_val)) in inputs.iter().enumerate() {
    let mut rules = bool_inv_semantics();
    rules.push(rewrite!("x_in"; (x_val.parse::<Pattern<Prop>>().unwrap()) => "x"));
    rules.push(rewrite!("y_in"; (y_val.parse::<Pattern<Prop>>().unwrap()) => "y"));
    eggs.push(get_rooted_egraph(outputs[i], &rules));
  }

  let mut unscrambled_egg = eggs[0].clone();
  for (i, egg) in eggs.iter().enumerate() {
    println!(
      "intersection {} {{x={}, y={}}} -> {}",
      i, inputs[i].0, inputs[i].1, outputs[i]
    );

    if i > 0 {
      unscrambled_egg = intersect_and_dump(format!("xor-{}", i).as_str(), &unscrambled_egg, &egg);
    }

    let mut extractor = Extractor::new(&unscrambled_egg.0, AstSize);
    for root_id in &unscrambled_egg.1 {
      let (best_cost, best) = extractor.find_best(*root_id);
      println!("best cost {}", best_cost);
      println!("best {}", best);
    }
    println!();
  }
}
