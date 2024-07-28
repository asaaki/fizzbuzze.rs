use crate::{Int, Map};
use std::io::Write;

type Integers = Vec<Int>;
type Predicates = Map<Predicate, String>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Predicate(fn(Int) -> bool);

// needed change to be able to benchmark:
// we don't hardcode stdout writing, but pass a writer instead,
// so we can measure runtime without stdout affecting the measurements
pub fn run(w: &mut impl Write, inputs: impl IntoIterator<Item = Int>) {
    let inputs = inputs.into_iter().collect();
    let mut predicates = Predicates::new();
    predicates.insert(Predicate(|v| v % 3 == 0), "Fizz".to_string());
    predicates.insert(Predicate(|v| v % 5 == 0), "Buzz".to_string());

    write!(w, "{}", evaluate(inputs, predicates)).expect("result");
}

fn evaluate(inputs: Integers, predicates: Predicates) -> String {
    inputs
        .iter()
        .map(move |v| {
            predicates
                .iter()
                .filter_map(|(p, fv)| (p.0)(*v).then_some(fv.to_owned()))
                .reduce(|a, b| format!("{a}{b}"))
                .unwrap_or(v.to_string())
        })
        .collect::<Vec<_>>()
        .join(" ")
}
