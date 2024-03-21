
/// Given an integer array nums sorted in non-decreasing order,
/// remove the duplicates in-place such that each unique element appears only once.
/// The relative order of the elements should be kept the same.
/// Then return the number of unique elements in nums.
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let (mut left, mut right) = (0, 0);
    while right < nums.len() {
        if nums[left] != nums[right] {
            left += 1;
            nums[left] = nums[right] as i32;
        }
        right += 1;
    }
    (left + 1) as i32
}

#[cfg(test)]
mod test {
    use crate::lc026::remove_duplicates;

    #[test]
    fn t1() {
        let mut nums =vec![1,1,2];
        assert_eq!(remove_duplicates(&mut nums), 2);
        assert_eq!(nums[..2], vec![1,2])
    }

    #[test]
    fn t2() {
        let mut nums =vec![0,0,1,1,1,2,2,3,3,4];
        assert_eq!(remove_duplicates(&mut nums), 5);
        assert_eq!(nums[..5], vec![0,1,2,3,4])
    }
}
