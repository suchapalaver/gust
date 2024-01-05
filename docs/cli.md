# CLI

## Contents

- [Help](#help)
- [Fetching Recipes](#fetching-recipes)

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
  import           import from 'items.json' and 'list.json' files
  export           export items to 'items.yaml' and list to 'list.yaml' files
  help             Print this message or the help of the given subcommand(s)

Options:
      --database <store>  which database to use [default: sqlite] [possible values: sqlite, sqlite-inmem]
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

## Importing and Exporting Data

See the [Gust Docker documentation](docker.md#) for instructions on how to [import](docker.md#import-from-json-files-to-sqlite)
and [export](docker.md#export-data-to-yaml) your grocery items and shopping list data.

Below is an example of typical `gust` data in JSON format that you could import to your 
`gust` library to start cooking - this example contains grocery items for a
"crispy sheet-pan noodles" recipe we love!:

```json
{
    "collection": [
        {
            "name": "garlic",
            "section": "fresh",
            "recipes": [
                "crispy sheet-pan noodles"
            ]
        },
        {
            "name": "tofu",
            "section": "protein",
            "recipes": [
                "crispy sheet-pan noodles"
            ]
        },
        {
            "name": "vegetable oil",
            "section": "pantry",
            "recipes": [
                "crispy sheet-pan noodles"
            ]
        },
        {
            "name": "salt",
            "section": "pantry",
            "recipes": [
                "crispy sheet-pan noodles"
            ]
        },
        {
            "name": "instant ramen noodles",
            "section": "pantry",
            "recipes": [
                "crispy sheet-pan noodles"
            ]
        },
        {
            "name": "sesame oil",
            "section": "pantry",
            "recipes": [
                "crispy sheet-pan noodles"
            ]
        },
        {
            "name": "soy sauce",
            "section": "pantry",
            "recipes": [
                "crispy sheet-pan noodles"
            ]
        },
        {
            "name": "baby bok choy",
            "section": "fresh",
            "recipes": [
                "crispy sheet-pan noodles"
            ]
        },
        {
            "name": "cilantro",
            "section": "fresh",
            "recipes": [
                "crispy sheet-pan noodles"
            ]
        },
        {
            "name": "hoisin",
            "section": "pantry",
            "recipes": [
                "crispy sheet-pan noodles"
            ]
        },
        {
            "name": "maple syrup",
            "section": "pantry",
            "recipes": [
                "crispy sheet-pan noodles"
            ]
        },
        {
            "name": "sesame seeds",
            "section": "pantry",
            "recipes": [
                "crispy sheet-pan noodles"
            ]
        }
    ]
}
```
