pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() == 2 {
        return vec![0, 1];
    }
    let mut left = 0;
    let mut right = 1;
    loop {
        if right >= nums.len() {
            left += 1;
            right = left + 1;
        }
        if nums[left] + nums[right] == target {
            return vec![left as i32, right as i32];
        }
        right += 1;
    }
}

#[cfg(test)]
mod test {
    use crate::lc001::two_sum;

    #[test]
    fn t1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn t2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn t3() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn t4() {
        assert_eq!(
            two_sum(
                vec![1, 2, 3, 4, 5, 6, 7, 8, 483, 11, 1, 84, 23, 48, 100],
                583
            ),
            vec![8, 14]
        );
    }
}
