/// Given an array nums with n objects colored red, white, or blue, sort them in-place so that objects of the same color are adjacent, with the colors in the order red, white, and blue.
/// We will use the integers 0, 1, and 2 to represent the color red, white, and blue, respectively.
/// You must solve this problem without using the library's sort function.
pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut color_counts = [0, 0, 0];
    for num in nums.iter() {
        color_counts[*num as usize] += 1;
    }

    for (idx, num) in nums.iter_mut().enumerate() {
        *num = match idx {
            _ if idx < color_counts[0] => 0,
            _ if idx < color_counts[0] + color_counts[1] => 1,
            _ => 2
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lc075::sort_colors;

    #[test]
    fn t1() {
        let mut nums = vec![2,0,2,1,1,0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0,0,1,1,2,2]);
    }

    #[test]
    fn t2() {
        let mut nums = vec![2,0,1];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0,1,2]);
    }
}
