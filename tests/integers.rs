use egg::*;
use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand_pcg::Pcg64;
use std::collections::hash_map::HashMap;
use unscramble::*;

type EGraph = egg::EGraph<Math, MathInterpreter>;

type Constant = i64;

define_language! {
    enum Math {
        "+" = Add([Id; 2]),
        "*" = Mul([Id; 2]),
        Num(Constant),
        Var(egg::Symbol),
    }
}

#[derive(Default, Clone)]
struct MathInterpreter {
    env: HashMap<egg::Symbol, Constant>,
}

impl Analysis<Math> for MathInterpreter {
    type Data = Option<Constant>;

    fn merge(&self, to: &mut Self::Data, from: Self::Data) -> bool {
        egg::merge_if_different(to, to.or(from))
    }

    fn make(egraph: &egg::EGraph<Math, Self>, enode: &Math) -> Self::Data {
        let data_of = |i: &Id| egraph[*i].data;
        match enode {
            Math::Num(n) => Some(*n),
            Math::Var(sym) => Some(*egraph.analysis.env.get(sym)?),
            Math::Add([a, b]) => Some(data_of(a)?.wrapping_add(data_of(b)?)),
            Math::Mul([a, b]) => Some(data_of(a)?.wrapping_mul(data_of(b)?)),
            _ => None,
        }
    }

    fn modify(egraph: &mut egg::EGraph<Math, Self>, id: Id) {
        if let Some(i) = egraph[id].data {
            let added = egraph.add(Math::Num(i));
            egraph.union(id, added);
        }
    }
}

struct MathSynth {
    rng: Pcg64,
    variables: Vec<egg::Symbol>,
    constants: Vec<Constant>,
}

struct SynthesisParams {
    iters: usize,
    per_iter: usize,
    env: HashMap<egg::Symbol, Constant>,
}

fn make_node(synth: &mut MathSynth, egraph: &EGraph, choices: Option<&Vec<Id>>) -> Math {
    let classes: Vec<_> = egraph.classes().collect();
    macro_rules! mk {
        () => {
            if synth.rng.gen_bool(0.5) {
                synth.rng.gen_range(0, synth.variables.len() as Id)
            } else if let Some(choices) = choices {
                *choices.choose(&mut synth.rng).unwrap()
            } else {
                classes.choose(&mut synth.rng).unwrap().id
            }
        };
    }
    let p: f32 = synth.rng.gen();
    match p {
        _ if p < 0.5 => Math::Add([mk!(), mk!()]),
        _ => Math::Mul([mk!(), mk!()]),
    }
}

fn initial_egraph(synth: &mut MathSynth, params: &SynthesisParams) -> EGraph {
    let mut egraph = EGraph::new(MathInterpreter {
        env: params.env.clone(),
    });

    for sym in &synth.variables {
        egraph.add(Math::Var(*sym));
    }

    for n in &synth.constants {
        egraph.add(Math::Num(*n));
    }

    egraph
}

fn random_egraph(synth: &mut MathSynth, params: &SynthesisParams) -> EGraph {
    let mut egraph = initial_egraph(synth, params);

    for it in 0..params.iters {
        let ids = egraph.classes().map(|c| c.id).collect();
        for _ in 0..params.per_iter {
            egraph.add(make_node(synth, &egraph, Some(&ids)));
        }
    }
    egraph.rebuild(); // TODO: is this necessary?

    egraph
}

fn extract_rewrites<N: Analysis<Math>>(
    egraph: &egg::EGraph<Math, N>,
) -> Vec<(usize, (RecExpr<Math>, RecExpr<Math>))> {
    let mut extractor = Extractor::new(&egraph, AstSize);
    let mut rewrites = vec![];

    for class in egraph.classes() {
        let mut exprs = vec![];
        for enode in class.iter() {
            let mut expr = RecExpr::default();
            let mut cost = 0;
            let mut id_map: HashMap<Id, Id> = HashMap::new();
            for child_id in enode.children() {
                let (child_cost, child_rexpr) = extractor.find_best(*child_id);
                cost += child_cost;
                for child_enode in child_rexpr.as_ref() {
                    let new_child_id = expr.add(child_enode.clone());
                    id_map.insert(*child_id, new_child_id);
                }
            }
            let mut new_enode = enode.clone();
            new_enode.update_children(|old_id| *id_map.get(&old_id).unwrap());
            expr.add(new_enode);
            cost += 1;
            exprs.push((cost, expr));
        }

        exprs.sort_by_key(|x| x.0);

        for i in 0..exprs.len() {
            for j in i + 1..exprs.len() {
                let sum_cost = exprs[i].0 + exprs[j].0;
                rewrites.push((sum_cost, (exprs[i].1.clone(), exprs[j].1.clone())));
            }
        }
    }

    rewrites.sort_by_key(|x| x.0);
    rewrites
}

