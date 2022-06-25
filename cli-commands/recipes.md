```
grusterylist-recipes 
Manages recipes library

USAGE:
    grusterylist recipes [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -h, --help           Print help information
        --path <path>    Provides path for groceries library [default: groceries.json]

SUBCOMMANDS:
    add       Adds recipes to library
    delete    Deletes recipe from library
    help      Print this message or the help of the given subcommand(s)
```
---
```
grusterylist-recipes-add 
Adds recipes to library

USAGE:
    grusterylist recipes add --name <name>... --ingredients <ingredients>...

OPTIONS:
    -h, --help                            Print help information
    -i, --ingredients <ingredients>...    Provides name of recipe to be added
    -n, --name <name>...                  Provides name of recipe to be added
```
---
```
grusterylist-recipes-delete 
Deletes recipe from library

USAGE:
    grusterylist recipes delete --name <name>...

OPTIONS:
    -h, --help              Print help information
        --name <name>...    Provides name of recipe to be deleted
```
---
## WISHES
`$ cargo run -- recipes edit --name <name> --add <ingredient>`
`$ cargo run -- recipes edit --name <name> --edit <ingredient>`
`$ cargo run -- recipes edit --name <name> --delete <ingredient>`
