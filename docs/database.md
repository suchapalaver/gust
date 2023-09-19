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

Here's an example of a very small example JSON store
with just two recipes:

```json
{
    "sections": [
        "fresh",
        "pantry",
        "protein",
        "dairy",
        "freezer"
    ],
    "collection": [
        {
            "name": "eggs",
            "section": "dairy",
            "recipes": [
                "oatmeal chocolate chip cookies",
                "fried eggs for breakfast"
            ]
        },
        {
            "name": "milk",
            "section": "dairy",
            "recipes": []
        },
        {
            "name": "spinach",
            "section": "fresh",
            "recipes": [
                "fried eggs for breakfast"
            ]
        },
        {
            "name": "beer",
            "section": "dairy",
            "recipes": []
        },
        {
            "name": "unsalted butter",
            "section": "dairy",
            "recipes": [
                "oatmeal chocolate chip cookies",
                "fried eggs for breakfast"
            ]
        },
        {
            "name": "bread",
            "section": "pantry",
            "recipes": [
                "fried eggs for breakfast"
            ]
        },
        {
            "name": "old fashioned rolled oats",
            "section": "pantry",
            "recipes": [
                "oatmeal chocolate chip cookies"
            ]
        },
        {
            "name": "chocolate chips",
            "section": "pantry",
            "recipes": [
                "oatmeal chocolate chip cookies"
            ]
        },
        {
            "name": "baking powder",
            "section": "pantry",
            "recipes": [
                "oatmeal chocolate chip cookies"
            ]
        },
        {
            "name": "baking soda",
            "section": "pantry",
            "recipes": [
                "oatmeal chocolate chip cookies"
            ]
        },
        {
            "name": "salt",
            "section": "pantry",
            "recipes": [
                "oatmeal chocolate chip cookies"
            ]
        },
        {
            "name": "white sugar",
            "section": "pantry",
            "recipes": [
                "oatmeal chocolate chip cookies"
            ]
        },
        {
            "name": "vanilla extract",
            "section": "pantry",
            "recipes": [
                "oatmeal chocolate chip cookies"
            ]
        },
        {
            "name": "whole-wheat flour",
            "section": "pantry",
            "recipes": [
                "oatmeal chocolate chip cookies"
            ]
        },
        {
            "name": "1/2 & 1/2",
            "section": "dairy",
            "recipes": [
                "fried eggs for breakfast"
            ]
        },
        {
            "name": "feta",
            "section": "dairy",
            "recipes": [
                "fried eggs for breakfast"
            ]
        }
    ],
    "recipes": [
        "oatmeal chocolate chip cookies",
        "fried eggs for breakfast"
    ]
}
```

### SQLite

SQLite is the default storage option.

---
## Migrating from JSON to SQLite

You can migrate a JSON store to an Sqlite database by running

```bash
cargo run -- --database sqlite migrate-json-store
```
