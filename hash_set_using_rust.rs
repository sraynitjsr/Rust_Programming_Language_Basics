use std::collections::HashSet;

fn main() {
    let mut hash_set: HashSet<i32> = HashSet::new();

    hash_set.insert(1);
    hash_set.insert(2);
    hash_set.insert(3);

    println!("HashSet contains 2: {}", hash_set.contains(&2));
    println!("HashSet contains 4: {}", hash_set.contains(&4));

    hash_set.remove(&3);

    println!("HashSet elements:");
    for num in &hash_set {
        println!("{}", num);
    }

    hash_set.clear();

    println!("HashSet is empty: {}", hash_set.is_empty());

    let numbers = vec![1, 2, 3, 4, 5];
    let hash_set_from_vec: HashSet<i32> = numbers.iter().cloned().collect();
    println!("HashSet from vector: {:?}", hash_set_from_vec);

    let set1: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let set2: HashSet<i32> = [3, 4, 5].iter().cloned().collect();

    let union: HashSet<_> = set1.union(&set2).cloned().collect();
    let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
    let difference: HashSet<_> = set1.difference(&set2).cloned().collect();

    println!("Union: {:?}", union);
    println!("Intersection: {:?}", intersection);
    println!("Difference: {:?}", difference);
}
