pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut biggest = calc_water(&left, &right, &height);

    while left != right {
        let water = calc_water(&left, &right, &height);
        if water > biggest {
            biggest = water;
        }
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1
        }
    }
    biggest
}




// pub fn max_area(height: Vec<i32>) -> i32 {
//     if height.len() <= 2 {
//         return calc_water(&0, &1, &height);
//     }
//     let mut left = 0;
//     let mut right = 1;
//     let mut max = (vec![left, right], calc_water(&left, &right, &height));
//     while left <= height.len() {
//         if right >= height.len() {
//             left += 1;
//             right = left + 1;
//             continue
//         }
//         let water = calc_water(&left, &right, &height);
//         if water > max.1 {
//             max = (vec![left, right], water)
//         }

//         right += 1;
//         dbg!(&max);
//     }
//     max.1
// }

fn calc_water(left: &usize, right: &usize, height: &Vec<i32>) -> i32 {
    let height = std::cmp::min(height[*left], height[*right]);
    let width = left.abs_diff(*right);

    height * width as i32
}

#[cfg(test)]
mod test {
    use crate::lc011::max_area;

    #[test]
    fn t1() {
        assert_eq!(max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
    }

    #[test]
    fn t2() {
        assert_eq!(max_area(vec![1,1]), 1);
    }
}
