# CLI

## Help

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

## Fetching Recipes

`gust` supports fetching recipes from [BBC Food](https://www.bbc.co.uk/food)
by providing a URL. For example, you can fetch the recipe for scrambled egg
and toast like this:

```bash
cargo run -- fetch --url https://www.bbc.co.uk/food/recipes/scrambledeggandtoast_75736
```

The output should look like this:

```text
Scrambled egg and toast with smoked salmon:
1 tbsp butter, plus extra for spreading:
2 large free-range eggs:
1 tbsp milk:
1 slice wholemeal bread, toasted:
2 slices smoked salmon:
salt and freshly ground black pepper:
```
