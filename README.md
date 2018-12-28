About
---

Rust realization of phonetic algorithm metaphone for Russian and Ukrainian languages (English is in progress).
Everything is not optimized and api is in flux.

Benchmark for general-length word
--

    test tests::bench_ru ... bench: 5,407 ns/iter (+/- 484)

Usage
-----

    let metaphone = Metaphone::new(Language::Russian);
    assert_eq!(metaphone.get("Шварценеггер"), "ШВАРЦИНИГИР");

License
-------
MIT