fn main() {
    let mut arr = vec![-2,0,5,-1,2];
    let mut k = 4;

    arr.sort();

    let mut i = 0;
    while k > 0 && i < arr.len() && arr[i] < 0 {
        arr[i] = -arr[i];
        k -= 1;
        i += 1;
    }

    if k % 2 != 0 {
        if i == 0 {
            arr[0] = -arr[0];
        } else {
            if i < arr.len() && (arr[i] < -arr[i - 1] || i == arr.len() - 1) {
                arr[i] = -arr[i];
            } else {
                arr[i - 1] = -arr[i - 1];
            }
        }
    }

    let sum: i32 = arr.iter().sum();
    println!("{}", sum);
}
