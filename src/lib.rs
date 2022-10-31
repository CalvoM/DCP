pub mod q1;
pub mod q2;
pub mod q3;
pub mod q4;

#[cfg(test)]
mod test {
    use self::q1 as q_one;
    use self::q2 as q_two;
    use crate::q3::{deserialize, serialize, Node};

    use super::*;
    #[test]
    fn test_question_one() {
        let list = vec![10, 15, 3, 7, 1, 8, 9];
        let res = q_one::solution(list.clone(), 17);
        assert_eq!(res, true);
        let res = q_one::solution(list, 21);
        assert_eq!(res, false);
    }

    #[test]
    fn test_question_two() {
        let list = vec![1, 2, 3, 4, 5];
        let res = q_two::solution(list);
        assert_eq!(res, [120, 60, 40, 30, 24]);
        let list = vec![3, 2, 1];
        let res = q_two::solution(list);
        assert_eq!(res, [2, 3, 6]);
        let list = vec![1, 2, 3, 4, 5];
        let res = q_two::solution_better(list);
        assert_eq!(res, [120, 60, 40, 30, 24]);
        let list = vec![3, 2, 1];
        let res = q_two::solution_better(list);
        assert_eq!(res, [2, 3, 6]);
    }
    #[test]
    fn test_question_three() {
        let node = Node {
            val: String::from("root"),
            right: Some(Box::new(Node {
                val: String::from("right"),
                left: None,
                right: None,
            })),
            left: Some(Box::new(Node {
                val: String::from("left"),
                right: None,
                left: Some(Box::new(Node {
                    val: String::from("left.left"),
                    right: None,
                    left: None,
                })),
            })),
        };
        assert_eq!(
            deserialize(serialize(node)).left.unwrap().left.unwrap().val,
            String::from("left.left")
        );
    }
    #[test]
    fn test_question_four() {
        let mut input: [isize; 4] = [3, 4, -1, 1];
        let ans = q4::solution(&mut input);
        assert_eq!(ans, 2);
        let mut input2: [isize; 3] = [1, 2, 0];
        let ans = q4::solution(&mut input2);
        assert_eq!(ans, 3);
    }
}
