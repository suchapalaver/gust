# `gust`: makes grocery lists, written in Rust

use `gust` to add recipes and grocery items to a local database,
making putting together shopping lists super quick.

## getting started

For help menu:

```bash
cargo run -- --help    
```

## Docs

- [cli](./docs/cli.md)
    - [help](./docs/cli.md#help)
    - [fetching recipes](./docs/cli.md#fetching-recipes)
- [database](./docs/database.md)
    - [storage options](./docs/database.md#storage-options)
        - [json](./docs/database.md#json)
        - [sqlite](./docs/database.md#sqlite)
- [docker](./docs/docker.md)

## example - querying recipes

We can query the recipes we have in our sqlite database like this:

```bash
cargo run -- --database sqlite read recipes
```

The result should look like this:

```text
chicken breasts with lemon
oatmeal chocolate chip cookies
cheese and apple snack
hummus
tomato pasta
turkey meatballs
sheet pan salmon with broccoli
peanut butter and jelly on toast
sheet-pan chicken with jammy tomatoes
turkey and cheese sandwiches
fried eggs for breakfast
swordfish pasta
crispy tofu with cashews and blistered snap peas
flue flighter chicken stew
crispy sheet-pan noodles

```
