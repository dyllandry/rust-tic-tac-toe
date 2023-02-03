# Rust Tic-tac-toe

## test modules with "tests" in the name

I think bug in vim plugin vim-test where tests in nested test modules aren't run by `:TestNearest` unless they are in a module with "tests" in the name.

I made an [issue](https://github.com/vim-test/vim-test/issues/708) for this in the vim-test repo.

## Architecture

- Module for game state & logic
	- This is the TicTacToe structure
- Game loop
	- Keeps grabbing input until someone wins
- Read input from system and give to board
		- turns character 1 into "MarkCell1" and gives to the TicTacToe game

