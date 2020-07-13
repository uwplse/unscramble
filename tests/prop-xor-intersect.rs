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

struct CostFn;
impl CostFunction<Prop> for CostFn {
  type Cost = f64;

  // you're passed in an enode whose children are costs instead of eclass ids
  fn cost<C>(&mut self, enode: &Prop, mut costs: C) -> Self::Cost
  where
    C: FnMut(Id) -> Self::Cost,
  {
    let op_cost = match enode {
      Prop::X => 0.001,
      Prop::Y => 0.001,
      _ => 1.0,
    };
    enode.fold(op_cost, |sum, id| sum + costs(id))
  }
}

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

rule! {input1x, "false", "x"}
rule! {input1y, "false", "y"}

rule! {input2x, "false", "x"}
rule! {input2y, "true", "y"}

rule! {input3x, "true", "x"}
rule! {input3y, "false", "y"}

rule! {input4x, "true", "x"}
rule! {input4y, "true", "y"}

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
fn prove_xor() {
  let _ = env_logger::builder().is_test(true).try_init();
  let rules1 = &[
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
    input1x(),
    input1y(),
  ];
  let rules2 = &[
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
    input2x(),
    input2y(),
  ];
  let rules3 = &[
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
    input3x(),
    input3y(),
  ];
  let rules4 = &[
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
    input4x(),
    input4y(),
  ];
  let inputs = &[
    (input1x(), input1y()),
    (input2x(), input2y()),
    (input3x(), input3y()),
    (input4x(), input4y()),
  ];
  let outputs = &["false", "true", "true", "false"];

  let egg1 = get_rooted_egraph("false", rules1);
  let egg2 = get_rooted_egraph("true", rules2);
  let egg3 = get_rooted_egraph("true", rules3);
  let egg4 = get_rooted_egraph("false", rules4);

  egg1.0.dot().to_dot(format!("tests/egg1.dot")).unwrap();
  egg2.0.dot().to_dot(format!("tests/egg2.dot")).unwrap();

  let basket = vec![&egg1, &egg2, &egg3, &egg4];

  let mut unscrambled_egg = egg1.clone();
  for (i, egg) in basket.iter().enumerate() {
    println!(
      "iteration {} [({}, {})] -> {}",
      i,
      inputs[i].0.long_name(),
      inputs[i].1.long_name(),
      outputs[i]
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
  // let eggo1 = intersect(&egg1, &egg2);
  // let eggo2 = intersect(&egg3, &egg4);
  // let leggo = intersect(&eggo1, &eggo2);

  // let mut extractor = Extractor::new(&leggo.0, AstSize);
  // for root_id in &leggo.1 {
  //   let (best_cost, best) = extractor.find_best(*root_id);
  //   println!("best cost {}", best_cost);
  //   println!("best {}", best);
  // }
}
