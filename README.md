# ACU

Autonomous Conscient Unit implementing the AEIF framework.

## Architecture

ACU follows Domain-Driven Design (DDD), Command Query Responsibility Segregation (CQRS) and Event Sourcing. It depends on the [AEIF framework](https://github.com/SynarionTechnologies/aei-framework).

## Build

```bash
cargo build --all
cargo test --all
```

## Run

```bash
cargo run -p acu-cli -- --help
```

## CI

Continuous integration runs formatting, linting, build, tests and documentation on GitHub Actions.

## Roadmap

- [x] Phase 0: bootstrap workspace
- [ ] Phase 1: core domain implementation
- [ ] Phase 2: infrastructure adapters
- [ ] Phase 3: policies and execution engine
