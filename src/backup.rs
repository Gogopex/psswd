extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    // Get the standard input stream.
    let stdin = stdin();
    // Get the standard output stream and go to raw mode.
    let mut stdout = stdout().into_raw_mode().unwrap();
    // let mut stdout = stdout();

    write!(
        stdout,
        r"
    psswd: a simple password manager
    
    {}{}q to exit. Type stuff, use alt, and so on.{}
    ",
        // Clear the screen.
        termion::clear::All,
        // Goto (1,1).
        termion::cursor::Goto(1, 1),
        // Hide the cursor.
        termion::cursor::Hide
    )
    .unwrap();

    // stdout.flush().unwrap();
    
    for k in stdin.keys() {
        // Clear the current line.
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::CurrentLine
        )
        .unwrap();

        // Print the key we type...
        match k.unwrap() {
            // Exit.
            Key::Char('q') => break,
            Key::Char(k) => println!("{}", k),
            Key::Alt(k) => println!("Alt-{}", k),
            _ => println!("Other"),
        }

        // Flush again.
        // stdout.flush().unwrap();
    }

    // Show the cursor again before we exit.
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
