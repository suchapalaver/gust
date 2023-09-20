# `gust`: rust-powered grocery list creator

use `gust` to add recipes and grocery items to a local database,
making putting together shopping lists super quick.

---
## docs

### overview

![`gust` design diagram](./docs/diagrams/design.svg)

### [cli](./docs/cli.md)

- [help](./docs/cli.md#help)
- [fetching recipes](./docs/cli.md#fetching-recipes)

### [database](./docs/database.md)

- [storage options](./docs/database.md#storage-options)
  - [json](./docs/database.md#json)
  - [sqlite](./docs/database.md#sqlite)

### [docker](./docs/docker.md)

- [data volumes](./docker.md#creating-a-gust_data-volume)
- [migrating from JSON to SQLite](./docker.md#migrate-a-json-gust-store-to-sqlite)

---
## getting started

### build the docker image

```bash
docker build --tag gust --file Dockerfile .
```

### help menu

```bash
docker run --rm gust
```

or:

```bash
cargo run -- -h    
```

### fetch a recipe and save it to the database

```bash
docker run --rm -v gust:/app gust fetch --url https://www.bbc.co.uk/food/recipes/vegetable_noodle_pancake_22079
```

### read the recipes in the database

```bash
docker run --rm -v gust:/app gust read recipes
```

### read a recipe's ingredients

```bash
docker run --rm -v gust:/app gust read --recipe 'vegetable noodle pancake'
```

### add a recipe to the list

```bash
docker run --rm -v gust:/app gust add list --recipe 'vegetable noodle pancake'
```

### read the items on the list

```bash
docker run --rm -v gust:/app gust read list
```

### clear the list

```bash
docker run --rm -v gust:/app gust update list clear
```
