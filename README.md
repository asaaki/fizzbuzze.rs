# fizzbuzze<code>rs</code>

FizzBuzz playground to try different implentations and ideas

## why

For the funsies.

Due to an unrelated article I discoverd the referenced [Ocean-Moist Java implementation][omfb-java].

Not gonna _enterprisify_ this project though.

## benchmarks

```sh
cargo bench

     Running benches/fizzbuzzers.rs (target/release/deps/fizzbuzzers-fede9833e2adfeb6)
Timer precision: 11 ns
fizzbuzzers                      fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 OceanMoistOriginal         6.144 µs      │ 9.878 µs      │ 6.345 µs      │ 6.5 µs        │ 1000    │ 1000000
├─ 01 OceanMoistImproved         4.767 µs      │ 8.434 µs      │ 4.937 µs      │ 5.12 µs       │ 1000    │ 1000000
├─ 02 OceanMoistImprovedSmolstr  2.566 µs      │ 5.62 µs       │ 2.641 µs      │ 2.698 µs      │ 1000    │ 1000000
├─ 03 OceanMoistImprovedDisplay  1.146 µs      │ 2.233 µs      │ 1.179 µs      │ 1.214 µs      │ 1000    │ 1000000
├─ 04 OceanMoistImprovedFast     600.8 ns      │ 1.159 µs      │ 633.7 ns      │ 638.6 ns      │ 1000    │ 1000000
├─ S1 SimpleIterForEach          705.6 ns      │ 2.023 µs      │ 723.4 ns      │ 748.3 ns      │ 1000    │ 1000000
╰─ S2 SimpleForLoop              438.9 ns      │ 1.202 µs      │ 452.7 ns      │ 466.5 ns      │ 1000    │ 1000000
```

* `OceanMoistOriginal` is a pretty close reimplementation of the [original Java version by Ocean-Moist][omfb-java]
* `OceanMoistImprovedFast` is **10 times faster** than `OceanMoistOriginal` _(or "900 % faster")_
* `OceanMoistImprovedFast` is in same ballpark as simple/naive implementations

<!-- Links -->
[omfb-java]: https://github.com/Ocean-Moist/FizzBuzz
