// Good morning! Here's your coding interview problem for today.
//
// This problem was asked by Jane Street.
//
// cons(a, b) constructs a pair, and car(pair) and cdr(pair) returns the first and last element of that pair. For example, car(cons(3, 4)) returns 3, and cdr(cons(3, 4)) returns 4.
//
// Given this implementation of cons:
//
// def cons(a, b):
//     def pair(f):
//         return f(a, b)
//     return pair
// Implement car and cdr.

pub fn cons(a: i32, b: i32) -> impl Fn(fn(i32, i32) -> i32) -> i32 {
    move |f: fn(i32, i32) -> i32| f(a, b)
}

pub fn car(f: impl Fn(fn(i32, i32) -> i32) -> i32) -> i32 {
    fn first_val(a: i32, _: i32) -> i32 {
        a
    }
    f(first_val)
}
pub fn cdr(f: impl Fn(fn(i32, i32) -> i32) -> i32) -> i32 {
    fn second_val(_: i32, b: i32) -> i32 {
        b
    }
    f(second_val)
}
