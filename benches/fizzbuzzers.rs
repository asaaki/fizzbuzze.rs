use divan::{black_box, Bencher};
use fizzbuzzers::{impls::*, INPUT_RANGE};
use std::io::{sink, Write};

fn main() {
    divan::main();
}

#[divan::bench(name = "00 OceanMoistOriginal")]
fn bench_ocean_moist_original(bencher: Bencher<'_, '_>) {
    let mut w = sink();
    bencher.bench_local(|| {
        origin_ocean_moist::run(&mut w, black_box(INPUT_RANGE));
    });
    w.flush().unwrap();
}

#[divan::bench(name = "01 OceanMoistImproved")]
fn bench_ocean_moist_improved(bencher: Bencher<'_, '_>) {
    let mut w = sink();
    bencher.bench_local(|| {
        improved_om::run(&mut w, black_box(INPUT_RANGE));
    });
    w.flush().unwrap();
}

#[divan::bench(name = "02 OceanMoistImprovedSmolstr")]
fn bench_ocean_moist_improved_smolstr(bencher: Bencher<'_, '_>) {
    let mut w = sink();
    bencher.bench_local(|| {
        improved_om_smolstr::run(&mut w, black_box(INPUT_RANGE));
    });
    w.flush().unwrap();
}

#[divan::bench(name = "03 OceanMoistImprovedDisplay")]
fn bench_ocean_moist_improved_display(bencher: Bencher<'_, '_>) {
    let mut w = sink();
    bencher.bench_local(|| {
        improved_om_display::run(&mut w, black_box(INPUT_RANGE));
    });
    w.flush().unwrap();
}

#[divan::bench(name = "04 OceanMoistImprovedFast")]
fn bench_ocean_moist_improved_fast(bencher: Bencher<'_, '_>) {
    let mut w = sink();
    bencher.bench_local(|| {
        improved_om_fast::run(&mut w, black_box(INPUT_RANGE));
    });
    w.flush().unwrap();
}

#[divan::bench(name = "S1 SimpleIterForEach")]
fn bench_simple_iterator(bencher: Bencher<'_, '_>) {
    let mut w = sink();
    bencher.bench_local(|| {
        simple_iterator::run(&mut w, black_box(INPUT_RANGE));
    });
    w.flush().unwrap();
}

#[divan::bench(name = "S2 SimpleForLoop")]
fn bench_simple(bencher: Bencher<'_, '_>) {
    let mut w = sink();
    bencher.bench_local(|| {
        simple_for_loop::run(&mut w, black_box(INPUT_RANGE));
    });
    w.flush().unwrap();
}
