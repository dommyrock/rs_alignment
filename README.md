### Great resources 
[Data layout - Reference](https://doc.rust-lang.org/reference/type-layout.html)

[Data layout - Book](https://doc.rust-lang.org/nomicon/data.html)

#### Talks 
- [Andrew Kelley   Practical Data Oriented Design (DoD)](https://www.youtube.com/watch?v=IroPQ150F6c)
- [Enter The Arena: Simplifying Memory Management (2023)](https://www.youtube.com/watch?v=TZ5a3gCCZYo)
- [Visualizing memory layout of Rust's data types](https://www.youtube.com/watch?v=7_o-YRxf_cc)


#### Profiling Rust memory
- [Profiling Rust Programs with valgrind, heaptrack, and hyperfine](https://www.youtube.com/watch?v=X6Xz4CRd6kw)
- [Profiling Code in Rust - by Vitaly Bragilevsky - Rust Linz, December 2022](https://www.youtube.com/watch?v=JRMOIE_wAFk)


#### Tools
```bash
RUSTFLAGS=-Zprint-type-sizes cargo +nightly build --release
```

> -Zprint-type-sizes only prints sizes for types for which the layout is computed in the first place. This is normally only for types that are actually used. 

