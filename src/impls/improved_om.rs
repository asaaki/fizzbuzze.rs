use crate::{Int, Map};
use std::io::Write;

type Predicates = Map<Predicate, String>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Predicate(fn(Int) -> bool);

pub fn run(w: &mut impl Write, inputs: impl IntoIterator<Item = Int>) {
    let mut predicates = Predicates::new();
    predicates.insert(Predicate(|v| v % 3 == 0), "Fizz".to_string());
    predicates.insert(Predicate(|v| v % 5 == 0), "Buzz".to_string());

    evaluate(inputs.into_iter(), predicates).for_each(|v| write!(w, "{v} ").expect("result item"));
}

fn evaluate(
    inputs: impl Iterator<Item = Int>,
    predicates: Predicates,
) -> impl Iterator<Item = String> {
    inputs.map(move |v| {
        predicates
            .iter()
            .filter_map(|(p, fv)| (p.0)(v).then_some(fv.to_owned()))
            .reduce(|a, b| format!("{a}{b}"))
            .unwrap_or(v.to_string())
    })
}
