pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut plus_one = digits.clone();
    let mut carry = false;
    for (idx, digit) in plus_one.iter_mut().rev().enumerate() {
        if !carry && idx != 0 {
            break;
        }
        if *digit == 9 {
            carry = true;
            *digit = 0;
        } else {
            carry = false;
            *digit += 1;
        }
    }
    if carry {
        plus_one.insert(0, 1)
    }
    plus_one
}

#[cfg(test)]
mod test {
    use super::plus_one;

    #[test]
    fn t1() {
        assert_eq!(plus_one(vec![1,2,3]), vec![1,2,4])
    }
    #[test]
    fn t2() {
        assert_eq!(plus_one(vec![1,9,9]), vec![2,0,0])
    }
    #[test]
    fn t3() {
        assert_eq!(plus_one(vec![9,9]), vec![1,0,0])
    }
    #[test]
    fn t4() {
        assert_eq!(plus_one(vec![1,2,9]), vec![1,3,0])
    }
}