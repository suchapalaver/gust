```
grusterylist 
Makes grocery lists

USAGE:
    grusterylist <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    groceries    Manages groceries library
    help         Print this message or the help of the given subcommand(s)
    list         Makes shopping lists
    recipes      Manages recipes library
```
---
```
grusterylist-groceries 
Manages groceries library

USAGE:
    grusterylist groceries [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -h, --help           Print help information
        --path <path>    Provides path for groceries library [default: groceries.json]

SUBCOMMANDS:
    add     Adds grocery items to library
    help    Print this message or the help of the given subcommand(s)
```
---
```
cargo run -- groceries add --help 
grusterylist-groceries-add 
Adds grocery items to library

USAGE:
    grusterylist groceries add

OPTIONS:
    -h, --help    Print help information
```
---
```
grusterylist-list 
Makes shopping lists

USAGE:
    grusterylist list [OPTIONS]

OPTIONS:
    -h, --help                       Print help information
        --lib-path <library path>    Provides path for groceries library [default: groceries.json]
        --path <path>                Provides path for shopping list [default: list.json]
```
---
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
grusterylist-recipes-add 
Adds recipes to library

USAGE:
    grusterylist recipes add --name <name>... --ingredients <ingredients>...

OPTIONS:
    -h, --help                            Print help information
    -i, --ingredients <ingredients>...    Provides name of recipe to be added
    -n, --name <name>...                  Provides name of recipe to be added
```
```
grusterylist-recipes-delete 
Deletes recipe from library

USAGE:
    grusterylist recipes delete --name <name>...

OPTIONS:
    -h, --help              Print help information
        --name <name>...    Provides name of recipe to be deleted
```
