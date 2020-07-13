// use egg::*;

// define_language! {
//     enum Prop {
//         Bool(bool),
//         "pr" = Pr([Id; 2]),
//         "vec" = PVec([Id; 4]),
//         "&" = And(Id),
//         "~" = Not(Id),
//         "pi1" = Pi1(Id),
//         "pi2" = Pi2(Id),
//         // "|" = Or([Id; 2]),
//         // "->" = Implies([Id; 2]),
//         "x" = X,
//     }
// }

// type EGraph = egg::EGraph<Prop, ()>;
// type Rewrite = egg::Rewrite<Prop, ()>;

// struct CostFn;
// impl CostFunction<Prop> for CostFn {
//   type Cost = f64;

//   // you're passed in an enode whose children are costs instead of eclass ids
//   fn cost<C>(&mut self, enode: &Prop, mut costs: C) -> Self::Cost
//   where
//     C: FnMut(Id) -> Self::Cost,
//   {
//     let op_cost = match enode {
//       Prop::X => 0.001,
//       _ => 1.0,
//     };
//     enode.fold(op_cost, |sum, id| sum + costs(id))
//   }
// }

// macro_rules! rule {
//     ($name:ident, $left:literal, $right:literal) => {
//         #[allow(dead_code)]
//         fn $name() -> Rewrite {
//             rewrite!(stringify!($name); $left => $right)
//         }
//     };
//     ($name:ident, $name2:ident, $left:literal, $right:literal) => {
//         rule!($name, $left, $right);
//         rule!($name2, $right, $left);
//     };
// }

// macro_rules! rev_rule {
//   ($name:ident, $left:literal, $right:literal) => {
//       #[allow(dead_code)]
//       fn $name() -> Rewrite {
//           rewrite!(stringify!($name); $right => $left)
//       }
//   };
// }

// // rule! {def_imply, def_imply_flip,   "(-> ?a ?b)",       "(| (~ ?a) ?b)"          }
// // rule! {double_neg, double_neg_flip,  "(~ (~ ?a))",       "?a"                     }
// // rule! {assoc_or,    "(| ?a (| ?b ?c))", "(| (| ?a ?b) ?c)"       }
// // rule! {dist_and_or, "(& ?a (| ?b ?c))", "(| (& ?a ?b) (& ?a ?c))"}
// // rule! {dist_or_and, "(| ?a (& ?b ?c))", "(& (| ?a ?b) (| ?a ?c))"}
// // rule! {comm_or,     "(| ?a ?b)",        "(| ?b ?a)"              }
// // rule! {comm_and,    "(& ?a ?b)",        "(& ?b ?a)"              }
// // rule! {lem,         "(| ?a (~ ?a))",    "true"                      }
// // rule! {or_true,     "(| ?a true)",         "true"                      }
// // rule! {and_true,    "(& ?a true)",         "?a"                     }
// // rule! {contrapositive, "(-> ?a ?b)",    "(-> (~ ?b) (~ ?a))"     }
// // rule! {lem_imply, "(& (-> ?a ?b) (-> (~ ?a) ?c))", "(| ?b ?c)"}

// // thanks I hate it

// rev_rule! {not_ffff, "(~ (vec false false false false))", "(vec true true true true)"}
// rev_rule! {not_ffft, "(~ (vec false false false true))", "(vec true true true false)"}
// rev_rule! {not_fftf, "(~ (vec false false true false))", "(vec true true false true)"}
// rev_rule! {not_fftt, "(~ (vec false false true true))", "(vec true true false false)"}
// rev_rule! {not_ftff, "(~ (vec false true false false))", "(vec true false true true)"}
// rev_rule! {not_ftft, "(~ (vec false true false true))", "(vec true false true false)"}
// rev_rule! {not_fttf, "(~ (vec false true true false))", "(vec true false false true)"}
// rev_rule! {not_fttt, "(~ (vec false true true true))", "(vec true false false false)"}
// rev_rule! {not_tfff, "(~ (vec true false false false))", "(vec false true true true)"}
// rev_rule! {not_tfft, "(~ (vec true false false true))", "(vec false true true false)"}
// rev_rule! {not_tftf, "(~ (vec true false true false))", "(vec false true false true)"}
// rev_rule! {not_tftt, "(~ (vec true false true true))", "(vec false true false false)"}
// rev_rule! {not_ttff, "(~ (vec true true false false))", "(vec false false true true)"}
// rev_rule! {not_ttft, "(~ (vec true true false true))", "(vec false false true false)"}
// rev_rule! {not_tttf, "(~ (vec true true true false))", "(vec false false false true)"}
// rev_rule! {not_tttt, "(~ (vec true true true true))", "(vec false false false false)"}

