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
}
