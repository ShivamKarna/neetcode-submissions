impl Solution
{
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();

    let mut leftProduct = 1;

    let mut output = vec![1;n];


    for i in 0..n{
        output[i] = leftProduct;
        leftProduct *= nums[i];
    }

    let mut rightProduct = 1;

    for i in (0..n).rev(){
        output[i] *= rightProduct;
        rightProduct = rightProduct * nums[i];
    }

    output
}
}