// rev_rule! {and_ffffffff, "(& (vec (pr false false) (pr false false) (pr false false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_ffffffft, "(& (vec (pr false false) (pr false false) (pr false false) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_fffffftf, "(& (vec (pr false false) (pr false false) (pr false false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_fffffftt, "(& (vec (pr false false) (pr false false) (pr false false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_ffffftff, "(& (vec (pr false false) (pr false false) (pr false true) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_ffffftft, "(& (vec (pr false false) (pr false false) (pr false true) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_fffffttf, "(& (vec (pr false false) (pr false false) (pr false true) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_fffffttt, "(& (vec (pr false false) (pr false false) (pr false true) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_fffftfff, "(& (vec (pr false false) (pr false false) (pr true false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_fffftfft, "(& (vec (pr false false) (pr false false) (pr true false) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_fffftftf, "(& (vec (pr false false) (pr false false) (pr true false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_fffftftt, "(& (vec (pr false false) (pr false false) (pr true false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_ffffttff, "(& (vec (pr false false) (pr false false) (pr true true) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {and_ffffttft, "(& (vec (pr false false) (pr false false) (pr true true) (pr false true)))",  "(vec false false true false)"}
// rev_rule! {and_fffftttf, "(& (vec (pr false false) (pr false false) (pr true true) (pr true false)))",  "(vec false false true false)"}
// rev_rule! {and_fffftttt, "(& (vec (pr false false) (pr false false) (pr true true) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {and_ffftffff, "(& (vec (pr false false) (pr false true) (pr false false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_ffftffft, "(& (vec (pr false false) (pr false true) (pr false false) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_ffftfftf, "(& (vec (pr false false) (pr false true) (pr false false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_ffftfftt, "(& (vec (pr false false) (pr false true) (pr false false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_ffftftff, "(& (vec (pr false false) (pr false true) (pr false true) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_ffftftft, "(& (vec (pr false false) (pr false true) (pr false true) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_ffftfttf, "(& (vec (pr false false) (pr false true) (pr false true) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_ffftfttt, "(& (vec (pr false false) (pr false true) (pr false true) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_fffttfff, "(& (vec (pr false false) (pr false true) (pr true false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_fffttfft, "(& (vec (pr false false) (pr false true) (pr true false) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_fffttftf, "(& (vec (pr false false) (pr false true) (pr true false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_fffttftt, "(& (vec (pr false false) (pr false true) (pr true false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_ffftttff, "(& (vec (pr false false) (pr false true) (pr true true) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {and_ffftttft, "(& (vec (pr false false) (pr false true) (pr true true) (pr false true)))",  "(vec false false true false)"}
// rev_rule! {and_fffttttf, "(& (vec (pr false false) (pr false true) (pr true true) (pr true false)))",  "(vec false false true false)"}
// rev_rule! {and_fffttttt, "(& (vec (pr false false) (pr false true) (pr true true) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {and_fftfffff, "(& (vec (pr false false) (pr true false) (pr false false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_fftfffft, "(& (vec (pr false false) (pr true false) (pr false false) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_fftffftf, "(& (vec (pr false false) (pr true false) (pr false false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_fftffftt, "(& (vec (pr false false) (pr true false) (pr false false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_fftfftff, "(& (vec (pr false false) (pr true false) (pr false true) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_fftfftft, "(& (vec (pr false false) (pr true false) (pr false true) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_fftffttf, "(& (vec (pr false false) (pr true false) (pr false true) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_fftffttt, "(& (vec (pr false false) (pr true false) (pr false true) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_fftftfff, "(& (vec (pr false false) (pr true false) (pr true false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_fftftfft, "(& (vec (pr false false) (pr true false) (pr true false) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_fftftftf, "(& (vec (pr false false) (pr true false) (pr true false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_fftftftt, "(& (vec (pr false false) (pr true false) (pr true false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_fftfttff, "(& (vec (pr false false) (pr true false) (pr true true) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {and_fftfttft, "(& (vec (pr false false) (pr true false) (pr true true) (pr false true)))",  "(vec false false true false)"}
// rev_rule! {and_fftftttf, "(& (vec (pr false false) (pr true false) (pr true true) (pr true false)))",  "(vec false false true false)"}
// rev_rule! {and_fftftttt, "(& (vec (pr false false) (pr true false) (pr true true) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {and_ffttffff, "(& (vec (pr false false) (pr true true) (pr false false) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {and_ffttffft, "(& (vec (pr false false) (pr true true) (pr false false) (pr false true)))",  "(vec false true false false)"}
// rev_rule! {and_ffttfftf, "(& (vec (pr false false) (pr true true) (pr false false) (pr true false)))",  "(vec false true false false)"}
// rev_rule! {and_ffttfftt, "(& (vec (pr false false) (pr true true) (pr false false) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {and_ffttftff, "(& (vec (pr false false) (pr true true) (pr false true) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {and_ffttftft, "(& (vec (pr false false) (pr true true) (pr false true) (pr false true)))",  "(vec false true false false)"}
// rev_rule! {and_ffttfttf, "(& (vec (pr false false) (pr true true) (pr false true) (pr true false)))",  "(vec false true false false)"}
// rev_rule! {and_ffttfttt, "(& (vec (pr false false) (pr true true) (pr false true) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {and_fftttfff, "(& (vec (pr false false) (pr true true) (pr true false) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {and_fftttfft, "(& (vec (pr false false) (pr true true) (pr true false) (pr false true)))",  "(vec false true false false)"}
// rev_rule! {and_fftttftf, "(& (vec (pr false false) (pr true true) (pr true false) (pr true false)))",  "(vec false true false false)"}
// rev_rule! {and_fftttftt, "(& (vec (pr false false) (pr true true) (pr true false) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {and_ffttttff, "(& (vec (pr false false) (pr true true) (pr true true) (pr false false)))",  "(vec false true true false)"}
// rev_rule! {and_ffttttft, "(& (vec (pr false false) (pr true true) (pr true true) (pr false true)))",  "(vec false true true false)"}
// rev_rule! {and_fftttttf, "(& (vec (pr false false) (pr true true) (pr true true) (pr true false)))",  "(vec false true true false)"}
// rev_rule! {and_fftttttt, "(& (vec (pr false false) (pr true true) (pr true true) (pr true true)))",  "(vec false true true true)"}
// rev_rule! {and_ftffffff, "(& (vec (pr false true) (pr false false) (pr false false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_ftffffft, "(& (vec (pr false true) (pr false false) (pr false false) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_ftfffftf, "(& (vec (pr false true) (pr false false) (pr false false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_ftfffftt, "(& (vec (pr false true) (pr false false) (pr false false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_ftffftff, "(& (vec (pr false true) (pr false false) (pr false true) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_ftffftft, "(& (vec (pr false true) (pr false false) (pr false true) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_ftfffttf, "(& (vec (pr false true) (pr false false) (pr false true) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_ftfffttt, "(& (vec (pr false true) (pr false false) (pr false true) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_ftfftfff, "(& (vec (pr false true) (pr false false) (pr true false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_ftfftfft, "(& (vec (pr false true) (pr false false) (pr true false) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_ftfftftf, "(& (vec (pr false true) (pr false false) (pr true false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_ftfftftt, "(& (vec (pr false true) (pr false false) (pr true false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_ftffttff, "(& (vec (pr false true) (pr false false) (pr true true) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {and_ftffttft, "(& (vec (pr false true) (pr false false) (pr true true) (pr false true)))",  "(vec false false true false)"}
// rev_rule! {and_ftfftttf, "(& (vec (pr false true) (pr false false) (pr true true) (pr true false)))",  "(vec false false true false)"}
// rev_rule! {and_ftfftttt, "(& (vec (pr false true) (pr false false) (pr true true) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {and_ftftffff, "(& (vec (pr false true) (pr false true) (pr false false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_ftftffft, "(& (vec (pr false true) (pr false true) (pr false false) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_ftftfftf, "(& (vec (pr false true) (pr false true) (pr false false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_ftftfftt, "(& (vec (pr false true) (pr false true) (pr false false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_ftftftff, "(& (vec (pr false true) (pr false true) (pr false true) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_ftftftft, "(& (vec (pr false true) (pr false true) (pr false true) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_ftftfttf, "(& (vec (pr false true) (pr false true) (pr false true) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_ftftfttt, "(& (vec (pr false true) (pr false true) (pr false true) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_ftfttfff, "(& (vec (pr false true) (pr false true) (pr true false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_ftfttfft, "(& (vec (pr false true) (pr false true) (pr true false) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_ftfttftf, "(& (vec (pr false true) (pr false true) (pr true false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_ftfttftt, "(& (vec (pr false true) (pr false true) (pr true false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_ftftttff, "(& (vec (pr false true) (pr false true) (pr true true) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {and_ftftttft, "(& (vec (pr false true) (pr false true) (pr true true) (pr false true)))",  "(vec false false true false)"}
// rev_rule! {and_ftfttttf, "(& (vec (pr false true) (pr false true) (pr true true) (pr true false)))",  "(vec false false true false)"}
// rev_rule! {and_ftfttttt, "(& (vec (pr false true) (pr false true) (pr true true) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {and_fttfffff, "(& (vec (pr false true) (pr true false) (pr false false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_fttfffft, "(& (vec (pr false true) (pr true false) (pr false false) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_fttffftf, "(& (vec (pr false true) (pr true false) (pr false false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_fttffftt, "(& (vec (pr false true) (pr true false) (pr false false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_fttfftff, "(& (vec (pr false true) (pr true false) (pr false true) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_fttfftft, "(& (vec (pr false true) (pr true false) (pr false true) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_fttffttf, "(& (vec (pr false true) (pr true false) (pr false true) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_fttffttt, "(& (vec (pr false true) (pr true false) (pr false true) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_fttftfff, "(& (vec (pr false true) (pr true false) (pr true false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_fttftfft, "(& (vec (pr false true) (pr true false) (pr true false) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_fttftftf, "(& (vec (pr false true) (pr true false) (pr true false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_fttftftt, "(& (vec (pr false true) (pr true false) (pr true false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_fttfttff, "(& (vec (pr false true) (pr true false) (pr true true) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {and_fttfttft, "(& (vec (pr false true) (pr true false) (pr true true) (pr false true)))",  "(vec false false true false)"}
// rev_rule! {and_fttftttf, "(& (vec (pr false true) (pr true false) (pr true true) (pr true false)))",  "(vec false false true false)"}
// rev_rule! {and_fttftttt, "(& (vec (pr false true) (pr true false) (pr true true) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {and_ftttffff, "(& (vec (pr false true) (pr true true) (pr false false) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {and_ftttffft, "(& (vec (pr false true) (pr true true) (pr false false) (pr false true)))",  "(vec false true false false)"}
// rev_rule! {and_ftttfftf, "(& (vec (pr false true) (pr true true) (pr false false) (pr true false)))",  "(vec false true false false)"}
// rev_rule! {and_ftttfftt, "(& (vec (pr false true) (pr true true) (pr false false) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {and_ftttftff, "(& (vec (pr false true) (pr true true) (pr false true) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {and_ftttftft, "(& (vec (pr false true) (pr true true) (pr false true) (pr false true)))",  "(vec false true false false)"}
// rev_rule! {and_ftttfttf, "(& (vec (pr false true) (pr true true) (pr false true) (pr true false)))",  "(vec false true false false)"}
// rev_rule! {and_ftttfttt, "(& (vec (pr false true) (pr true true) (pr false true) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {and_fttttfff, "(& (vec (pr false true) (pr true true) (pr true false) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {and_fttttfft, "(& (vec (pr false true) (pr true true) (pr true false) (pr false true)))",  "(vec false true false false)"}
// rev_rule! {and_fttttftf, "(& (vec (pr false true) (pr true true) (pr true false) (pr true false)))",  "(vec false true false false)"}
// rev_rule! {and_fttttftt, "(& (vec (pr false true) (pr true true) (pr true false) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {and_ftttttff, "(& (vec (pr false true) (pr true true) (pr true true) (pr false false)))",  "(vec false true true false)"}
// rev_rule! {and_ftttttft, "(& (vec (pr false true) (pr true true) (pr true true) (pr false true)))",  "(vec false true true false)"}
// rev_rule! {and_fttttttf, "(& (vec (pr false true) (pr true true) (pr true true) (pr true false)))",  "(vec false true true false)"}
// rev_rule! {and_fttttttt, "(& (vec (pr false true) (pr true true) (pr true true) (pr true true)))",  "(vec false true true true)"}
// rev_rule! {and_tfffffff, "(& (vec (pr true false) (pr false false) (pr false false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_tfffffft, "(& (vec (pr true false) (pr false false) (pr false false) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_tffffftf, "(& (vec (pr true false) (pr false false) (pr false false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_tffffftt, "(& (vec (pr true false) (pr false false) (pr false false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_tfffftff, "(& (vec (pr true false) (pr false false) (pr false true) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_tfffftft, "(& (vec (pr true false) (pr false false) (pr false true) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_tffffttf, "(& (vec (pr true false) (pr false false) (pr false true) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_tffffttt, "(& (vec (pr true false) (pr false false) (pr false true) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_tffftfff, "(& (vec (pr true false) (pr false false) (pr true false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_tffftfft, "(& (vec (pr true false) (pr false false) (pr true false) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_tffftftf, "(& (vec (pr true false) (pr false false) (pr true false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_tffftftt, "(& (vec (pr true false) (pr false false) (pr true false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_tfffttff, "(& (vec (pr true false) (pr false false) (pr true true) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {and_tfffttft, "(& (vec (pr true false) (pr false false) (pr true true) (pr false true)))",  "(vec false false true false)"}
// rev_rule! {and_tffftttf, "(& (vec (pr true false) (pr false false) (pr true true) (pr true false)))",  "(vec false false true false)"}
// rev_rule! {and_tffftttt, "(& (vec (pr true false) (pr false false) (pr true true) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {and_tfftffff, "(& (vec (pr true false) (pr false true) (pr false false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_tfftffft, "(& (vec (pr true false) (pr false true) (pr false false) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_tfftfftf, "(& (vec (pr true false) (pr false true) (pr false false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_tfftfftt, "(& (vec (pr true false) (pr false true) (pr false false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_tfftftff, "(& (vec (pr true false) (pr false true) (pr false true) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_tfftftft, "(& (vec (pr true false) (pr false true) (pr false true) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_tfftfttf, "(& (vec (pr true false) (pr false true) (pr false true) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_tfftfttt, "(& (vec (pr true false) (pr false true) (pr false true) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_tffttfff, "(& (vec (pr true false) (pr false true) (pr true false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_tffttfft, "(& (vec (pr true false) (pr false true) (pr true false) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_tffttftf, "(& (vec (pr true false) (pr false true) (pr true false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_tffttftt, "(& (vec (pr true false) (pr false true) (pr true false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_tfftttff, "(& (vec (pr true false) (pr false true) (pr true true) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {and_tfftttft, "(& (vec (pr true false) (pr false true) (pr true true) (pr false true)))",  "(vec false false true false)"}
// rev_rule! {and_tffttttf, "(& (vec (pr true false) (pr false true) (pr true true) (pr true false)))",  "(vec false false true false)"}
// rev_rule! {and_tffttttt, "(& (vec (pr true false) (pr false true) (pr true true) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {and_tftfffff, "(& (vec (pr true false) (pr true false) (pr false false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_tftfffft, "(& (vec (pr true false) (pr true false) (pr false false) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_tftffftf, "(& (vec (pr true false) (pr true false) (pr false false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_tftffftt, "(& (vec (pr true false) (pr true false) (pr false false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_tftfftff, "(& (vec (pr true false) (pr true false) (pr false true) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_tftfftft, "(& (vec (pr true false) (pr true false) (pr false true) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_tftffttf, "(& (vec (pr true false) (pr true false) (pr false true) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_tftffttt, "(& (vec (pr true false) (pr true false) (pr false true) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_tftftfff, "(& (vec (pr true false) (pr true false) (pr true false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {and_tftftfft, "(& (vec (pr true false) (pr true false) (pr true false) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {and_tftftftf, "(& (vec (pr true false) (pr true false) (pr true false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {and_tftftftt, "(& (vec (pr true false) (pr true false) (pr true false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {and_tftfttff, "(& (vec (pr true false) (pr true false) (pr true true) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {and_tftfttft, "(& (vec (pr true false) (pr true false) (pr true true) (pr false true)))",  "(vec false false true false)"}
// rev_rule! {and_tftftttf, "(& (vec (pr true false) (pr true false) (pr true true) (pr true false)))",  "(vec false false true false)"}
// rev_rule! {and_tftftttt, "(& (vec (pr true false) (pr true false) (pr true true) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {and_tfttffff, "(& (vec (pr true false) (pr true true) (pr false false) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {and_tfttffft, "(& (vec (pr true false) (pr true true) (pr false false) (pr false true)))",  "(vec false true false false)"}
// rev_rule! {and_tfttfftf, "(& (vec (pr true false) (pr true true) (pr false false) (pr true false)))",  "(vec false true false false)"}
// rev_rule! {and_tfttfftt, "(& (vec (pr true false) (pr true true) (pr false false) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {and_tfttftff, "(& (vec (pr true false) (pr true true) (pr false true) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {and_tfttftft, "(& (vec (pr true false) (pr true true) (pr false true) (pr false true)))",  "(vec false true false false)"}
// rev_rule! {and_tfttfttf, "(& (vec (pr true false) (pr true true) (pr false true) (pr true false)))",  "(vec false true false false)"}
// rev_rule! {and_tfttfttt, "(& (vec (pr true false) (pr true true) (pr false true) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {and_tftttfff, "(& (vec (pr true false) (pr true true) (pr true false) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {and_tftttfft, "(& (vec (pr true false) (pr true true) (pr true false) (pr false true)))",  "(vec false true false false)"}
// rev_rule! {and_tftttftf, "(& (vec (pr true false) (pr true true) (pr true false) (pr true false)))",  "(vec false true false false)"}
// rev_rule! {and_tftttftt, "(& (vec (pr true false) (pr true true) (pr true false) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {and_tfttttff, "(& (vec (pr true false) (pr true true) (pr true true) (pr false false)))",  "(vec false true true false)"}
// rev_rule! {and_tfttttft, "(& (vec (pr true false) (pr true true) (pr true true) (pr false true)))",  "(vec false true true false)"}
// rev_rule! {and_tftttttf, "(& (vec (pr true false) (pr true true) (pr true true) (pr true false)))",  "(vec false true true false)"}
// rev_rule! {and_tftttttt, "(& (vec (pr true false) (pr true true) (pr true true) (pr true true)))",  "(vec false true true true)"}
// rev_rule! {and_ttffffff, "(& (vec (pr true true) (pr false false) (pr false false) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {and_ttffffft, "(& (vec (pr true true) (pr false false) (pr false false) (pr false true)))",  "(vec true false false false)"}
// rev_rule! {and_ttfffftf, "(& (vec (pr true true) (pr false false) (pr false false) (pr true false)))",  "(vec true false false false)"}
// rev_rule! {and_ttfffftt, "(& (vec (pr true true) (pr false false) (pr false false) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {and_ttffftff, "(& (vec (pr true true) (pr false false) (pr false true) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {and_ttffftft, "(& (vec (pr true true) (pr false false) (pr false true) (pr false true)))",  "(vec true false false false)"}
// rev_rule! {and_ttfffttf, "(& (vec (pr true true) (pr false false) (pr false true) (pr true false)))",  "(vec true false false false)"}
// rev_rule! {and_ttfffttt, "(& (vec (pr true true) (pr false false) (pr false true) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {and_ttfftfff, "(& (vec (pr true true) (pr false false) (pr true false) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {and_ttfftfft, "(& (vec (pr true true) (pr false false) (pr true false) (pr false true)))",  "(vec true false false false)"}
// rev_rule! {and_ttfftftf, "(& (vec (pr true true) (pr false false) (pr true false) (pr true false)))",  "(vec true false false false)"}
// rev_rule! {and_ttfftftt, "(& (vec (pr true true) (pr false false) (pr true false) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {and_ttffttff, "(& (vec (pr true true) (pr false false) (pr true true) (pr false false)))",  "(vec true false true false)"}
// rev_rule! {and_ttffttft, "(& (vec (pr true true) (pr false false) (pr true true) (pr false true)))",  "(vec true false true false)"}
// rev_rule! {and_ttfftttf, "(& (vec (pr true true) (pr false false) (pr true true) (pr true false)))",  "(vec true false true false)"}
// rev_rule! {and_ttfftttt, "(& (vec (pr true true) (pr false false) (pr true true) (pr true true)))",  "(vec true false true true)"}
// rev_rule! {and_ttftffff, "(& (vec (pr true true) (pr false true) (pr false false) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {and_ttftffft, "(& (vec (pr true true) (pr false true) (pr false false) (pr false true)))",  "(vec true false false false)"}
// rev_rule! {and_ttftfftf, "(& (vec (pr true true) (pr false true) (pr false false) (pr true false)))",  "(vec true false false false)"}
// rev_rule! {and_ttftfftt, "(& (vec (pr true true) (pr false true) (pr false false) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {and_ttftftff, "(& (vec (pr true true) (pr false true) (pr false true) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {and_ttftftft, "(& (vec (pr true true) (pr false true) (pr false true) (pr false true)))",  "(vec true false false false)"}
// rev_rule! {and_ttftfttf, "(& (vec (pr true true) (pr false true) (pr false true) (pr true false)))",  "(vec true false false false)"}
// rev_rule! {and_ttftfttt, "(& (vec (pr true true) (pr false true) (pr false true) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {and_ttfttfff, "(& (vec (pr true true) (pr false true) (pr true false) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {and_ttfttfft, "(& (vec (pr true true) (pr false true) (pr true false) (pr false true)))",  "(vec true false false false)"}
// rev_rule! {and_ttfttftf, "(& (vec (pr true true) (pr false true) (pr true false) (pr true false)))",  "(vec true false false false)"}
// rev_rule! {and_ttfttftt, "(& (vec (pr true true) (pr false true) (pr true false) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {and_ttftttff, "(& (vec (pr true true) (pr false true) (pr true true) (pr false false)))",  "(vec true false true false)"}
// rev_rule! {and_ttftttft, "(& (vec (pr true true) (pr false true) (pr true true) (pr false true)))",  "(vec true false true false)"}
// rev_rule! {and_ttfttttf, "(& (vec (pr true true) (pr false true) (pr true true) (pr true false)))",  "(vec true false true false)"}
// rev_rule! {and_ttfttttt, "(& (vec (pr true true) (pr false true) (pr true true) (pr true true)))",  "(vec true false true true)"}
// rev_rule! {and_tttfffff, "(& (vec (pr true true) (pr true false) (pr false false) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {and_tttfffft, "(& (vec (pr true true) (pr true false) (pr false false) (pr false true)))",  "(vec true false false false)"}
// rev_rule! {and_tttffftf, "(& (vec (pr true true) (pr true false) (pr false false) (pr true false)))",  "(vec true false false false)"}
// rev_rule! {and_tttffftt, "(& (vec (pr true true) (pr true false) (pr false false) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {and_tttfftff, "(& (vec (pr true true) (pr true false) (pr false true) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {and_tttfftft, "(& (vec (pr true true) (pr true false) (pr false true) (pr false true)))",  "(vec true false false false)"}
// rev_rule! {and_tttffttf, "(& (vec (pr true true) (pr true false) (pr false true) (pr true false)))",  "(vec true false false false)"}
// rev_rule! {and_tttffttt, "(& (vec (pr true true) (pr true false) (pr false true) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {and_tttftfff, "(& (vec (pr true true) (pr true false) (pr true false) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {and_tttftfft, "(& (vec (pr true true) (pr true false) (pr true false) (pr false true)))",  "(vec true false false false)"}
// rev_rule! {and_tttftftf, "(& (vec (pr true true) (pr true false) (pr true false) (pr true false)))",  "(vec true false false false)"}
// rev_rule! {and_tttftftt, "(& (vec (pr true true) (pr true false) (pr true false) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {and_tttfttff, "(& (vec (pr true true) (pr true false) (pr true true) (pr false false)))",  "(vec true false true false)"}
// rev_rule! {and_tttfttft, "(& (vec (pr true true) (pr true false) (pr true true) (pr false true)))",  "(vec true false true false)"}
// rev_rule! {and_tttftttf, "(& (vec (pr true true) (pr true false) (pr true true) (pr true false)))",  "(vec true false true false)"}
// rev_rule! {and_tttftttt, "(& (vec (pr true true) (pr true false) (pr true true) (pr true true)))",  "(vec true false true true)"}
// rev_rule! {and_ttttffff, "(& (vec (pr true true) (pr true true) (pr false false) (pr false false)))",  "(vec true true false false)"}
// rev_rule! {and_ttttffft, "(& (vec (pr true true) (pr true true) (pr false false) (pr false true)))",  "(vec true true false false)"}
// rev_rule! {and_ttttfftf, "(& (vec (pr true true) (pr true true) (pr false false) (pr true false)))",  "(vec true true false false)"}
// rev_rule! {and_ttttfftt, "(& (vec (pr true true) (pr true true) (pr false false) (pr true true)))",  "(vec true true false true)"}
// rev_rule! {and_ttttftff, "(& (vec (pr true true) (pr true true) (pr false true) (pr false false)))",  "(vec true true false false)"}
// rev_rule! {and_ttttftft, "(& (vec (pr true true) (pr true true) (pr false true) (pr false true)))",  "(vec true true false false)"}
// rev_rule! {and_ttttfttf, "(& (vec (pr true true) (pr true true) (pr false true) (pr true false)))",  "(vec true true false false)"}
// rev_rule! {and_ttttfttt, "(& (vec (pr true true) (pr true true) (pr false true) (pr true true)))",  "(vec true true false true)"}
// rev_rule! {and_tttttfff, "(& (vec (pr true true) (pr true true) (pr true false) (pr false false)))",  "(vec true true false false)"}
// rev_rule! {and_tttttfft, "(& (vec (pr true true) (pr true true) (pr true false) (pr false true)))",  "(vec true true false false)"}
// rev_rule! {and_tttttftf, "(& (vec (pr true true) (pr true true) (pr true false) (pr true false)))",  "(vec true true false false)"}
// rev_rule! {and_tttttftt, "(& (vec (pr true true) (pr true true) (pr true false) (pr true true)))",  "(vec true true false true)"}
// rev_rule! {and_ttttttff, "(& (vec (pr true true) (pr true true) (pr true true) (pr false false)))",  "(vec true true true false)"}
// rev_rule! {and_ttttttft, "(& (vec (pr true true) (pr true true) (pr true true) (pr false true)))",  "(vec true true true false)"}
// rev_rule! {and_tttttttf, "(& (vec (pr true true) (pr true true) (pr true true) (pr true false)))",  "(vec true true true false)"}
// rev_rule! {and_tttttttt, "(& (vec (pr true true) (pr true true) (pr true true) (pr true true)))",  "(vec true true true true)"}

// // rev_rule! {pi1, "(pi1 (vec (pr ?x1 ?y1) (pr ?x2 ?y2) (pr ?x3 ?y3) (pr ?x4 ?y4)))", "(vec ?x1 ?x2 ?x3 ?x4)"}
// // rev_rule! {pi2, "(pi2 (vec (pr ?x1 ?y1) (pr ?x2 ?y2) (pr ?x3 ?y3) (pr ?x4 ?y4)))", "(vec ?y1 ?y2 ?y3 ?y4)"}

