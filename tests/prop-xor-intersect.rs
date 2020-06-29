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

macro_rules! rev_rule {
  ($name:ident, $left:literal, $right:literal) => {
      #[allow(dead_code)]
      fn $name() -> Rewrite {
          rewrite!(stringify!($name); $right => $left)
      }
  };
}

// rule! {def_imply, def_imply_flip,   "(-> ?a ?b)",       "(| (~ ?a) ?b)"          }
// rule! {double_neg, double_neg_flip,  "(~ (~ ?a))",       "?a"                     }
// rule! {assoc_or,    "(| ?a (| ?b ?c))", "(| (| ?a ?b) ?c)"       }
// rule! {dist_and_or, "(& ?a (| ?b ?c))", "(| (& ?a ?b) (& ?a ?c))"}
// rule! {dist_or_and, "(| ?a (& ?b ?c))", "(& (| ?a ?b) (| ?a ?c))"}
// rule! {comm_or,     "(| ?a ?b)",        "(| ?b ?a)"              }
// rule! {comm_and,    "(& ?a ?b)",        "(& ?b ?a)"              }
// rule! {lem,         "(| ?a (~ ?a))",    "true"                      }
// rule! {or_true,     "(| ?a true)",         "true"                      }
// rule! {and_true,    "(& ?a true)",         "?a"                     }
// rule! {contrapositive, "(-> ?a ?b)",    "(-> (~ ?b) (~ ?a))"     }
// rule! {lem_imply, "(& (-> ?a ?b) (-> (~ ?a) ?c))", "(| ?b ?c)"}
rev_rule! {not_true, "(~ true)", "false"}
rev_rule! {not_false, "(~ false)", "true"}

rev_rule! {and_ff, "(& false false)", "false"}
rev_rule! {and_ft, "(& false true)", "false"}
rev_rule! {and_tf, "(& true false)", "false"}
rev_rule! {and_tt, "(& true true)", "true"}

rev_rule! {or_ff, "(| false false)", "false"}
rev_rule! {or_ft, "(| false true)", "true"}
rev_rule! {or_tf, "(| true false)", "true"}
rev_rule! {or_tt, "(| true true)", "true"}

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

fn get_egraph(start: &str, rewrites: &[Rewrite]) -> EGraph {
  let start_expr: RecExpr<_> = start.parse().unwrap();

  Runner::default()
    .with_iter_limit(20)
    .with_node_limit(5_000)
    .with_expr(&start_expr)
    .run(rewrites)
    .egraph
}

fn intersect_and_dump(name: &str, egg1: &EGraph, egg2: &EGraph) -> EGraph {
  let intersection = intersect(&egg1, &egg2);
  intersection
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
  let egg1 = get_egraph("false", rules1);
  let egg2 = get_egraph("true", rules2);
  let egg3 = get_egraph("true", rules3);
  let egg4 = get_egraph("false", rules4);

  egg1.dot().to_dot(format!("tests/egg1.dot")).unwrap();
  egg2.dot().to_dot(format!("tests/egg2.dot")).unwrap();

  let basket = vec![egg2, egg3, egg4];

  let mut unscrambled_egg = egg1;
  for (i, egg) in basket.iter().enumerate() {
    println!("iteration {}", i);
    unscrambled_egg = intersect_and_dump(format!("xor-{}", i).as_str(), &unscrambled_egg, egg);

    let mut extractor = Extractor::new(&unscrambled_egg, CostFn);
    for eclass in unscrambled_egg.classes() {
      let (best_cost, best) = extractor.find_best(eclass.id);
      println!("best cost {}", best_cost);
      println!("best {}", best);
    }
    println!();
  }
}
