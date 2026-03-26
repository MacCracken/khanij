# Contributing to Khanij

Thank you for your interest in contributing to Khanij! This document outlines how
to get involved.

## Getting Started

1. Fork the repository and clone your fork
2. Install Rust 1.89+ (see `rust-toolchain.toml`)
3. Run `make check` to verify your environment

## Development Workflow

```bash
make fmt       # Format code
make clippy    # Lint
make test      # Run all tests (unit + integration + doc-tests)
make bench     # Run benchmarks
make doc       # Build documentation
make coverage  # Generate coverage report
```

## Pull Requests

- Keep PRs focused — one feature or fix per PR
- Add doc-tests for any new public API items
- Add unit tests for new logic
- Run `make check` before submitting (runs fmt, clippy, test, doc)
- Update `CHANGELOG.md` under an `[Unreleased]` heading

## Code Style

- Follow existing patterns in the codebase
- Use `f64` for all numeric computations
- Suffix parameter names with units (e.g., `depth_m`, `temperature_k`, `pressure_pa`)
- Return `Option<T>` for functions that can fail — avoid panics
- Add `#[must_use]` to pure functions
- Keep doc-tests minimal (3-5 lines) with `assert!` verification

## Adding a New Module

1. Create `src/module_name.rs`
2. Add `pub mod module_name;` to `src/lib.rs`
3. Add re-exports for all public items in `src/lib.rs`
4. Add doc-tests to every public function, struct, and enum
5. Add unit tests in a `#[cfg(test)] mod tests` block
6. Add integration tests in `tests/integration.rs`
7. Add benchmarks in `benches/benchmarks.rs`

## Reporting Issues

Open an issue on [GitHub](https://github.com/MacCracken/khanij/issues) with:
- What you expected to happen
- What actually happened
- Steps to reproduce
- Rust version (`rustc --version`)

## License

By contributing, you agree that your contributions will be licensed under
GPL-3.0-only, consistent with the project license.