// rev_rule! {pi1_ffffffff, "(pi1 (vec (pr false false) (pr false false) (pr false false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {pi1_ffffffft, "(pi1 (vec (pr false false) (pr false false) (pr false false) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {pi1_fffffftf, "(pi1 (vec (pr false false) (pr false false) (pr false false) (pr true false)))",  "(vec false false false true)"}
// rev_rule! {pi1_fffffftt, "(pi1 (vec (pr false false) (pr false false) (pr false false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {pi1_ffffftff, "(pi1 (vec (pr false false) (pr false false) (pr false true) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {pi1_ffffftft, "(pi1 (vec (pr false false) (pr false false) (pr false true) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {pi1_fffffttf, "(pi1 (vec (pr false false) (pr false false) (pr false true) (pr true false)))",  "(vec false false false true)"}
// rev_rule! {pi1_fffffttt, "(pi1 (vec (pr false false) (pr false false) (pr false true) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {pi1_fffftfff, "(pi1 (vec (pr false false) (pr false false) (pr true false) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {pi1_fffftfft, "(pi1 (vec (pr false false) (pr false false) (pr true false) (pr false true)))",  "(vec false false true false)"}
// rev_rule! {pi1_fffftftf, "(pi1 (vec (pr false false) (pr false false) (pr true false) (pr true false)))",  "(vec false false true true)"}
// rev_rule! {pi1_fffftftt, "(pi1 (vec (pr false false) (pr false false) (pr true false) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {pi1_ffffttff, "(pi1 (vec (pr false false) (pr false false) (pr true true) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {pi1_ffffttft, "(pi1 (vec (pr false false) (pr false false) (pr true true) (pr false true)))",  "(vec false false true false)"}
// rev_rule! {pi1_fffftttf, "(pi1 (vec (pr false false) (pr false false) (pr true true) (pr true false)))",  "(vec false false true true)"}
// rev_rule! {pi1_fffftttt, "(pi1 (vec (pr false false) (pr false false) (pr true true) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {pi1_ffftffff, "(pi1 (vec (pr false false) (pr false true) (pr false false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {pi1_ffftffft, "(pi1 (vec (pr false false) (pr false true) (pr false false) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {pi1_ffftfftf, "(pi1 (vec (pr false false) (pr false true) (pr false false) (pr true false)))",  "(vec false false false true)"}
// rev_rule! {pi1_ffftfftt, "(pi1 (vec (pr false false) (pr false true) (pr false false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {pi1_ffftftff, "(pi1 (vec (pr false false) (pr false true) (pr false true) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {pi1_ffftftft, "(pi1 (vec (pr false false) (pr false true) (pr false true) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {pi1_ffftfttf, "(pi1 (vec (pr false false) (pr false true) (pr false true) (pr true false)))",  "(vec false false false true)"}
// rev_rule! {pi1_ffftfttt, "(pi1 (vec (pr false false) (pr false true) (pr false true) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {pi1_fffttfff, "(pi1 (vec (pr false false) (pr false true) (pr true false) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {pi1_fffttfft, "(pi1 (vec (pr false false) (pr false true) (pr true false) (pr false true)))",  "(vec false false true false)"}
// rev_rule! {pi1_fffttftf, "(pi1 (vec (pr false false) (pr false true) (pr true false) (pr true false)))",  "(vec false false true true)"}
// rev_rule! {pi1_fffttftt, "(pi1 (vec (pr false false) (pr false true) (pr true false) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {pi1_ffftttff, "(pi1 (vec (pr false false) (pr false true) (pr true true) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {pi1_ffftttft, "(pi1 (vec (pr false false) (pr false true) (pr true true) (pr false true)))",  "(vec false false true false)"}
// rev_rule! {pi1_fffttttf, "(pi1 (vec (pr false false) (pr false true) (pr true true) (pr true false)))",  "(vec false false true true)"}
// rev_rule! {pi1_fffttttt, "(pi1 (vec (pr false false) (pr false true) (pr true true) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {pi1_fftfffff, "(pi1 (vec (pr false false) (pr true false) (pr false false) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {pi1_fftfffft, "(pi1 (vec (pr false false) (pr true false) (pr false false) (pr false true)))",  "(vec false true false false)"}
// rev_rule! {pi1_fftffftf, "(pi1 (vec (pr false false) (pr true false) (pr false false) (pr true false)))",  "(vec false true false true)"}
// rev_rule! {pi1_fftffftt, "(pi1 (vec (pr false false) (pr true false) (pr false false) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {pi1_fftfftff, "(pi1 (vec (pr false false) (pr true false) (pr false true) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {pi1_fftfftft, "(pi1 (vec (pr false false) (pr true false) (pr false true) (pr false true)))",  "(vec false true false false)"}
// rev_rule! {pi1_fftffttf, "(pi1 (vec (pr false false) (pr true false) (pr false true) (pr true false)))",  "(vec false true false true)"}
// rev_rule! {pi1_fftffttt, "(pi1 (vec (pr false false) (pr true false) (pr false true) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {pi1_fftftfff, "(pi1 (vec (pr false false) (pr true false) (pr true false) (pr false false)))",  "(vec false true true false)"}
// rev_rule! {pi1_fftftfft, "(pi1 (vec (pr false false) (pr true false) (pr true false) (pr false true)))",  "(vec false true true false)"}
// rev_rule! {pi1_fftftftf, "(pi1 (vec (pr false false) (pr true false) (pr true false) (pr true false)))",  "(vec false true true true)"}
// rev_rule! {pi1_fftftftt, "(pi1 (vec (pr false false) (pr true false) (pr true false) (pr true true)))",  "(vec false true true true)"}
// rev_rule! {pi1_fftfttff, "(pi1 (vec (pr false false) (pr true false) (pr true true) (pr false false)))",  "(vec false true true false)"}
// rev_rule! {pi1_fftfttft, "(pi1 (vec (pr false false) (pr true false) (pr true true) (pr false true)))",  "(vec false true true false)"}
// rev_rule! {pi1_fftftttf, "(pi1 (vec (pr false false) (pr true false) (pr true true) (pr true false)))",  "(vec false true true true)"}
// rev_rule! {pi1_fftftttt, "(pi1 (vec (pr false false) (pr true false) (pr true true) (pr true true)))",  "(vec false true true true)"}
// rev_rule! {pi1_ffttffff, "(pi1 (vec (pr false false) (pr true true) (pr false false) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {pi1_ffttffft, "(pi1 (vec (pr false false) (pr true true) (pr false false) (pr false true)))",  "(vec false true false false)"}
// rev_rule! {pi1_ffttfftf, "(pi1 (vec (pr false false) (pr true true) (pr false false) (pr true false)))",  "(vec false true false true)"}
// rev_rule! {pi1_ffttfftt, "(pi1 (vec (pr false false) (pr true true) (pr false false) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {pi1_ffttftff, "(pi1 (vec (pr false false) (pr true true) (pr false true) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {pi1_ffttftft, "(pi1 (vec (pr false false) (pr true true) (pr false true) (pr false true)))",  "(vec false true false false)"}
// rev_rule! {pi1_ffttfttf, "(pi1 (vec (pr false false) (pr true true) (pr false true) (pr true false)))",  "(vec false true false true)"}
// rev_rule! {pi1_ffttfttt, "(pi1 (vec (pr false false) (pr true true) (pr false true) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {pi1_fftttfff, "(pi1 (vec (pr false false) (pr true true) (pr true false) (pr false false)))",  "(vec false true true false)"}
// rev_rule! {pi1_fftttfft, "(pi1 (vec (pr false false) (pr true true) (pr true false) (pr false true)))",  "(vec false true true false)"}
// rev_rule! {pi1_fftttftf, "(pi1 (vec (pr false false) (pr true true) (pr true false) (pr true false)))",  "(vec false true true true)"}
// rev_rule! {pi1_fftttftt, "(pi1 (vec (pr false false) (pr true true) (pr true false) (pr true true)))",  "(vec false true true true)"}
// rev_rule! {pi1_ffttttff, "(pi1 (vec (pr false false) (pr true true) (pr true true) (pr false false)))",  "(vec false true true false)"}
// rev_rule! {pi1_ffttttft, "(pi1 (vec (pr false false) (pr true true) (pr true true) (pr false true)))",  "(vec false true true false)"}
// rev_rule! {pi1_fftttttf, "(pi1 (vec (pr false false) (pr true true) (pr true true) (pr true false)))",  "(vec false true true true)"}
// rev_rule! {pi1_fftttttt, "(pi1 (vec (pr false false) (pr true true) (pr true true) (pr true true)))",  "(vec false true true true)"}
// rev_rule! {pi1_ftffffff, "(pi1 (vec (pr false true) (pr false false) (pr false false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {pi1_ftffffft, "(pi1 (vec (pr false true) (pr false false) (pr false false) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {pi1_ftfffftf, "(pi1 (vec (pr false true) (pr false false) (pr false false) (pr true false)))",  "(vec false false false true)"}
// rev_rule! {pi1_ftfffftt, "(pi1 (vec (pr false true) (pr false false) (pr false false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {pi1_ftffftff, "(pi1 (vec (pr false true) (pr false false) (pr false true) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {pi1_ftffftft, "(pi1 (vec (pr false true) (pr false false) (pr false true) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {pi1_ftfffttf, "(pi1 (vec (pr false true) (pr false false) (pr false true) (pr true false)))",  "(vec false false false true)"}
// rev_rule! {pi1_ftfffttt, "(pi1 (vec (pr false true) (pr false false) (pr false true) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {pi1_ftfftfff, "(pi1 (vec (pr false true) (pr false false) (pr true false) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {pi1_ftfftfft, "(pi1 (vec (pr false true) (pr false false) (pr true false) (pr false true)))",  "(vec false false true false)"}
// rev_rule! {pi1_ftfftftf, "(pi1 (vec (pr false true) (pr false false) (pr true false) (pr true false)))",  "(vec false false true true)"}
// rev_rule! {pi1_ftfftftt, "(pi1 (vec (pr false true) (pr false false) (pr true false) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {pi1_ftffttff, "(pi1 (vec (pr false true) (pr false false) (pr true true) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {pi1_ftffttft, "(pi1 (vec (pr false true) (pr false false) (pr true true) (pr false true)))",  "(vec false false true false)"}
// rev_rule! {pi1_ftfftttf, "(pi1 (vec (pr false true) (pr false false) (pr true true) (pr true false)))",  "(vec false false true true)"}
// rev_rule! {pi1_ftfftttt, "(pi1 (vec (pr false true) (pr false false) (pr true true) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {pi1_ftftffff, "(pi1 (vec (pr false true) (pr false true) (pr false false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {pi1_ftftffft, "(pi1 (vec (pr false true) (pr false true) (pr false false) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {pi1_ftftfftf, "(pi1 (vec (pr false true) (pr false true) (pr false false) (pr true false)))",  "(vec false false false true)"}
// rev_rule! {pi1_ftftfftt, "(pi1 (vec (pr false true) (pr false true) (pr false false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {pi1_ftftftff, "(pi1 (vec (pr false true) (pr false true) (pr false true) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {pi1_ftftftft, "(pi1 (vec (pr false true) (pr false true) (pr false true) (pr false true)))",  "(vec false false false false)"}
// rev_rule! {pi1_ftftfttf, "(pi1 (vec (pr false true) (pr false true) (pr false true) (pr true false)))",  "(vec false false false true)"}
// rev_rule! {pi1_ftftfttt, "(pi1 (vec (pr false true) (pr false true) (pr false true) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {pi1_ftfttfff, "(pi1 (vec (pr false true) (pr false true) (pr true false) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {pi1_ftfttfft, "(pi1 (vec (pr false true) (pr false true) (pr true false) (pr false true)))",  "(vec false false true false)"}
// rev_rule! {pi1_ftfttftf, "(pi1 (vec (pr false true) (pr false true) (pr true false) (pr true false)))",  "(vec false false true true)"}
// rev_rule! {pi1_ftfttftt, "(pi1 (vec (pr false true) (pr false true) (pr true false) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {pi1_ftftttff, "(pi1 (vec (pr false true) (pr false true) (pr true true) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {pi1_ftftttft, "(pi1 (vec (pr false true) (pr false true) (pr true true) (pr false true)))",  "(vec false false true false)"}
// rev_rule! {pi1_ftfttttf, "(pi1 (vec (pr false true) (pr false true) (pr true true) (pr true false)))",  "(vec false false true true)"}
// rev_rule! {pi1_ftfttttt, "(pi1 (vec (pr false true) (pr false true) (pr true true) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {pi1_fttfffff, "(pi1 (vec (pr false true) (pr true false) (pr false false) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {pi1_fttfffft, "(pi1 (vec (pr false true) (pr true false) (pr false false) (pr false true)))",  "(vec false true false false)"}
// rev_rule! {pi1_fttffftf, "(pi1 (vec (pr false true) (pr true false) (pr false false) (pr true false)))",  "(vec false true false true)"}
// rev_rule! {pi1_fttffftt, "(pi1 (vec (pr false true) (pr true false) (pr false false) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {pi1_fttfftff, "(pi1 (vec (pr false true) (pr true false) (pr false true) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {pi1_fttfftft, "(pi1 (vec (pr false true) (pr true false) (pr false true) (pr false true)))",  "(vec false true false false)"}
// rev_rule! {pi1_fttffttf, "(pi1 (vec (pr false true) (pr true false) (pr false true) (pr true false)))",  "(vec false true false true)"}
// rev_rule! {pi1_fttffttt, "(pi1 (vec (pr false true) (pr true false) (pr false true) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {pi1_fttftfff, "(pi1 (vec (pr false true) (pr true false) (pr true false) (pr false false)))",  "(vec false true true false)"}
// rev_rule! {pi1_fttftfft, "(pi1 (vec (pr false true) (pr true false) (pr true false) (pr false true)))",  "(vec false true true false)"}
// rev_rule! {pi1_fttftftf, "(pi1 (vec (pr false true) (pr true false) (pr true false) (pr true false)))",  "(vec false true true true)"}
// rev_rule! {pi1_fttftftt, "(pi1 (vec (pr false true) (pr true false) (pr true false) (pr true true)))",  "(vec false true true true)"}
// rev_rule! {pi1_fttfttff, "(pi1 (vec (pr false true) (pr true false) (pr true true) (pr false false)))",  "(vec false true true false)"}
// rev_rule! {pi1_fttfttft, "(pi1 (vec (pr false true) (pr true false) (pr true true) (pr false true)))",  "(vec false true true false)"}
// rev_rule! {pi1_fttftttf, "(pi1 (vec (pr false true) (pr true false) (pr true true) (pr true false)))",  "(vec false true true true)"}
// rev_rule! {pi1_fttftttt, "(pi1 (vec (pr false true) (pr true false) (pr true true) (pr true true)))",  "(vec false true true true)"}
// rev_rule! {pi1_ftttffff, "(pi1 (vec (pr false true) (pr true true) (pr false false) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {pi1_ftttffft, "(pi1 (vec (pr false true) (pr true true) (pr false false) (pr false true)))",  "(vec false true false false)"}
// rev_rule! {pi1_ftttfftf, "(pi1 (vec (pr false true) (pr true true) (pr false false) (pr true false)))",  "(vec false true false true)"}
// rev_rule! {pi1_ftttfftt, "(pi1 (vec (pr false true) (pr true true) (pr false false) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {pi1_ftttftff, "(pi1 (vec (pr false true) (pr true true) (pr false true) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {pi1_ftttftft, "(pi1 (vec (pr false true) (pr true true) (pr false true) (pr false true)))",  "(vec false true false false)"}
// rev_rule! {pi1_ftttfttf, "(pi1 (vec (pr false true) (pr true true) (pr false true) (pr true false)))",  "(vec false true false true)"}
// rev_rule! {pi1_ftttfttt, "(pi1 (vec (pr false true) (pr true true) (pr false true) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {pi1_fttttfff, "(pi1 (vec (pr false true) (pr true true) (pr true false) (pr false false)))",  "(vec false true true false)"}
// rev_rule! {pi1_fttttfft, "(pi1 (vec (pr false true) (pr true true) (pr true false) (pr false true)))",  "(vec false true true false)"}
// rev_rule! {pi1_fttttftf, "(pi1 (vec (pr false true) (pr true true) (pr true false) (pr true false)))",  "(vec false true true true)"}
// rev_rule! {pi1_fttttftt, "(pi1 (vec (pr false true) (pr true true) (pr true false) (pr true true)))",  "(vec false true true true)"}
// rev_rule! {pi1_ftttttff, "(pi1 (vec (pr false true) (pr true true) (pr true true) (pr false false)))",  "(vec false true true false)"}
// rev_rule! {pi1_ftttttft, "(pi1 (vec (pr false true) (pr true true) (pr true true) (pr false true)))",  "(vec false true true false)"}
// rev_rule! {pi1_fttttttf, "(pi1 (vec (pr false true) (pr true true) (pr true true) (pr true false)))",  "(vec false true true true)"}
// rev_rule! {pi1_fttttttt, "(pi1 (vec (pr false true) (pr true true) (pr true true) (pr true true)))",  "(vec false true true true)"}
// rev_rule! {pi1_tfffffff, "(pi1 (vec (pr true false) (pr false false) (pr false false) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {pi1_tfffffft, "(pi1 (vec (pr true false) (pr false false) (pr false false) (pr false true)))",  "(vec true false false false)"}
// rev_rule! {pi1_tffffftf, "(pi1 (vec (pr true false) (pr false false) (pr false false) (pr true false)))",  "(vec true false false true)"}
// rev_rule! {pi1_tffffftt, "(pi1 (vec (pr true false) (pr false false) (pr false false) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {pi1_tfffftff, "(pi1 (vec (pr true false) (pr false false) (pr false true) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {pi1_tfffftft, "(pi1 (vec (pr true false) (pr false false) (pr false true) (pr false true)))",  "(vec true false false false)"}
// rev_rule! {pi1_tffffttf, "(pi1 (vec (pr true false) (pr false false) (pr false true) (pr true false)))",  "(vec true false false true)"}
// rev_rule! {pi1_tffffttt, "(pi1 (vec (pr true false) (pr false false) (pr false true) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {pi1_tffftfff, "(pi1 (vec (pr true false) (pr false false) (pr true false) (pr false false)))",  "(vec true false true false)"}
// rev_rule! {pi1_tffftfft, "(pi1 (vec (pr true false) (pr false false) (pr true false) (pr false true)))",  "(vec true false true false)"}
// rev_rule! {pi1_tffftftf, "(pi1 (vec (pr true false) (pr false false) (pr true false) (pr true false)))",  "(vec true false true true)"}
// rev_rule! {pi1_tffftftt, "(pi1 (vec (pr true false) (pr false false) (pr true false) (pr true true)))",  "(vec true false true true)"}
// rev_rule! {pi1_tfffttff, "(pi1 (vec (pr true false) (pr false false) (pr true true) (pr false false)))",  "(vec true false true false)"}
// rev_rule! {pi1_tfffttft, "(pi1 (vec (pr true false) (pr false false) (pr true true) (pr false true)))",  "(vec true false true false)"}
// rev_rule! {pi1_tffftttf, "(pi1 (vec (pr true false) (pr false false) (pr true true) (pr true false)))",  "(vec true false true true)"}
// rev_rule! {pi1_tffftttt, "(pi1 (vec (pr true false) (pr false false) (pr true true) (pr true true)))",  "(vec true false true true)"}
// rev_rule! {pi1_tfftffff, "(pi1 (vec (pr true false) (pr false true) (pr false false) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {pi1_tfftffft, "(pi1 (vec (pr true false) (pr false true) (pr false false) (pr false true)))",  "(vec true false false false)"}
// rev_rule! {pi1_tfftfftf, "(pi1 (vec (pr true false) (pr false true) (pr false false) (pr true false)))",  "(vec true false false true)"}
// rev_rule! {pi1_tfftfftt, "(pi1 (vec (pr true false) (pr false true) (pr false false) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {pi1_tfftftff, "(pi1 (vec (pr true false) (pr false true) (pr false true) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {pi1_tfftftft, "(pi1 (vec (pr true false) (pr false true) (pr false true) (pr false true)))",  "(vec true false false false)"}
// rev_rule! {pi1_tfftfttf, "(pi1 (vec (pr true false) (pr false true) (pr false true) (pr true false)))",  "(vec true false false true)"}
// rev_rule! {pi1_tfftfttt, "(pi1 (vec (pr true false) (pr false true) (pr false true) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {pi1_tffttfff, "(pi1 (vec (pr true false) (pr false true) (pr true false) (pr false false)))",  "(vec true false true false)"}
// rev_rule! {pi1_tffttfft, "(pi1 (vec (pr true false) (pr false true) (pr true false) (pr false true)))",  "(vec true false true false)"}
// rev_rule! {pi1_tffttftf, "(pi1 (vec (pr true false) (pr false true) (pr true false) (pr true false)))",  "(vec true false true true)"}
// rev_rule! {pi1_tffttftt, "(pi1 (vec (pr true false) (pr false true) (pr true false) (pr true true)))",  "(vec true false true true)"}
// rev_rule! {pi1_tfftttff, "(pi1 (vec (pr true false) (pr false true) (pr true true) (pr false false)))",  "(vec true false true false)"}
// rev_rule! {pi1_tfftttft, "(pi1 (vec (pr true false) (pr false true) (pr true true) (pr false true)))",  "(vec true false true false)"}
// rev_rule! {pi1_tffttttf, "(pi1 (vec (pr true false) (pr false true) (pr true true) (pr true false)))",  "(vec true false true true)"}
// rev_rule! {pi1_tffttttt, "(pi1 (vec (pr true false) (pr false true) (pr true true) (pr true true)))",  "(vec true false true true)"}
// rev_rule! {pi1_tftfffff, "(pi1 (vec (pr true false) (pr true false) (pr false false) (pr false false)))",  "(vec true true false false)"}
// rev_rule! {pi1_tftfffft, "(pi1 (vec (pr true false) (pr true false) (pr false false) (pr false true)))",  "(vec true true false false)"}
// rev_rule! {pi1_tftffftf, "(pi1 (vec (pr true false) (pr true false) (pr false false) (pr true false)))",  "(vec true true false true)"}
// rev_rule! {pi1_tftffftt, "(pi1 (vec (pr true false) (pr true false) (pr false false) (pr true true)))",  "(vec true true false true)"}
// rev_rule! {pi1_tftfftff, "(pi1 (vec (pr true false) (pr true false) (pr false true) (pr false false)))",  "(vec true true false false)"}
// rev_rule! {pi1_tftfftft, "(pi1 (vec (pr true false) (pr true false) (pr false true) (pr false true)))",  "(vec true true false false)"}
// rev_rule! {pi1_tftffttf, "(pi1 (vec (pr true false) (pr true false) (pr false true) (pr true false)))",  "(vec true true false true)"}
// rev_rule! {pi1_tftffttt, "(pi1 (vec (pr true false) (pr true false) (pr false true) (pr true true)))",  "(vec true true false true)"}
// rev_rule! {pi1_tftftfff, "(pi1 (vec (pr true false) (pr true false) (pr true false) (pr false false)))",  "(vec true true true false)"}
// rev_rule! {pi1_tftftfft, "(pi1 (vec (pr true false) (pr true false) (pr true false) (pr false true)))",  "(vec true true true false)"}
// rev_rule! {pi1_tftftftf, "(pi1 (vec (pr true false) (pr true false) (pr true false) (pr true false)))",  "(vec true true true true)"}
// rev_rule! {pi1_tftftftt, "(pi1 (vec (pr true false) (pr true false) (pr true false) (pr true true)))",  "(vec true true true true)"}
// rev_rule! {pi1_tftfttff, "(pi1 (vec (pr true false) (pr true false) (pr true true) (pr false false)))",  "(vec true true true false)"}
// rev_rule! {pi1_tftfttft, "(pi1 (vec (pr true false) (pr true false) (pr true true) (pr false true)))",  "(vec true true true false)"}
// rev_rule! {pi1_tftftttf, "(pi1 (vec (pr true false) (pr true false) (pr true true) (pr true false)))",  "(vec true true true true)"}
// rev_rule! {pi1_tftftttt, "(pi1 (vec (pr true false) (pr true false) (pr true true) (pr true true)))",  "(vec true true true true)"}
// rev_rule! {pi1_tfttffff, "(pi1 (vec (pr true false) (pr true true) (pr false false) (pr false false)))",  "(vec true true false false)"}
// rev_rule! {pi1_tfttffft, "(pi1 (vec (pr true false) (pr true true) (pr false false) (pr false true)))",  "(vec true true false false)"}
// rev_rule! {pi1_tfttfftf, "(pi1 (vec (pr true false) (pr true true) (pr false false) (pr true false)))",  "(vec true true false true)"}
// rev_rule! {pi1_tfttfftt, "(pi1 (vec (pr true false) (pr true true) (pr false false) (pr true true)))",  "(vec true true false true)"}
// rev_rule! {pi1_tfttftff, "(pi1 (vec (pr true false) (pr true true) (pr false true) (pr false false)))",  "(vec true true false false)"}
// rev_rule! {pi1_tfttftft, "(pi1 (vec (pr true false) (pr true true) (pr false true) (pr false true)))",  "(vec true true false false)"}
// rev_rule! {pi1_tfttfttf, "(pi1 (vec (pr true false) (pr true true) (pr false true) (pr true false)))",  "(vec true true false true)"}
// rev_rule! {pi1_tfttfttt, "(pi1 (vec (pr true false) (pr true true) (pr false true) (pr true true)))",  "(vec true true false true)"}
// rev_rule! {pi1_tftttfff, "(pi1 (vec (pr true false) (pr true true) (pr true false) (pr false false)))",  "(vec true true true false)"}
// rev_rule! {pi1_tftttfft, "(pi1 (vec (pr true false) (pr true true) (pr true false) (pr false true)))",  "(vec true true true false)"}
// rev_rule! {pi1_tftttftf, "(pi1 (vec (pr true false) (pr true true) (pr true false) (pr true false)))",  "(vec true true true true)"}
// rev_rule! {pi1_tftttftt, "(pi1 (vec (pr true false) (pr true true) (pr true false) (pr true true)))",  "(vec true true true true)"}
// rev_rule! {pi1_tfttttff, "(pi1 (vec (pr true false) (pr true true) (pr true true) (pr false false)))",  "(vec true true true false)"}
// rev_rule! {pi1_tfttttft, "(pi1 (vec (pr true false) (pr true true) (pr true true) (pr false true)))",  "(vec true true true false)"}
// rev_rule! {pi1_tftttttf, "(pi1 (vec (pr true false) (pr true true) (pr true true) (pr true false)))",  "(vec true true true true)"}
// rev_rule! {pi1_tftttttt, "(pi1 (vec (pr true false) (pr true true) (pr true true) (pr true true)))",  "(vec true true true true)"}
// rev_rule! {pi1_ttffffff, "(pi1 (vec (pr true true) (pr false false) (pr false false) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {pi1_ttffffft, "(pi1 (vec (pr true true) (pr false false) (pr false false) (pr false true)))",  "(vec true false false false)"}
// rev_rule! {pi1_ttfffftf, "(pi1 (vec (pr true true) (pr false false) (pr false false) (pr true false)))",  "(vec true false false true)"}
// rev_rule! {pi1_ttfffftt, "(pi1 (vec (pr true true) (pr false false) (pr false false) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {pi1_ttffftff, "(pi1 (vec (pr true true) (pr false false) (pr false true) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {pi1_ttffftft, "(pi1 (vec (pr true true) (pr false false) (pr false true) (pr false true)))",  "(vec true false false false)"}
// rev_rule! {pi1_ttfffttf, "(pi1 (vec (pr true true) (pr false false) (pr false true) (pr true false)))",  "(vec true false false true)"}
// rev_rule! {pi1_ttfffttt, "(pi1 (vec (pr true true) (pr false false) (pr false true) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {pi1_ttfftfff, "(pi1 (vec (pr true true) (pr false false) (pr true false) (pr false false)))",  "(vec true false true false)"}
// rev_rule! {pi1_ttfftfft, "(pi1 (vec (pr true true) (pr false false) (pr true false) (pr false true)))",  "(vec true false true false)"}
// rev_rule! {pi1_ttfftftf, "(pi1 (vec (pr true true) (pr false false) (pr true false) (pr true false)))",  "(vec true false true true)"}
// rev_rule! {pi1_ttfftftt, "(pi1 (vec (pr true true) (pr false false) (pr true false) (pr true true)))",  "(vec true false true true)"}
// rev_rule! {pi1_ttffttff, "(pi1 (vec (pr true true) (pr false false) (pr true true) (pr false false)))",  "(vec true false true false)"}
// rev_rule! {pi1_ttffttft, "(pi1 (vec (pr true true) (pr false false) (pr true true) (pr false true)))",  "(vec true false true false)"}
// rev_rule! {pi1_ttfftttf, "(pi1 (vec (pr true true) (pr false false) (pr true true) (pr true false)))",  "(vec true false true true)"}
// rev_rule! {pi1_ttfftttt, "(pi1 (vec (pr true true) (pr false false) (pr true true) (pr true true)))",  "(vec true false true true)"}
// rev_rule! {pi1_ttftffff, "(pi1 (vec (pr true true) (pr false true) (pr false false) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {pi1_ttftffft, "(pi1 (vec (pr true true) (pr false true) (pr false false) (pr false true)))",  "(vec true false false false)"}
// rev_rule! {pi1_ttftfftf, "(pi1 (vec (pr true true) (pr false true) (pr false false) (pr true false)))",  "(vec true false false true)"}
// rev_rule! {pi1_ttftfftt, "(pi1 (vec (pr true true) (pr false true) (pr false false) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {pi1_ttftftff, "(pi1 (vec (pr true true) (pr false true) (pr false true) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {pi1_ttftftft, "(pi1 (vec (pr true true) (pr false true) (pr false true) (pr false true)))",  "(vec true false false false)"}
// rev_rule! {pi1_ttftfttf, "(pi1 (vec (pr true true) (pr false true) (pr false true) (pr true false)))",  "(vec true false false true)"}
// rev_rule! {pi1_ttftfttt, "(pi1 (vec (pr true true) (pr false true) (pr false true) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {pi1_ttfttfff, "(pi1 (vec (pr true true) (pr false true) (pr true false) (pr false false)))",  "(vec true false true false)"}
// rev_rule! {pi1_ttfttfft, "(pi1 (vec (pr true true) (pr false true) (pr true false) (pr false true)))",  "(vec true false true false)"}
// rev_rule! {pi1_ttfttftf, "(pi1 (vec (pr true true) (pr false true) (pr true false) (pr true false)))",  "(vec true false true true)"}
// rev_rule! {pi1_ttfttftt, "(pi1 (vec (pr true true) (pr false true) (pr true false) (pr true true)))",  "(vec true false true true)"}
// rev_rule! {pi1_ttftttff, "(pi1 (vec (pr true true) (pr false true) (pr true true) (pr false false)))",  "(vec true false true false)"}
// rev_rule! {pi1_ttftttft, "(pi1 (vec (pr true true) (pr false true) (pr true true) (pr false true)))",  "(vec true false true false)"}
// rev_rule! {pi1_ttfttttf, "(pi1 (vec (pr true true) (pr false true) (pr true true) (pr true false)))",  "(vec true false true true)"}
// rev_rule! {pi1_ttfttttt, "(pi1 (vec (pr true true) (pr false true) (pr true true) (pr true true)))",  "(vec true false true true)"}
// rev_rule! {pi1_tttfffff, "(pi1 (vec (pr true true) (pr true false) (pr false false) (pr false false)))",  "(vec true true false false)"}
// rev_rule! {pi1_tttfffft, "(pi1 (vec (pr true true) (pr true false) (pr false false) (pr false true)))",  "(vec true true false false)"}
// rev_rule! {pi1_tttffftf, "(pi1 (vec (pr true true) (pr true false) (pr false false) (pr true false)))",  "(vec true true false true)"}
// rev_rule! {pi1_tttffftt, "(pi1 (vec (pr true true) (pr true false) (pr false false) (pr true true)))",  "(vec true true false true)"}
// rev_rule! {pi1_tttfftff, "(pi1 (vec (pr true true) (pr true false) (pr false true) (pr false false)))",  "(vec true true false false)"}
// rev_rule! {pi1_tttfftft, "(pi1 (vec (pr true true) (pr true false) (pr false true) (pr false true)))",  "(vec true true false false)"}
// rev_rule! {pi1_tttffttf, "(pi1 (vec (pr true true) (pr true false) (pr false true) (pr true false)))",  "(vec true true false true)"}
// rev_rule! {pi1_tttffttt, "(pi1 (vec (pr true true) (pr true false) (pr false true) (pr true true)))",  "(vec true true false true)"}
// rev_rule! {pi1_tttftfff, "(pi1 (vec (pr true true) (pr true false) (pr true false) (pr false false)))",  "(vec true true true false)"}
// rev_rule! {pi1_tttftfft, "(pi1 (vec (pr true true) (pr true false) (pr true false) (pr false true)))",  "(vec true true true false)"}
// rev_rule! {pi1_tttftftf, "(pi1 (vec (pr true true) (pr true false) (pr true false) (pr true false)))",  "(vec true true true true)"}
// rev_rule! {pi1_tttftftt, "(pi1 (vec (pr true true) (pr true false) (pr true false) (pr true true)))",  "(vec true true true true)"}
// rev_rule! {pi1_tttfttff, "(pi1 (vec (pr true true) (pr true false) (pr true true) (pr false false)))",  "(vec true true true false)"}
// rev_rule! {pi1_tttfttft, "(pi1 (vec (pr true true) (pr true false) (pr true true) (pr false true)))",  "(vec true true true false)"}
// rev_rule! {pi1_tttftttf, "(pi1 (vec (pr true true) (pr true false) (pr true true) (pr true false)))",  "(vec true true true true)"}
// rev_rule! {pi1_tttftttt, "(pi1 (vec (pr true true) (pr true false) (pr true true) (pr true true)))",  "(vec true true true true)"}
// rev_rule! {pi1_ttttffff, "(pi1 (vec (pr true true) (pr true true) (pr false false) (pr false false)))",  "(vec true true false false)"}
// rev_rule! {pi1_ttttffft, "(pi1 (vec (pr true true) (pr true true) (pr false false) (pr false true)))",  "(vec true true false false)"}
// rev_rule! {pi1_ttttfftf, "(pi1 (vec (pr true true) (pr true true) (pr false false) (pr true false)))",  "(vec true true false true)"}
// rev_rule! {pi1_ttttfftt, "(pi1 (vec (pr true true) (pr true true) (pr false false) (pr true true)))",  "(vec true true false true)"}
// rev_rule! {pi1_ttttftff, "(pi1 (vec (pr true true) (pr true true) (pr false true) (pr false false)))",  "(vec true true false false)"}
// rev_rule! {pi1_ttttftft, "(pi1 (vec (pr true true) (pr true true) (pr false true) (pr false true)))",  "(vec true true false false)"}
// rev_rule! {pi1_ttttfttf, "(pi1 (vec (pr true true) (pr true true) (pr false true) (pr true false)))",  "(vec true true false true)"}
// rev_rule! {pi1_ttttfttt, "(pi1 (vec (pr true true) (pr true true) (pr false true) (pr true true)))",  "(vec true true false true)"}
// rev_rule! {pi1_tttttfff, "(pi1 (vec (pr true true) (pr true true) (pr true false) (pr false false)))",  "(vec true true true false)"}
// rev_rule! {pi1_tttttfft, "(pi1 (vec (pr true true) (pr true true) (pr true false) (pr false true)))",  "(vec true true true false)"}
// rev_rule! {pi1_tttttftf, "(pi1 (vec (pr true true) (pr true true) (pr true false) (pr true false)))",  "(vec true true true true)"}
// rev_rule! {pi1_tttttftt, "(pi1 (vec (pr true true) (pr true true) (pr true false) (pr true true)))",  "(vec true true true true)"}
// rev_rule! {pi1_ttttttff, "(pi1 (vec (pr true true) (pr true true) (pr true true) (pr false false)))",  "(vec true true true false)"}
// rev_rule! {pi1_ttttttft, "(pi1 (vec (pr true true) (pr true true) (pr true true) (pr false true)))",  "(vec true true true false)"}
// rev_rule! {pi1_tttttttf, "(pi1 (vec (pr true true) (pr true true) (pr true true) (pr true false)))",  "(vec true true true true)"}
// rev_rule! {pi1_tttttttt, "(pi1 (vec (pr true true) (pr true true) (pr true true) (pr true true)))",  "(vec true true true true)"}

