fn min_cost_to_reduce_to_one(mut arr: Vec<i32>) -> i32 {
    let mut total_cost = 0;
    arr.sort();
    while arr.len() > 1 {
        let cost = arr[0] + arr[1];
        total_cost += cost;
        arr[1] = cost;
        arr.remove(0);
    }
    total_cost
}

fn main() {
    let arr = vec![4, 3, 2, 6];
    let min_cost = min_cost_to_reduce_to_one(arr);
    println!("Minimum cost: {}", min_cost);
}
