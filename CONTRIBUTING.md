<!--
Copyright 2026 ADBC Drivers Contributors
SPDX-License-Identifier: Apache-2.0
-->

# Contributing

All contributors are expected to follow the [Code of
Conduct](https://github.com/adbc-drivers/template-rs?tab=coc-ov-file#readme).

## Issues and Feature Requests

Please file issues and feature requests on the GitHub issue tracker:
https://github.com/adbc-drivers/template-rs/issues

Potential security vulnerabilities should be reported to
[security@adbc-drivers.org](mailto:security@adbc-drivers.org) instead.

## Development

Install [`cargo-generate`](https://github.com/cargo-generate/cargo-generate):

```bash
cargo install cargo-generate
```

Generate a new ADBC driver project:

```bash
cargo generate --path path/to/template-rs
```

## Pull Requests

Before opening a pull request:

- Review your changes and ensure no stray files or folders are included.
- Add the Apache license header to the top of all files outside of `template/`.
- Use the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) format for all commit messages.
- Run the static checks by installing [pre-commit](https://pre-commit.com/), then running `pre-commit run --all-files` from inside the repository. Ensure all your changes are staged/committed (unstaged changes will be ignored).
- Check if there is an existing issue. If not, please file one, unless the change is trivial.

When writing a pull request description:

- Use the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) format for the pull request title.
- Ensure the description ends with `Closes #NNN`, `Fixes #NNN`, or similar, so that the issue will be linked to your pull request.
