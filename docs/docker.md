# Docker

## Contents

- [Build](#build)
- [`run`](#run)

## Build

```terminal
docker build --tag gust --file Dockerfile .
```

## `run`

### Creating a `gust_data` Volume

To run the application, creating a volume called `gust_data`,
and migrating data from an existing `items.json` file store, use:

```terminal
docker run --rm -v gust_data:/app gust --database sqlite import
```

### Reading from a `gust_data` Volume

To read from the persisted migrated data:

```terminal
docker run --rm -v gust_data:/app gust --database sqlite read recipes
```

### Import From JSON files to SQLite

To use existing items and list saved to JSON and import to SQLite:

```terminal
docker run --rm \
-v gust_data:/app \
-v $(pwd)/items.json:/app/items.json \
-v $(pwd)/list.json:/app/list.json \
gust \
import
```

Note that for now it has to be `items.json` and `list.json`.

### Export data to YAML

Exporting data currently requires writing to files named `items.yaml`
and `list.yaml`, respectively, for items data and list data.

This can be done using the following `docker` command:

```terminal
docker run --rm \
-v gust_data:/app \
-v $(pwd)/items.yaml:/app/items.yaml \
-v $(pwd)/list.yaml:/app/list.yaml gust \
export
```
