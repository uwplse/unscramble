use egg::*;

// use std::fmt::Display;
// use std::str::FromStr;

// enum Dir {
//   Left,
//   Right,
//   Up,
//   Down,
// }

// impl Display for Dir {
//   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
//     match *self {
//       Dir::Left => f.write_str("Left"),
//       Dir::Right => f.write_str("Right"),
//       Dir::Up => f.write_str("Up"),
//       Dir::Down => f.write_str("Down"),
//     }
//   }
// }

// impl FromStr for Dir {
//   type Err = ();
//   fn from_str(s: &str) -> Result<Self, Self::Err> {
//     match s {
//       "Left" => Ok(Dir::Left),
//       "Right" => Ok(Dir::Right),
//       "Up" => Ok(Dir::Up),
//       "Down" => Ok(Dir::Down),
//       _ => Err(()),
//     }
//   }
// }

define_language! {
    enum Prop {
        // Bool(bool),
        /* TODO: use enum */
        Dir(Symbol),
        /* TODO: second arg should only be a Dir. how to specify in egg? */
        "GetCell" = GetCell([Id; 2]),
        "cell" = Cell([Id; 2]),
        Num(i32),
        "list" = List([Id; 2]),
        // "&" = And([Id; 2]),
        // "~" = Not(Id),
        // "|" = Or([Id; 2]),
        // "->" = Implies([Id; 2]),
        // Symbol(Symbol),
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
// rev_rule! {not_true, "(~ true)", "false"}
// rev_rule! {not_false, "(~ false)", "true"}

// rev_rule! {and_ff, "(& false false)", "false"}
// rev_rule! {and_ft, "(& false true)", "false"}
// rev_rule! {and_tf, "(& true false)", "false"}
// rev_rule! {and_tt, "(& true true)", "true"}

/* (?x, 0) -> GetCell_{up, true}((?x, 1))
(?x, 0) -> GetCell_{up, true}((?x, 0)) */
rev_rule! {up, "(GetCell u (cell ?x 1))", "(cell ?x 0)"}
rev_rule! {up_edge, "(GetCell u (cell ?x 0))", "(cell ?x 0)"}

rev_rule! {down, "(GetCell d (cell ?x 0))", "(cell ?x 1)"}
rev_rule! {down_edge, "(GetCell d (cell ?x 1))", "(cell ?x 1)"}

rev_rule! {left, "(GetCell l (cell 1 ?y))", "(cell 0 ?y)"}
rev_rule! {left_edge, "(GetCell l (cell 0 ?y))", "(cell 0 ?y)"}

rev_rule! {right, "(GetCell r (cell 0 ?y))", "(cell 1 ?y)"}
rev_rule! {right_edge, "(GetCell r (cell 1 ?y))", "(cell 1 ?y)"}

rule! {input, "(cell 1 1)", "x"}

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

  egraph.dot().to_dot("target/getcell.dot").unwrap();

  for (i, (goal_expr, goal_str)) in goal_exprs.iter().zip(goals).enumerate() {
    println!("Trying to prove goal {}: {}", i, goal_str);
    let equivs = egraph.equivs(&start_expr, &goal_expr);
    if equivs.is_empty() {
      panic!("Couldn't prove goal {}: {}", i, goal_str);
    }
  }
}

#[test]
fn prove_simple_cell() {
  let _ = env_logger::builder().is_test(true).try_init();
  let rules = &[
    up(),
    up_edge(),
    down(),
    down_edge(),
    left(),
    left_edge(),
    right(),
    right_edge(),
  ];
  prove_something(
    "simple",
    "(cell 0 0)",
    rules,
    &[
      "(cell 0 0)",
      "(GetCell u (cell 0 0))",
      "(GetCell u (GetCell d (cell 0 0)))",
      "(GetCell l (cell 1 0))",
    ],
  );
}

#[test]
fn synthesize() {
  let _ = env_logger::builder().is_test(true).try_init();
  let rules = &[
    up(),
    up_edge(),
    down(),
    down_edge(),
    left(),
    left_edge(),
    right(),
    right_edge(),
    input(),
  ];
  prove_something(
    "synthesize",
    "(list (cell 0 1) (cell 1 0))",
    rules,
    &["(list (GetCell l x) (GetCell u x))"],
  );
}

// #[test]
// fn prove_contrapositive() {
//   let _ = env_logger::builder().is_test(true).try_init();
//   let rules = &[def_imply(), def_imply_flip(), double_neg_flip(), comm_or()];
//   prove_something(
//     "contrapositive",
//     "(-> x y)",
//     rules,
//     &[
//       "(-> x y)",
//       "(| (~ x) y)",
//       "(| (~ x) (~ (~ y)))",
//       "(| (~ (~ y)) (~ x))",
//       "(-> (~ y) (~ x))",
//     ],
//   );
// }

// #[test]
// fn prove_chain() {
//   let _ = env_logger::builder().is_test(true).try_init();
//   let rules = &[
//     // rules needed for contrapositive
//     def_imply(),
//     def_imply_flip(),
//     double_neg_flip(),
//     comm_or(),
//     // and some others
//     comm_and(),
//     lem_imply(),
//   ];
//   prove_something(
//     "chain",
//     "(& (-> x y) (-> y z))",
//     rules,
//     &[
//       "(& (-> x y) (-> y z))",
//       "(& (-> (~ y) (~ x)) (-> y z))",
//       "(& (-> y z)         (-> (~ y) (~ x)))",
//       "(| z (~ x))",
//       "(| (~ x) z)",
//       "(-> x z)",
//     ],
//   );
// }

// #[test]
// fn const_fold() {
//   let start = "(| (& false true) (& true false))";
//   let start_expr = start.parse().unwrap();
//   let end = "false";
//   let end_expr = end.parse().unwrap();
//   let mut eg = EGraph::default();
//   eg.add_expr(&start_expr);
//   eg.rebuild();
//   assert!(!eg.equivs(&start_expr, &end_expr).is_empty());
// }
