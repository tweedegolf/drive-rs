# drive-rs

A WIP alternative search for crates on awesome embedded Rust.

Please don't judge my web skills... I usually write embedded code.

# Running the project

Fetch the db-dump from [crates.io](https://crates.io):

```bash
wget https://static.crates.io/db-dump.tar.gz
```

Run the backend to extract the data:

```bash
cargo run --release --bin read-driver-db
```

Copy the output to the frontend:

```bash
cp full-crate-db.json frontend/src/
```

Install frontend dependencies:

```bash
npm install --prefix frontend
```

Run the frontend:

```bash
npm run dev --prefix frontend
```
