use std::cmp::max_by_key;

pub struct Solution;
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut car = gas.iter().zip(cost.iter()).enumerate().fold(
            None::<(usize, i32)>,
            |mut gassiest_car, (station, (gas, cost))| {
                // car starting from this station
                let current_car = Some((station, *gas - cost)).filter(|(_origin, tank)| *tank >= 0);
                // car with the fullest tank, if any made it this far
                gassiest_car = Self::drive_car(&mut gassiest_car, *gas, *cost);
                max_by_key(current_car, gassiest_car, |car| {
                    car.map(|(_origin, tank)| tank).unwrap_or(i32::MIN)
                })
            },
        );

        for (station, (gas, cost)) in gas.iter().zip(cost.iter()).enumerate() {
            if car.map(|(origin, _)| station == origin).unwrap_or_default() {
                break;
            }
            car = Self::drive_car(&mut car, *gas, *cost);
        }
        // cars.into_iter().max_by_key(|car|car.1).map(|car| if car.1 >= 0 {car.0 as i32} else {-1}).unwrap_or(-1)
        car.map(|(origin, _tank)| origin as i32).unwrap_or(-1)
    }

    pub fn drive_car(car: &mut Option<(usize, i32)>, gas: i32, cost: i32) -> Option<(usize, i32)> {
        car.map(|(_origin, mut tank)| {
            // fill
            tank += gas;
            // travel
            tank -= cost;
            (_origin, tank)
        })
        // make sure car hasn't run out of gas
        .filter(|(_origin, tank)| *tank >= 0)
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn t1() {
        let gas = vec![1, 2, 3, 4, 5];
        let cost = vec![3, 4, 5, 1, 2];
        assert_eq!(super::Solution::can_complete_circuit(gas, cost), 3);
    }

    #[test]
    fn t2() {
        let gas = vec![2, 3, 4];
        let cost = vec![3, 4, 3];
        assert_eq!(super::Solution::can_complete_circuit(gas, cost), -1);
    }

    #[test]
    fn t3() {
        let gas = vec![3, 1, 1];
        let cost = vec![1, 2, 2];
        assert_eq!(super::Solution::can_complete_circuit(gas, cost), 0);
    }

    #[test]
    fn t4() {
        let gas = vec![2];
        let cost = vec![2];
        assert_eq!(super::Solution::can_complete_circuit(gas, cost), 0);
    }

    #[test]
    fn t5() {
        let gas = vec![4];
        let cost = vec![5];
        assert_eq!(super::Solution::can_complete_circuit(gas, cost), -1);
    }

}
