# Rust fullstack app

This is just a learning project. Nothing is super series about this.

# Index

- [Run the program](#run-the-program)
- [Setup rust, trunk, cargo](#install-rust)
- [Setup PostgreSQL](#install-postgresql)
- [Migrations](#db-migrations)

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

I'm using Arch linux. So this install guide will show the installation on Arch.
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

# DB Migrations

For migration we are using sqlx-cli.  
That is fearly obvius, as we are usin sqlx for the sql connection.

## Index

- [Install](#install-sqlx-cli)
- [Create migrations](#create-migrations)
- [Run migrations](#run-migrations)

## Install SQLx CLI

There is a well documented install guide on their [Github](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli).  

```
# supports all databases supported by SQLx
$ cargo install sqlx-cli

# only for postgres
$ cargo install sqlx-cli --no-default-features --features native-tls,postgres

# use vendored OpenSSL (build from source)
$ cargo install sqlx-cli --features openssl-vendored

# use Rustls rather than OpenSSL (be sure to add the features for the databases you intend to use!)
$ cargo install sqlx-cli --no-default-features --features rustls
```

__NOTE__: Remember to copy the `.env.example` to `.env`.

## Create migrations

```
sqlx migrate add <name>
```

Creates a new file in `migrations/<timestamp>-<name>.sql`. Add your database schema changes to this new file.

## Run migrations

```
sqlx migrate run
```

Compares the migration history of the running database against the migrations/ folder and runs any scripts that are still pending.