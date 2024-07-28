use crate::Int;
use std::io::Write;

pub fn run(w: &mut impl Write, inputs: impl IntoIterator<Item = Int>) {
    // for loop seems to be slightly more efficient than iter/for_each
    for v in inputs {
        if v % 3 == 0 && v % 5 == 0 {
            write!(w, "FizzBuzz ").expect("result item")
        } else if v % 3 == 0 {
            write!(w, "Fizz ").expect("result item")
        } else if v % 5 == 0 {
            write!(w, "Buzz ").expect("result item")
        } else {
            write!(w, "{} ", v).expect("result item")
        }
    }
}
