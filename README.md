# `grusterylist`: makes grocery lists, written in Rust  
`grusterylist` uses and can add to local libraries of user-added recipes and grocery items to put together shopping lists
## how to ...
- make `grusterylist` the present working directory by entering `$ cd grusterylist`  
- run `$ cargo run -- --help` for available options  
(no need to type the `$`, that's just a convention to show that these are command line interface commands)
## in order to clone the repository and run a program using Rust ...
- [clone the grusterylist repository](https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository)
- to set up git for the first time [The Odin Project](https://www.theodinproject.com/paths/foundations/courses/foundations/lessons/setting-up-git) has a useful tutorial
- see [The Rust Programming Language's installation chapter](https://doc.rust-lang.org/book/ch01-01-installation.html) if you want help installing Rust
### note
`recipes.json`, `groceries.json`, and `list.json` contain libraries of some recipes, groceries, and a saved list that could be emptied out manually to start from scratch. The `cargo run -- --list` option allows you to begin a new shopping list or use the most recently saved list.
