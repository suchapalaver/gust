# `grusterylist`: makes grocery lists, written in Rust  
use 'grusterylist' to add recipes and grocery items to a local library that makes putting together shopping lists super quick.    
## getting started     
- `$ cargo run -- --help` for available options      
- `$ cargo run -- l` allows you to begin a new shopping list or use the most recently saved list. 
          
### example shopping list    
```
Print shopping list?
*y*
*any other key* to continue
y
recipes:
	tomato pasta
groceries:
	garlic
	tomatoes
	basil
	lemons
	pasta
	olive oil
	short grain brown rice
	parmigiana
	eggs
	sausages
	dumplings
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
- [`custom_error`](https://docs.rs/custom_error/latest/custom_error/)    
---