// rev_rule! {pi2_ffffffff, "(pi2 (vec (pr false false) (pr false false) (pr false false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {pi2_ffffffft, "(pi2 (vec (pr false false) (pr false false) (pr false false) (pr false true)))",  "(vec false false false true)"}
// rev_rule! {pi2_fffffftf, "(pi2 (vec (pr false false) (pr false false) (pr false false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {pi2_fffffftt, "(pi2 (vec (pr false false) (pr false false) (pr false false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {pi2_ffffftff, "(pi2 (vec (pr false false) (pr false false) (pr false true) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {pi2_ffffftft, "(pi2 (vec (pr false false) (pr false false) (pr false true) (pr false true)))",  "(vec false false true true)"}
// rev_rule! {pi2_fffffttf, "(pi2 (vec (pr false false) (pr false false) (pr false true) (pr true false)))",  "(vec false false true false)"}
// rev_rule! {pi2_fffffttt, "(pi2 (vec (pr false false) (pr false false) (pr false true) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {pi2_fffftfff, "(pi2 (vec (pr false false) (pr false false) (pr true false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {pi2_fffftfft, "(pi2 (vec (pr false false) (pr false false) (pr true false) (pr false true)))",  "(vec false false false true)"}
// rev_rule! {pi2_fffftftf, "(pi2 (vec (pr false false) (pr false false) (pr true false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {pi2_fffftftt, "(pi2 (vec (pr false false) (pr false false) (pr true false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {pi2_ffffttff, "(pi2 (vec (pr false false) (pr false false) (pr true true) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {pi2_ffffttft, "(pi2 (vec (pr false false) (pr false false) (pr true true) (pr false true)))",  "(vec false false true true)"}
// rev_rule! {pi2_fffftttf, "(pi2 (vec (pr false false) (pr false false) (pr true true) (pr true false)))",  "(vec false false true false)"}
// rev_rule! {pi2_fffftttt, "(pi2 (vec (pr false false) (pr false false) (pr true true) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {pi2_ffftffff, "(pi2 (vec (pr false false) (pr false true) (pr false false) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {pi2_ffftffft, "(pi2 (vec (pr false false) (pr false true) (pr false false) (pr false true)))",  "(vec false true false true)"}
// rev_rule! {pi2_ffftfftf, "(pi2 (vec (pr false false) (pr false true) (pr false false) (pr true false)))",  "(vec false true false false)"}
// rev_rule! {pi2_ffftfftt, "(pi2 (vec (pr false false) (pr false true) (pr false false) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {pi2_ffftftff, "(pi2 (vec (pr false false) (pr false true) (pr false true) (pr false false)))",  "(vec false true true false)"}
// rev_rule! {pi2_ffftftft, "(pi2 (vec (pr false false) (pr false true) (pr false true) (pr false true)))",  "(vec false true true true)"}
// rev_rule! {pi2_ffftfttf, "(pi2 (vec (pr false false) (pr false true) (pr false true) (pr true false)))",  "(vec false true true false)"}
// rev_rule! {pi2_ffftfttt, "(pi2 (vec (pr false false) (pr false true) (pr false true) (pr true true)))",  "(vec false true true true)"}
// rev_rule! {pi2_fffttfff, "(pi2 (vec (pr false false) (pr false true) (pr true false) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {pi2_fffttfft, "(pi2 (vec (pr false false) (pr false true) (pr true false) (pr false true)))",  "(vec false true false true)"}
// rev_rule! {pi2_fffttftf, "(pi2 (vec (pr false false) (pr false true) (pr true false) (pr true false)))",  "(vec false true false false)"}
// rev_rule! {pi2_fffttftt, "(pi2 (vec (pr false false) (pr false true) (pr true false) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {pi2_ffftttff, "(pi2 (vec (pr false false) (pr false true) (pr true true) (pr false false)))",  "(vec false true true false)"}
// rev_rule! {pi2_ffftttft, "(pi2 (vec (pr false false) (pr false true) (pr true true) (pr false true)))",  "(vec false true true true)"}
// rev_rule! {pi2_fffttttf, "(pi2 (vec (pr false false) (pr false true) (pr true true) (pr true false)))",  "(vec false true true false)"}
// rev_rule! {pi2_fffttttt, "(pi2 (vec (pr false false) (pr false true) (pr true true) (pr true true)))",  "(vec false true true true)"}
// rev_rule! {pi2_fftfffff, "(pi2 (vec (pr false false) (pr true false) (pr false false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {pi2_fftfffft, "(pi2 (vec (pr false false) (pr true false) (pr false false) (pr false true)))",  "(vec false false false true)"}
// rev_rule! {pi2_fftffftf, "(pi2 (vec (pr false false) (pr true false) (pr false false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {pi2_fftffftt, "(pi2 (vec (pr false false) (pr true false) (pr false false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {pi2_fftfftff, "(pi2 (vec (pr false false) (pr true false) (pr false true) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {pi2_fftfftft, "(pi2 (vec (pr false false) (pr true false) (pr false true) (pr false true)))",  "(vec false false true true)"}
// rev_rule! {pi2_fftffttf, "(pi2 (vec (pr false false) (pr true false) (pr false true) (pr true false)))",  "(vec false false true false)"}
// rev_rule! {pi2_fftffttt, "(pi2 (vec (pr false false) (pr true false) (pr false true) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {pi2_fftftfff, "(pi2 (vec (pr false false) (pr true false) (pr true false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {pi2_fftftfft, "(pi2 (vec (pr false false) (pr true false) (pr true false) (pr false true)))",  "(vec false false false true)"}
// rev_rule! {pi2_fftftftf, "(pi2 (vec (pr false false) (pr true false) (pr true false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {pi2_fftftftt, "(pi2 (vec (pr false false) (pr true false) (pr true false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {pi2_fftfttff, "(pi2 (vec (pr false false) (pr true false) (pr true true) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {pi2_fftfttft, "(pi2 (vec (pr false false) (pr true false) (pr true true) (pr false true)))",  "(vec false false true true)"}
// rev_rule! {pi2_fftftttf, "(pi2 (vec (pr false false) (pr true false) (pr true true) (pr true false)))",  "(vec false false true false)"}
// rev_rule! {pi2_fftftttt, "(pi2 (vec (pr false false) (pr true false) (pr true true) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {pi2_ffttffff, "(pi2 (vec (pr false false) (pr true true) (pr false false) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {pi2_ffttffft, "(pi2 (vec (pr false false) (pr true true) (pr false false) (pr false true)))",  "(vec false true false true)"}
// rev_rule! {pi2_ffttfftf, "(pi2 (vec (pr false false) (pr true true) (pr false false) (pr true false)))",  "(vec false true false false)"}
// rev_rule! {pi2_ffttfftt, "(pi2 (vec (pr false false) (pr true true) (pr false false) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {pi2_ffttftff, "(pi2 (vec (pr false false) (pr true true) (pr false true) (pr false false)))",  "(vec false true true false)"}
// rev_rule! {pi2_ffttftft, "(pi2 (vec (pr false false) (pr true true) (pr false true) (pr false true)))",  "(vec false true true true)"}
// rev_rule! {pi2_ffttfttf, "(pi2 (vec (pr false false) (pr true true) (pr false true) (pr true false)))",  "(vec false true true false)"}
// rev_rule! {pi2_ffttfttt, "(pi2 (vec (pr false false) (pr true true) (pr false true) (pr true true)))",  "(vec false true true true)"}
// rev_rule! {pi2_fftttfff, "(pi2 (vec (pr false false) (pr true true) (pr true false) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {pi2_fftttfft, "(pi2 (vec (pr false false) (pr true true) (pr true false) (pr false true)))",  "(vec false true false true)"}
// rev_rule! {pi2_fftttftf, "(pi2 (vec (pr false false) (pr true true) (pr true false) (pr true false)))",  "(vec false true false false)"}
// rev_rule! {pi2_fftttftt, "(pi2 (vec (pr false false) (pr true true) (pr true false) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {pi2_ffttttff, "(pi2 (vec (pr false false) (pr true true) (pr true true) (pr false false)))",  "(vec false true true false)"}
// rev_rule! {pi2_ffttttft, "(pi2 (vec (pr false false) (pr true true) (pr true true) (pr false true)))",  "(vec false true true true)"}
// rev_rule! {pi2_fftttttf, "(pi2 (vec (pr false false) (pr true true) (pr true true) (pr true false)))",  "(vec false true true false)"}
// rev_rule! {pi2_fftttttt, "(pi2 (vec (pr false false) (pr true true) (pr true true) (pr true true)))",  "(vec false true true true)"}
// rev_rule! {pi2_ftffffff, "(pi2 (vec (pr false true) (pr false false) (pr false false) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {pi2_ftffffft, "(pi2 (vec (pr false true) (pr false false) (pr false false) (pr false true)))",  "(vec true false false true)"}
// rev_rule! {pi2_ftfffftf, "(pi2 (vec (pr false true) (pr false false) (pr false false) (pr true false)))",  "(vec true false false false)"}
// rev_rule! {pi2_ftfffftt, "(pi2 (vec (pr false true) (pr false false) (pr false false) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {pi2_ftffftff, "(pi2 (vec (pr false true) (pr false false) (pr false true) (pr false false)))",  "(vec true false true false)"}
// rev_rule! {pi2_ftffftft, "(pi2 (vec (pr false true) (pr false false) (pr false true) (pr false true)))",  "(vec true false true true)"}
// rev_rule! {pi2_ftfffttf, "(pi2 (vec (pr false true) (pr false false) (pr false true) (pr true false)))",  "(vec true false true false)"}
// rev_rule! {pi2_ftfffttt, "(pi2 (vec (pr false true) (pr false false) (pr false true) (pr true true)))",  "(vec true false true true)"}
// rev_rule! {pi2_ftfftfff, "(pi2 (vec (pr false true) (pr false false) (pr true false) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {pi2_ftfftfft, "(pi2 (vec (pr false true) (pr false false) (pr true false) (pr false true)))",  "(vec true false false true)"}
// rev_rule! {pi2_ftfftftf, "(pi2 (vec (pr false true) (pr false false) (pr true false) (pr true false)))",  "(vec true false false false)"}
// rev_rule! {pi2_ftfftftt, "(pi2 (vec (pr false true) (pr false false) (pr true false) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {pi2_ftffttff, "(pi2 (vec (pr false true) (pr false false) (pr true true) (pr false false)))",  "(vec true false true false)"}
// rev_rule! {pi2_ftffttft, "(pi2 (vec (pr false true) (pr false false) (pr true true) (pr false true)))",  "(vec true false true true)"}
// rev_rule! {pi2_ftfftttf, "(pi2 (vec (pr false true) (pr false false) (pr true true) (pr true false)))",  "(vec true false true false)"}
// rev_rule! {pi2_ftfftttt, "(pi2 (vec (pr false true) (pr false false) (pr true true) (pr true true)))",  "(vec true false true true)"}
// rev_rule! {pi2_ftftffff, "(pi2 (vec (pr false true) (pr false true) (pr false false) (pr false false)))",  "(vec true true false false)"}
// rev_rule! {pi2_ftftffft, "(pi2 (vec (pr false true) (pr false true) (pr false false) (pr false true)))",  "(vec true true false true)"}
// rev_rule! {pi2_ftftfftf, "(pi2 (vec (pr false true) (pr false true) (pr false false) (pr true false)))",  "(vec true true false false)"}
// rev_rule! {pi2_ftftfftt, "(pi2 (vec (pr false true) (pr false true) (pr false false) (pr true true)))",  "(vec true true false true)"}
// rev_rule! {pi2_ftftftff, "(pi2 (vec (pr false true) (pr false true) (pr false true) (pr false false)))",  "(vec true true true false)"}
// rev_rule! {pi2_ftftftft, "(pi2 (vec (pr false true) (pr false true) (pr false true) (pr false true)))",  "(vec true true true true)"}
// rev_rule! {pi2_ftftfttf, "(pi2 (vec (pr false true) (pr false true) (pr false true) (pr true false)))",  "(vec true true true false)"}
// rev_rule! {pi2_ftftfttt, "(pi2 (vec (pr false true) (pr false true) (pr false true) (pr true true)))",  "(vec true true true true)"}
// rev_rule! {pi2_ftfttfff, "(pi2 (vec (pr false true) (pr false true) (pr true false) (pr false false)))",  "(vec true true false false)"}
// rev_rule! {pi2_ftfttfft, "(pi2 (vec (pr false true) (pr false true) (pr true false) (pr false true)))",  "(vec true true false true)"}
// rev_rule! {pi2_ftfttftf, "(pi2 (vec (pr false true) (pr false true) (pr true false) (pr true false)))",  "(vec true true false false)"}
// rev_rule! {pi2_ftfttftt, "(pi2 (vec (pr false true) (pr false true) (pr true false) (pr true true)))",  "(vec true true false true)"}
// rev_rule! {pi2_ftftttff, "(pi2 (vec (pr false true) (pr false true) (pr true true) (pr false false)))",  "(vec true true true false)"}
// rev_rule! {pi2_ftftttft, "(pi2 (vec (pr false true) (pr false true) (pr true true) (pr false true)))",  "(vec true true true true)"}
// rev_rule! {pi2_ftfttttf, "(pi2 (vec (pr false true) (pr false true) (pr true true) (pr true false)))",  "(vec true true true false)"}
// rev_rule! {pi2_ftfttttt, "(pi2 (vec (pr false true) (pr false true) (pr true true) (pr true true)))",  "(vec true true true true)"}
// rev_rule! {pi2_fttfffff, "(pi2 (vec (pr false true) (pr true false) (pr false false) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {pi2_fttfffft, "(pi2 (vec (pr false true) (pr true false) (pr false false) (pr false true)))",  "(vec true false false true)"}
// rev_rule! {pi2_fttffftf, "(pi2 (vec (pr false true) (pr true false) (pr false false) (pr true false)))",  "(vec true false false false)"}
// rev_rule! {pi2_fttffftt, "(pi2 (vec (pr false true) (pr true false) (pr false false) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {pi2_fttfftff, "(pi2 (vec (pr false true) (pr true false) (pr false true) (pr false false)))",  "(vec true false true false)"}
// rev_rule! {pi2_fttfftft, "(pi2 (vec (pr false true) (pr true false) (pr false true) (pr false true)))",  "(vec true false true true)"}
// rev_rule! {pi2_fttffttf, "(pi2 (vec (pr false true) (pr true false) (pr false true) (pr true false)))",  "(vec true false true false)"}
// rev_rule! {pi2_fttffttt, "(pi2 (vec (pr false true) (pr true false) (pr false true) (pr true true)))",  "(vec true false true true)"}
// rev_rule! {pi2_fttftfff, "(pi2 (vec (pr false true) (pr true false) (pr true false) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {pi2_fttftfft, "(pi2 (vec (pr false true) (pr true false) (pr true false) (pr false true)))",  "(vec true false false true)"}
// rev_rule! {pi2_fttftftf, "(pi2 (vec (pr false true) (pr true false) (pr true false) (pr true false)))",  "(vec true false false false)"}
// rev_rule! {pi2_fttftftt, "(pi2 (vec (pr false true) (pr true false) (pr true false) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {pi2_fttfttff, "(pi2 (vec (pr false true) (pr true false) (pr true true) (pr false false)))",  "(vec true false true false)"}
// rev_rule! {pi2_fttfttft, "(pi2 (vec (pr false true) (pr true false) (pr true true) (pr false true)))",  "(vec true false true true)"}
// rev_rule! {pi2_fttftttf, "(pi2 (vec (pr false true) (pr true false) (pr true true) (pr true false)))",  "(vec true false true false)"}
// rev_rule! {pi2_fttftttt, "(pi2 (vec (pr false true) (pr true false) (pr true true) (pr true true)))",  "(vec true false true true)"}
// rev_rule! {pi2_ftttffff, "(pi2 (vec (pr false true) (pr true true) (pr false false) (pr false false)))",  "(vec true true false false)"}
// rev_rule! {pi2_ftttffft, "(pi2 (vec (pr false true) (pr true true) (pr false false) (pr false true)))",  "(vec true true false true)"}
// rev_rule! {pi2_ftttfftf, "(pi2 (vec (pr false true) (pr true true) (pr false false) (pr true false)))",  "(vec true true false false)"}
// rev_rule! {pi2_ftttfftt, "(pi2 (vec (pr false true) (pr true true) (pr false false) (pr true true)))",  "(vec true true false true)"}
// rev_rule! {pi2_ftttftff, "(pi2 (vec (pr false true) (pr true true) (pr false true) (pr false false)))",  "(vec true true true false)"}
// rev_rule! {pi2_ftttftft, "(pi2 (vec (pr false true) (pr true true) (pr false true) (pr false true)))",  "(vec true true true true)"}
// rev_rule! {pi2_ftttfttf, "(pi2 (vec (pr false true) (pr true true) (pr false true) (pr true false)))",  "(vec true true true false)"}
// rev_rule! {pi2_ftttfttt, "(pi2 (vec (pr false true) (pr true true) (pr false true) (pr true true)))",  "(vec true true true true)"}
// rev_rule! {pi2_fttttfff, "(pi2 (vec (pr false true) (pr true true) (pr true false) (pr false false)))",  "(vec true true false false)"}
// rev_rule! {pi2_fttttfft, "(pi2 (vec (pr false true) (pr true true) (pr true false) (pr false true)))",  "(vec true true false true)"}
// rev_rule! {pi2_fttttftf, "(pi2 (vec (pr false true) (pr true true) (pr true false) (pr true false)))",  "(vec true true false false)"}
// rev_rule! {pi2_fttttftt, "(pi2 (vec (pr false true) (pr true true) (pr true false) (pr true true)))",  "(vec true true false true)"}
// rev_rule! {pi2_ftttttff, "(pi2 (vec (pr false true) (pr true true) (pr true true) (pr false false)))",  "(vec true true true false)"}
// rev_rule! {pi2_ftttttft, "(pi2 (vec (pr false true) (pr true true) (pr true true) (pr false true)))",  "(vec true true true true)"}
// rev_rule! {pi2_fttttttf, "(pi2 (vec (pr false true) (pr true true) (pr true true) (pr true false)))",  "(vec true true true false)"}
// rev_rule! {pi2_fttttttt, "(pi2 (vec (pr false true) (pr true true) (pr true true) (pr true true)))",  "(vec true true true true)"}
// rev_rule! {pi2_tfffffff, "(pi2 (vec (pr true false) (pr false false) (pr false false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {pi2_tfffffft, "(pi2 (vec (pr true false) (pr false false) (pr false false) (pr false true)))",  "(vec false false false true)"}
// rev_rule! {pi2_tffffftf, "(pi2 (vec (pr true false) (pr false false) (pr false false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {pi2_tffffftt, "(pi2 (vec (pr true false) (pr false false) (pr false false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {pi2_tfffftff, "(pi2 (vec (pr true false) (pr false false) (pr false true) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {pi2_tfffftft, "(pi2 (vec (pr true false) (pr false false) (pr false true) (pr false true)))",  "(vec false false true true)"}
// rev_rule! {pi2_tffffttf, "(pi2 (vec (pr true false) (pr false false) (pr false true) (pr true false)))",  "(vec false false true false)"}
// rev_rule! {pi2_tffffttt, "(pi2 (vec (pr true false) (pr false false) (pr false true) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {pi2_tffftfff, "(pi2 (vec (pr true false) (pr false false) (pr true false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {pi2_tffftfft, "(pi2 (vec (pr true false) (pr false false) (pr true false) (pr false true)))",  "(vec false false false true)"}
// rev_rule! {pi2_tffftftf, "(pi2 (vec (pr true false) (pr false false) (pr true false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {pi2_tffftftt, "(pi2 (vec (pr true false) (pr false false) (pr true false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {pi2_tfffttff, "(pi2 (vec (pr true false) (pr false false) (pr true true) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {pi2_tfffttft, "(pi2 (vec (pr true false) (pr false false) (pr true true) (pr false true)))",  "(vec false false true true)"}
// rev_rule! {pi2_tffftttf, "(pi2 (vec (pr true false) (pr false false) (pr true true) (pr true false)))",  "(vec false false true false)"}
// rev_rule! {pi2_tffftttt, "(pi2 (vec (pr true false) (pr false false) (pr true true) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {pi2_tfftffff, "(pi2 (vec (pr true false) (pr false true) (pr false false) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {pi2_tfftffft, "(pi2 (vec (pr true false) (pr false true) (pr false false) (pr false true)))",  "(vec false true false true)"}
// rev_rule! {pi2_tfftfftf, "(pi2 (vec (pr true false) (pr false true) (pr false false) (pr true false)))",  "(vec false true false false)"}
// rev_rule! {pi2_tfftfftt, "(pi2 (vec (pr true false) (pr false true) (pr false false) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {pi2_tfftftff, "(pi2 (vec (pr true false) (pr false true) (pr false true) (pr false false)))",  "(vec false true true false)"}
// rev_rule! {pi2_tfftftft, "(pi2 (vec (pr true false) (pr false true) (pr false true) (pr false true)))",  "(vec false true true true)"}
// rev_rule! {pi2_tfftfttf, "(pi2 (vec (pr true false) (pr false true) (pr false true) (pr true false)))",  "(vec false true true false)"}
// rev_rule! {pi2_tfftfttt, "(pi2 (vec (pr true false) (pr false true) (pr false true) (pr true true)))",  "(vec false true true true)"}
// rev_rule! {pi2_tffttfff, "(pi2 (vec (pr true false) (pr false true) (pr true false) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {pi2_tffttfft, "(pi2 (vec (pr true false) (pr false true) (pr true false) (pr false true)))",  "(vec false true false true)"}
// rev_rule! {pi2_tffttftf, "(pi2 (vec (pr true false) (pr false true) (pr true false) (pr true false)))",  "(vec false true false false)"}
// rev_rule! {pi2_tffttftt, "(pi2 (vec (pr true false) (pr false true) (pr true false) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {pi2_tfftttff, "(pi2 (vec (pr true false) (pr false true) (pr true true) (pr false false)))",  "(vec false true true false)"}
// rev_rule! {pi2_tfftttft, "(pi2 (vec (pr true false) (pr false true) (pr true true) (pr false true)))",  "(vec false true true true)"}
// rev_rule! {pi2_tffttttf, "(pi2 (vec (pr true false) (pr false true) (pr true true) (pr true false)))",  "(vec false true true false)"}
// rev_rule! {pi2_tffttttt, "(pi2 (vec (pr true false) (pr false true) (pr true true) (pr true true)))",  "(vec false true true true)"}
// rev_rule! {pi2_tftfffff, "(pi2 (vec (pr true false) (pr true false) (pr false false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {pi2_tftfffft, "(pi2 (vec (pr true false) (pr true false) (pr false false) (pr false true)))",  "(vec false false false true)"}
// rev_rule! {pi2_tftffftf, "(pi2 (vec (pr true false) (pr true false) (pr false false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {pi2_tftffftt, "(pi2 (vec (pr true false) (pr true false) (pr false false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {pi2_tftfftff, "(pi2 (vec (pr true false) (pr true false) (pr false true) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {pi2_tftfftft, "(pi2 (vec (pr true false) (pr true false) (pr false true) (pr false true)))",  "(vec false false true true)"}
// rev_rule! {pi2_tftffttf, "(pi2 (vec (pr true false) (pr true false) (pr false true) (pr true false)))",  "(vec false false true false)"}
// rev_rule! {pi2_tftffttt, "(pi2 (vec (pr true false) (pr true false) (pr false true) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {pi2_tftftfff, "(pi2 (vec (pr true false) (pr true false) (pr true false) (pr false false)))",  "(vec false false false false)"}
// rev_rule! {pi2_tftftfft, "(pi2 (vec (pr true false) (pr true false) (pr true false) (pr false true)))",  "(vec false false false true)"}
// rev_rule! {pi2_tftftftf, "(pi2 (vec (pr true false) (pr true false) (pr true false) (pr true false)))",  "(vec false false false false)"}
// rev_rule! {pi2_tftftftt, "(pi2 (vec (pr true false) (pr true false) (pr true false) (pr true true)))",  "(vec false false false true)"}
// rev_rule! {pi2_tftfttff, "(pi2 (vec (pr true false) (pr true false) (pr true true) (pr false false)))",  "(vec false false true false)"}
// rev_rule! {pi2_tftfttft, "(pi2 (vec (pr true false) (pr true false) (pr true true) (pr false true)))",  "(vec false false true true)"}
// rev_rule! {pi2_tftftttf, "(pi2 (vec (pr true false) (pr true false) (pr true true) (pr true false)))",  "(vec false false true false)"}
// rev_rule! {pi2_tftftttt, "(pi2 (vec (pr true false) (pr true false) (pr true true) (pr true true)))",  "(vec false false true true)"}
// rev_rule! {pi2_tfttffff, "(pi2 (vec (pr true false) (pr true true) (pr false false) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {pi2_tfttffft, "(pi2 (vec (pr true false) (pr true true) (pr false false) (pr false true)))",  "(vec false true false true)"}
// rev_rule! {pi2_tfttfftf, "(pi2 (vec (pr true false) (pr true true) (pr false false) (pr true false)))",  "(vec false true false false)"}
// rev_rule! {pi2_tfttfftt, "(pi2 (vec (pr true false) (pr true true) (pr false false) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {pi2_tfttftff, "(pi2 (vec (pr true false) (pr true true) (pr false true) (pr false false)))",  "(vec false true true false)"}
// rev_rule! {pi2_tfttftft, "(pi2 (vec (pr true false) (pr true true) (pr false true) (pr false true)))",  "(vec false true true true)"}
// rev_rule! {pi2_tfttfttf, "(pi2 (vec (pr true false) (pr true true) (pr false true) (pr true false)))",  "(vec false true true false)"}
// rev_rule! {pi2_tfttfttt, "(pi2 (vec (pr true false) (pr true true) (pr false true) (pr true true)))",  "(vec false true true true)"}
// rev_rule! {pi2_tftttfff, "(pi2 (vec (pr true false) (pr true true) (pr true false) (pr false false)))",  "(vec false true false false)"}
// rev_rule! {pi2_tftttfft, "(pi2 (vec (pr true false) (pr true true) (pr true false) (pr false true)))",  "(vec false true false true)"}
// rev_rule! {pi2_tftttftf, "(pi2 (vec (pr true false) (pr true true) (pr true false) (pr true false)))",  "(vec false true false false)"}
// rev_rule! {pi2_tftttftt, "(pi2 (vec (pr true false) (pr true true) (pr true false) (pr true true)))",  "(vec false true false true)"}
// rev_rule! {pi2_tfttttff, "(pi2 (vec (pr true false) (pr true true) (pr true true) (pr false false)))",  "(vec false true true false)"}
// rev_rule! {pi2_tfttttft, "(pi2 (vec (pr true false) (pr true true) (pr true true) (pr false true)))",  "(vec false true true true)"}
// rev_rule! {pi2_tftttttf, "(pi2 (vec (pr true false) (pr true true) (pr true true) (pr true false)))",  "(vec false true true false)"}
// rev_rule! {pi2_tftttttt, "(pi2 (vec (pr true false) (pr true true) (pr true true) (pr true true)))",  "(vec false true true true)"}
// rev_rule! {pi2_ttffffff, "(pi2 (vec (pr true true) (pr false false) (pr false false) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {pi2_ttffffft, "(pi2 (vec (pr true true) (pr false false) (pr false false) (pr false true)))",  "(vec true false false true)"}
// rev_rule! {pi2_ttfffftf, "(pi2 (vec (pr true true) (pr false false) (pr false false) (pr true false)))",  "(vec true false false false)"}
// rev_rule! {pi2_ttfffftt, "(pi2 (vec (pr true true) (pr false false) (pr false false) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {pi2_ttffftff, "(pi2 (vec (pr true true) (pr false false) (pr false true) (pr false false)))",  "(vec true false true false)"}
// rev_rule! {pi2_ttffftft, "(pi2 (vec (pr true true) (pr false false) (pr false true) (pr false true)))",  "(vec true false true true)"}
// rev_rule! {pi2_ttfffttf, "(pi2 (vec (pr true true) (pr false false) (pr false true) (pr true false)))",  "(vec true false true false)"}
// rev_rule! {pi2_ttfffttt, "(pi2 (vec (pr true true) (pr false false) (pr false true) (pr true true)))",  "(vec true false true true)"}
// rev_rule! {pi2_ttfftfff, "(pi2 (vec (pr true true) (pr false false) (pr true false) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {pi2_ttfftfft, "(pi2 (vec (pr true true) (pr false false) (pr true false) (pr false true)))",  "(vec true false false true)"}
// rev_rule! {pi2_ttfftftf, "(pi2 (vec (pr true true) (pr false false) (pr true false) (pr true false)))",  "(vec true false false false)"}
// rev_rule! {pi2_ttfftftt, "(pi2 (vec (pr true true) (pr false false) (pr true false) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {pi2_ttffttff, "(pi2 (vec (pr true true) (pr false false) (pr true true) (pr false false)))",  "(vec true false true false)"}
// rev_rule! {pi2_ttffttft, "(pi2 (vec (pr true true) (pr false false) (pr true true) (pr false true)))",  "(vec true false true true)"}
// rev_rule! {pi2_ttfftttf, "(pi2 (vec (pr true true) (pr false false) (pr true true) (pr true false)))",  "(vec true false true false)"}
// rev_rule! {pi2_ttfftttt, "(pi2 (vec (pr true true) (pr false false) (pr true true) (pr true true)))",  "(vec true false true true)"}
// rev_rule! {pi2_ttftffff, "(pi2 (vec (pr true true) (pr false true) (pr false false) (pr false false)))",  "(vec true true false false)"}
// rev_rule! {pi2_ttftffft, "(pi2 (vec (pr true true) (pr false true) (pr false false) (pr false true)))",  "(vec true true false true)"}
// rev_rule! {pi2_ttftfftf, "(pi2 (vec (pr true true) (pr false true) (pr false false) (pr true false)))",  "(vec true true false false)"}
// rev_rule! {pi2_ttftfftt, "(pi2 (vec (pr true true) (pr false true) (pr false false) (pr true true)))",  "(vec true true false true)"}
// rev_rule! {pi2_ttftftff, "(pi2 (vec (pr true true) (pr false true) (pr false true) (pr false false)))",  "(vec true true true false)"}
// rev_rule! {pi2_ttftftft, "(pi2 (vec (pr true true) (pr false true) (pr false true) (pr false true)))",  "(vec true true true true)"}
// rev_rule! {pi2_ttftfttf, "(pi2 (vec (pr true true) (pr false true) (pr false true) (pr true false)))",  "(vec true true true false)"}
// rev_rule! {pi2_ttftfttt, "(pi2 (vec (pr true true) (pr false true) (pr false true) (pr true true)))",  "(vec true true true true)"}
// rev_rule! {pi2_ttfttfff, "(pi2 (vec (pr true true) (pr false true) (pr true false) (pr false false)))",  "(vec true true false false)"}
// rev_rule! {pi2_ttfttfft, "(pi2 (vec (pr true true) (pr false true) (pr true false) (pr false true)))",  "(vec true true false true)"}
// rev_rule! {pi2_ttfttftf, "(pi2 (vec (pr true true) (pr false true) (pr true false) (pr true false)))",  "(vec true true false false)"}
// rev_rule! {pi2_ttfttftt, "(pi2 (vec (pr true true) (pr false true) (pr true false) (pr true true)))",  "(vec true true false true)"}
// rev_rule! {pi2_ttftttff, "(pi2 (vec (pr true true) (pr false true) (pr true true) (pr false false)))",  "(vec true true true false)"}
// rev_rule! {pi2_ttftttft, "(pi2 (vec (pr true true) (pr false true) (pr true true) (pr false true)))",  "(vec true true true true)"}
// rev_rule! {pi2_ttfttttf, "(pi2 (vec (pr true true) (pr false true) (pr true true) (pr true false)))",  "(vec true true true false)"}
// rev_rule! {pi2_ttfttttt, "(pi2 (vec (pr true true) (pr false true) (pr true true) (pr true true)))",  "(vec true true true true)"}
// rev_rule! {pi2_tttfffff, "(pi2 (vec (pr true true) (pr true false) (pr false false) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {pi2_tttfffft, "(pi2 (vec (pr true true) (pr true false) (pr false false) (pr false true)))",  "(vec true false false true)"}
// rev_rule! {pi2_tttffftf, "(pi2 (vec (pr true true) (pr true false) (pr false false) (pr true false)))",  "(vec true false false false)"}
// rev_rule! {pi2_tttffftt, "(pi2 (vec (pr true true) (pr true false) (pr false false) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {pi2_tttfftff, "(pi2 (vec (pr true true) (pr true false) (pr false true) (pr false false)))",  "(vec true false true false)"}
// rev_rule! {pi2_tttfftft, "(pi2 (vec (pr true true) (pr true false) (pr false true) (pr false true)))",  "(vec true false true true)"}
// rev_rule! {pi2_tttffttf, "(pi2 (vec (pr true true) (pr true false) (pr false true) (pr true false)))",  "(vec true false true false)"}
// rev_rule! {pi2_tttffttt, "(pi2 (vec (pr true true) (pr true false) (pr false true) (pr true true)))",  "(vec true false true true)"}
// rev_rule! {pi2_tttftfff, "(pi2 (vec (pr true true) (pr true false) (pr true false) (pr false false)))",  "(vec true false false false)"}
// rev_rule! {pi2_tttftfft, "(pi2 (vec (pr true true) (pr true false) (pr true false) (pr false true)))",  "(vec true false false true)"}
// rev_rule! {pi2_tttftftf, "(pi2 (vec (pr true true) (pr true false) (pr true false) (pr true false)))",  "(vec true false false false)"}
// rev_rule! {pi2_tttftftt, "(pi2 (vec (pr true true) (pr true false) (pr true false) (pr true true)))",  "(vec true false false true)"}
// rev_rule! {pi2_tttfttff, "(pi2 (vec (pr true true) (pr true false) (pr true true) (pr false false)))",  "(vec true false true false)"}
// rev_rule! {pi2_tttfttft, "(pi2 (vec (pr true true) (pr true false) (pr true true) (pr false true)))",  "(vec true false true true)"}
// rev_rule! {pi2_tttftttf, "(pi2 (vec (pr true true) (pr true false) (pr true true) (pr true false)))",  "(vec true false true false)"}
// rev_rule! {pi2_tttftttt, "(pi2 (vec (pr true true) (pr true false) (pr true true) (pr true true)))",  "(vec true false true true)"}
// rev_rule! {pi2_ttttffff, "(pi2 (vec (pr true true) (pr true true) (pr false false) (pr false false)))",  "(vec true true false false)"}
// rev_rule! {pi2_ttttffft, "(pi2 (vec (pr true true) (pr true true) (pr false false) (pr false true)))",  "(vec true true false true)"}
// rev_rule! {pi2_ttttfftf, "(pi2 (vec (pr true true) (pr true true) (pr false false) (pr true false)))",  "(vec true true false false)"}
// rev_rule! {pi2_ttttfftt, "(pi2 (vec (pr true true) (pr true true) (pr false false) (pr true true)))",  "(vec true true false true)"}
// rev_rule! {pi2_ttttftff, "(pi2 (vec (pr true true) (pr true true) (pr false true) (pr false false)))",  "(vec true true true false)"}
// rev_rule! {pi2_ttttftft, "(pi2 (vec (pr true true) (pr true true) (pr false true) (pr false true)))",  "(vec true true true true)"}
// rev_rule! {pi2_ttttfttf, "(pi2 (vec (pr true true) (pr true true) (pr false true) (pr true false)))",  "(vec true true true false)"}
// rev_rule! {pi2_ttttfttt, "(pi2 (vec (pr true true) (pr true true) (pr false true) (pr true true)))",  "(vec true true true true)"}
// rev_rule! {pi2_tttttfff, "(pi2 (vec (pr true true) (pr true true) (pr true false) (pr false false)))",  "(vec true true false false)"}
// rev_rule! {pi2_tttttfft, "(pi2 (vec (pr true true) (pr true true) (pr true false) (pr false true)))",  "(vec true true false true)"}
// rev_rule! {pi2_tttttftf, "(pi2 (vec (pr true true) (pr true true) (pr true false) (pr true false)))",  "(vec true true false false)"}
// rev_rule! {pi2_tttttftt, "(pi2 (vec (pr true true) (pr true true) (pr true false) (pr true true)))",  "(vec true true false true)"}
// rev_rule! {pi2_ttttttff, "(pi2 (vec (pr true true) (pr true true) (pr true true) (pr false false)))",  "(vec true true true false)"}
// rev_rule! {pi2_ttttttft, "(pi2 (vec (pr true true) (pr true true) (pr true true) (pr false true)))",  "(vec true true true true)"}
// rev_rule! {pi2_tttttttf, "(pi2 (vec (pr true true) (pr true true) (pr true true) (pr true false)))",  "(vec true true true false)"}
// rev_rule! {pi2_tttttttt, "(pi2 (vec (pr true true) (pr true true) (pr true true) (pr true true)))",  "(vec true true true true)"}

