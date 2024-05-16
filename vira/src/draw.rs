use crossterm::*;
use crossterm::style::*;
use std::io::Write;

pub fn test_1()-> std::io::Result<()> {

    let mut stdout = std::io::stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    let block = PrintStyledContent("█".magenta());
    for y in 0..20 {
        for x in 0..150 {
            if (y == 0 || y == 20 - 1) || (x == 0 || x == 150 - 1) {
                queue!(stdout, cursor::MoveTo(x, y), block)?;
            }
        }
    }
    stdout.flush()?; // ここでqueueされた順で遅延評価される
    Ok(())
}

pub fn draw1() -> std::io::Result<()>{
    let mut stdout = std::io::stdout();

    let textArear = PrintStyledContent("~".cyan().bold());

    for x in 0..1{
        for n in 0..40{
            if (n == 0 || n == 20 - 1) || (x == 0 || x == 150 - 1) {

                queue!(stdout, cursor::MoveTo(x, n), textArear)?;


            }
        }
    }
    stdout.flush();





    Ok(())
}