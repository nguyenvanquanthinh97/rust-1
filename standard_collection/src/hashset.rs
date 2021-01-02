use std::collections::HashSet;

// hashset only contain uniqe values ( like Set in JS )
pub fn hashset() {
  let mut greeks = HashSet::new();
  greeks.insert("alpha");
  greeks.insert("beta");

  println!("{:?}", greeks);

  // This would insert fail due to beta has already existed in greeks
  greeks.insert("beta");
  println!("{:?}", greeks);

  let added_vega = greeks.insert("vega");
  if added_vega {
    println!("we successfully added vega!");
  }

  if !greeks.contains("kappa") {
    println!("we don't have kappa");
  }

  let _1_5:HashSet<_> = (1..=5).collect();
  let _6_10:HashSet<_> = (6..=10).collect();
  let _1_10:HashSet<_> = (1..=10).collect();
  let _2_8:HashSet<_> = (2..=8).collect();

  // subset
  println!(
    "is {:?} a subset of {:?} ? {}",
    _1_5,
    _1_10,
    _1_5.is_subset(&_1_10)
  );

  // disjoint = no common elements
  println!(
    "is {:?} a disjoint of {:?} ? {}",
    _1_5,
    _6_10,
    _1_5.is_disjoint(&_6_10)
  );

  // union, intersection
  println!(
    "items in either {:?} and {:?} are {:?}",
    _1_5,
    _2_8,
    _1_5.union(&_2_8)
  );

  println!(
    "items in both {:?} and {:?} are {:?}",
    _1_5,
    _2_8,
    _1_5.intersection(&_2_8)
  );
  
  // difference
  println!(
    "items in {:?} but not in {:?} are {:?}",
    _1_5,
    _2_8,
    _1_5.difference(&_2_8)
  );

  // symmetric differece = union - difference
  println!(
    "items in either {:?}, {:?} but not intersection {:?} are {:?}",
    _6_10,
    _2_8,
    _6_10.intersection(&_2_8),
    _6_10.symmetric_difference(&_2_8)
  );
}
