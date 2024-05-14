use std::io::{self, stdout};

use crossterm::cursor::{Hide, Show};
use crossterm::terminal::{disable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;

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
    disable_raw_mode()?;
    Ok(())
}