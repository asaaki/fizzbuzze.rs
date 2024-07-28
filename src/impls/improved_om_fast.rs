use crate::Int;
use std::io::Write;
use strum_macros::{AsRefStr, Display, IntoStaticStr};

type Predicates = [(Predicate, Value); 3];

#[derive(Clone, Copy)]
struct Predicate(fn(Int) -> bool);

// the Value enum helps us to avoid allocating a string for each result value;
// the most costly part should be the int->string conversion
#[derive(Clone, Copy, Display, AsRefStr, IntoStaticStr)]
enum Value {
    Fizz,
    Buzz,
    FizzBuzz,
    #[strum(default)]
    Num(Int),
}

// currently the most efficient implementation in the OM family
pub fn run(w: &mut impl Write, inputs: impl IntoIterator<Item = Int>) {
    // the predicates are iterated over in order of declaration, an array is more efficient than a BTreeMap for that;
    // it's more efficient to also add the 3+5 case, so we can use `find` instead of `filter`+`reduce`;
    // note: apparently it's a bit more efficient to initialize them here instead of lifitng them to a constant
    //       (roughly 30 to 40 ns on an 800 ns median - 5% loss)
    let predicates: [(Predicate, Value); 3] = [
        (Predicate(|v| v % 3 == 0 && v % 5 == 0), Value::FizzBuzz),
        (Predicate(|v| v % 3 == 0), Value::Fizz),
        (Predicate(|v| v % 5 == 0), Value::Buzz),
    ];

    // NOTES:
    // * inputs: if you can use an iterateble range type, use it instead of pre-allocating the data
    // * results: same as with the input, the result should be an iterator and we can pass the values into a writer;
    // this also has the advantage of decoupling a potential IO blocking operation from the evaluation,
    // since the writer can be a buffer which can be more efficient than directly writing to stdout;
    // definitely no complete output string for the result, which would be an unnecessary allocation
    evaluate(inputs.into_iter(), &predicates).for_each(|v| write!(w, "{v} ").expect("result item"));
}

#[inline]
fn evaluate<'a>(
    inputs: impl Iterator<Item = Int> + 'a,
    predicates: &'a Predicates,
) -> impl Iterator<Item = Value> + 'a {
    inputs.map(|v| {
        predicates
            .iter()
            // due to the extended ruleset including the 3+5 case, we can use `find` instead of `filter`+`reduce`
            .find_map(|(p, fv)| (p.0)(v).then_some(fv.to_owned()))
            .unwrap_or(Value::Num(v))
    })
}