// rule! {input, "(vec (pr false false) (pr false true) (pr true false) (pr true true))", "x"}

// // (vec false true true false)

// fn prove_something(name: &str, start: &str, rewrites: &[Rewrite], goals: &[&str]) {
//   let _ = env_logger::builder().is_test(true).try_init();
//   println!("Proving {}", name);

//   let start_expr: RecExpr<_> = start.parse().unwrap();
//   let goal_exprs: Vec<RecExpr<_>> = goals.iter().map(|g| g.parse().unwrap()).collect();

//   let runner = Runner::default()
//     .with_iter_limit(20)
//     .with_node_limit(5_000)
//     .with_expr(&start_expr)
//     .run(rewrites);

//   let (egraph, root) = (runner.egraph, runner.roots[0]);

//   egraph.dot().to_dot("target/getcell.dot").unwrap();

//   let mut extractor = Extractor::new(&egraph, CostFn);
//   let (best_cost, best) = extractor.find_best(root);
//   println!("best cost {}", best_cost);
//   println!("best {}", best);

//   for (i, (goal_expr, goal_str)) in goal_exprs.iter().zip(goals).enumerate() {
//     println!("Trying to prove goal {}: {}", i, goal_str);
//     let equivs = egraph.equivs(&start_expr, &goal_expr);
//     if equivs.is_empty() {
//       panic!("Couldn't prove goal {}: {}", i, goal_str);
//     }
//   }
// }

