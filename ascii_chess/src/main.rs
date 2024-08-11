mod board;
use board::Matrix;
use crossterm::event::{read, Event, KeyCode, KeyEventKind};

fn main() {
    print!("\x1B[2J"); //clears terminal
    print!("\x1B[H"); //move cursor top left

    let mut select_mode: bool = false;
    let mut chess_board = Matrix::new(); //generates board with no pieces
    chess_board.populate_pieces(); //populates board with pieces
    chess_board.display(); //displays to screen

    loop {
        let input = read();
        match input {
            //github wiork
            Ok(Event::Key(key_event)) => {
                if key_event.kind == KeyEventKind::Release {
                    continue;
                }
                match key_event.code {
                    KeyCode::Char('a') => {
                        chess_board.mutate_arrow((0, -1));
                        update_terminal(&mut chess_board);
                    }
                    KeyCode::Char('d') => {
                        chess_board.mutate_arrow((0, 1));
                        update_terminal(&mut chess_board);
                    }
                    KeyCode::Char('w') => {
                        chess_board.mutate_arrow((-1, 0));
                        update_terminal(&mut chess_board);
                    }
                    KeyCode::Char('s') => {
                        chess_board.mutate_arrow((1, 0));
                        update_terminal(&mut chess_board);
                    }
                    KeyCode::Char(' ') => {
                        chess_board.select_piece();
                        select_mode = true;
                        //selection mode, select piece and append to an appropriate coordinate to move the piece
                    }
                    KeyCode::Enter => {
                        //PROBLEM: move arrows to place you want to move,
                        if select_mode {
                            select_mode = chess_board.movement(chess_board.selected_piece);
                            update_terminal(&mut chess_board);
                        }
                    }

                    KeyCode::Esc => break,
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn show_keybinds() {
    println!("Useful Keybinds: ");
    println!("Move keys: AWSD");
    println!("Move piece: Enter");
    println!("Select piece to move: Space");
    println!("Quit game: Esc");
}

fn update_terminal(ref_board: &mut Matrix) {
    print!("\x1B[2J"); //clears terminal
    print!("\x1B[H"); //move cursor top left
    ref_board.display();
    //println!("Piece Selected: {:?}",ref_board.arrow);
    //println!("");
}

fn welcome() {
    println!("Welcome to Ascii Chess on the Terminal. Press any key to begin");
    show_keybinds();
}
