pub struct Matrix {
    pub board: [[BlockData; 8]; 8],
    pub arrow: (usize, usize),
    pub selected_piece: (usize, usize),
    pub turn: Colour
}

#[derive(Clone, Copy, PartialEq)]
pub struct BlockData {
    colour: Colour,
    piece: Piece,
}
#[derive(Clone, Copy, PartialEq)]
pub enum Colour {
    Black,
    White,
    Null,
}
#[derive(Clone, Copy, PartialEq)]
pub enum Piece {
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
    Null,
}

impl Piece {
    fn to_string(&self, colour: Colour) -> &str {
        match self {
            Piece::Pawn => {
                return if Colour::Black == colour {
                    "♟ "
                } else {
                    "♙ "
                }
            }
            Piece::Rook => {
                return if Colour::Black == colour {
                    "♜ "
                } else {
                    "♖ "
                }
            }
            Piece::Bishop => {
                return if Colour::Black == colour {
                    "♝ "
                } else {
                    "♗ "
                }
            }
            Piece::Knight => {
                return if Colour::Black == colour {
                    "♞ "
                } else {
                    "♘ "
                }
            }
            Piece::Queen => {
                return if Colour::Black == colour {
                    "♛ "
                } else {
                    "♕ "
                }
            }
            Piece::King => {
                return if Colour::Black == colour {
                    "♚ "
                } else {
                    "♔ "
                }
            }

            Piece::Null => {
                return if Colour::Black == colour {
                    "⬛"
                } else {
                    "⬜"
                }
            }
        }
    }
}

impl Matrix {
    pub fn new() -> Matrix {
        let mut board: [[BlockData; 8]; 8] = [[BlockData {
            colour: Colour::Null,
            piece: Piece::Null,
        }; 8]; 8];
        for row in 0..8 {
            for block in 0..8 {
                //checkerboards the board, so that every odd row starts with white, and every even starts with black
                board[row][block].colour = if row % 2 == 0 {
                    if block % 2 == 0 {
                        Colour::Black
                    } else {
                        Colour::White
                    }
                } else {
                    if block % 2 == 0 {
                        Colour::White
                    } else {
                        Colour::Black
                    }
                };
            }
        }

        Matrix {
            board: board,
            arrow: (7, 0),
            selected_piece: (7, 0),
            turn: Colour::White
        }
    }

    fn switch_pieces(&mut self, piece_moved: (usize, usize), spot: (usize, usize)) {
        self.board[spot.0][spot.1] = self.board[piece_moved.0][piece_moved.1];
        //sets consistant tile color for checkerboard pattern
        let new_colour: Colour = if piece_moved.0 % 2 == 0 {
            if piece_moved.1 % 2 == 0 {
                Colour::Black
            } else {
                Colour::White
            }
        } else {
            if piece_moved.1 % 2 == 0 {
                Colour::White
            } else {
                Colour::Black
            }
        };
        let new_blockdata = BlockData {
            colour: new_colour,
            piece: Piece::Null,
        }; //defines new blockdata variable to set consistent tile colors

        self.board[piece_moved.0][piece_moved.1] = new_blockdata;
    }

    //checks if a piece is taking its teammate (false==taking its teammate, true==valid movement)
    fn validate_friend(&self, piece_team: Colour, move_coords: (usize, usize)) -> bool {
        let spot_info = self.board[move_coords.0][move_coords.1];
        if spot_info.piece == Piece::Null {
            //if the spot piece is jumping to is a tile
            return true;
        }
        if spot_info.colour == piece_team {
            return false;
        } else {
            return true;
        }
    }

