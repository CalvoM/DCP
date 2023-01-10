// Good morning! Here's your coding interview problem for today.
//
// This problem was asked by Facebook.
//
// Given the mapping a = 1, b = 2, ... z = 26, and an encoded message, count the number of ways it can be decoded.
//
// For example, the message '111' would give 3, since it could be decoded as 'aaa', 'ka', and 'ak'.
//
// You can assume that the messages are decodable. For example, '001' is not allowed.
pub fn solution(text: &str) -> Vec<&str> {
    let a = 97;
    let one = 49;
    for c in text.chars().collect::<Vec<char>>() {
        let diff = c as u32 - one;
        let letter = a + diff;
        println!("{}", (letter as u8) as char);
    }
    Vec::new()
}
