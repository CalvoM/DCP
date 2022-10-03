pub use self::q1 as q_one;
pub mod q1;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_question_one() {
        let list = vec![10, 15, 3, 7, 1, 8, 9];
        let res = q_one::solution(list.clone(), 17);
        assert_eq!(res, true);
        let res = q_one::solution(list, 21);
        assert_eq!(res, false);
    }
}
