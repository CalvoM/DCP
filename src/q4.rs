// Good morning! Here's your coding interview problem for today.

// This problem was asked by Stripe.

// Given an array of integers, find the first missing positive integer in linear time and constant space. In other words, find the lowest positive integer that does not exist in the array. The array can contain duplicates and negative numbers as well.

// For example, the input [3, 4, -1, 1] should give 2. The input [1, 2, 0] should give 3.

// You can modify the input array in-place.
pub fn solution(input: &mut [isize]) -> usize {
    let mut res: usize = 0;
    let mut i = 0;
    while i < input.len() {
        while i + 1 != (input[i] as usize) && 0 < input[i] && input[i] <= (input.len() as isize) {
            let v = input[i] as usize;
            (input[i], input[v - 1]) = (input[v - 1], input[i]);
            if input[i] == input[v - 1] {
                break;
            }
        }
        i += 1;
    }
    i = 0;
    while i < input.len() {
        if input[i] != ((i + 1) as isize) {
            res = i + 1;
            break;
        }
        i += 1;
    }
    res
}
