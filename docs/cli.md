# CLI

For a menu of CLI commands run:

```bash
cargo run -- --help
```

The CLI help menu should look something like this:

```text
gust: makes grocery lists, written in rust

Usage: gust [OPTIONS] <COMMAND>

Commands:
  add              add stuff
  delete           delete stuff
  read             read stuff
  update           update stuff
  migrate-json-db  migrate JSON store to Sqlite database
  help             Print this message or the help of the given subcommand(s)

Options:
      --database <db>  which database to use [default: json] [possible values: json, sqlite]
  -h, --help           Print help
```
