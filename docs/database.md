# Databases and Storage

## Storage Options

There are two supported storage options:

- JSON
- SQLite

### JSON

To use the application with JSON as the database, use
the `--database` option flag. For example,

```bash
cargo run -- --database json read recipes
```

### SQLite

SQLite is the default storage option.

---
## Migrating from JSON to SQLite

You can migrate a JSON store to an Sqlite database by running

```bash
cargo run -- --database sqlite migrate-json-store
```
