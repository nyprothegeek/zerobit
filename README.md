<div align="center">
  <a href="https://github.com/nyprothegeek/versa" target="_blank">
    <img src="https://raw.githubusercontent.com/nyprothegeek/versa/main/assets/a_logo.png" alt="versa Logo" width="100"></img>
  </a>

  <h1 align="center">versa</h1>

  <p>
    <a href="https://crates.io/crates/versa">
      <img src="https://img.shields.io/crates/v/versa?label=crates" alt="Crate">
    </a>
    <a href="https://codecov.io/gh/nyprothegeek/versa">
      <img src="https://codecov.io/gh/nyprothegeek/versa/branch/main/graph/badge.svg?token=SOMETOKEN" alt="Code Coverage"/>
    </a>
    <a href="https://github.com/nyprothegeek/versa/actions?query=">
      <img src="https://github.com/nyprothegeek/versa/actions/workflows/tests_and_checks.yml/badge.svg" alt="Build Status">
    </a>
    <a href="https://github.com/nyprothegeek/versa/blob/main/LICENSE">
      <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License">
    </a>
    <a href="https://docs.rs/versa">
      <img src="https://img.shields.io/static/v1?label=Docs&message=docs.rs&color=blue" alt="Docs">
    </a>
    <a href="https://discord.gg/WqTEScR4">
      <img src="https://img.shields.io/static/v1?label=Discord&message=join%20us!&color=mediumslateblue" alt="Discord">
    </a>
  </p>
</div>

<div align="center"><sub>:warning: Work in progress :warning:</sub></div>

##

## Outline

- [Installation](#installation)
- [Testing the Project](#testing-the-project)
- [Benchmarking the Project](#benchmarking-the-project)
- [Contributing](#contributing)
- [Getting Help](#getting-help)
- [External Resources](#external-resources)
- [License](#license)

## Installation

### Using `cargo`

```console
cargo install versa
```

## Testing the Project

- Run tests

  ```console
  cargo test
  ```

## Benchmarking the Project

For benchmarking and measuring performance, this project leverages
[criterion][criterion] and a `test_utils` feature flag
for integrating [proptest][proptest] within the the suite for working with
[strategies][strategies] and sampling from randomly generated values.

- Run benchmarks

  ```console
  cargo bench --features test_utils
  ```

## Contributing

:balloon: We're thankful for any feedback and help in improving our project!
We have a [contributing guide](./CONTRIBUTING.md) to help you get involved. We
also adhere to our [Code of Conduct](./CODE_OF_CONDUCT.md).

### Formatting

For formatting Rust in particular, please use `cargo +nightly fmt` as it uses
specific nightly features we recommend. **Make sure you have nightly
installed**.

### Pre-commit Hook

This project recommends using [pre-commit][pre-commit] for running pre-commit
hooks. Please run this before every commit and/or push.

- Once installed, Run `pre-commit install` and `pre-commit install --hook-type commit-msg`
  to setup the pre-commit hooks locally. This will reduce failed CI builds.

- If you are doing interim commits locally, and for some reason if you _don't_
  want pre-commit hooks to fire, you can run
  `git commit -a -m "Your message here" --no-verify`.

### Recommended Development Flow

- We recommend installing and leveraging [cargo-watch][cargo-watch],
  [cargo-expand][cargo-expand] and [irust][irust] for Rust development.

### Conventional Commits

This project _lightly_ follows the [Conventional Commits
convention][commit-spec-site] to help explain
commit history and tie in with our release process. The full specification
can be found [here][commit-spec]. We recommend prefixing your commits with
a type of `fix`, `feat`, `docs`, `ci`, `refactor`, etc..., structured like so:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

## Getting Help

For usage questions, usecases, or issues reach out to us in our [Discord channel](https://discord.gg/WqTEScR4).

We would be happy to try to answer your question or try opening a new issue on Github.

## External Resources

These are references to specifications, talks and presentations, etc.

## License

This project is licensed under the [Apache License 2.0][apache-license].

[apache-license]: https://github.com/nyprothegeek/versa/blob/main/LICENSE
[cargo-expand]: https://github.com/dtolnay/cargo-expand
[cargo-udeps]: https://github.com/est31/cargo-udeps
[cargo-watch]: https://github.com/watchexec/cargo-watch
[commit-spec]: https://www.conventionalcommits.org/en/v1.0.0/#specification
[commit-spec-site]: https://www.conventionalcommits.org/
[criterion]: https://github.com/bheisler/criterion.rs
[irust]: https://github.com/sigmaSd/IRust
[pre-commit]: https://pre-commit.com/
[proptest]: https://github.com/proptest-rs/proptest
[strategies]: https://docs.rs/proptest/latest/proptest/strategy/trait.Strategy.html
