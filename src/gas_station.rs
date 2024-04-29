// # Gas Station
// There are `n` gas stations along a circular route, where the amount of gas at the `ith` station is `gas[i]`.
//
// You have a car with an unlimited gas tank and it costs `cost[i]` of gas to travel from the `ith` station to its next `(i + 1)th` station.
// You begin the journey with an empty tank at one of the gas stations.
//
// Given two integer arrays gas and cost, return the starting gas station's index if you can travel around the circuit once in the clockwise direction, otherwise return -1. If there exists a solution, it is guaranteed to be unique

pub fn solution(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut total_tank_size = 0;
    let mut current_try_tank_size = 0;
    let mut start_item = 0;

    let travel_hop = gas.iter().zip(cost.iter());

    for (idx, (gas_available, hop_cost)) in travel_hop.enumerate() {
        let hop_gas_surplus = gas_available - hop_cost;
        total_tank_size += hop_gas_surplus;
        current_try_tank_size += hop_gas_surplus;

        if current_try_tank_size < 0 {
            current_try_tank_size = 0;
            start_item = idx as i32 + 1;
        }
    }

    if total_tank_size < 0 {
        -1
    } else {
        start_item
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let gas = vec![1, 2, 3, 4, 5];
        let cost = vec![3, 4, 5, 1, 2];
        assert_eq!(solution(gas, cost), 3);
    }

    #[test]
    fn example_2() {
        let gas = vec![2, 3, 4];
        let cost = vec![3, 4, 3];
        assert_eq!(solution(gas, cost), -1);
    }

    #[test]
    fn example_3() {
        let gas = vec![5, 1, 2, 3, 4];
        let cost = vec![4, 4, 1, 5, 1];
        assert_eq!(solution(gas, cost), 4);
    }
}