// #[test]
// fn prove_simple() {
//   let _ = env_logger::builder().is_test(true).try_init();
//   let rules = &[
//     not_ffff(),
//     not_ffft(),
//     not_fftf(),
//     not_fftt(),
//     not_ftff(),
//     not_ftft(),
//     not_fttf(),
//     not_fttt(),
//     not_tfff(),
//     not_tfft(),
//     not_tftf(),
//     not_tftt(),
//     not_ttff(),
//     not_ttft(),
//     not_tttf(),
//     not_tttt(),
//     and_ffffffff(),
//     and_ffffffft(),
//     and_fffffftf(),
//     and_fffffftt(),
//     and_ffffftff(),
//     and_ffffftft(),
//     and_fffffttf(),
//     and_fffffttt(),
//     and_fffftfff(),
//     and_fffftfft(),
//     and_fffftftf(),
//     and_fffftftt(),
//     and_ffffttff(),
//     and_ffffttft(),
//     and_fffftttf(),
//     and_fffftttt(),
//     and_ffftffff(),
//     and_ffftffft(),
//     and_ffftfftf(),
//     and_ffftfftt(),
//     and_ffftftff(),
//     and_ffftftft(),
//     and_ffftfttf(),
//     and_ffftfttt(),
//     and_fffttfff(),
//     and_fffttfft(),
//     and_fffttftf(),
//     and_fffttftt(),
//     and_ffftttff(),
//     and_ffftttft(),
//     and_fffttttf(),
//     and_fffttttt(),
//     and_fftfffff(),
//     and_fftfffft(),
//     and_fftffftf(),
//     and_fftffftt(),
//     and_fftfftff(),
//     and_fftfftft(),
//     and_fftffttf(),
//     and_fftffttt(),
//     and_fftftfff(),
//     and_fftftfft(),
//     and_fftftftf(),
//     and_fftftftt(),
//     and_fftfttff(),
//     and_fftfttft(),
//     and_fftftttf(),
//     and_fftftttt(),
//     and_ffttffff(),
//     and_ffttffft(),
//     and_ffttfftf(),
//     and_ffttfftt(),
//     and_ffttftff(),
//     and_ffttftft(),
//     and_ffttfttf(),
//     and_ffttfttt(),
//     and_fftttfff(),
//     and_fftttfft(),
//     and_fftttftf(),
//     and_fftttftt(),
//     and_ffttttff(),
//     and_ffttttft(),
//     and_fftttttf(),
//     and_fftttttt(),
//     and_ftffffff(),
//     and_ftffffft(),
//     and_ftfffftf(),
//     and_ftfffftt(),
//     and_ftffftff(),
//     and_ftffftft(),
//     and_ftfffttf(),
//     and_ftfffttt(),
//     and_ftfftfff(),
//     and_ftfftfft(),
//     and_ftfftftf(),
//     and_ftfftftt(),
//     and_ftffttff(),
//     and_ftffttft(),
//     and_ftfftttf(),
//     and_ftfftttt(),
//     and_ftftffff(),
//     and_ftftffft(),
//     and_ftftfftf(),
//     and_ftftfftt(),
//     and_ftftftff(),
//     and_ftftftft(),
//     and_ftftfttf(),
//     and_ftftfttt(),
//     and_ftfttfff(),
//     and_ftfttfft(),
//     and_ftfttftf(),
//     and_ftfttftt(),
//     and_ftftttff(),
//     and_ftftttft(),
//     and_ftfttttf(),
//     and_ftfttttt(),
//     and_fttfffff(),
//     and_fttfffft(),
//     and_fttffftf(),
//     and_fttffftt(),
//     and_fttfftff(),
//     and_fttfftft(),
//     and_fttffttf(),
//     and_fttffttt(),
//     and_fttftfff(),
//     and_fttftfft(),
//     and_fttftftf(),
//     and_fttftftt(),
//     and_fttfttff(),
//     and_fttfttft(),
//     and_fttftttf(),
//     and_fttftttt(),
//     and_ftttffff(),
//     and_ftttffft(),
//     and_ftttfftf(),
//     and_ftttfftt(),
//     and_ftttftff(),
//     and_ftttftft(),
//     and_ftttfttf(),
//     and_ftttfttt(),
//     and_fttttfff(),
//     and_fttttfft(),
//     and_fttttftf(),
//     and_fttttftt(),
//     and_ftttttff(),
//     and_ftttttft(),
//     and_fttttttf(),
//     and_fttttttt(),
//     and_tfffffff(),
//     and_tfffffft(),
//     and_tffffftf(),
//     and_tffffftt(),
//     and_tfffftff(),
//     and_tfffftft(),
//     and_tffffttf(),
//     and_tffffttt(),
//     and_tffftfff(),
//     and_tffftfft(),
//     and_tffftftf(),
//     and_tffftftt(),
//     and_tfffttff(),
//     and_tfffttft(),
//     and_tffftttf(),
//     and_tffftttt(),
//     and_tfftffff(),
//     and_tfftffft(),
//     and_tfftfftf(),
//     and_tfftfftt(),
//     and_tfftftff(),
//     and_tfftftft(),
//     and_tfftfttf(),
//     and_tfftfttt(),
//     and_tffttfff(),
//     and_tffttfft(),
//     and_tffttftf(),
//     and_tffttftt(),
//     and_tfftttff(),
//     and_tfftttft(),
//     and_tffttttf(),
//     and_tffttttt(),
//     and_tftfffff(),
//     and_tftfffft(),
//     and_tftffftf(),
//     and_tftffftt(),
//     and_tftfftff(),
//     and_tftfftft(),
//     and_tftffttf(),
//     and_tftffttt(),
//     and_tftftfff(),
//     and_tftftfft(),
//     and_tftftftf(),
//     and_tftftftt(),
//     and_tftfttff(),
//     and_tftfttft(),
//     and_tftftttf(),
//     and_tftftttt(),
//     and_tfttffff(),
//     and_tfttffft(),
//     and_tfttfftf(),
//     and_tfttfftt(),
//     and_tfttftff(),
//     and_tfttftft(),
//     and_tfttfttf(),
//     and_tfttfttt(),
//     and_tftttfff(),
//     and_tftttfft(),
//     and_tftttftf(),
//     and_tftttftt(),
//     and_tfttttff(),
//     and_tfttttft(),
//     and_tftttttf(),
//     and_tftttttt(),
//     and_ttffffff(),
//     and_ttffffft(),
//     and_ttfffftf(),
//     and_ttfffftt(),
//     and_ttffftff(),
//     and_ttffftft(),
//     and_ttfffttf(),
//     and_ttfffttt(),
//     and_ttfftfff(),
//     and_ttfftfft(),
//     and_ttfftftf(),
//     and_ttfftftt(),
//     and_ttffttff(),
//     and_ttffttft(),
//     and_ttfftttf(),
//     and_ttfftttt(),
//     and_ttftffff(),
//     and_ttftffft(),
//     and_ttftfftf(),
//     and_ttftfftt(),
//     and_ttftftff(),
//     and_ttftftft(),
//     and_ttftfttf(),
//     and_ttftfttt(),
//     and_ttfttfff(),
//     and_ttfttfft(),
//     and_ttfttftf(),
//     and_ttfttftt(),
//     and_ttftttff(),
//     and_ttftttft(),
//     and_ttfttttf(),
//     and_ttfttttt(),
//     and_tttfffff(),
//     and_tttfffft(),
//     and_tttffftf(),
//     and_tttffftt(),
//     and_tttfftff(),
//     and_tttfftft(),
//     and_tttffttf(),
//     and_tttffttt(),
//     and_tttftfff(),
//     and_tttftfft(),
//     and_tttftftf(),
//     and_tttftftt(),
//     and_tttfttff(),
//     and_tttfttft(),
//     and_tttftttf(),
//     and_tttftttt(),
//     and_ttttffff(),
//     and_ttttffft(),
//     and_ttttfftf(),
//     and_ttttfftt(),
//     and_ttttftff(),
//     and_ttttftft(),
//     and_ttttfttf(),
//     and_ttttfttt(),
//     and_tttttfff(),
//     and_tttttfft(),
//     and_tttttftf(),
//     and_tttttftt(),
//     and_ttttttff(),
//     and_ttttttft(),
//     and_tttttttf(),
//     and_tttttttt(),
//     pi1_ffffffff(),
//     pi1_ffffffft(),
//     pi1_fffffftf(),
//     pi1_fffffftt(),
//     pi1_ffffftff(),
//     pi1_ffffftft(),
//     pi1_fffffttf(),
//     pi1_fffffttt(),
//     pi1_fffftfff(),
//     pi1_fffftfft(),
//     pi1_fffftftf(),
//     pi1_fffftftt(),
//     pi1_ffffttff(),
//     pi1_ffffttft(),
//     pi1_fffftttf(),
//     pi1_fffftttt(),
//     pi1_ffftffff(),
//     pi1_ffftffft(),
//     pi1_ffftfftf(),
//     pi1_ffftfftt(),
//     pi1_ffftftff(),
//     pi1_ffftftft(),
//     pi1_ffftfttf(),
//     pi1_ffftfttt(),
//     pi1_fffttfff(),
//     pi1_fffttfft(),
//     pi1_fffttftf(),
//     pi1_fffttftt(),
//     pi1_ffftttff(),
//     pi1_ffftttft(),
//     pi1_fffttttf(),
//     pi1_fffttttt(),
//     pi1_fftfffff(),
//     pi1_fftfffft(),
//     pi1_fftffftf(),
//     pi1_fftffftt(),
//     pi1_fftfftff(),
//     pi1_fftfftft(),
//     pi1_fftffttf(),
//     pi1_fftffttt(),
//     pi1_fftftfff(),
//     pi1_fftftfft(),
//     pi1_fftftftf(),
//     pi1_fftftftt(),
//     pi1_fftfttff(),
//     pi1_fftfttft(),
//     pi1_fftftttf(),
//     pi1_fftftttt(),
//     pi1_ffttffff(),
//     pi1_ffttffft(),
//     pi1_ffttfftf(),
//     pi1_ffttfftt(),
//     pi1_ffttftff(),
//     pi1_ffttftft(),
//     pi1_ffttfttf(),
//     pi1_ffttfttt(),
//     pi1_fftttfff(),
//     pi1_fftttfft(),
//     pi1_fftttftf(),
//     pi1_fftttftt(),
//     pi1_ffttttff(),
//     pi1_ffttttft(),
//     pi1_fftttttf(),
//     pi1_fftttttt(),
//     pi1_ftffffff(),
//     pi1_ftffffft(),
//     pi1_ftfffftf(),
//     pi1_ftfffftt(),
//     pi1_ftffftff(),
//     pi1_ftffftft(),
//     pi1_ftfffttf(),
//     pi1_ftfffttt(),
//     pi1_ftfftfff(),
//     pi1_ftfftfft(),
//     pi1_ftfftftf(),
//     pi1_ftfftftt(),
//     pi1_ftffttff(),
//     pi1_ftffttft(),
//     pi1_ftfftttf(),
//     pi1_ftfftttt(),
//     pi1_ftftffff(),
//     pi1_ftftffft(),
//     pi1_ftftfftf(),
//     pi1_ftftfftt(),
//     pi1_ftftftff(),
//     pi1_ftftftft(),
//     pi1_ftftfttf(),
//     pi1_ftftfttt(),
//     pi1_ftfttfff(),
//     pi1_ftfttfft(),
//     pi1_ftfttftf(),
//     pi1_ftfttftt(),
//     pi1_ftftttff(),
//     pi1_ftftttft(),
//     pi1_ftfttttf(),
//     pi1_ftfttttt(),
//     pi1_fttfffff(),
//     pi1_fttfffft(),
//     pi1_fttffftf(),
//     pi1_fttffftt(),
//     pi1_fttfftff(),
//     pi1_fttfftft(),
//     pi1_fttffttf(),
//     pi1_fttffttt(),
//     pi1_fttftfff(),
//     pi1_fttftfft(),
//     pi1_fttftftf(),
//     pi1_fttftftt(),
//     pi1_fttfttff(),
//     pi1_fttfttft(),
//     pi1_fttftttf(),
//     pi1_fttftttt(),
//     pi1_ftttffff(),
//     pi1_ftttffft(),
//     pi1_ftttfftf(),
//     pi1_ftttfftt(),
//     pi1_ftttftff(),
//     pi1_ftttftft(),
//     pi1_ftttfttf(),
//     pi1_ftttfttt(),
//     pi1_fttttfff(),
//     pi1_fttttfft(),
//     pi1_fttttftf(),
//     pi1_fttttftt(),
//     pi1_ftttttff(),
//     pi1_ftttttft(),
//     pi1_fttttttf(),
//     pi1_fttttttt(),
//     pi1_tfffffff(),
//     pi1_tfffffft(),
//     pi1_tffffftf(),
//     pi1_tffffftt(),
//     pi1_tfffftff(),
//     pi1_tfffftft(),
//     pi1_tffffttf(),
//     pi1_tffffttt(),
//     pi1_tffftfff(),
//     pi1_tffftfft(),
//     pi1_tffftftf(),
//     pi1_tffftftt(),
//     pi1_tfffttff(),
//     pi1_tfffttft(),
//     pi1_tffftttf(),
//     pi1_tffftttt(),
//     pi1_tfftffff(),
//     pi1_tfftffft(),
//     pi1_tfftfftf(),
//     pi1_tfftfftt(),
//     pi1_tfftftff(),
//     pi1_tfftftft(),
//     pi1_tfftfttf(),
//     pi1_tfftfttt(),
//     pi1_tffttfff(),
//     pi1_tffttfft(),
//     pi1_tffttftf(),
//     pi1_tffttftt(),
//     pi1_tfftttff(),
//     pi1_tfftttft(),
//     pi1_tffttttf(),
//     pi1_tffttttt(),
//     pi1_tftfffff(),
//     pi1_tftfffft(),
//     pi1_tftffftf(),
//     pi1_tftffftt(),
//     pi1_tftfftff(),
//     pi1_tftfftft(),
//     pi1_tftffttf(),
//     pi1_tftffttt(),
//     pi1_tftftfff(),
//     pi1_tftftfft(),
//     pi1_tftftftf(),
//     pi1_tftftftt(),
//     pi1_tftfttff(),
//     pi1_tftfttft(),
//     pi1_tftftttf(),
//     pi1_tftftttt(),
//     pi1_tfttffff(),
//     pi1_tfttffft(),
//     pi1_tfttfftf(),
//     pi1_tfttfftt(),
//     pi1_tfttftff(),
//     pi1_tfttftft(),
//     pi1_tfttfttf(),
//     pi1_tfttfttt(),
//     pi1_tftttfff(),
//     pi1_tftttfft(),
//     pi1_tftttftf(),
//     pi1_tftttftt(),
//     pi1_tfttttff(),
//     pi1_tfttttft(),
//     pi1_tftttttf(),
//     pi1_tftttttt(),
//     pi1_ttffffff(),
//     pi1_ttffffft(),
//     pi1_ttfffftf(),
//     pi1_ttfffftt(),
//     pi1_ttffftff(),
//     pi1_ttffftft(),
//     pi1_ttfffttf(),
//     pi1_ttfffttt(),
//     pi1_ttfftfff(),
//     pi1_ttfftfft(),
//     pi1_ttfftftf(),
//     pi1_ttfftftt(),
//     pi1_ttffttff(),
//     pi1_ttffttft(),
//     pi1_ttfftttf(),
//     pi1_ttfftttt(),
//     pi1_ttftffff(),
//     pi1_ttftffft(),
//     pi1_ttftfftf(),
//     pi1_ttftfftt(),
//     pi1_ttftftff(),
//     pi1_ttftftft(),
//     pi1_ttftfttf(),
//     pi1_ttftfttt(),
//     pi1_ttfttfff(),
//     pi1_ttfttfft(),
//     pi1_ttfttftf(),
//     pi1_ttfttftt(),
//     pi1_ttftttff(),
//     pi1_ttftttft(),
//     pi1_ttfttttf(),
//     pi1_ttfttttt(),
//     pi1_tttfffff(),
//     pi1_tttfffft(),
//     pi1_tttffftf(),
//     pi1_tttffftt(),
//     pi1_tttfftff(),
//     pi1_tttfftft(),
//     pi1_tttffttf(),
//     pi1_tttffttt(),
//     pi1_tttftfff(),
//     pi1_tttftfft(),
//     pi1_tttftftf(),
//     pi1_tttftftt(),
//     pi1_tttfttff(),
//     pi1_tttfttft(),
//     pi1_tttftttf(),
//     pi1_tttftttt(),
//     pi1_ttttffff(),
//     pi1_ttttffft(),
//     pi1_ttttfftf(),
//     pi1_ttttfftt(),
//     pi1_ttttftff(),
//     pi1_ttttftft(),
//     pi1_ttttfttf(),
//     pi1_ttttfttt(),
//     pi1_tttttfff(),
//     pi1_tttttfft(),
//     pi1_tttttftf(),
//     pi1_tttttftt(),
//     pi1_ttttttff(),
//     pi1_ttttttft(),
//     pi1_tttttttf(),
//     pi1_tttttttt(),
//     pi2_ffffffff(),
//     pi2_ffffffft(),
//     pi2_fffffftf(),
//     pi2_fffffftt(),
//     pi2_ffffftff(),
//     pi2_ffffftft(),
//     pi2_fffffttf(),
//     pi2_fffffttt(),
//     pi2_fffftfff(),
//     pi2_fffftfft(),
//     pi2_fffftftf(),
//     pi2_fffftftt(),
//     pi2_ffffttff(),
//     pi2_ffffttft(),
//     pi2_fffftttf(),
//     pi2_fffftttt(),
//     pi2_ffftffff(),
//     pi2_ffftffft(),
//     pi2_ffftfftf(),
//     pi2_ffftfftt(),
//     pi2_ffftftff(),
//     pi2_ffftftft(),
//     pi2_ffftfttf(),
//     pi2_ffftfttt(),
//     pi2_fffttfff(),
//     pi2_fffttfft(),
//     pi2_fffttftf(),
//     pi2_fffttftt(),
//     pi2_ffftttff(),
//     pi2_ffftttft(),
//     pi2_fffttttf(),
//     pi2_fffttttt(),
//     pi2_fftfffff(),
//     pi2_fftfffft(),
//     pi2_fftffftf(),
//     pi2_fftffftt(),
//     pi2_fftfftff(),
//     pi2_fftfftft(),
//     pi2_fftffttf(),
//     pi2_fftffttt(),
//     pi2_fftftfff(),
//     pi2_fftftfft(),
//     pi2_fftftftf(),
//     pi2_fftftftt(),
//     pi2_fftfttff(),
//     pi2_fftfttft(),
//     pi2_fftftttf(),
//     pi2_fftftttt(),
//     pi2_ffttffff(),
//     pi2_ffttffft(),
//     pi2_ffttfftf(),
//     pi2_ffttfftt(),
//     pi2_ffttftff(),
//     pi2_ffttftft(),
//     pi2_ffttfttf(),
//     pi2_ffttfttt(),
//     pi2_fftttfff(),
//     pi2_fftttfft(),
//     pi2_fftttftf(),
//     pi2_fftttftt(),
//     pi2_ffttttff(),
//     pi2_ffttttft(),
//     pi2_fftttttf(),
//     pi2_fftttttt(),
//     pi2_ftffffff(),
//     pi2_ftffffft(),
//     pi2_ftfffftf(),
//     pi2_ftfffftt(),
//     pi2_ftffftff(),
//     pi2_ftffftft(),
//     pi2_ftfffttf(),
//     pi2_ftfffttt(),
//     pi2_ftfftfff(),
//     pi2_ftfftfft(),
//     pi2_ftfftftf(),
//     pi2_ftfftftt(),
//     pi2_ftffttff(),
//     pi2_ftffttft(),
//     pi2_ftfftttf(),
//     pi2_ftfftttt(),
//     pi2_ftftffff(),
//     pi2_ftftffft(),
//     pi2_ftftfftf(),
//     pi2_ftftfftt(),
//     pi2_ftftftff(),
//     pi2_ftftftft(),
//     pi2_ftftfttf(),
//     pi2_ftftfttt(),
//     pi2_ftfttfff(),
//     pi2_ftfttfft(),
//     pi2_ftfttftf(),
//     pi2_ftfttftt(),
//     pi2_ftftttff(),
//     pi2_ftftttft(),
//     pi2_ftfttttf(),
//     pi2_ftfttttt(),
//     pi2_fttfffff(),
//     pi2_fttfffft(),
//     pi2_fttffftf(),
//     pi2_fttffftt(),
//     pi2_fttfftff(),
//     pi2_fttfftft(),
//     pi2_fttffttf(),
//     pi2_fttffttt(),
//     pi2_fttftfff(),
//     pi2_fttftfft(),
//     pi2_fttftftf(),
//     pi2_fttftftt(),
//     pi2_fttfttff(),
//     pi2_fttfttft(),
//     pi2_fttftttf(),
//     pi2_fttftttt(),
//     pi2_ftttffff(),
//     pi2_ftttffft(),
//     pi2_ftttfftf(),
//     pi2_ftttfftt(),
//     pi2_ftttftff(),
//     pi2_ftttftft(),
//     pi2_ftttfttf(),
//     pi2_ftttfttt(),
//     pi2_fttttfff(),
//     pi2_fttttfft(),
//     pi2_fttttftf(),
//     pi2_fttttftt(),
//     pi2_ftttttff(),
//     pi2_ftttttft(),
//     pi2_fttttttf(),
//     pi2_fttttttt(),
//     pi2_tfffffff(),
//     pi2_tfffffft(),
//     pi2_tffffftf(),
//     pi2_tffffftt(),
//     pi2_tfffftff(),
//     pi2_tfffftft(),
//     pi2_tffffttf(),
//     pi2_tffffttt(),
//     pi2_tffftfff(),
//     pi2_tffftfft(),
//     pi2_tffftftf(),
//     pi2_tffftftt(),
//     pi2_tfffttff(),
//     pi2_tfffttft(),
//     pi2_tffftttf(),
//     pi2_tffftttt(),
//     pi2_tfftffff(),
//     pi2_tfftffft(),
//     pi2_tfftfftf(),
//     pi2_tfftfftt(),
//     pi2_tfftftff(),
//     pi2_tfftftft(),
//     pi2_tfftfttf(),
//     pi2_tfftfttt(),
//     pi2_tffttfff(),
//     pi2_tffttfft(),
//     pi2_tffttftf(),
//     pi2_tffttftt(),
//     pi2_tfftttff(),
//     pi2_tfftttft(),
//     pi2_tffttttf(),
//     pi2_tffttttt(),
//     pi2_tftfffff(),
//     pi2_tftfffft(),
//     pi2_tftffftf(),
//     pi2_tftffftt(),
//     pi2_tftfftff(),
//     pi2_tftfftft(),
//     pi2_tftffttf(),
//     pi2_tftffttt(),
//     pi2_tftftfff(),
//     pi2_tftftfft(),
//     pi2_tftftftf(),
//     pi2_tftftftt(),
//     pi2_tftfttff(),
//     pi2_tftfttft(),
//     pi2_tftftttf(),
//     pi2_tftftttt(),
//     pi2_tfttffff(),
//     pi2_tfttffft(),
//     pi2_tfttfftf(),
//     pi2_tfttfftt(),
//     pi2_tfttftff(),
//     pi2_tfttftft(),
//     pi2_tfttfttf(),
//     pi2_tfttfttt(),
//     pi2_tftttfff(),
//     pi2_tftttfft(),
//     pi2_tftttftf(),
//     pi2_tftttftt(),
//     pi2_tfttttff(),
//     pi2_tfttttft(),
//     pi2_tftttttf(),
//     pi2_tftttttt(),
//     pi2_ttffffff(),
//     pi2_ttffffft(),
//     pi2_ttfffftf(),
//     pi2_ttfffftt(),
//     pi2_ttffftff(),
//     pi2_ttffftft(),
//     pi2_ttfffttf(),
//     pi2_ttfffttt(),
//     pi2_ttfftfff(),
//     pi2_ttfftfft(),
//     pi2_ttfftftf(),
//     pi2_ttfftftt(),
//     pi2_ttffttff(),
//     pi2_ttffttft(),
//     pi2_ttfftttf(),
//     pi2_ttfftttt(),
//     pi2_ttftffff(),
//     pi2_ttftffft(),
//     pi2_ttftfftf(),
//     pi2_ttftfftt(),
//     pi2_ttftftff(),
//     pi2_ttftftft(),
//     pi2_ttftfttf(),
//     pi2_ttftfttt(),
//     pi2_ttfttfff(),
//     pi2_ttfttfft(),
//     pi2_ttfttftf(),
//     pi2_ttfttftt(),
//     pi2_ttftttff(),
//     pi2_ttftttft(),
//     pi2_ttfttttf(),
//     pi2_ttfttttt(),
//     pi2_tttfffff(),
//     pi2_tttfffft(),
//     pi2_tttffftf(),
//     pi2_tttffftt(),
//     pi2_tttfftff(),
//     pi2_tttfftft(),
//     pi2_tttffttf(),
//     pi2_tttffttt(),
//     pi2_tttftfff(),
//     pi2_tttftfft(),
//     pi2_tttftftf(),
//     pi2_tttftftt(),
//     pi2_tttfttff(),
//     pi2_tttfttft(),
//     pi2_tttftttf(),
//     pi2_tttftttt(),
//     pi2_ttttffff(),
//     pi2_ttttffft(),
//     pi2_ttttfftf(),
//     pi2_ttttfftt(),
//     pi2_ttttftff(),
//     pi2_ttttftft(),
//     pi2_ttttfttf(),
//     pi2_ttttfttt(),
//     pi2_tttttfff(),
//     pi2_tttttfft(),
//     pi2_tttttftf(),
//     pi2_tttttftt(),
//     pi2_ttttttff(),
//     pi2_ttttttft(),
//     pi2_tttttttf(),
//     pi2_tttttttt(),
//     // pi1(),
//     // pi2(),
//     input(),
//   ];
//   prove_something("simple", "(vec false true false true)", rules, &[]);
// }

