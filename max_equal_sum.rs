fn max_equal_sum_of_three_stacks(stack1: Vec<i32>, stack2: Vec<i32>, stack3: Vec<i32>) -> i32 {
    let sum1: i32 = stack1.iter().sum();
    let sum2: i32 = stack2.iter().sum();
    let sum3: i32 = stack3.iter().sum();

    let min_sum = sum1.min(sum2).min(sum3);

    let diff1 = sum1 - min_sum;
    let diff2 = sum2 - min_sum;
    let diff3 = sum3 - min_sum;

    let mut new_sum1 = 0;
    let mut new_sum2 = 0;
    let mut new_sum3 = 0;

    for i in 0..stack1.len() {
        let mut temp = min_sum - stack1[i];
        if temp <= diff1 {
            new_sum1 += temp;
            diff1 -= temp;
        } else {
            new_sum1 += diff1;
            break;
        }
    }

    for i in 0..stack2.len() {
        let mut temp = min_sum - stack2[i];
        if temp <= diff2 {
            new_sum2 += temp;
            diff2 -= temp;
        } else {
            new_sum2 += diff2;
            break;
        }
    }

    for i in 0..stack3.len() {
        let mut temp = min_sum - stack3[i];
        if temp <= diff3 {
            new_sum3 += temp;
            diff3 -= temp;
        } else {
            new_sum3 += diff3;
            break;
        }
    }

    min_sum
}

fn main() {
    let stack1 = vec![3, 2, 1, 1, 1];
    let stack2 = vec![4, 3, 2];
    let stack3 = vec![1, 1, 4, 1];

    let result = max_equal_sum_of_three_stacks(stack1, stack2, stack3);
    println!("{}", result);
}
