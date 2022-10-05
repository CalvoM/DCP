// Good morning! Here's your coding interview problem for today.

// This problem was asked by Uber.

// Given an array of integers, return a new array such that each element at index i of the new array is the product of all the numbers in the original array except the one at i.

// For example, if our input was [1, 2, 3, 4, 5], the expected output would be [120, 60, 40, 30, 24]. If our input was [3, 2, 1], the expected output would be [2, 3, 6].

pub fn solution(list_of_numbers: Vec<usize>) -> Vec<usize> {
    let sum:usize = list_of_numbers.iter().product();
    let mut res: Vec<usize> = vec![];
    for number in list_of_numbers.iter() {
        res.push(sum/number);
    }
    res
}