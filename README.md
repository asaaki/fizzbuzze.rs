# fizzbuzze<code>rs</code>

FizzBuzz playground to try different implentations and ideas

## benchmarks

```sh
cargo bench

     Running benches/fizzbuzzers.rs (target/release/deps/fizzbuzzers-fede9833e2adfeb6)
Timer precision: 10 ns
fizzbuzzers                      fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 OceanMoistOriginal         6.636 µs      │ 8.509 µs      │ 6.794 µs      │ 6.831 µs      │ 1000    │ 1000000
├─ 01 OceanMoistImproved         4.801 µs      │ 6.529 µs      │ 4.942 µs      │ 4.981 µs      │ 1000    │ 1000000
├─ 02 OceanMoistImprovedSmolstr  3.569 µs      │ 5.813 µs      │ 3.651 µs      │ 3.7 µs        │ 1000    │ 1000000
├─ 03 OceanMoistImprovedDisplay  1.192 µs      │ 2.548 µs      │ 1.324 µs      │ 1.341 µs      │ 1000    │ 1000000
╰─ 04 OceanMoistImprovedFast     781.6 ns      │ 1.557 µs      │ 799.3 ns      │ 810.6 ns      │ 1000    │ 1000000
```

* `OceanMoistOriginal` is a pretty close reimplementation of the [original Java version by Ocean-Moist][omfb-java]
* `OceanMoistImprovedFast` is up to 8.5 times faster than `OceanMoistOriginal`

<!-- Links -->
[omfb-java]: https://github.com/Ocean-Moist/FizzBuzz
