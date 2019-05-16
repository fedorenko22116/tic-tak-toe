extern crate crossterm;

use minimax_game::minimax::{Board, Player, Guesser};
use minimax_game::utils::{build_interface, start_offset, get_offset};
use std::{thread, time::Duration};
use crossterm::{input, InputEvent, KeyEvent, ClearType, Crossterm, AlternateScreen};
use std::io;

fn print_wait_screen() -> io::Result<()> {
    let mut board: Board = vec![Player::Blank; 9];
    let crossterm = Crossterm::new();
    let terminal = crossterm.terminal();
    let cursor = crossterm.cursor();
    let input = input();
    let mut async_stdin = input.read_async();
    let start_offset = start_offset();
    let mut current_pos = start_offset;
    let offset = get_offset();

    loop {
        terminal.clear(ClearType::All)?;

        let mut msg = String::new();

        if winning(&board, &Player::Computer) {
            board = vec![Player::Blank; 9];
            msg = String::from("You loosed!");
        }

        if board.is_finished() {
            board = vec![Player::Blank; 9];
            msg = String::from("Draw!");
        }

        build_interface(&board, &msg);

        if !msg.is_empty() {
            thread::sleep(Duration::from_millis(1500));
        }

        cursor.goto(current_pos.0, current_pos.1)?;

        if let Some(key_event) = async_stdin.next() {
            match key_event {
                InputEvent::Keyboard(event) => match event {
                    KeyEvent::Esc | KeyEvent::Ctrl('c') => {
                        println!("\n\rProgram closing ...\n\r");
                        break
                    },
                    KeyEvent::Right => {
                        if current_pos.0 != start_offset.0 + (offset.0 * 2) {
                            current_pos = (current_pos.0 + 4, current_pos.1)
                        }
                    },
                    KeyEvent::Left => {
                        if current_pos.0 != start_offset.0 {
                            current_pos = (current_pos.0 - 4, current_pos.1)
                        }
                    },
                    KeyEvent::Up => {
                        if current_pos.1 != start_offset.1 {
                            current_pos = (current_pos.0, current_pos.1 - 1)
                        }
                    },
                    KeyEvent::Down => {
                        if current_pos.1 != start_offset.1 + (offset.1 * 2) {
                            current_pos = (current_pos.0, current_pos.1 + 1)
                        }
                    },
                    KeyEvent::Char('x') => {
                        let coord = ((current_pos.0 - start_offset.0) / offset.0,  (current_pos.1 - start_offset.1) / offset.1);
                        let i = (coord.0 + (coord.1 * 3)) as usize;

                        if let Player::Blank = board[i] {
                            board[i] = Player::Human;

                            if !board.is_finished() {
                                let turn = board.guess(&Player::Computer, winning);

                                board[turn.pos as usize] = Player::Computer;
                            }
                        }

                    }
                    _ => { }
                }
                _ => { }
            }
        }
        thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}

fn print_wait_screen_on_alternate_window() -> io::Result<()> {
    if let Ok(_alternate) = AlternateScreen::to_alternate(true) {
        print_wait_screen()?;
    }

    Ok(())
}

fn winning(board: &Board, player: &Player) -> bool {
    (board[0].is(player) && board[1].is(player) && board[2].is(player)) ||
    (board[3].is(player) && board[4].is(player) && board[5].is(player)) ||
    (board[6].is(player) && board[7].is(player) && board[8].is(player)) ||
    (board[0].is(player) && board[3].is(player) && board[6].is(player)) ||
    (board[1].is(player) && board[4].is(player) && board[7].is(player)) ||
    (board[2].is(player) && board[5].is(player) && board[8].is(player)) ||
    (board[0].is(player) && board[4].is(player) && board[8].is(player)) ||
    (board[2].is(player) && board[4].is(player) && board[6].is(player))
}

fn main() {
    print_wait_screen_on_alternate_window().unwrap();
}
