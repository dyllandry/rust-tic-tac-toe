- game
	- players
	- board
- input
	- reads input from keyboard & gives to game


would be cool if you could mark cells using an enum like

board.mark_cell(CellPosition::TopLeft, Player::p1)

CellPosition since Cell is already a struct

Could change Cell.index to Cell.position: CellPosition

Would get rid of the technical confusion about modeling cells as numbered 0-8 then having the board print out 1-9 to the user.
