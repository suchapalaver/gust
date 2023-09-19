# `gust`: rust-powered grocery list creator

use `gust` to add recipes and grocery items to a local database,
making putting together shopping lists super quick.

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
## example - querying recipes

We can query the recipes we have in our default storage 
option like this:

```bash
docker run --rm -v gust:/app gust read recipes
```

The result should look like this:

```text
oatmeal chocolate chip cookies
hummus
tomato pasta
crispy tofu with cashews and blistered snap peas
crispy sheet-pan noodles

```
