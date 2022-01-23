[![Crates.io](https://img.shields.io/crates/v/fixed2float)](https://crates.io/crates/fixed2float)
[![PyPI](https://img.shields.io/pypi/v/fixed2float)](https://pypi.org/project/fixed2float/)

# fixed2float

Simple utility for fixed point to real number conversions, using [the Q notation](https://en.wikipedia.org/wiki/Fixed-point_arithmetic#Notations)*.

### Usage

- as a dependency of your Rust library

```toml
[dependencies]
fixed2float = { git = "https://github.com/urbanij/fixed2float" }
```

or

```
cargo add fixed2float
```

which will automatically fetch the most recent version from the registry.

- as a dependency of your Python library

```sh
pip install fixed2float
```


### Examples

See `example.py` or `example/basic.rs`


### Screencast

[![asciicast](https://asciinema.org/a/463661.svg)](https://asciinema.org/a/463661)

### Similar projects

- javascripts' [fixed2float](https://www.npmjs.com/package/fixed2float)


---

\*: sign is omitted here
