// use std::collections::HashSet;

/// Given an unsorted integer array `nums`. Return the smallest positive integer that is not present in `nums`.
pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    // let soka = nums.iter().min_by_key(|n| if **n > 0 {**n} else {i32::MAX});
    // *soka.unwrap() + 1

    // let set: HashSet<i32> = HashSet::from_iter(nums.clone().into_iter());
    // // ℤ+
    // let mut zp = std::iter::successors(Some(1), |n|Some(n+1));
    // zp.find(|n| !set.contains(n)).unwrap()

    if nums.len() == 1 {
        return if nums[0] != 1 {
            1
        } else {
            2
        }
    }
    for idx in 0..nums.len() {
        while nums[idx] > 0 && nums[idx] <= nums.len() as i32 && nums[idx] != nums[(nums[idx] - 1) as usize] {
            let num = nums[idx];
            nums.swap(idx, (num - 1) as usize);
        }
    }
    if nums[0] != 1 {
        return 1
    }
    // dbg!(&nums);
    for idx in 1..nums.len() {
        if nums[idx] != (idx + 1) as i32 {
            return (idx + 1) as i32
        }
    }
    nums.last().map_or(1, |l| *l + 1)
}

#[cfg(test)]
mod test {
    use crate::lc041::first_missing_positive;

    #[test]
    fn t1() {
        assert_eq!(first_missing_positive(vec![4,8,6,2,5,1,8,3,7]), 9);
    }

    #[test]
    fn t2() {
        assert_eq!(first_missing_positive(vec![1,1]), 2);
    }

    #[test]
    fn t3() {
        assert_eq!(first_missing_positive(vec![1,-1,4,8,7,2,5,6]), 3);
    }

    #[test]
    fn t4() {
        assert_eq!(first_missing_positive(vec![1,2,0]), 3);
    }

    #[test]
    fn t5() {
        assert_eq!(first_missing_positive(vec![3,4,-1,1]), 2);
    }
    #[test]
    fn t6() {
        assert_eq!(first_missing_positive(vec![7,8,9,11,12]), 1);
    }
    #[test]
    fn t7() {
        assert_eq!(first_missing_positive(vec![1]), 2);
    }
    #[test]
    fn t8() {
        assert_eq!(first_missing_positive(vec![2,1]), 3);
    }
}