// // #[test]
// // fn prove_contrapositive() {
// //   let _ = env_logger::builder().is_test(true).try_init();
// //   let rules = &[def_imply(), def_imply_flip(), double_neg_flip(), comm_or()];
// //   prove_something(
// //     "contrapositive",
// //     "(-> x y)",
// //     rules,
// //     &[
// //       "(-> x y)",
// //       "(| (~ x) y)",
// //       "(| (~ x) (~ (~ y)))",
// //       "(| (~ (~ y)) (~ x))",
// //       "(-> (~ y) (~ x))",
// //     ],
// //   );
// // }

// // #[test]
// // fn prove_chain() {
// //   let _ = env_logger::builder().is_test(true).try_init();
// //   let rules = &[
// //     // rules needed for contrapositive
// //     def_imply(),
// //     def_imply_flip(),
// //     double_neg_flip(),
// //     comm_or(),
// //     // and some others
// //     comm_and(),
// //     lem_imply(),
// //   ];
// //   prove_something(
// //     "chain",
// //     "(& (-> x y) (-> y z))",
// //     rules,
// //     &[
// //       "(& (-> x y) (-> y z))",
// //       "(& (-> (~ y) (~ x)) (-> y z))",
// //       "(& (-> y z)         (-> (~ y) (~ x)))",
// //       "(| z (~ x))",
// //       "(| (~ x) z)",
// //       "(-> x z)",
// //     ],
// //   );
// // }

// // #[test]
// // fn const_fold() {
// //   let start = "(| (& false true) (& true false))";
// //   let start_expr = start.parse().unwrap();
// //   let end = "false";
// //   let end_expr = end.parse().unwrap();
// //   let mut eg = EGraph::default();
// //   eg.add_expr(&start_expr);
// //   eg.rebuild();
// //   assert!(!eg.equivs(&start_expr, &end_expr).is_empty());
// // }
