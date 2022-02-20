/***************************************************************
 * Product of Array Except Self
 *
 * https://leetcode.com/problems/product-of-array-except-self/
 * *************************************************************/

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..nums.len() {
        let mut pro = 1;
        for j in 0..nums.len() {
            if j != i {
                pro = pro * nums[j];
            }
        }
        result.push(pro);
    }
    result
}

pub fn product_except_self2(nums: Vec<i32>) -> Vec<i32> {
    // product of left all values of num[i]
    let mut left = vec![1; nums.len()];
    // product of right all values of num[i]
    let mut right = vec![1; nums.len()];
    // result[i] comes by product of each left[i], right[i]
    let mut result = vec![1; nums.len()];

    //starts from 1
    for i in 1..nums.len() {
        //      1,     2,     3,     4

        left[i] = left[i - 1] * nums[i - 1];
        //      1, (1*1),
        //      1,     1, (1*2),
        //      1,     1,     2,   (2*3)

        right[nums.len() - 1 - i] = right[nums.len() - i] * nums[nums.len() - i];
        //                (1*4),      1
        //         (4*3),     4,      1
        // (12*2),    12,     4,      1
    }

    for i in 0..nums.len() {
        result[i] = left[i] * right[i];
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;
    // use crate::product_except_self;
    // use crate::product_except_self2;

    #[test]
    fn it_works() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(product_except_self(nums), vec![24, 12, 8, 6]);

        let nums = vec![-1, 1, 0, -3, 3];
        assert_eq!(product_except_self(nums), vec![0, 0, 9, 0, 0]);
    }

    #[test]
    fn it_works2() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(product_except_self2(nums), vec![24, 12, 8, 6]);
    }
}

// for enable println!(), run with
// $ cargo test -- --nocapture
