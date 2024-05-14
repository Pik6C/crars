use std::io::{self, stdout};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Hide, Show};
use crossterm::execute;
use termion::raw::IntoRawMode;

pub fn newbuff() -> std::io::Result<()>{ 
    let mut stdout = stdout();
    execute!(stdout, Hide, EnterAlternateScreen, Clear(ClearType::All))?;
    Ok(())
}

pub fn closebuff() -> io::Result<()>
{
    let mut stdout = stdout();
    execute!(stdout, Show, LeaveAlternateScreen,)?;
    Ok(())
}

pub fn rawmode() -> io::Result<()>
{
    let mut stdout = stdout().into_raw_mode().unwrap();
    Ok(())
}