# Databases and Storage

## Storage Options

There are two supported storage options:

- JSON
- Sqlite

### JSON

JSON is the default storage option.

### Sqlite

To use the application with Sqlite as the database, use
the `--database` option flag. For example,

```bash
cargo run -- --database sqlite read recipes
```

## Migrating from JSON to Sqlite

You can migrate a JSON store to an Sqlite database by running

```bash
cargo run -- --database sqlite migrate-json-db
```

## Note on Diesel and Running SQL Migrations

The `.env` URL needs to be changed to `./sqlite.db` when running `diesel-cli`
commands, such as `diesel migration run`.
