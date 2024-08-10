pub struct Matrix {
    pub board: [[BlockData; 8]; 8],
    pub arrow: (usize, usize),
    pub selected_piece: (usize,usize)
    
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
            selected_piece: (7,0)
            
        }
    }


    fn switch_pieces(&mut self,piece_moved: (usize, usize), tile: (usize, usize)) {
        self.board[tile.0][tile.1] = self.board[piece_moved.0][piece_moved.1];
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
        let new_blockdata = BlockData{colour:new_colour,piece:Piece::Null}; //defines new blockdata variable to set consistent tile colors

        self.board[piece_moved.0][piece_moved.1] = new_blockdata;
    }

    pub fn movement(&mut self, piece_coords: (usize,usize))->bool{
        let piece_type = self.board[piece_coords.0][piece_coords.1];  
        match piece_type.piece{
            Piece::Pawn => {
                let new_spot = (if piece_type.colour==Colour::Black {piece_coords.0-1} else {piece_coords.0+1},piece_coords.1);
                if new_spot != self.arrow{
                    return false;
                }
                self.switch_pieces(piece_coords,new_spot);
                return true},
            Piece::Knight => {
                let mut count = 0;
                let mut new_spots: [(usize,usize);8] = [(0,0);8];
                for i in 0..2{
                    for j in -2..2{
                        let x_spot: i32 = if j%2==0{if j<0{-2} else{2}} else{0};
                        let y_spot: i32 = if j%2!=0{if j<0{-2} else{2}} else{0};
                        new_spots[count] = (0,0);
                        count += 1;
                    }
                }
                

                if !new_spots.contains(&self.arrow){
                    return false;
                }
                else{
                    self.switch_pieces(piece_coords, self.arrow);
                    return true;
                }
                
            },
            Piece::Bishop => {
                return true;
            }
            Piece::Rook => {
                return true;
            }
            Piece::Queen => {
                return true;
            }
            Piece::King => {
                return true;
            }

            Piece::Null => {return false;}

        }

    }



    pub fn select_piece(&mut self){
        self.selected_piece = (self.arrow.0,self.arrow.1);
    }

    pub fn populate_pieces(&mut self) {
        self.pawn_layout(1, Colour::White);
        self.pawn_layout(6, Colour::Black);
        self.piece_layout(0, Colour::White);
        self.piece_layout(7, Colour::Black);
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
