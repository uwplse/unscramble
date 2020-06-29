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

// type Transition = (Vec<Id>, Id);
type Transition<L: Language> = (L, Id);
type ProdTransition<L: Language> = (Transition<L>, Transition<L>);
type ProdWorklist<L: Language> = HashMap<String, Vec<ProdTransition<L>>>;

fn enode_hash<L: Language>(enode: &L) -> String {
  format!("{}_{}", enode.display_op(), enode.len())
}

fn enode_map<L: Language, N: Analysis<L>>(g: &EGraph<L, N>) -> HashMap<String, Vec<Transition<L>>> {
  let mut map: HashMap<String, Vec<Transition<L>>> = HashMap::new();
  for class in g.classes() {
    for node in class.iter() {
      let hash = enode_hash(node);
      let vals = map.entry(hash).or_insert(vec![]);
      // vals.push((node.children().to_vec(), class.id));
      vals.push((node.clone(), class.id));
    }
  }
  map
}

pub fn intersect<L: Language, N: Analysis<L>>(a: &EGraph<L, N>, b: &EGraph<L, N>) -> EGraph<L, ()> {
  let map_a = enode_map(a);
  println!("A map:");
  for (key, value) in &map_a {
    println!("  {}: {:?}", key, value);
  }
  let map_b = enode_map(b);
  println!("B map:");
  for (key, value) in &map_b {
    println!("  {}: {:?}", key, value);
  }

  // construct ProdWorklist
  let mut worklist: ProdWorklist<L> = ProdWorklist::new();
  for (key, value_a) in &map_a {
    if let Some(value_b) = map_b.get(key) {
      for (t_a, t_b) in itertools::iproduct!(value_a.iter(), value_b.iter()) {
        let vals = worklist.entry(key.clone()).or_insert(vec![]);
        vals.push((t_a.clone(), t_b.clone()));
      }
    }
  }

  println!("generated worklist: {:?}", worklist);

  let mut intersection: EGraph<L, ()> = EGraph::new(()); /* TODO: transfer analysis */
  let mut prod_ids: HashMap<(Id, Id), Id> = HashMap::new();
  let mut did_something = true;

  let mut iteration = 0;
  while did_something {
    did_something = false;
    for (key, value) in &mut worklist {
      let mut finished_idxs = vec![];
      for (idx, ((en1, parent_ec1), (en2, parent_ec2))) in value.iter().enumerate() {
        let children_inhabited = en1
          .children()
          .iter()
          .zip(en2.children())
          .all(|(c1, c2)| prod_ids.contains_key(&(*c1, *c2)));
        if children_inhabited {
          println!("map: {:?}", prod_ids);
          /* add to new egraph */
          let mut new_children: Vec<Id> = en1
            .children()
            .iter()
            .zip(en2.children())
            .map(|(c1, c2)| *prod_ids.get(&(*c1, *c2)).unwrap())
            .collect();
          // for (c1, c2) in en1.children().iter().zip(en2.children()) {
          //   new_children.insert(c1, prod_ids[&(*c1, *c2)]);
          // }
          // let new_en1 = en1.clone().map_children(|c1| new_children[&c1]);
          let mut new_en1 = en1.clone();
          for (i, c) in new_en1.children_mut().iter_mut().enumerate() {
            *c = new_children[i];
          }
          println!("Adding node: {:?}", &new_en1);
          println!("  old -> new: {:?}", new_children);
          let prod_parent = intersection.add(new_en1);
          for (_, idv) in prod_ids.iter_mut() {
            *idv = intersection.find(*idv);
          }
          println!(
            "  from: {:?}[{}] {:?}[{}]",
            &en1, parent_ec1, &en2, parent_ec2
          );
          println!("  new parent: {}", &prod_parent);
          did_something = true;
          finished_idxs.push(idx);
          let parent_inhabited = prod_ids.get(&(*parent_ec1, *parent_ec2));
          if let Some(intersected_parent) = parent_inhabited {
            println!("Merging {} and {}", prod_parent, intersected_parent);
            let (new_parent, _) = intersection.union(prod_parent, *intersected_parent);
            // go through prod_ids map and recanonicalize
            for (_, idv) in prod_ids.iter_mut() {
              *idv = intersection.find(*idv);
            }
            prod_ids.insert((*parent_ec1, *parent_ec2), new_parent);
            println!("  to {}", new_parent);
          } else {
            prod_ids.insert((*parent_ec1, *parent_ec2), prod_parent);
          }
        }
        /* intersection
        .dot()
        .to_dot(format!(
          "tests/intersect/{}-{}-{}-intersect.dot",
          iteration, key, idx
        ))
        .unwrap(); */
      }
      while !finished_idxs.is_empty() {
        value.remove(finished_idxs.pop().unwrap());
      }
    }
    iteration += 1;
  }
  intersection
}

// for key_a in map_a.keys() {
//   if map_b.contains_key(key_a) {}
// }
