# `grusterylist`: makes grocery lists, written in Rust  
`grusterylist` uses and can add to local libraries of user-added recipes and grocery items to put together shopping lists
## how to ... 
- run `$ cargo run -- --help` for available options  
### example shopping list
```
Print shopping list?
(*y* for yes, *any other key* to continue)
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
- make `grusterylist` the present working directory by entering `$ cd grusterylist`
(no need to type the `$`, that's just a convention to show that these are command line interface commands)
### note
`recipes.json`, `groceries.json`, and `list.json` contain libraries of some recipes, groceries, and a saved list that could be emptied out manually to start from scratch. The `cargo run -- --list` option allows you to begin a new shopping list or use the most recently saved list.
