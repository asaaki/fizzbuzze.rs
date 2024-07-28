use crate::Int;
use std::io::Write;

pub fn run(w: &mut impl Write, inputs: impl IntoIterator<Item = Int>) {
    // iter/for_each seems to be slightly less efficient than a for loop
    inputs.into_iter().for_each(|v| {
        if v % 3 == 0 && v % 5 == 0 {
            write!(w, "FizzBuzz ").expect("result item")
        } else if v % 3 == 0 {
            write!(w, "Fizz ").expect("result item")
        } else if v % 5 == 0 {
            write!(w, "Buzz ").expect("result item")
        } else {
            write!(w, "{} ", v).expect("result item")
        }
    });
}
