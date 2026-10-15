# website

My website — [Dioxus](https://dioxuslabs.com) 0.7 fullstack (Rust) + [Tailwind CSS](https://tailwindcss.com) v4.

## Develop

```bash
nix develop
dx serve --web --fullstack
```

Open <http://localhost:8080>. `dx serve` rebuilds `assets/tailwind.css` from `tailwind.css` whenever the source changes.

The dev shell's `wasm-bindgen-cli` must match the `wasm-bindgen` version in `Cargo.lock`. If `dx` reports a mismatch after `cargo update`, update `flake.lock` too (or pin the crate).

## Build

```bash
dx bundle --web --fullstack --release
```

## Layout

| path | role |
|------|------|
| `src/main.rs` | app root + server functions |
| `tailwind.css` | Tailwind input |
| `assets/` | static assets (incl. generated `tailwind.css`) |
| `AGENTS.md` | agent rules: keep `notes/specs/` as the live user spec |
| `notes/specs/` | spec docs — current desired state of every user desire and instruction |
