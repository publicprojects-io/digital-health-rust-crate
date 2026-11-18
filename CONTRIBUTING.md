# Contributing to Digital Health Rust crate

Thanks for helping improve this project.

## Ways to contribute

- **Report a problem:** open an issue describing what is wrong, unclear, or missing.
- **Fix or improve something:** open a pull request. Small fixes can go straight to a pull request; for larger changes, please open an issue first so we can agree on the approach.
- **Add content or examples:** follow the structure and style of the existing files.

## Development

Read [AGENTS.md](AGENTS.md) and [spec/README.md](spec/README.md) first: the `spec/` directory is the source of truth, and a change to a formula, signature, or `None`/`Err` condition updates the spec, the function in `src/`, its rustdoc, and its tests together. To add a metric, follow [docs/tutorials/adding-a-metric.md](docs/tutorials/adding-a-metric.md).

Run the same checks CI runs (format, clippy, tests, doctests, docs):

```sh
make ci                                     # runs bin/ci
make test-minimum-supported-rust-version    # MSRV (1.98) and the next release
make benchmarks                             # criterion benchmarks (report: target/criterion/)
make coverage                               # tarpaulin coverage (needs: cargo install cargo-tarpaulin)
```

## Code of conduct

Participation is governed by our [Code of Conduct](CODE_OF_CONDUCT.md).

## License

By contributing, you agree that your contribution is offered under the terms in [LICENSE.md](LICENSE.md).
