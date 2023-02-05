# Rust Tic-tac-toe

## test modules with "tests" in the name

I think bug in vim plugin vim-test where tests in nested test modules aren't run by `:TestNearest` unless they are in a module with "tests" in the name.

I made an [issue](https://github.com/vim-test/vim-test/issues/708) for this in the vim-test repo.

## Architecture

- Module for game state & logic
	- This is the TicTacToe structure
- Game loop
	- Keeps giving input to game until game is over, then closes
- Read input from system
	- this is the user_input system

