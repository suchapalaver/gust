# Docker

To run the application, creating a volume called `grusterylist_data`,
and migrating data from an existing `groceries.json` JSON store, use:

```bash
docker run --rm -v grusterylist_data:/app grusterylist --database sqlite migrate-json-db
```

To read from the persisted migrated data:

```bash
docker run --rm -v grusterylist_data:/app grusterylist --database sqlite read recipes
```
