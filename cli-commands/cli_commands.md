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
        --lib-path <libpath>    Provides path for groceries library [default: groceries.json]
        --path <path>                Provides path for shopping list [default: list.json]
```
---