    //returns false if piece hop is invalid, true if piece hop is valid.
    //only use for bishop, rook, and queen
    fn check_piece_hop(&self, piece_coords: (usize, usize), move_coords: (usize, usize)) -> bool {
        //checks if piece is moving straight horizontally or vertically
        if piece_coords.0 == move_coords.0 || piece_coords.1 == move_coords.1 {
            let mut range: (usize, usize, bool) = (0, 0, false); //bool determines vertical or horizontal movement
            if piece_coords.0 == move_coords.0 {
                range.0 = piece_coords.1.min(move_coords.1);
                range.1 = piece_coords.1.max(move_coords.1);
                range.2 = false; //false for horizontal movement
            } else if piece_coords.1 == move_coords.1 {
                range.0 = piece_coords.0.min(move_coords.0);
                range.1 = piece_coords.0.max(move_coords.0);
                range.2 = true; //true for vertical movement
            } else {
                return false;
            }
            for index in range.0 + 1..range.1 {
                let spot = self.board[if range.2 { index } else { piece_coords.0 }]
                    [if range.2 { piece_coords.1 } else { index }];
                if spot.piece != Piece::Null {
                    return false;
                }
            }
            return true;
        }
        //checks if piece is moving in a diagnol fashion
        else if (piece_coords.0 + piece_coords.1) == (move_coords.0 + move_coords.1)
            || (piece_coords.1 as i32 - piece_coords.0 as i32)
                == (move_coords.1 as i32 - move_coords.0 as i32)
        {
            let range_x: (usize, usize) = (
                piece_coords.0.min(move_coords.0),
                piece_coords.0.max(move_coords.0),
            );
            let range_y: (usize, usize) = (
                if piece_coords.0 == range_x.0 {
                    piece_coords.1
                } else {
                    move_coords.1
                },
                if piece_coords.0 == range_x.1 {
                    piece_coords.1
                } else {
                    move_coords.1
                },
            );
            let mut count = if range_y.0 < range_y.1 { 1 } else { -1 };
            for index in range_x.0 + 1..range_x.1 {
                let spot: BlockData = self.board[index][(range_y.0 as i32 + count) as usize];
                count += if range_y.0 < range_y.1 { 1 } else { -1 };
                if spot.piece != Piece::Null {
                    return false;
                }
            }
            return true;
        } else {
            return false;
        }
    }


    fn toggle_turn(&mut self){
        if self.turn == Colour::Black{
            self.turn = Colour::White;
        }
        else if self.turn == Colour::White{
            self.turn = Colour::Black;
        }
        else{
            self.turn = Colour::Null;
        }
    }

