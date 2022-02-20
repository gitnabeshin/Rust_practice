/**************************************************************
 * LeetCode
 *     Container With Most Water
 * https://leetcode.com/problems/container-with-most-water/
***************************************************************/

pub fn contains_most(heights: Vec<usize>) -> usize {
    let mut max_area = 0;
    let mut area;

    for i in 0..heights.len() {
        for j in i..heights.len() {
            if i == j {
                continue;
            }

            let x = j - i;

            if heights[i] > heights[j] {
                area = x * heights[j];
            } else {
                area = x * heights[i];
            }

            if max_area < area {
                max_area = area;
            }
        }
    }

    max_area
}

pub fn contains_most2(heights: Vec<usize>) -> usize {
    // check from the end of value (starts with biggest width)
    // and from the first value,
    // compare 2 hights, pick lower one and get current area.
    // compare with MAX area
    // continue (move next of lower one) till the end.

    use std::cmp;

    let mut i = 0;
    let mut j = heights.len() - 1;
    let mut max_area = 0;

    while i < j {
        let area = cmp::min(heights[i], heights[j]) * (j - i);
        if max_area < area {
            max_area = area;
        }

        if heights[i] < heights[j] {
            i += 1;
        } else {
            j -= 1;
        }
    }

    max_area
}

#[cfg(test)]
mod tests {

    use crate::contains_most;
    use crate::contains_most2;

    #[test]
    fn it_works1() {
        let heights = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(contains_most(heights), 49);
    }

    #[test]
    fn it_works2() {
        let heights = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(contains_most2(heights), 49);
    }

    #[test]
    fn it_works3() {
        let heights = vec![10, 1, 1, 20, 1, 1, 1, 1, 10];
        assert_eq!(contains_most2(heights), 80);
    }
}
