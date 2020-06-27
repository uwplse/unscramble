use egg::*;

define_language! {
    enum Prop {
        Bool(bool),
        "&" = And([Id; 2]),
        "~" = Not(Id),
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
rev_rule! {not_true, "(~ true)", "false"}
rev_rule! {not_false, "(~ false)", "true"}

rev_rule! {and_ff, "(& false false)", "false"}
rev_rule! {and_ft, "(& false true)", "false"}
rev_rule! {and_tf, "(& true false)", "false"}
rev_rule! {and_tt, "(& true true)", "true"}

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