    pub fn movement(&mut self, piece_coords: (usize, usize)) -> bool {
        //returns true if movement was a success, false if piece failed to move
        let piece_type = self.board[piece_coords.0][piece_coords.1];
        //check if it is the selected piece's colors turn to go
        if !(piece_type.colour == self.turn){
            return false;
        }

        match piece_type.piece {
            Piece::Pawn => {
                //validate if spot jumping to is occupied by friendly piece
                if !self.validate_friend(piece_type.colour, self.arrow) {
                    return false;
                }
                //creates a list of 4 possible spots pawn can move
                let mut valid_spots: [(i32, i32); 4] = [(0, 0); 4];
                //typecasting piece and arrow coords to i32 because usize doesnt support negatives and i'm too lazy to create a custom exception
                let temp_coords: (i32, i32) = (piece_coords.0 as i32, piece_coords.1 as i32);
                let temp_arrow: (i32, i32) = (self.arrow.0 as i32, self.arrow.1 as i32);
                //one spot straight ahead
                valid_spots[0] = (
                    if piece_type.colour == Colour::Black {
                        temp_coords.0 - 1
                    } else {
                        temp_coords.0 + 1
                    },
                    temp_coords.1,
                );
                //two spots ahead
                valid_spots[1] = (
                    if piece_type.colour == Colour::Black {
                        temp_coords.0 - 2
                    } else {
                        temp_coords.0 + 2
                    },
                    temp_coords.1,
                );
                //to the right and to the left, taking an enemy piece
                valid_spots[2] = if piece_type.colour == Colour::Black {
                    (temp_coords.0 - 1, temp_coords.1 + 1)
                } else {
                    (temp_coords.0 + 1, temp_coords.1 + 1)
                };
                valid_spots[3] = if piece_type.colour == Colour::Black {
                    (temp_coords.0 - 1, temp_coords.1 - 1)
                } else {
                    (temp_coords.0 + 1, temp_coords.1 - 1)
                };

                //check if new spot is not an option in the valid_spots list
                if !valid_spots.contains(&temp_arrow) {
                    return false;
                }

                //validate if there is an empty slot when moving diagnoly
                if temp_arrow == valid_spots[2] || temp_arrow == valid_spots[3] {
                    if self.board[self.arrow.0][self.arrow.1].piece == Piece::Null {
                        return false;
                    }
                }

                //validate if there is a piece in front of the pawn when moving straight
                if temp_arrow == valid_spots[0] {
                    if self.board[self.arrow.0][self.arrow.1].piece != Piece::Null {
                        return false;
                    }
                }

                //validate conditions for jumping 2 spaces (no space in between, first pawn movement for instance)
                if temp_arrow == valid_spots[1] {
                    if piece_coords.0 != 6 && piece_type.colour == Colour::Black {
                        return false;
                    }
                    if piece_coords.0 != 1 && piece_type.colour == Colour::White {
                        return false;
                    }
                    if self.board[if Colour::Black == piece_type.colour {
                        piece_coords.0 - 1
                    } else {
                        piece_coords.0 + 1
                    }][piece_coords.1]
                        .piece
                        != Piece::Null
                    {
                        return false;
                    }
                }

                self.switch_pieces(piece_coords, self.arrow);
                self.toggle_turn();
                return true;
            }
            Piece::Knight => {
                if !self.validate_friend(piece_type.colour, self.arrow) {
                    return false;
                }
                let mut new_spots: [(usize, usize); 8] = [(0, 0); 8];
                for index in 0..8 {
                    let mut x_pos: i32 = 0;
                    let mut y_pos: i32 = 0;
                    if index % 2 == 0 {
                        x_pos = if index > 3 { 2 } else { -2 };
                    } else {
                        y_pos = if index > 3 { 2 } else { -2 };
                    }
                    if 2 <= index && index <= 5 {
                        if x_pos == 0 {
                            x_pos = -1;
                        } else if y_pos == 0 {
                            y_pos = -1;
                        }
                    } else {
                        if x_pos == 0 {
                            x_pos = 1;
                        } else if y_pos == 0 {
                            y_pos = 1;
                        }
                    }

                    new_spots[index] = (
                        (piece_coords.0 as i32 + x_pos) as usize,
                        (piece_coords.1 as i32 + y_pos) as usize,
                    );
                }

                if !new_spots.contains(&self.arrow) {
                    return false;
                } else {
                    self.switch_pieces(piece_coords, self.arrow);
                    self.toggle_turn();
                    return true;
                }
            }
            Piece::Rook => {
                //cases in which rook should not move. Includes jumping to the same tile currently residing
                if self.arrow == piece_coords {
                    return false;
                }
                //taking friendly piece
                if !self.validate_friend(piece_type.colour, self.arrow) {
                    return false;
                }
                //trying to move to a tile either not in the same x plane or y plane
                if self.arrow.0 != piece_coords.0 && self.arrow.1 != piece_coords.1 {
                    return false;
                }
                //invalidating false jumps over extra pieces
                if !self.check_piece_hop(piece_coords, self.arrow) {
                    return false;
                }

                self.switch_pieces(piece_coords, self.arrow);
                self.toggle_turn();
                return true;
            }

            Piece::Bishop => {
                if self.arrow == piece_coords {
                    return false;
                }
                if !self.validate_friend(piece_type.colour, self.arrow) {
                    return false;
                }
                if self.arrow.0 + self.arrow.1 != piece_coords.0 + piece_coords.1
                    && self.arrow.0 as i32 - self.arrow.1 as i32
                        != piece_coords.0 as i32 - piece_coords.1 as i32
                {
                    return false;
                }
                //disable bishop piece hopping
                if !self.check_piece_hop(piece_coords, self.arrow) {
                    return false;
                }
                self.switch_pieces(piece_coords, self.arrow);
                self.toggle_turn();
                return true;
            }
            Piece::Queen => {
                if self.arrow == piece_coords {
                    return false;
                }
                if !self.validate_friend(piece_type.colour, self.arrow) {
                    return false;
                }
                if !self.check_piece_hop(piece_coords, self.arrow) {
                    return false;
                }

                self.switch_pieces(piece_coords, self.arrow);
                self.toggle_turn();
                return true;
            }
            Piece::King => {
                if self.arrow == piece_coords {
                    return false;
                }
                if !self.validate_friend(piece_type.colour, self.arrow) {
                    return false;
                }
                if !(self.arrow.0==piece_coords.0+1 || self.arrow.0 as i32 ==piece_coords.0 as i32-1 || self.arrow.0 == piece_coords.0){
                    return false;
                }
                if !(self.arrow.1==piece_coords.1+1 || self.arrow.1 as i32 ==piece_coords.1 as i32 -1 || self.arrow.1 == piece_coords.1){
                    return false;
                }

                self.switch_pieces(piece_coords, self.arrow);
                self.toggle_turn();
                return true;
            }

            Piece::Null => {
                return false;
            }
        }
    }

