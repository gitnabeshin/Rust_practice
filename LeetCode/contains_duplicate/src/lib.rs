/***************************************************************
 * LeetCode
 *    Product of Array Except Self
 *    https://leetcode.com/problems/contains-duplicate/
 * *************************************************************/

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut ret = false;

    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                continue;
            }
            if nums[i] == nums[j] {
                ret = true;
                break;
            }
        }
    }
    ret
}

use std::collections::HashSet;

pub fn contains_duplicate2(nums: Vec<i32>) -> bool {
    let mut ret = false;
    let mut keys = HashSet::new();

    for i in 0..nums.len() {
        // HashSet overwrites same existing key with new one.
        keys.insert(nums[i]);
    }

    println!("{:?}", keys);

    if keys.len() != nums.len() {
        ret = true;
    }

    ret
}

pub fn contains_duplicate3(nums: Vec<i32>) -> bool {
    let mut ret = false;
    let mut keys = HashSet::new();

    for i in 0..nums.len() {
        if keys.contains(&nums[i]) {
            ret = true;
            break;
        } else {
            keys.insert(nums[i]);
        }
    }

    println!("{:?}", keys);

    ret
}

#[cfg(test)]
mod tests {

    use super::*;
    // use crate::contains_duplicate;

    #[test]
    fn it_works() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(contains_duplicate(nums), true);
    }

    #[test]
    fn it_works2() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(contains_duplicate2(nums), true);
    }

    #[test]
    fn it_works3() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(contains_duplicate3(nums), true);
    }
}
