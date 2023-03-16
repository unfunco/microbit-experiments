# micro:bit experiments

## 🔨 Getting started

### Requirements

- [Rust] 1.68

### Development and testing

```bash
brew install --cask gcc-arm-embedded
brew install minicom openocd
```

```bash
cargo install flip-link probe-run
```

```bash
cargo run --release --target thumbv7em-none-eabihf
```

## License

© 2023 [Daniel Morris]\
Made available under the terms of the [Apache License 2.0].

[apache license 2.0]: LICENSE.md
[daniel morris]: https://unfun.co
[rust]: https://www.rust-lang.org