    pub fn select_piece(&mut self) {
        self.selected_piece = (self.arrow.0, self.arrow.1);
    }

    pub fn populate_pieces(&mut self) {
        self.pawn_layout(1, Colour::White);
        self.pawn_layout(6, Colour::Black);
        self.piece_layout(0, Colour::White);
        self.piece_layout(7, Colour::Black);
    }

    pub fn mutate_arrow(&mut self, displace: (i32, i32)) {
        if self.arrow.0 == 7 && displace.0 > 0 {
            self.arrow.0 = 0;
        } else if self.arrow.0 == 0 && displace.0 < 0 {
            self.arrow.0 = 7;
        } else if self.arrow.1 == 7 && displace.1 > 0 {
            self.arrow.1 = 0;
        } else if self.arrow.1 == 0 && displace.1 < 0 {
            self.arrow.1 = 7;
        } else {
            self.arrow.0 = (self.arrow.0 as i32 + displace.0) as usize;
            self.arrow.1 = (self.arrow.1 as i32 + displace.1) as usize;
        }
    }

    pub fn display(&mut self) {
        for (index, row) in self.board.iter().enumerate() {
            if index == self.arrow.0 {
                print!(">");
            } else {
                print!(" ");
            }
            for item in row {
                let ref_colour = item.colour;
                print!("{}", item.piece.to_string(ref_colour));
            }
            print!("\n");
        }
        print!(" {}^", " ".repeat(self.arrow.1 * 2));

        print!("\n");
    }

    fn pawn_layout(&mut self, row: usize, colour: Colour) {
        for item in 0..8 {
            let ref_board = &mut self.board[row][item];
            ref_board.piece = Piece::Pawn;
            ref_board.colour = if colour == Colour::White {
                Colour::White
            } else {
                Colour::Black
            };
        }
    }
    fn piece_layout(&mut self, row: usize, colour: Colour) {
        let layout = [
            Piece::Rook,
            Piece::Knight,
            Piece::Bishop,
            if colour == Colour::White {
                Piece::King
            } else {
                Piece::Queen
            },
            if colour == Colour::White {
                Piece::Queen
            } else {
                Piece::King
            },
            Piece::Bishop,
            Piece::Knight,
            Piece::Rook,
        ];
        for (index, item) in layout.iter().enumerate() {
            let ref_board = &mut self.board[row][index];
            ref_board.colour = colour;
            ref_board.piece = *item;
        }
    }
}
