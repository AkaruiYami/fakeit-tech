# fakeit-tech

Some fakery tools that make you feel like you THE hackerman but actually not.

fakeit-tech is a small command-line Rust tool that produces Hollywood-style "hacker scene" terminal output — flashy, dramatic progress lines and visual effects meant for demos, videos, or just for fun. This project is in early development; there is no packaged download yet — run from source.

Languages: Rust (100%)

## Features
- Multiple modular "FakeModules" producing different cinematic hacker effects
- Run through all featured modules in sequence or run a single module
- `--loop` mode to repeat output indefinitely for live demos
- Simple to extend by adding new modules under `src/modules/`


## Quickstart (run from source)
1. Clone the repo:
```bash
git clone https://github.com/AkaruiYami/fakeit-tech.git
cd fakeit-tech
```

2. Build and run with cargo:
- Debug run:
```bash
cargo run -- <args...>
```
- Release build and run:
```bash
cargo build --release
./target/release/fakeit-tech <args...>
```

(Replace `<args...>` with the examples below. If you prefer to run via `cargo run`, pass args after `--`: `cargo run -- --loop`.)

## Usage examples
- Run the full cinematic sequence once:
```bash
./target/release/fakeit-tech
```

- Run full sequence in an infinite loop:
```bash
./target/release/fakeit-tech --loop
```

- Run a single module once:
```bash
./target/release/fakeit-tech ai
./target/release/fakeit-tech build
./target/release/fakeit-tech "cypher-square"
./target/release/fakeit-tech hack
```

- Run a module and use `--loop`:
```bash
./target/release/fakeit-tech build --loop
```

- Show help (if the CLI provides it):
```bash
./target/release/fakeit-tech --help
```

## Modules (details)
- ai
  - Produces blue/green training progress lines and increments progress to 100%.
- build
  - Simulates a build pipeline with steps like "Collecting source files", file path outputs, random warnings/errors, and a final "[100%] Build finished successfully!" message.
- cypher-square
  - Renders a grid of randomized characters that continuously mutate and colors some characters as errors; it hides the cursor while running and restores it on exit.
- hack
  - Prints a fake IP and a single "Scanning" line for quick dramatic effects.

Module code lives under `src/modules/` and is automatically registered at startup via the `registry` module.

## Development
- Add a new module:
  1. Create a Rust file under `src/modules/` (e.g., `my_module.rs`).
  2. Implement the `FakeModule` trait from `crate::engine`:
     - `fn name(&self) -> &str` — return the module name used on the CLI.
     - `fn run(&self, rng: &mut ThreadRng)` — perform the module output.
  3. Register the module at compile time:
     - `#[ctor::ctor] fn register_build() { registry::register(Box::new(MyModule)); }`

- Registry behavior:
  - `src/modules/registry.rs` uses `lazy_static` to provide a global `MODULES` and a `get_registered()` function that drains the registration list. Modules register themselves at compile time using `ctor`.

- Run locally while developing:
```bash
cargo run -- <module-name>
```

- Notable dependencies used in modules:
  - `rand` for random output behavior
  - `colored` for terminal colors
  - `ctor` for compile-time module registration
  - `lazy_static` for registry
  - `ctrlc` in cypher-square to restore cursor state on Ctrl-C

## Contributing
Contributions welcome! Suggested workflow:
1. Open an issue to discuss large changes or a new module idea.
2. Create a branch:
```bash
git checkout -b feat/<name>
```
3. Implement the module or change, add tests or examples, and open a PR.


## Contact
Maintained by AkaruiYami — open issues for bugs, feature requests, or module ideas.

--- written by GitHub Copilot
