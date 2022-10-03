# Rust fullstack app

This is just a learning project. Nothing is supser series about this.

# Prerequisites (native)

1. Install rust [Rust official install guide](https://www.rust-lang.org/tools/install):
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Note:** This may require you to restart your terminal.

2. Install cargo watch(This is for the backend)
```
cargo install cargo-watch
```

3. Install trunk(This is to serve & compile the frontend) [Yew install guide](https://yew.rs/docs/tutorial#prerequisites)
```
cargo install trunk
```

4. Add wasm32 build target to rust/cargo
```
rustup target add wasm32-unknown-unknown
```

# Run the program

Execute the "start_dev.sh" script to start the backend & frontend.
```
./start_dev.sh
```