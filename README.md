# Rust fullstack app

This is just a learning project. Nothing is supser series about this.

# Requirements

- [Setup rust, trunk, cargo](#install-rust)
- [Setup PostgreSQL](#install-postgresql)

# Run the program

Execute the "start_dev.sh" script to start the backend & frontend.
```
./start_dev.sh
```


# Install rust

1. Install rust [Rust official install guide](https://www.rust-lang.org/tools/install)
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Note** This may require you to restart your terminal.

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

# Install PostgreSQL

I'm using Arch linux. So this install guide will show the installlation on Arch.
[Original guide](https://wiki.archlinux.org/title/PostgreSQL#Installation)

1. Install PostgreSQL
```
sudo pacman -S postgresql
```

2. Switch to the `postgres` user
```
sudo -iu postgres
```

3. Initial configuration
```
initdb -D /var/lib/postgres/data
```

4. Exit out of the postgress user and start the service
```
sudo systemctl start postgresql.service
sudo systemctl enable postgresql.service
```