
# BitArrayList

The library provides a grow-able array data structure which can represent a list of bits. So far there is `FIFO` functionality, bits can be set `ON` and `OFF` and two `BitArrayList`s can be concatenated.

[![Build Status](https://travis-ci.org/MrKonstantinT/bit-array-list.svg?branch=master)](https://travis-ci.org/MrKonstantinT/bit-array-list)

## Documentation

To view project documentation first build it:

```
$ cargo doc
```

and then open the following file with a web browser:

```
[crate root]/target/doc/bit_array_list/index.html
```

## Usage

Add this entry under `Cargo.toml` `dependencies` section name:

```toml
[dependencies]
bit_array_list = { git = "https://github.com/MrKonstantinT/bit-array-list" }
```

and the following to your crate root:

```rust
extern crate bit_array_list;
```

## License

See the [LICENSE](LICENSE.md) file for license rights and limitations (MIT).