#[test]
fn math_interp() {
    let rules = &[
        // rewrite!("commute-add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
        // rewrite!("commute-mul"; "(* ?a ?b)" => "(* ?b ?a)"),
        // rewrite!("add-0"; "(+ ?a 0)" => "?a"),
        // rewrite!("mul-0"; "(* ?a 0)" => "0"),
        // rewrite!("mul-1"; "(* ?a 1)" => "?a"),
    ];
    let expr1 = "(* x y)".parse().unwrap();
    let expr2 = "(* y x)".parse().unwrap();
    let interp = MathInterpreter {
        env: vec![(egg::Symbol::from("x"), 3), (egg::Symbol::from("y"), 1)]
            .into_iter()
            .collect(),
    };
    let mut runner: Runner<Math, MathInterpreter, ()> = Runner::new(interp)
        .with_expr(&expr1)
        .with_expr(&expr2)
        .run(rules);
    println!("{:?}", runner.egraph.dump());
}

#[test]
fn math_rand() {
    let mut synth = MathSynth {
        rng: Pcg64::seed_from_u64(0),
        variables: vec![
            egg::Symbol::from("x"),
            egg::Symbol::from("y"),
            egg::Symbol::from("z"),
        ],
        constants: vec![0, 1],
    };

    let params = SynthesisParams {
        iters: 4,
        per_iter: 100,
        env: vec![
            (egg::Symbol::from("x"), 3),
            (egg::Symbol::from("y"), 1),
            (egg::Symbol::from("z"), 5),
        ]
        .into_iter()
        .collect(),
    };

    let rand_egg = random_egraph(&mut synth, &params);
    rand_egg.dot().to_dot("target/rand_egg.dot");

    println!("{:?}", rand_egg.dump());
}

#[test]
fn math_synth() {
    let mut synth = MathSynth {
        rng: Pcg64::seed_from_u64(0),
        variables: vec![
            egg::Symbol::from("x"),
            egg::Symbol::from("y"),
            egg::Symbol::from("z"),
        ],
        constants: vec![0, 1],
    };
    let samples = 10;
    let iters = 4;
    let per_iter = 100;
    let rng_low = -30;
    let rng_high = 30;

    let mut eggo: Option<egg::EGraph<Math, ()>> = None;
    let mut eggs = vec![];

    for _ in 0..samples {
        let params = SynthesisParams {
            iters: iters,
            per_iter: per_iter,
            env: vec![
                (
                    egg::Symbol::from("x"),
                    synth.rng.gen_range(rng_low, rng_high),
                ),
                (
                    egg::Symbol::from("y"),
                    synth.rng.gen_range(rng_low, rng_high),
                ),
                (
                    egg::Symbol::from("z"),
                    synth.rng.gen_range(rng_low, rng_high),
                ),
            ]
            .into_iter()
            .collect(),
        };
        eggs.push(random_egraph(&mut synth, &params));
    }

    for i in 1..eggs.len() {
        if let None = eggo {
            // TODO: fix typedef for RootedEGraph
            eggo = Some(intersect(&(eggs[i - 1].clone(), vec![]), &(eggs[i].clone(), vec![])).0);
        } else if let Some(leggo) = eggo {
            eggo = Some(intersect(&(leggo.clone(), vec![]), &(eggs[i].clone(), vec![])).0);
        }
    }

    println!("extracted rewrites:");
    for (cost, (lhs, rhs)) in extract_rewrites(&eggo.unwrap()) {
        println!("[{}] {} <=> {}", cost, lhs.pretty(80), rhs.pretty(80));
    }

    // let params1 = SynthesisParams {
    //     iters: 4,
    //     per_iter: 60,
    //     env: vec![
    //         (egg::Symbol::from("x"), 2),
    //         (egg::Symbol::from("y"), 3),
    //         (egg::Symbol::from("z"), 4),
    //     ]
    //     .into_iter()
    //     .collect(),
    // };
    // let params2 = SynthesisParams {
    //     iters: 4,
    //     per_iter: 60,
    //     env: vec![
    //         (egg::Symbol::from("x"), -1),
    //         (egg::Symbol::from("y"), 6),
    //         (egg::Symbol::from("z"), 7),
    //     ]
    //     .into_iter()
    //     .collect(),
    // };

    // let rand_egg1 = random_egraph(&mut synth, &params1);
    // let rand_egg2 = random_egraph(&mut synth, &params2);
    // rand_egg1.dot().to_dot("target/rand_egg1.dot");
    // rand_egg2.dot().to_dot("target/rand_egg2.dot");

    // let r1 = rand_egg1.total_number_of_nodes();
    // let r2 = rand_egg2.total_number_of_nodes();

    // let (eggo, _) = intersect(&(rand_egg1, vec![]), &(rand_egg2, vec![]));

    // eggo.dot().to_dot("target/eggo.dot");
    // println!("{:?}", eggo.dump());
    // println!(
    //     "size {} intersect size {} = size {}",
    //     r1,
    //     r2,
    //     eggo.total_number_of_nodes()
    // );

    // println!("extracted rewrites:");
    // for (cost, (lhs, rhs)) in extract_rewrites(&eggo) {
    //     println!("[{}] {} ~> {}", cost, lhs.pretty(80), rhs.pretty(80));
    // }
}
