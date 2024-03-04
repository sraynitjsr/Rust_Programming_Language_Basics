fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let squared: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    println!("Squared numbers: {:?}", squared);

    let names = vec!["Tufan", "Virus", "Bhai"];
    let uppercase_names: Vec<String> = names.iter().map(|name| name.to_uppercase()).collect();
    println!("Uppercase names: {:?}", uppercase_names);

    let mut unsorted_numbers = vec![5, 2, 8, 1, 9];
    unsorted_numbers.sort();
    println!("Sorted numbers (ascending): {:?}", unsorted_numbers);

    let mut unsorted_numbers_desc = vec![5, 2, 8, 1, 9];
    unsorted_numbers_desc.sort_by(|a, b| b.cmp(a));
    println!("Sorted numbers (descending): {:?}", unsorted_numbers_desc);
}
