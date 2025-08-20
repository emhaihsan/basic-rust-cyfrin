#![allow(unused)]
use std::collections::HashMap;

fn main() {
    // Example 1: map + collect into Vec<u32> (borrowed iteration with iter())
    let vals: Vec<u32> = vec![1, 2, 3, 4, 5];
    // iter() yields &u32; closure takes &u32 and returns u32
    let v2: Vec<u32> = vals.iter().map(|x: &u32| *x + 1).collect();
    println!("map + collect (Vec): {:?}", v2); // [2, 3, 4, 5, 6]

    // Example 2: filter then map with iter()
    let v_filtered_mapped: Vec<u32> = vals
        .iter() // yields &u32
        .filter(|x: &&u32| **x <= 3) // predicate sees &&u32
        .map(|x: &u32| *x + 1) // back to &u32
        .collect();
    println!("filter -> map (iter): {:?}", v_filtered_mapped); // [2, 3, 4]

    // Example 3: into_iter() variant (moves values), then filter/map/collect
    let vals_owned: Vec<u32> = vec![1, 2, 3, 4, 5];
    let v_into_filtered_mapped: Vec<u32> = vals_owned
        .into_iter() // yields u32 (owned)
        .filter(|x: &u32| *x <= 3) // predicate takes &u32 to avoid moving x
        .map(|x: u32| x + 1) // now we own x (u32)
        .collect();
    println!("filter -> map (into_iter): {:?}", v_into_filtered_mapped); // [2, 3, 4]

    // Example 4: collect into HashMap<String, u32>
    let kvs: Vec<(&str, u32)> = vec![("a", 1), ("b", 2), ("c", 3)];
    let v_vec: Vec<(String, u32)> = kvs
        .iter() // &(&str, u32)
        .map(|v| (v.0.to_string(), v.1 + 1))
        .collect();
    println!("collect Vec<(String, u32)>: {:?}", v_vec);

    let v_map: HashMap<String, u32> = kvs.iter().map(|v| (v.0.to_string(), v.1 + 1)).collect(); // type of target inferred from annotation
    println!("collect HashMap<String, u32>: {:?}", v_map);

    // Tip: You can also guide collect with turbofish
    let _v3 = vals.iter().map(|x| x + 1).collect::<Vec<u32>>();
}
