kunai
===

A competitive programming tool for Rust.

## Installing from source

```
cargo install --path . --locked
```

## Usage

### Unify code

Below command outputs a unified code of `src/bin/<binname>.rs`.
```
kunai unify <binname>
```

For AtCoder(Rust 1.15.1)
```
kunai unify --no-eprint --rust2015 <binname>
```

To use your snippet crates, write dependencies in Cargo.toml like below.

```
[dependencies]
my_snippets = { path = "/path/to/my_snippets" }
```

### Download testcases (for AtCoder)
Below command stores sample testcases in /*path-to-cache_dir*/kunai/atcoder/*contest_name*/*problem_name*/sample_*id*.{in,out}
```
kunai download <problem_url>
```

### Login (for AtCoder)
```
kunai atcoder login
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
