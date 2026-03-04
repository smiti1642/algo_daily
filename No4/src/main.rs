#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_median_sorted_arrays(_nums1: Vec<i32>, _nums2: Vec<i32>) -> f64 {
        todo!()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_example1() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
    }

    #[test]
    #[ignore]
    fn test_example2() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}
