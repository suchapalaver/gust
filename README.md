# `grusterylist`: makes grocery lists, written in Rust  
`grusterylist` uses and can add to local libraries of user-added recipes and grocery items to put together shopping lists
## how to ... 
- run `$ cargo run -- --help` for available options  
- `cargo run -- --list` allows you to begin a new shopping list or use the most recently saved list.  
`recipes.json`, `groceries.json`, and `list.json` contain libraries of some recipes, groceries, and a saved list that could be emptied out manually to start from scratch.  
### example shopping list
```
Print out shopping list?  
--y  
--any other key to continue  
y  
We're making ...
	crispy tofu with cashews and blistered snap peas
We need ...
	extra firm tofu
	vegetable oil
	snap peas
	ginger
	garlic
	13 oz can of unsweetened coconut milk
	soy sauce
	honey
	cashews
	rice vinegar
	scallions
	red pepper flakes
	mint
	milk
```
## in order to clone the repository and run the program using Rust ...
- [clone the `grusterylist` repository](https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository)
- to set up git for the first time [The Odin Project](https://www.theodinproject.com/paths/foundations/courses/foundations/lessons/setting-up-git) has a useful tutorial
- see [The Rust Programming Language's installation chapter](https://doc.rust-lang.org/book/ch01-01-installation.html) if you want help installing Rust
- make `grusterylist` the present working directory by entering  
`$ cd grusterylist`  
(no need to type the `$`, that's just a convention to show that these are command line interface commands)
### note
This is a learning project that uses:  
- [clap](https://docs.rs/clap/2.33.3/clap/) for command line argument parsing  
- [serde](https://docs.serde.rs/serde/index.html) to deserialize and serialize JSON files  
- Rust's [`Result`](https://doc.rust-lang.org/std/result/) type and [`?` operator](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator) to handle errors.
- custom error types (`ReadError`) in Rust
