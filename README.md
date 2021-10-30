# rust-fizzbuzz: FizzBuzz as a Rust stdout performance measure
Bart Massey 2021

This hastily thrown-together code is a Rust implementation
of FizzBuzz using various techniques for writing to
`stdout`:

* `--slow`: The obvious implementation using `println!()`.
* `--almost-slow`: Get rid of `format!()` calls by directly
  calling `write()` on `stdout`.
* `--fast`: Directly call write on a 32KB `BufWriter` atop `stdout`.

The performance of "almost-slow" is about 15% faster than
"slow" on my box. The performance of "fast" is almost 10×
faster than "slow". So… yeah.

This code was inspired by this
[Reddit thread](https://www.reddit.com/r/rust/comments/qiyqlo/fizzbuzz_in_rust_is_slower_than_python/?utm_source=share&utm_medium=web2x&context=3).

For more on these lines, see the
[fasthello](http://github.com/BartMassey/fasthello) project.

This code is made available under the "MIT License". Please
see the file `LICENSE` in this distribution for license terms.
