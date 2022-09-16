use anyhow::Result;
use chess::*;
use std::io::{self, Write};

fn main() -> Result<()> {
    let mut board = GameBoard::new();

    loop {
        println!("{}", board);
        print!("(src dst)> ");
        io::stdout().flush()?;

        let user_inp = 'outer: loop {
            let mut inp = String::new();
            io::stdin().read_line(&mut inp)?;

            let inp = inp.split_whitespace().collect::<Vec<&str>>();
            if inp.len() != 2 {
                continue;
            }

            let mut inp_vec = Vec::new();

            for token in inp {
                if token.chars().count() != 2 {
                    continue 'outer;
                }

                let mut token_chars = token.chars();

                let token_col = match token_chars.next() {
                    Some(c) => match c {
                        'a'..='h' => c.to_ascii_uppercase() as u8 - 65,
                        _ => continue 'outer,
                    },
                    _ => unreachable!(),
                };

                let token_row = match token_chars.next() {
                    Some(c) => {
                        if !c.is_ascii_digit() {
                            continue 'outer;
                        }

                        c.to_digit(10).unwrap() - 1
                    }
                    _ => unreachable!(),
                };

                inp_vec.push(Position::new(token_row as u8, token_col).unwrap());
            }

            break inp_vec;
        };

        board.move_piece(user_inp[0], user_inp[1])?;
    }
}
