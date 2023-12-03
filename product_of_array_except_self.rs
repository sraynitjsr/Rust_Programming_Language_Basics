impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();        
        let mut left_products = vec![1; n];
        let mut right_products = vec![1; n];
        let mut left_product = 1;
        for i in 1..n {
            left_product *= nums[i - 1];
            left_products[i] = left_product;
        }
        let mut right_product = 1;
        for i in (0..(n - 1)).rev() {
            right_product *= nums[i + 1];
            right_products[i] = right_product;
        }
        let mut result = vec![0; n];
        for i in 0..n {
            result[i] = left_products[i] * right_products[i];
        }
        
        result
    }
}
fn main() {    
    let nums = vec![1, 2, 3, 4];
    let result = Solution::product_except_self(nums);
    println!("{:?}", result);
}
