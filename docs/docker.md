# Docker

To run the application, creating a volume called `gust_data`,
and migrating data from an existing `groceries.json` JSON store, use:

```bash
docker run --rm -v gust_data:/app gust --database sqlite migrate-json-db
```

To read from the persisted migrated data:

```bash
docker run --rm -v gust_data:/app gust --database sqlite read recipes
```

To use an existing JSON store and migrate it to SQLite:

```bash
docker run --rm \
-v gust_data:/app \
-v /host/machine/absolute/path/to/file.json:/app/groceries.json \
gust \
--database sqlite \
migrate-json-db
```
