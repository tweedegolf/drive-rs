# drive-rs

A search and discovery tool for Rust drivers for embedded components.

# Adding a driver to the list

1. Add a new file in [`driver-db`](driver-db), named `<crate-name>.toml`
2. Either copy over one of the other drivers contests, or use a [taplo](https://taplo.tamasfe.dev/) based editor
   extension like [even better toml](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml) to
   fill in fields
3. (optional) run `taplo fmt` and `taplo lint` to check the file is valid
4. (if applicable) remove the matching line from the legacy list in [`aer.csv`](aer.csv)
5. [Open a PR](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request)
   to this repo with the added driver

Once we have figured out which data is useful to list, we plan to read this data from published crates. But for now
keeping all the data in one place makes it easier to change fields.

# Contributing to the project

This project is still a work in progress and any help is appreciated. If you have an idea for how to improve it feel
free to open issues and PRs.

Some tasks that need help right now:

1. Add more information to listed drivers (see previous point), a list of missing drivers can be found in [
   `aer.csv`](aer.csv)
2. Improving the layout and design of the listing
3. Make it more intuitive to find good chips to use for newcomers

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
