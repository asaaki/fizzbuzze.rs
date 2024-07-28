use crate::{Int, Map};
use smol_str::format_smolstr;
use std::io::Write;

type Str = smol_str::SmolStr;
type Predicates = Map<Predicate, Str>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Predicate(fn(Int) -> bool);

pub fn run(w: &mut impl Write, inputs: impl IntoIterator<Item = Int>) {
    let mut predicates = Predicates::new();
    predicates.insert(Predicate(|v| v % 3 == 0), Str::new_static("Fizz"));
    predicates.insert(Predicate(|v| v % 5 == 0), Str::new_static("Buzz"));

    evaluate(inputs.into_iter(), predicates).for_each(|v| write!(w, "{v} ").expect("result item"));
}

fn evaluate(
    inputs: impl Iterator<Item = Int>,
    predicates: Predicates,
) -> impl Iterator<Item = Str> {
    let mut itoa = itoa::Buffer::new();
    inputs.map(move |v| {
        predicates
            .iter()
            .filter_map(|(p, fv)| (p.0)(v).then_some(fv.to_owned()))
            .reduce(|a, b| format_smolstr!("{a}{b}"))
            .unwrap_or(Str::from(itoa.format(v)))
    })
}
