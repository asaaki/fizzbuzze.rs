use crate::{Int, Map};
use std::io::Write;
use strum_macros::{AsRefStr, Display, IntoStaticStr};

type Predicates = Map<Predicate, Value>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Predicate(fn(Int) -> bool);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, AsRefStr, IntoStaticStr)]
enum Value {
    Fizz,
    Buzz,
    FizzBuzz,
    #[strum(default)]
    Num(Int),
}

pub fn run(w: &mut impl Write, inputs: impl IntoIterator<Item = Int>) {
    let mut predicates = Predicates::new();
    predicates.insert(Predicate(|v| v % 3 == 0), Value::Fizz);
    predicates.insert(Predicate(|v| v % 5 == 0), Value::Buzz);

    evaluate(inputs.into_iter(), predicates).for_each(|v| write!(w, "{v} ").expect("result item"));
}

fn evaluate(
    inputs: impl Iterator<Item = Int>,
    predicates: Predicates,
) -> impl Iterator<Item = Value> {
    inputs.map(move |v| {
        predicates
            .iter()
            .filter_map(|(p, fv)| (p.0)(v).then_some(fv.to_owned()))
            .reduce(|_, _| Value::FizzBuzz)
            .unwrap_or(Value::Num(v))
    })
}
