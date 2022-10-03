// Good morning! Here's your coding interview problem for today.

// This problem was recently asked by Google.

// Given a list of numbers and a number k, return whether any two numbers from the list add up to k.

// For example, given [10, 15, 3, 7] and k of 17, return true since 10 + 7 is 17.
pub fn solution(list_of_numbers: Vec<usize>, expected_sum: usize) -> bool {
    for (i, v) in list_of_numbers.iter().enumerate() {
        let mut inner = i + 1;
        loop {
            if inner < list_of_numbers.len() {
                let sum = v + list_of_numbers[inner];
                if sum == expected_sum {
                    return true;
                }
                inner += 1;
            } else {
                break;
            }
        }
    }
    false
}
