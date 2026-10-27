# nn-zero-to-hero-rs

Rust implementations of Karpathy's Neural Networks: Zero to Hero, written by hand as a learning exercise.
The point is that the human builds the understanding. An agent that writes the solution defeats the repo.

## Do not write the implementation

- Never fill in a `todo!()` body or write lecture logic (forward ops, backward, training loops, model code) unless the user explicitly says to write that specific piece.
- When the user is stuck, show the intermediate reasoning (the math, the chain rule step, the ownership problem) and ask a question that points at the next step. Do not hand over finished code.
- When the user brings design candidates, compare their trade-offs. Leave the choice to them.
- Review the user's code when asked: correctness first, then idiomatic Rust (Rust API Guidelines), then performance.

You may write freely: tests that pin the lecture's expected values, scaffolding (new crates, `todo!()` stubs, Cargo config), README updates, and explanations.

## Layout

- One crate per lecture (`micrograd`, then `makemore`, `nanogpt`, ...). Add each to `members` in the root `Cargo.toml` and to the table in `README.md`.
- Test values come from the lecture or the reference Python repos (karpathy/micrograd, karpathy/makemore, karpathy/nanoGPT). Cite the source in a comment.
- Stay on std until a lecture genuinely needs a dependency, and ask before adding one.

## Commands

- `cargo test` — failing tests on unimplemented `todo!()` are expected progress markers, not something to fix.
- `cargo clippy --all-targets` before committing finished work.

Commit messages are short English imperative sentences.
