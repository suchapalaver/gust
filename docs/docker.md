# Docker

## Contents

- [Build](#build)
- [`run`](#run)

## Build

```bash
docker build --tag gust --file Dockerfile .
```

## `run`

### Creating a `gust_data` Volume

To run the application, creating a volume called `gust_data`,
and migrating data from an existing `items.json` file store, use:

```bash
docker run --rm -v gust_data:/app gust --database sqlite import
```

### Reading from a `gust_data` Volume

To read from the persisted migrated data:

```bash
docker run --rm -v gust_data:/app gust --database sqlite read recipes
```

### Import From JSON files to SQLite

To use existing items and list saved to JSON and import to SQLite:

```bash
docker run --rm \
-v gust_data:/app \
-v $(pwd)/items.json:/app/items.json \
-v $(pwd)/list.json:/app/list.json \
gust \
--database sqlite \
import
```

Note that for now it has to be `items.json` and `list.json`.
