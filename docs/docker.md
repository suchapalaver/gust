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
and migrating data from an existing `groceries.json` JSON store, use:

```bash
docker run --rm -v gust_data:/app gust --database sqlite migrate-json-store
```

### Reading from a `gust_data` Volume

To read from the persisted migrated data:

```bash
docker run --rm -v gust_data:/app gust --database sqlite read recipes
```

### Migrate a JSON `gust` Store to SQLite

To use an existing JSON store and migrate it to SQLite:

```bash
docker run --rm \
-v gust_data:/app \
-v /host/machine/absolute/path/to/groceries.json:/app/groceries.json \
-v /host/machine/absolute/path/to/list.json:/app/list.json \
gust \
--database sqlite \
migrate-json-store
```

Note that for now it has to be `groceries.json` and `list.json`.
