use std::io::{stdout, BufWriter, Write};

pub mod impls;

pub type Int = u8;

// the only external dependency to fully and reliably adhere to the Java implementation;
// IndexMap/OrderedHashMap preserves insertion order independent of the hash,
// while BTreeMap's order is based on the hash of the key;
// since the Predicate is a function, we don't know if the hash-based order would match our intention
#[cfg(all(feature = "indexmap", not(feature = "ordered-hash-map")))]
pub type Map<K, V> = indexmap::IndexMap<K, V>;
#[cfg(all(not(feature = "indexmap"), feature = "ordered-hash-map"))]
pub type Map<K, V> = ordered_hash_map::OrderedHashMap<K, V>;

pub const INPUT_RANGE: std::ops::RangeInclusive<u8> = 1..=100;

pub fn all() {
    let mut w = BufWriter::new(stdout());

    writeln!(w, "ocean moist original (Java reimplementation):").expect("info");
    impls::origin_ocean_moist::run(&mut w, INPUT_RANGE);
    writeln!(w).expect("newline");
    w.flush().expect("flush");

    writeln!(w, "ocean moist improved (more Rust idiomatic):").expect("info");
    impls::improved_om::run(&mut w, INPUT_RANGE);
    writeln!(w).expect("newline");
    w.flush().expect("flush");

    writeln!(w, "ocean moist improved (with SmolStr):").expect("info");
    impls::improved_om_smolstr::run(&mut w, INPUT_RANGE);
    writeln!(w).expect("newline");
    w.flush().expect("flush");

    writeln!(w, "ocean moist improved (with displayable enum):").expect("info");
    impls::improved_om_display::run(&mut w, INPUT_RANGE);
    writeln!(w).expect("newline");
    w.flush().expect("flush");

    writeln!(
        w,
        "ocean moist improved (with displayable enum, rules is a list):"
    )
    .expect("info");
    impls::improved_om_fast::run(&mut w, INPUT_RANGE);
    writeln!(w).expect("newline");
    w.flush().expect("flush");
}
