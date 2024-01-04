# Databases and Storage

## Contents

- [Storage Options](#storage-options)
- [Importing from files](#importing-from-files)

## Storage Options

Here are the supported storage options:

- [x] [SQLite](#sqlite)
- [ ] Postgres

### SQLite

SQLite is the default storage option.

### PostgreSQL

Coming!

---
## Importing from files

You can import from JSON files, for now named by default `items.json` and `list.json`
to SQLite by running

```bash
cargo run -- --database sqlite import
```
