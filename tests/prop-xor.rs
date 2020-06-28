use egg::*;

define_language! {
    enum Prop {
        Bool(bool),
        "pr" = Pr([Id; 2]),
        "vec" = PVec([Id; 4]),
        "&" = And(Id),
        "~" = Not(Id),
        // "|" = Or([Id; 2]),
        // "->" = Implies([Id; 2]),
        "x" = X,
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

// thanks I hate it

rev_rule! {not_ffff, "(~ (vec false false false false))", "(vec true true true true)"}
rev_rule! {not_ffft, "(~ (vec false false false true))", "(vec true true true false)"}
rev_rule! {not_fftf, "(~ (vec false false true false))", "(vec true true false true)"}
rev_rule! {not_fftt, "(~ (vec false false true true))", "(vec true true false false)"}
rev_rule! {not_ftff, "(~ (vec false true false false))", "(vec true false true true)"}
rev_rule! {not_ftft, "(~ (vec false true false true))", "(vec true false true false)"}
rev_rule! {not_fttf, "(~ (vec false true true false))", "(vec true false false true)"}
rev_rule! {not_fttt, "(~ (vec false true true true))", "(vec true false false false)"}
rev_rule! {not_tfff, "(~ (vec true false false false))", "(vec false true true true)"}
rev_rule! {not_tfft, "(~ (vec true false false true))", "(vec false true true false)"}
rev_rule! {not_tftf, "(~ (vec true false true false))", "(vec false true false true)"}
rev_rule! {not_tftt, "(~ (vec true false true true))", "(vec false true false false)"}
rev_rule! {not_ttff, "(~ (vec true true false false))", "(vec false false true true)"}
rev_rule! {not_ttft, "(~ (vec true true false true))", "(vec false false true false)"}
rev_rule! {not_tttf, "(~ (vec true true true false))", "(vec false false false true)"}
rev_rule! {not_tttt, "(~ (vec true true true true))", "(vec false false false false)"}

rev_rule! {and_ffffffff, "(& (vec (pr false false) (pr false false) (pr false false) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_ffffffft, "(& (vec (pr false false) (pr false false) (pr false false) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_fffffftf, "(& (vec (pr false false) (pr false false) (pr false false) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_fffffftt, "(& (vec (pr false false) (pr false false) (pr false false) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_ffffftff, "(& (vec (pr false false) (pr false false) (pr false true) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_ffffftft, "(& (vec (pr false false) (pr false false) (pr false true) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_fffffttf, "(& (vec (pr false false) (pr false false) (pr false true) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_fffffttt, "(& (vec (pr false false) (pr false false) (pr false true) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_fffftfff, "(& (vec (pr false false) (pr false false) (pr true false) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_fffftfft, "(& (vec (pr false false) (pr false false) (pr true false) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_fffftftf, "(& (vec (pr false false) (pr false false) (pr true false) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_fffftftt, "(& (vec (pr false false) (pr false false) (pr true false) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_ffffttff, "(& (vec (pr false false) (pr false false) (pr true true) (pr false false)))",  "(vec false false true false)"}
rev_rule! {and_ffffttft, "(& (vec (pr false false) (pr false false) (pr true true) (pr false true)))",  "(vec false false true false)"}
rev_rule! {and_fffftttf, "(& (vec (pr false false) (pr false false) (pr true true) (pr true false)))",  "(vec false false true false)"}
rev_rule! {and_fffftttt, "(& (vec (pr false false) (pr false false) (pr true true) (pr true true)))",  "(vec false false true true)"}
rev_rule! {and_ffftffff, "(& (vec (pr false false) (pr false true) (pr false false) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_ffftffft, "(& (vec (pr false false) (pr false true) (pr false false) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_ffftfftf, "(& (vec (pr false false) (pr false true) (pr false false) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_ffftfftt, "(& (vec (pr false false) (pr false true) (pr false false) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_ffftftff, "(& (vec (pr false false) (pr false true) (pr false true) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_ffftftft, "(& (vec (pr false false) (pr false true) (pr false true) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_ffftfttf, "(& (vec (pr false false) (pr false true) (pr false true) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_ffftfttt, "(& (vec (pr false false) (pr false true) (pr false true) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_fffttfff, "(& (vec (pr false false) (pr false true) (pr true false) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_fffttfft, "(& (vec (pr false false) (pr false true) (pr true false) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_fffttftf, "(& (vec (pr false false) (pr false true) (pr true false) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_fffttftt, "(& (vec (pr false false) (pr false true) (pr true false) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_ffftttff, "(& (vec (pr false false) (pr false true) (pr true true) (pr false false)))",  "(vec false false true false)"}
rev_rule! {and_ffftttft, "(& (vec (pr false false) (pr false true) (pr true true) (pr false true)))",  "(vec false false true false)"}
rev_rule! {and_fffttttf, "(& (vec (pr false false) (pr false true) (pr true true) (pr true false)))",  "(vec false false true false)"}
rev_rule! {and_fffttttt, "(& (vec (pr false false) (pr false true) (pr true true) (pr true true)))",  "(vec false false true true)"}
rev_rule! {and_fftfffff, "(& (vec (pr false false) (pr true false) (pr false false) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_fftfffft, "(& (vec (pr false false) (pr true false) (pr false false) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_fftffftf, "(& (vec (pr false false) (pr true false) (pr false false) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_fftffftt, "(& (vec (pr false false) (pr true false) (pr false false) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_fftfftff, "(& (vec (pr false false) (pr true false) (pr false true) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_fftfftft, "(& (vec (pr false false) (pr true false) (pr false true) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_fftffttf, "(& (vec (pr false false) (pr true false) (pr false true) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_fftffttt, "(& (vec (pr false false) (pr true false) (pr false true) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_fftftfff, "(& (vec (pr false false) (pr true false) (pr true false) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_fftftfft, "(& (vec (pr false false) (pr true false) (pr true false) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_fftftftf, "(& (vec (pr false false) (pr true false) (pr true false) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_fftftftt, "(& (vec (pr false false) (pr true false) (pr true false) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_fftfttff, "(& (vec (pr false false) (pr true false) (pr true true) (pr false false)))",  "(vec false false true false)"}
rev_rule! {and_fftfttft, "(& (vec (pr false false) (pr true false) (pr true true) (pr false true)))",  "(vec false false true false)"}
rev_rule! {and_fftftttf, "(& (vec (pr false false) (pr true false) (pr true true) (pr true false)))",  "(vec false false true false)"}
rev_rule! {and_fftftttt, "(& (vec (pr false false) (pr true false) (pr true true) (pr true true)))",  "(vec false false true true)"}
rev_rule! {and_ffttffff, "(& (vec (pr false false) (pr true true) (pr false false) (pr false false)))",  "(vec false true false false)"}
rev_rule! {and_ffttffft, "(& (vec (pr false false) (pr true true) (pr false false) (pr false true)))",  "(vec false true false false)"}
rev_rule! {and_ffttfftf, "(& (vec (pr false false) (pr true true) (pr false false) (pr true false)))",  "(vec false true false false)"}
rev_rule! {and_ffttfftt, "(& (vec (pr false false) (pr true true) (pr false false) (pr true true)))",  "(vec false true false true)"}
rev_rule! {and_ffttftff, "(& (vec (pr false false) (pr true true) (pr false true) (pr false false)))",  "(vec false true false false)"}
rev_rule! {and_ffttftft, "(& (vec (pr false false) (pr true true) (pr false true) (pr false true)))",  "(vec false true false false)"}
rev_rule! {and_ffttfttf, "(& (vec (pr false false) (pr true true) (pr false true) (pr true false)))",  "(vec false true false false)"}
rev_rule! {and_ffttfttt, "(& (vec (pr false false) (pr true true) (pr false true) (pr true true)))",  "(vec false true false true)"}
rev_rule! {and_fftttfff, "(& (vec (pr false false) (pr true true) (pr true false) (pr false false)))",  "(vec false true false false)"}
rev_rule! {and_fftttfft, "(& (vec (pr false false) (pr true true) (pr true false) (pr false true)))",  "(vec false true false false)"}
rev_rule! {and_fftttftf, "(& (vec (pr false false) (pr true true) (pr true false) (pr true false)))",  "(vec false true false false)"}
rev_rule! {and_fftttftt, "(& (vec (pr false false) (pr true true) (pr true false) (pr true true)))",  "(vec false true false true)"}
rev_rule! {and_ffttttff, "(& (vec (pr false false) (pr true true) (pr true true) (pr false false)))",  "(vec false true true false)"}
rev_rule! {and_ffttttft, "(& (vec (pr false false) (pr true true) (pr true true) (pr false true)))",  "(vec false true true false)"}
rev_rule! {and_fftttttf, "(& (vec (pr false false) (pr true true) (pr true true) (pr true false)))",  "(vec false true true false)"}
rev_rule! {and_fftttttt, "(& (vec (pr false false) (pr true true) (pr true true) (pr true true)))",  "(vec false true true true)"}
rev_rule! {and_ftffffff, "(& (vec (pr false true) (pr false false) (pr false false) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_ftffffft, "(& (vec (pr false true) (pr false false) (pr false false) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_ftfffftf, "(& (vec (pr false true) (pr false false) (pr false false) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_ftfffftt, "(& (vec (pr false true) (pr false false) (pr false false) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_ftffftff, "(& (vec (pr false true) (pr false false) (pr false true) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_ftffftft, "(& (vec (pr false true) (pr false false) (pr false true) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_ftfffttf, "(& (vec (pr false true) (pr false false) (pr false true) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_ftfffttt, "(& (vec (pr false true) (pr false false) (pr false true) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_ftfftfff, "(& (vec (pr false true) (pr false false) (pr true false) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_ftfftfft, "(& (vec (pr false true) (pr false false) (pr true false) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_ftfftftf, "(& (vec (pr false true) (pr false false) (pr true false) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_ftfftftt, "(& (vec (pr false true) (pr false false) (pr true false) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_ftffttff, "(& (vec (pr false true) (pr false false) (pr true true) (pr false false)))",  "(vec false false true false)"}
rev_rule! {and_ftffttft, "(& (vec (pr false true) (pr false false) (pr true true) (pr false true)))",  "(vec false false true false)"}
rev_rule! {and_ftfftttf, "(& (vec (pr false true) (pr false false) (pr true true) (pr true false)))",  "(vec false false true false)"}
rev_rule! {and_ftfftttt, "(& (vec (pr false true) (pr false false) (pr true true) (pr true true)))",  "(vec false false true true)"}
rev_rule! {and_ftftffff, "(& (vec (pr false true) (pr false true) (pr false false) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_ftftffft, "(& (vec (pr false true) (pr false true) (pr false false) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_ftftfftf, "(& (vec (pr false true) (pr false true) (pr false false) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_ftftfftt, "(& (vec (pr false true) (pr false true) (pr false false) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_ftftftff, "(& (vec (pr false true) (pr false true) (pr false true) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_ftftftft, "(& (vec (pr false true) (pr false true) (pr false true) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_ftftfttf, "(& (vec (pr false true) (pr false true) (pr false true) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_ftftfttt, "(& (vec (pr false true) (pr false true) (pr false true) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_ftfttfff, "(& (vec (pr false true) (pr false true) (pr true false) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_ftfttfft, "(& (vec (pr false true) (pr false true) (pr true false) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_ftfttftf, "(& (vec (pr false true) (pr false true) (pr true false) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_ftfttftt, "(& (vec (pr false true) (pr false true) (pr true false) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_ftftttff, "(& (vec (pr false true) (pr false true) (pr true true) (pr false false)))",  "(vec false false true false)"}
rev_rule! {and_ftftttft, "(& (vec (pr false true) (pr false true) (pr true true) (pr false true)))",  "(vec false false true false)"}
rev_rule! {and_ftfttttf, "(& (vec (pr false true) (pr false true) (pr true true) (pr true false)))",  "(vec false false true false)"}
rev_rule! {and_ftfttttt, "(& (vec (pr false true) (pr false true) (pr true true) (pr true true)))",  "(vec false false true true)"}
rev_rule! {and_fttfffff, "(& (vec (pr false true) (pr true false) (pr false false) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_fttfffft, "(& (vec (pr false true) (pr true false) (pr false false) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_fttffftf, "(& (vec (pr false true) (pr true false) (pr false false) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_fttffftt, "(& (vec (pr false true) (pr true false) (pr false false) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_fttfftff, "(& (vec (pr false true) (pr true false) (pr false true) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_fttfftft, "(& (vec (pr false true) (pr true false) (pr false true) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_fttffttf, "(& (vec (pr false true) (pr true false) (pr false true) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_fttffttt, "(& (vec (pr false true) (pr true false) (pr false true) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_fttftfff, "(& (vec (pr false true) (pr true false) (pr true false) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_fttftfft, "(& (vec (pr false true) (pr true false) (pr true false) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_fttftftf, "(& (vec (pr false true) (pr true false) (pr true false) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_fttftftt, "(& (vec (pr false true) (pr true false) (pr true false) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_fttfttff, "(& (vec (pr false true) (pr true false) (pr true true) (pr false false)))",  "(vec false false true false)"}
rev_rule! {and_fttfttft, "(& (vec (pr false true) (pr true false) (pr true true) (pr false true)))",  "(vec false false true false)"}
rev_rule! {and_fttftttf, "(& (vec (pr false true) (pr true false) (pr true true) (pr true false)))",  "(vec false false true false)"}
rev_rule! {and_fttftttt, "(& (vec (pr false true) (pr true false) (pr true true) (pr true true)))",  "(vec false false true true)"}
rev_rule! {and_ftttffff, "(& (vec (pr false true) (pr true true) (pr false false) (pr false false)))",  "(vec false true false false)"}
rev_rule! {and_ftttffft, "(& (vec (pr false true) (pr true true) (pr false false) (pr false true)))",  "(vec false true false false)"}
rev_rule! {and_ftttfftf, "(& (vec (pr false true) (pr true true) (pr false false) (pr true false)))",  "(vec false true false false)"}
rev_rule! {and_ftttfftt, "(& (vec (pr false true) (pr true true) (pr false false) (pr true true)))",  "(vec false true false true)"}
rev_rule! {and_ftttftff, "(& (vec (pr false true) (pr true true) (pr false true) (pr false false)))",  "(vec false true false false)"}
rev_rule! {and_ftttftft, "(& (vec (pr false true) (pr true true) (pr false true) (pr false true)))",  "(vec false true false false)"}
rev_rule! {and_ftttfttf, "(& (vec (pr false true) (pr true true) (pr false true) (pr true false)))",  "(vec false true false false)"}
rev_rule! {and_ftttfttt, "(& (vec (pr false true) (pr true true) (pr false true) (pr true true)))",  "(vec false true false true)"}
rev_rule! {and_fttttfff, "(& (vec (pr false true) (pr true true) (pr true false) (pr false false)))",  "(vec false true false false)"}
rev_rule! {and_fttttfft, "(& (vec (pr false true) (pr true true) (pr true false) (pr false true)))",  "(vec false true false false)"}
rev_rule! {and_fttttftf, "(& (vec (pr false true) (pr true true) (pr true false) (pr true false)))",  "(vec false true false false)"}
rev_rule! {and_fttttftt, "(& (vec (pr false true) (pr true true) (pr true false) (pr true true)))",  "(vec false true false true)"}
rev_rule! {and_ftttttff, "(& (vec (pr false true) (pr true true) (pr true true) (pr false false)))",  "(vec false true true false)"}
rev_rule! {and_ftttttft, "(& (vec (pr false true) (pr true true) (pr true true) (pr false true)))",  "(vec false true true false)"}
rev_rule! {and_fttttttf, "(& (vec (pr false true) (pr true true) (pr true true) (pr true false)))",  "(vec false true true false)"}
rev_rule! {and_fttttttt, "(& (vec (pr false true) (pr true true) (pr true true) (pr true true)))",  "(vec false true true true)"}
rev_rule! {and_tfffffff, "(& (vec (pr true false) (pr false false) (pr false false) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_tfffffft, "(& (vec (pr true false) (pr false false) (pr false false) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_tffffftf, "(& (vec (pr true false) (pr false false) (pr false false) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_tffffftt, "(& (vec (pr true false) (pr false false) (pr false false) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_tfffftff, "(& (vec (pr true false) (pr false false) (pr false true) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_tfffftft, "(& (vec (pr true false) (pr false false) (pr false true) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_tffffttf, "(& (vec (pr true false) (pr false false) (pr false true) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_tffffttt, "(& (vec (pr true false) (pr false false) (pr false true) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_tffftfff, "(& (vec (pr true false) (pr false false) (pr true false) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_tffftfft, "(& (vec (pr true false) (pr false false) (pr true false) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_tffftftf, "(& (vec (pr true false) (pr false false) (pr true false) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_tffftftt, "(& (vec (pr true false) (pr false false) (pr true false) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_tfffttff, "(& (vec (pr true false) (pr false false) (pr true true) (pr false false)))",  "(vec false false true false)"}
rev_rule! {and_tfffttft, "(& (vec (pr true false) (pr false false) (pr true true) (pr false true)))",  "(vec false false true false)"}
rev_rule! {and_tffftttf, "(& (vec (pr true false) (pr false false) (pr true true) (pr true false)))",  "(vec false false true false)"}
rev_rule! {and_tffftttt, "(& (vec (pr true false) (pr false false) (pr true true) (pr true true)))",  "(vec false false true true)"}
rev_rule! {and_tfftffff, "(& (vec (pr true false) (pr false true) (pr false false) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_tfftffft, "(& (vec (pr true false) (pr false true) (pr false false) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_tfftfftf, "(& (vec (pr true false) (pr false true) (pr false false) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_tfftfftt, "(& (vec (pr true false) (pr false true) (pr false false) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_tfftftff, "(& (vec (pr true false) (pr false true) (pr false true) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_tfftftft, "(& (vec (pr true false) (pr false true) (pr false true) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_tfftfttf, "(& (vec (pr true false) (pr false true) (pr false true) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_tfftfttt, "(& (vec (pr true false) (pr false true) (pr false true) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_tffttfff, "(& (vec (pr true false) (pr false true) (pr true false) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_tffttfft, "(& (vec (pr true false) (pr false true) (pr true false) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_tffttftf, "(& (vec (pr true false) (pr false true) (pr true false) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_tffttftt, "(& (vec (pr true false) (pr false true) (pr true false) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_tfftttff, "(& (vec (pr true false) (pr false true) (pr true true) (pr false false)))",  "(vec false false true false)"}
rev_rule! {and_tfftttft, "(& (vec (pr true false) (pr false true) (pr true true) (pr false true)))",  "(vec false false true false)"}
rev_rule! {and_tffttttf, "(& (vec (pr true false) (pr false true) (pr true true) (pr true false)))",  "(vec false false true false)"}
rev_rule! {and_tffttttt, "(& (vec (pr true false) (pr false true) (pr true true) (pr true true)))",  "(vec false false true true)"}
rev_rule! {and_tftfffff, "(& (vec (pr true false) (pr true false) (pr false false) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_tftfffft, "(& (vec (pr true false) (pr true false) (pr false false) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_tftffftf, "(& (vec (pr true false) (pr true false) (pr false false) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_tftffftt, "(& (vec (pr true false) (pr true false) (pr false false) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_tftfftff, "(& (vec (pr true false) (pr true false) (pr false true) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_tftfftft, "(& (vec (pr true false) (pr true false) (pr false true) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_tftffttf, "(& (vec (pr true false) (pr true false) (pr false true) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_tftffttt, "(& (vec (pr true false) (pr true false) (pr false true) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_tftftfff, "(& (vec (pr true false) (pr true false) (pr true false) (pr false false)))",  "(vec false false false false)"}
rev_rule! {and_tftftfft, "(& (vec (pr true false) (pr true false) (pr true false) (pr false true)))",  "(vec false false false false)"}
rev_rule! {and_tftftftf, "(& (vec (pr true false) (pr true false) (pr true false) (pr true false)))",  "(vec false false false false)"}
rev_rule! {and_tftftftt, "(& (vec (pr true false) (pr true false) (pr true false) (pr true true)))",  "(vec false false false true)"}
rev_rule! {and_tftfttff, "(& (vec (pr true false) (pr true false) (pr true true) (pr false false)))",  "(vec false false true false)"}
rev_rule! {and_tftfttft, "(& (vec (pr true false) (pr true false) (pr true true) (pr false true)))",  "(vec false false true false)"}
rev_rule! {and_tftftttf, "(& (vec (pr true false) (pr true false) (pr true true) (pr true false)))",  "(vec false false true false)"}
rev_rule! {and_tftftttt, "(& (vec (pr true false) (pr true false) (pr true true) (pr true true)))",  "(vec false false true true)"}
rev_rule! {and_tfttffff, "(& (vec (pr true false) (pr true true) (pr false false) (pr false false)))",  "(vec false true false false)"}
rev_rule! {and_tfttffft, "(& (vec (pr true false) (pr true true) (pr false false) (pr false true)))",  "(vec false true false false)"}
rev_rule! {and_tfttfftf, "(& (vec (pr true false) (pr true true) (pr false false) (pr true false)))",  "(vec false true false false)"}
rev_rule! {and_tfttfftt, "(& (vec (pr true false) (pr true true) (pr false false) (pr true true)))",  "(vec false true false true)"}
rev_rule! {and_tfttftff, "(& (vec (pr true false) (pr true true) (pr false true) (pr false false)))",  "(vec false true false false)"}
rev_rule! {and_tfttftft, "(& (vec (pr true false) (pr true true) (pr false true) (pr false true)))",  "(vec false true false false)"}
rev_rule! {and_tfttfttf, "(& (vec (pr true false) (pr true true) (pr false true) (pr true false)))",  "(vec false true false false)"}
rev_rule! {and_tfttfttt, "(& (vec (pr true false) (pr true true) (pr false true) (pr true true)))",  "(vec false true false true)"}
rev_rule! {and_tftttfff, "(& (vec (pr true false) (pr true true) (pr true false) (pr false false)))",  "(vec false true false false)"}
rev_rule! {and_tftttfft, "(& (vec (pr true false) (pr true true) (pr true false) (pr false true)))",  "(vec false true false false)"}
rev_rule! {and_tftttftf, "(& (vec (pr true false) (pr true true) (pr true false) (pr true false)))",  "(vec false true false false)"}
rev_rule! {and_tftttftt, "(& (vec (pr true false) (pr true true) (pr true false) (pr true true)))",  "(vec false true false true)"}
rev_rule! {and_tfttttff, "(& (vec (pr true false) (pr true true) (pr true true) (pr false false)))",  "(vec false true true false)"}
rev_rule! {and_tfttttft, "(& (vec (pr true false) (pr true true) (pr true true) (pr false true)))",  "(vec false true true false)"}
rev_rule! {and_tftttttf, "(& (vec (pr true false) (pr true true) (pr true true) (pr true false)))",  "(vec false true true false)"}
rev_rule! {and_tftttttt, "(& (vec (pr true false) (pr true true) (pr true true) (pr true true)))",  "(vec false true true true)"}
rev_rule! {and_ttffffff, "(& (vec (pr true true) (pr false false) (pr false false) (pr false false)))",  "(vec true false false false)"}
rev_rule! {and_ttffffft, "(& (vec (pr true true) (pr false false) (pr false false) (pr false true)))",  "(vec true false false false)"}
rev_rule! {and_ttfffftf, "(& (vec (pr true true) (pr false false) (pr false false) (pr true false)))",  "(vec true false false false)"}
rev_rule! {and_ttfffftt, "(& (vec (pr true true) (pr false false) (pr false false) (pr true true)))",  "(vec true false false true)"}
rev_rule! {and_ttffftff, "(& (vec (pr true true) (pr false false) (pr false true) (pr false false)))",  "(vec true false false false)"}
rev_rule! {and_ttffftft, "(& (vec (pr true true) (pr false false) (pr false true) (pr false true)))",  "(vec true false false false)"}
rev_rule! {and_ttfffttf, "(& (vec (pr true true) (pr false false) (pr false true) (pr true false)))",  "(vec true false false false)"}
rev_rule! {and_ttfffttt, "(& (vec (pr true true) (pr false false) (pr false true) (pr true true)))",  "(vec true false false true)"}
rev_rule! {and_ttfftfff, "(& (vec (pr true true) (pr false false) (pr true false) (pr false false)))",  "(vec true false false false)"}
rev_rule! {and_ttfftfft, "(& (vec (pr true true) (pr false false) (pr true false) (pr false true)))",  "(vec true false false false)"}
rev_rule! {and_ttfftftf, "(& (vec (pr true true) (pr false false) (pr true false) (pr true false)))",  "(vec true false false false)"}
rev_rule! {and_ttfftftt, "(& (vec (pr true true) (pr false false) (pr true false) (pr true true)))",  "(vec true false false true)"}
rev_rule! {and_ttffttff, "(& (vec (pr true true) (pr false false) (pr true true) (pr false false)))",  "(vec true false true false)"}
rev_rule! {and_ttffttft, "(& (vec (pr true true) (pr false false) (pr true true) (pr false true)))",  "(vec true false true false)"}
rev_rule! {and_ttfftttf, "(& (vec (pr true true) (pr false false) (pr true true) (pr true false)))",  "(vec true false true false)"}
rev_rule! {and_ttfftttt, "(& (vec (pr true true) (pr false false) (pr true true) (pr true true)))",  "(vec true false true true)"}
rev_rule! {and_ttftffff, "(& (vec (pr true true) (pr false true) (pr false false) (pr false false)))",  "(vec true false false false)"}
rev_rule! {and_ttftffft, "(& (vec (pr true true) (pr false true) (pr false false) (pr false true)))",  "(vec true false false false)"}
rev_rule! {and_ttftfftf, "(& (vec (pr true true) (pr false true) (pr false false) (pr true false)))",  "(vec true false false false)"}
rev_rule! {and_ttftfftt, "(& (vec (pr true true) (pr false true) (pr false false) (pr true true)))",  "(vec true false false true)"}
rev_rule! {and_ttftftff, "(& (vec (pr true true) (pr false true) (pr false true) (pr false false)))",  "(vec true false false false)"}
rev_rule! {and_ttftftft, "(& (vec (pr true true) (pr false true) (pr false true) (pr false true)))",  "(vec true false false false)"}
rev_rule! {and_ttftfttf, "(& (vec (pr true true) (pr false true) (pr false true) (pr true false)))",  "(vec true false false false)"}
rev_rule! {and_ttftfttt, "(& (vec (pr true true) (pr false true) (pr false true) (pr true true)))",  "(vec true false false true)"}
rev_rule! {and_ttfttfff, "(& (vec (pr true true) (pr false true) (pr true false) (pr false false)))",  "(vec true false false false)"}
rev_rule! {and_ttfttfft, "(& (vec (pr true true) (pr false true) (pr true false) (pr false true)))",  "(vec true false false false)"}
rev_rule! {and_ttfttftf, "(& (vec (pr true true) (pr false true) (pr true false) (pr true false)))",  "(vec true false false false)"}
rev_rule! {and_ttfttftt, "(& (vec (pr true true) (pr false true) (pr true false) (pr true true)))",  "(vec true false false true)"}
rev_rule! {and_ttftttff, "(& (vec (pr true true) (pr false true) (pr true true) (pr false false)))",  "(vec true false true false)"}
rev_rule! {and_ttftttft, "(& (vec (pr true true) (pr false true) (pr true true) (pr false true)))",  "(vec true false true false)"}
rev_rule! {and_ttfttttf, "(& (vec (pr true true) (pr false true) (pr true true) (pr true false)))",  "(vec true false true false)"}
rev_rule! {and_ttfttttt, "(& (vec (pr true true) (pr false true) (pr true true) (pr true true)))",  "(vec true false true true)"}
rev_rule! {and_tttfffff, "(& (vec (pr true true) (pr true false) (pr false false) (pr false false)))",  "(vec true false false false)"}
rev_rule! {and_tttfffft, "(& (vec (pr true true) (pr true false) (pr false false) (pr false true)))",  "(vec true false false false)"}
rev_rule! {and_tttffftf, "(& (vec (pr true true) (pr true false) (pr false false) (pr true false)))",  "(vec true false false false)"}
rev_rule! {and_tttffftt, "(& (vec (pr true true) (pr true false) (pr false false) (pr true true)))",  "(vec true false false true)"}
rev_rule! {and_tttfftff, "(& (vec (pr true true) (pr true false) (pr false true) (pr false false)))",  "(vec true false false false)"}
rev_rule! {and_tttfftft, "(& (vec (pr true true) (pr true false) (pr false true) (pr false true)))",  "(vec true false false false)"}
rev_rule! {and_tttffttf, "(& (vec (pr true true) (pr true false) (pr false true) (pr true false)))",  "(vec true false false false)"}
rev_rule! {and_tttffttt, "(& (vec (pr true true) (pr true false) (pr false true) (pr true true)))",  "(vec true false false true)"}
rev_rule! {and_tttftfff, "(& (vec (pr true true) (pr true false) (pr true false) (pr false false)))",  "(vec true false false false)"}
rev_rule! {and_tttftfft, "(& (vec (pr true true) (pr true false) (pr true false) (pr false true)))",  "(vec true false false false)"}
rev_rule! {and_tttftftf, "(& (vec (pr true true) (pr true false) (pr true false) (pr true false)))",  "(vec true false false false)"}
rev_rule! {and_tttftftt, "(& (vec (pr true true) (pr true false) (pr true false) (pr true true)))",  "(vec true false false true)"}
rev_rule! {and_tttfttff, "(& (vec (pr true true) (pr true false) (pr true true) (pr false false)))",  "(vec true false true false)"}
rev_rule! {and_tttfttft, "(& (vec (pr true true) (pr true false) (pr true true) (pr false true)))",  "(vec true false true false)"}
rev_rule! {and_tttftttf, "(& (vec (pr true true) (pr true false) (pr true true) (pr true false)))",  "(vec true false true false)"}
rev_rule! {and_tttftttt, "(& (vec (pr true true) (pr true false) (pr true true) (pr true true)))",  "(vec true false true true)"}
rev_rule! {and_ttttffff, "(& (vec (pr true true) (pr true true) (pr false false) (pr false false)))",  "(vec true true false false)"}
rev_rule! {and_ttttffft, "(& (vec (pr true true) (pr true true) (pr false false) (pr false true)))",  "(vec true true false false)"}
rev_rule! {and_ttttfftf, "(& (vec (pr true true) (pr true true) (pr false false) (pr true false)))",  "(vec true true false false)"}
rev_rule! {and_ttttfftt, "(& (vec (pr true true) (pr true true) (pr false false) (pr true true)))",  "(vec true true false true)"}
rev_rule! {and_ttttftff, "(& (vec (pr true true) (pr true true) (pr false true) (pr false false)))",  "(vec true true false false)"}
rev_rule! {and_ttttftft, "(& (vec (pr true true) (pr true true) (pr false true) (pr false true)))",  "(vec true true false false)"}
rev_rule! {and_ttttfttf, "(& (vec (pr true true) (pr true true) (pr false true) (pr true false)))",  "(vec true true false false)"}
rev_rule! {and_ttttfttt, "(& (vec (pr true true) (pr true true) (pr false true) (pr true true)))",  "(vec true true false true)"}
rev_rule! {and_tttttfff, "(& (vec (pr true true) (pr true true) (pr true false) (pr false false)))",  "(vec true true false false)"}
rev_rule! {and_tttttfft, "(& (vec (pr true true) (pr true true) (pr true false) (pr false true)))",  "(vec true true false false)"}
rev_rule! {and_tttttftf, "(& (vec (pr true true) (pr true true) (pr true false) (pr true false)))",  "(vec true true false false)"}
rev_rule! {and_tttttftt, "(& (vec (pr true true) (pr true true) (pr true false) (pr true true)))",  "(vec true true false true)"}
rev_rule! {and_ttttttff, "(& (vec (pr true true) (pr true true) (pr true true) (pr false false)))",  "(vec true true true false)"}
rev_rule! {and_ttttttft, "(& (vec (pr true true) (pr true true) (pr true true) (pr false true)))",  "(vec true true true false)"}
rev_rule! {and_tttttttf, "(& (vec (pr true true) (pr true true) (pr true true) (pr true false)))",  "(vec true true true false)"}
rev_rule! {and_tttttttt, "(& (vec (pr true true) (pr true true) (pr true true) (pr true true)))",  "(vec true true true true)"}

rule! {input, "(vec (pr false false) (pr false true) (pr true false) (pr true true))", "x"}

// (vec false true true false)

fn prove_something(name: &str, start: &str, rewrites: &[Rewrite], goals: &[&str]) {
  let _ = env_logger::builder().is_test(true).try_init();
  println!("Proving {}", name);

  let start_expr: RecExpr<_> = start.parse().unwrap();
  let goal_exprs: Vec<RecExpr<_>> = goals.iter().map(|g| g.parse().unwrap()).collect();

  let runner = Runner::default()
    .with_iter_limit(20)
    .with_node_limit(5_000)
    .with_expr(&start_expr)
    .run(rewrites);

  let (egraph, root) = (runner.egraph, runner.roots[0]);

  egraph.dot().to_dot("target/getcell.dot").unwrap();

  let mut extractor = Extractor::new(&egraph, CostFn);
  let (best_cost, best) = extractor.find_best(root);
  println!("best cost {}", best_cost);
  println!("best {}", best);

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
    not_ffff(),
    not_ffft(),
    not_fftf(),
    not_fftt(),
    not_ftff(),
    not_ftft(),
    not_fttf(),
    not_fttt(),
    not_tfff(),
    not_tfft(),
    not_tftf(),
    not_tftt(),
    not_ttff(),
    not_ttft(),
    not_tttf(),
    not_tttt(),
    and_ffffffff(),
    and_ffffffft(),
    and_fffffftf(),
    and_fffffftt(),
    and_ffffftff(),
    and_ffffftft(),
    and_fffffttf(),
    and_fffffttt(),
    and_fffftfff(),
    and_fffftfft(),
    and_fffftftf(),
    and_fffftftt(),
    and_ffffttff(),
    and_ffffttft(),
    and_fffftttf(),
    and_fffftttt(),
    and_ffftffff(),
    and_ffftffft(),
    and_ffftfftf(),
    and_ffftfftt(),
    and_ffftftff(),
    and_ffftftft(),
    and_ffftfttf(),
    and_ffftfttt(),
    and_fffttfff(),
    and_fffttfft(),
    and_fffttftf(),
    and_fffttftt(),
    and_ffftttff(),
    and_ffftttft(),
    and_fffttttf(),
    and_fffttttt(),
    and_fftfffff(),
    and_fftfffft(),
    and_fftffftf(),
    and_fftffftt(),
    and_fftfftff(),
    and_fftfftft(),
    and_fftffttf(),
    and_fftffttt(),
    and_fftftfff(),
    and_fftftfft(),
    and_fftftftf(),
    and_fftftftt(),
    and_fftfttff(),
    and_fftfttft(),
    and_fftftttf(),
    and_fftftttt(),
    and_ffttffff(),
    and_ffttffft(),
    and_ffttfftf(),
    and_ffttfftt(),
    and_ffttftff(),
    and_ffttftft(),
    and_ffttfttf(),
    and_ffttfttt(),
    and_fftttfff(),
    and_fftttfft(),
    and_fftttftf(),
    and_fftttftt(),
    and_ffttttff(),
    and_ffttttft(),
    and_fftttttf(),
    and_fftttttt(),
    and_ftffffff(),
    and_ftffffft(),
    and_ftfffftf(),
    and_ftfffftt(),
    and_ftffftff(),
    and_ftffftft(),
    and_ftfffttf(),
    and_ftfffttt(),
    and_ftfftfff(),
    and_ftfftfft(),
    and_ftfftftf(),
    and_ftfftftt(),
    and_ftffttff(),
    and_ftffttft(),
    and_ftfftttf(),
    and_ftfftttt(),
    and_ftftffff(),
    and_ftftffft(),
    and_ftftfftf(),
    and_ftftfftt(),
    and_ftftftff(),
    and_ftftftft(),
    and_ftftfttf(),
    and_ftftfttt(),
    and_ftfttfff(),
    and_ftfttfft(),
    and_ftfttftf(),
    and_ftfttftt(),
    and_ftftttff(),
    and_ftftttft(),
    and_ftfttttf(),
    and_ftfttttt(),
    and_fttfffff(),
    and_fttfffft(),
    and_fttffftf(),
    and_fttffftt(),
    and_fttfftff(),
    and_fttfftft(),
    and_fttffttf(),
    and_fttffttt(),
    and_fttftfff(),
    and_fttftfft(),
    and_fttftftf(),
    and_fttftftt(),
    and_fttfttff(),
    and_fttfttft(),
    and_fttftttf(),
    and_fttftttt(),
    and_ftttffff(),
    and_ftttffft(),
    and_ftttfftf(),
    and_ftttfftt(),
    and_ftttftff(),
    and_ftttftft(),
    and_ftttfttf(),
    and_ftttfttt(),
    and_fttttfff(),
    and_fttttfft(),
    and_fttttftf(),
    and_fttttftt(),
    and_ftttttff(),
    and_ftttttft(),
    and_fttttttf(),
    and_fttttttt(),
    and_tfffffff(),
    and_tfffffft(),
    and_tffffftf(),
    and_tffffftt(),
    and_tfffftff(),
    and_tfffftft(),
    and_tffffttf(),
    and_tffffttt(),
    and_tffftfff(),
    and_tffftfft(),
    and_tffftftf(),
    and_tffftftt(),
    and_tfffttff(),
    and_tfffttft(),
    and_tffftttf(),
    and_tffftttt(),
    and_tfftffff(),
    and_tfftffft(),
    and_tfftfftf(),
    and_tfftfftt(),
    and_tfftftff(),
    and_tfftftft(),
    and_tfftfttf(),
    and_tfftfttt(),
    and_tffttfff(),
    and_tffttfft(),
    and_tffttftf(),
    and_tffttftt(),
    and_tfftttff(),
    and_tfftttft(),
    and_tffttttf(),
    and_tffttttt(),
    and_tftfffff(),
    and_tftfffft(),
    and_tftffftf(),
    and_tftffftt(),
    and_tftfftff(),
    and_tftfftft(),
    and_tftffttf(),
    and_tftffttt(),
    and_tftftfff(),
    and_tftftfft(),
    and_tftftftf(),
    and_tftftftt(),
    and_tftfttff(),
    and_tftfttft(),
    and_tftftttf(),
    and_tftftttt(),
    and_tfttffff(),
    and_tfttffft(),
    and_tfttfftf(),
    and_tfttfftt(),
    and_tfttftff(),
    and_tfttftft(),
    and_tfttfttf(),
    and_tfttfttt(),
    and_tftttfff(),
    and_tftttfft(),
    and_tftttftf(),
    and_tftttftt(),
    and_tfttttff(),
    and_tfttttft(),
    and_tftttttf(),
    and_tftttttt(),
    and_ttffffff(),
    and_ttffffft(),
    and_ttfffftf(),
    and_ttfffftt(),
    and_ttffftff(),
    and_ttffftft(),
    and_ttfffttf(),
    and_ttfffttt(),
    and_ttfftfff(),
    and_ttfftfft(),
    and_ttfftftf(),
    and_ttfftftt(),
    and_ttffttff(),
    and_ttffttft(),
    and_ttfftttf(),
    and_ttfftttt(),
    and_ttftffff(),
    and_ttftffft(),
    and_ttftfftf(),
    and_ttftfftt(),
    and_ttftftff(),
    and_ttftftft(),
    and_ttftfttf(),
    and_ttftfttt(),
    and_ttfttfff(),
    and_ttfttfft(),
    and_ttfttftf(),
    and_ttfttftt(),
    and_ttftttff(),
    and_ttftttft(),
    and_ttfttttf(),
    and_ttfttttt(),
    and_tttfffff(),
    and_tttfffft(),
    and_tttffftf(),
    and_tttffftt(),
    and_tttfftff(),
    and_tttfftft(),
    and_tttffttf(),
    and_tttffttt(),
    and_tttftfff(),
    and_tttftfft(),
    and_tttftftf(),
    and_tttftftt(),
    and_tttfttff(),
    and_tttfttft(),
    and_tttftttf(),
    and_tttftttt(),
    and_ttttffff(),
    and_ttttffft(),
    and_ttttfftf(),
    and_ttttfftt(),
    and_ttttftff(),
    and_ttttftft(),
    and_ttttfttf(),
    and_ttttfttt(),
    and_tttttfff(),
    and_tttttfft(),
    and_tttttftf(),
    and_tttttftt(),
    and_ttttttff(),
    and_ttttttft(),
    and_tttttttf(),
    and_tttttttt(),
    input(),
  ];
  prove_something("simple", "(vec false false false true)", rules, &[]);
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
