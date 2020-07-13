/*

1. Construct mapping from cross product of ids to new ids in intersection. (id1, id2) -> id3
2. Construct a vector of vectors of enodes, with identical enode operators (using L.matches())???
3. Construct cross products of each [vector of enodes with identical ops] in the vector. Use mapping from step 1 to construct new child ids and destination eclass

[*(q2,q3)->q1, *(q3,q2)->q1] in egraph1: *_2 -> [([q2, q3], q1), ([q3, q2], q1)]

[*(r2,r3)->r1, *(r3,r2)->r1] in egraph2: *_2 -> [([r2, r3], r1), ([r3,r2], r1)]

intersection: *_2 -> [([q2r2, q3r3], q1r1)]

[
  *(q2,q3)->q1 x *(r2,r3)->r1 -> *(q2r2, q3r3)->q1r1
]

[defer] cycles might be bad since we need to do bottom up

*/
use egg::*;
use itertools::iproduct;
use std::collections::HashMap;
use std::string::String;

pub type RootedEGraph<L: Language, N: Analysis<L>> = (EGraph<L, N>, Vec<Id>);

type Transition<L: Language> = (L, Id);
type ProdTransition<L: Language> = (Transition<L>, Transition<L>);
type ProdWorklist<L: Language> = HashMap<String, Vec<ProdTransition<L>>>;

// TODO: figure out a portable way of doing this
fn enode_hash<L: Language>(enode: &L) -> String {
  format!("{}_{}", enode.display_op(), enode.len())
}

fn enode_map<L: Language, N: Analysis<L>>(g: &EGraph<L, N>) -> HashMap<String, Vec<Transition<L>>> {
  let mut map: HashMap<String, Vec<Transition<L>>> = HashMap::new();
  for class in g.classes() {
    for node in class.iter() {
      let hash = enode_hash(node);
      let vals = map.entry(hash).or_insert(vec![]);
      vals.push((node.clone(), class.id));
    }
  }
  map
}

pub fn intersect<L: Language, Na: Analysis<L>, Nb: Analysis<L>>(
  (a, a_roots): &RootedEGraph<L, Na>,
  (b, b_roots): &RootedEGraph<L, Nb>,
) -> RootedEGraph<L, ()> {
  let map_a = enode_map(a);
  // println!("A map:");
  // for (key, value) in &map_a {
  //   println!("  {}: {:?}", key, value);
  // }
  let map_b = enode_map(b);
  // println!("B map:");
  // for (key, value) in &map_b {
  //   println!("  {}: {:?}", key, value);
  // }

  // TODO: lazy bottom-up construction to avoid enumerating spurious states
  // construct worklist
  let mut worklist: ProdWorklist<L> = ProdWorklist::new();
  for (key, value_a) in &map_a {
    if let Some(value_b) = map_b.get(key) {
      for (t_a, t_b) in itertools::iproduct!(value_a.iter(), value_b.iter()) {
        let vals = worklist.entry(key.clone()).or_insert(vec![]);
        vals.push((t_a.clone(), t_b.clone()));
      }
    }
  }

  // println!("generated worklist: {:?}", worklist);

  let mut intersection: EGraph<L, ()> = EGraph::new(()); /* TODO: transfer analysis */
  let mut intersection_roots: Vec<Id> = vec![];

  let mut prod_ids: HashMap<(Id, Id), Id> = HashMap::new();
  let mut did_something = true;

  while did_something {
    did_something = false;
    for (_, value) in &mut worklist {
      let mut finished_idxs = vec![];
      for (idx, ((en1, parent_ec1), (en2, parent_ec2))) in value.iter().enumerate() {
        let children_inhabited = en1
          .children()
          .iter()
          .zip(en2.children())
          .all(|(c1, c2)| prod_ids.contains_key(&(*c1, *c2)));
        if children_inhabited {
          // println!("map: {:?}", prod_ids);
          /* add to new egraph */
          let new_children: Vec<Id> = en1
            .children()
            .iter()
            .zip(en2.children())
            .map(|(c1, c2)| *prod_ids.get(&(*c1, *c2)).unwrap())
            .collect();
          let mut new_en1 = en1.clone();
          for (i, c) in new_en1.children_mut().iter_mut().enumerate() {
            *c = new_children[i];
          }
          // println!("Adding node: {:?}", &new_en1);
          // println!("  old -> new: {:?}", new_children);
          let prod_parent = intersection.add(new_en1);
          // println!(
          //   "  from: {:?}[{}] {:?}[{}]",
          //   &en1, parent_ec1, &en2, parent_ec2
          // );
          // println!("  new parent: {}", &prod_parent);
          did_something = true;
          finished_idxs.push(idx);
          let parent_inhabited = prod_ids.get(&(*parent_ec1, *parent_ec2));
          if let Some(intersected_parent) = parent_inhabited {
            // println!("Merging {} and {}", prod_parent, intersected_parent);
            let (new_parent, _) = intersection.union(prod_parent, *intersected_parent);
            prod_ids.insert((*parent_ec1, *parent_ec2), new_parent);
          // println!("  to {}", new_parent);
          } else {
            prod_ids.insert((*parent_ec1, *parent_ec2), prod_parent);
          }
        }
      }
      while !finished_idxs.is_empty() {
        value.remove(finished_idxs.pop().unwrap());
      }
    }
  }

  // TODO: this can probably be done in the loop above/lazily
  // identify new roots
  for (r_a, r_b) in itertools::iproduct!(a_roots.iter(), b_roots.iter()) {
    if let Some(r_i) = prod_ids.get(&(*r_a, *r_b)) {
      intersection_roots.push(*r_i);
    }
  }

  (intersection, intersection_roots)
}

/*

Let (C, R_C) = (A, R_A) \cap (B, R_B).
If R_C = {}, then the intersection L(A) \cap L(B) is empty, where L(X) is a
language of finite-length conditional-free programs consistent with some
input-output constraint.

  TODO: formalize, generally if possible, the flow of IO constraints

To recover from this, we define a set of  "introductions" of the form

  ?x -> f(?1, ..., ?n), where any ?i can be ?x and ?x need not be a wildcard

Supposing R_C = {}, and assuming for now that |R_A| = |R_B| = 1,
let r_A and r_B be the root eclasses of A and B respectively.

  TODO: I'm currently proceeding by extracting programs from A and B,
        and adding them to a new egraph C' (under the assumption that their
        roots will be disjoint).

        A potentially better approach, which I think would decrease the time
        needed to (re)saturate, is to literally union the egraphs hypergraph
        style and then rebuild. Probably need @mwillsey to see if this makes sense.

We extract two RecExprs p_A and p_B from r_A and r_B respectively, and add them
to a new (initially empty) egraph C'. Let r_A' and r_B' be the new eclasses for
these exprs respectively. We also add two new symbol exprs !A and !B into r_A'
and r_B' respectively.

We now perform a "concretization" of the introductions defined previously.
For each introduction ?x -> f(?1,...,?n), we instantiate rewrite rules

  !x -> f(?1,...,?n) for ?1,...,?n \in {!A, !B}, where the wildcard relations
                     between the LHS and RHS are preserved.

These rules are then supplied together with the original non-input rewrite rules
to saturate C', and we extract the final program from

*/
