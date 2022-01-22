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



### Similar projects

- javascripts' [fixed2float](https://www.npmjs.com/package/fixed2float)


---

\*: sign is discarded here
