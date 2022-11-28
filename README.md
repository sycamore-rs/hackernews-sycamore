# hackernews-sycamore

This is a simple Hacker News SPA built using [Sycamore](https://sycamore-rs.netlify.app) in approx.
500 lines of Rust.

## Building

Requirements:

- [Trunk](https://trunkrs.dev): Use Trunk to build and bundle the app.

```sh
git clone https://github.com/lukechu10/hackernews-sycamore
cd hackernews-sycamore
rustup target add wasm32-unknown-unknown
npm install
trunk serve --open
```

[Live demo](https://sycamore-rs.github.io/hackernews-sycamore/)
