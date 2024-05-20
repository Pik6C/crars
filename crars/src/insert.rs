use crossterm::*;
use crossterm::cursor::{MoveTo,MoveUp,MoveDown};
use crossterm::event::*;
use crate::insert::style::*;
use std::io::Write;

pub fn input_events() -> std::io::Result<()> {
    loop {
        if let Event::Key(KeyEvent {
            code,
            modifiers,
            kind,
            state,
        }) = read()?
        {
            if code == KeyCode::Esc {
                break;
            }
            println!("{:?}{:?}{:?}{:?}", code, modifiers, kind, state);
        }
    }
    Ok(())
}




pub fn insert(filestr:String) -> bool{
    true

}