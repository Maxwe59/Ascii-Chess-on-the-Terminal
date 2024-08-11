pub struct Matrix {
    pub board: [[BlockData; 8]; 8],
    pub arrow: (usize, usize),
    pub selected_piece: (usize, usize),
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

    //checks if a piece is taking its teammate (True==taking its teammate, false==valid movement)
    fn taking_team_piece(&self, piece_team: Colour, move_coords: (usize, usize)) -> bool {
        let spot_info = self.board[move_coords.0][move_coords.1];
        if spot_info.piece == Piece::Null {
            return false;
        }
        if spot_info.colour == piece_team {
            return true;
        } else {
            return false;
        }
    }

    pub fn movement(&mut self, piece_coords: (usize, usize)) -> bool {
        //returns true if movement was a success, false if piece failed to move
        let piece_type = self.board[piece_coords.0][piece_coords.1];
        match piece_type.piece {
            Piece::Pawn => {
                if self.taking_team_piece(piece_type.colour, self.arrow) {
                    return false;
                }
                let new_spot = (
                    if piece_type.colour == Colour::Black {
                        piece_coords.0 - 1
                    } else {
                        piece_coords.0 + 1
                    },
                    piece_coords.1,
                );
                if new_spot != self.arrow {
                    return false;
                }
                self.switch_pieces(piece_coords, self.arrow);
                return true;
            }
            Piece::Knight => {
                if self.taking_team_piece(piece_type.colour, self.arrow) {
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
                    return true;
                }
            }
            Piece::Rook => {
                //cases in which rook should not move. Includes jumping to the same tile currently residing
                if self.arrow == piece_coords {
                    return false;
                }
                //taking friendly piece
                if self.taking_team_piece(piece_type.colour, self.arrow) {
                    return false;
                }
                //trying to move to a tile either not in the same x plane or y plane
                if self.arrow.0 != piece_coords.0 && self.arrow.1 != piece_coords.1 {
                    return false;
                }
                //jumping over an invalid piece when moving horizontally
                if self.arrow.0 == piece_coords.0 {
                    let range = (
                        self.arrow.1.min(piece_coords.1),
                        self.arrow.1.max(piece_coords.1),
                    );
                    for spot in range.0 + 1..range.1 {
                        if self.board[self.arrow.0][spot].piece != Piece::Null {
                            return false;
                        }
                    }
                }
                //jumping over an invalid piece when moving vertically
                else if self.arrow.1 == piece_coords.1 {
                    let range = (
                        self.arrow.0.min(piece_coords.0),
                        self.arrow.0.max(piece_coords.0),
                    );
                    for spot in range.0 + 1..range.1 {
                        if self.board[spot][self.arrow.1].piece != Piece::Null {
                            return false;
                        }
                    }
                }
                self.switch_pieces(piece_coords, self.arrow);
                return true;
            }

            Piece::Bishop => {
                if self.arrow == piece_coords {
                    return false;
                }
                if self.taking_team_piece(piece_type.colour, self.arrow) {
                    return false;
                }
                if self.arrow.0 + self.arrow.1 != piece_coords.0 + piece_coords.1
                    && self.arrow.0 - self.arrow.1 != piece_coords.0 - piece_coords.1
                {
                    return false;
                }
                //disable bishop piece hopping

                self.switch_pieces(piece_coords, self.arrow);
                return true;
            }
            Piece::Queen => {
                return true;
            }
            Piece::King => {
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
