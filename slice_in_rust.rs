fn main() {
    let s = "Hello World From RUST";
    let slice = &s[0..5];
    println!("Substring: {}", slice);

    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    println!("Array slice: {:?}", slice);

    let numbers = vec![1, 2, 3, 4, 5];
    process_slice(&numbers[1..4]);

    let slice = &numbers[1..4];
    let doubled: Vec<i32> = slice.iter().map(|&x| x * 2).collect();
    println!("Doubled slice: {:?}", doubled);
}

fn process_slice(slice: &[i32]) {
    println!("Processing Slice:");
    for &num in slice {
        println!("{}", num);
    }
}
