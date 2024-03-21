fn main() {
    let mut arr = vec![5, 3, 7, 2, 8, 4, 6, 1];
    reverse_around_middle(&mut arr);
    
    sort_with_reverse_middle(&mut arr);
    println!("{:?}", arr);
}

fn reverse_around_middle(arr: &mut Vec<i32>) {
    let middle = arr.len() / 2;
    for i in 0..middle / 2 {
        arr.swap(i, middle - 1 - i);
    }
}

fn sort_with_reverse_middle(arr: &mut Vec<i32>) {
    let middle = arr.len() / 2;
    arr.sort_by(|a, b| {
        if a < b {
            return std::cmp::Ordering::Less;
        } else if a > b {
            return std::cmp::Ordering::Greater;
        } else {
            if *a == arr[middle] {
                return std::cmp::Ordering::Equal;
            } else if *b == arr[middle] {
                return std::cmp::Ordering::Equal;
            } else if *a < arr[middle] {
                return std::cmp::Ordering::Less;
            } else {
                return std::cmp::Ordering::Greater;
            }
        }
    });
}
