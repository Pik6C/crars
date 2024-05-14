use std::io::{stdout, Result, Write};
use std::time::Duration;

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Stylize;
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue, style::PrintStyledContent};
use rand::seq::SliceRandom;

fn main() -> Result<()> {
    let mut stdout = stdout();
    execute!(stdout, Hide, EnterAlternateScreen, Clear(ClearType::All))?;

    let mut maze = [[1usize; MAZE_X]; MAZE_Y];
    dig_maze(11, 11, &mut maze)?;

    execute!(stdout, Show, LeaveAlternateScreen,)?;
    Ok(())
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn new_vec() -> Vec<Direction> {
        vec![Self::Up, Self::Down, Self::Left, Self::Right]
    }
}

// 穴掘り法
// ブロック(WALL)の大きさが2で掘る長さが4なのでそこだけ注意
const MAZE_Y: usize = 31;
const MAZE_X: usize = 51;
const WALL: &str = "██";
const TIME: Duration = Duration::from_millis(10); // ゆっくり見たいときにはここを大きい数字にする
fn dig_maze(mut x: usize, mut y: usize, maze: &mut [[usize; MAZE_X]; MAZE_Y]) -> Result<()> {
    maze[y][x] = 0;
    maze[y][x + 1] = 0;
    let mut direction = Direction::new_vec();
    let mut rng = rand::thread_rng();
    direction.shuffle(&mut rng);
    for d in direction.iter() {
        match d {
            Direction::Up => {
                let dy = y.checked_sub(2);
                if dy.is_none() || maze[dy.unwrap()][x] == 0 {
                    continue;
                }
                maze[y - 1][x] = 0;
                maze[y - 1][x + 1] = 0;
                maze[y - 2][x] = 0;
                maze[y - 2][x + 1] = 0;
                y -= 2;
            }
            Direction::Right => {
                if (x + 4) >= MAZE_X || maze[y][x + 4] == 0 {
                    continue;
                }
                maze[y][x + 1] = 0;
                maze[y][x + 2] = 0;
                maze[y][x + 3] = 0;
                maze[y][x + 4] = 0;
                x += 4;
            }
            Direction::Down => {
                if (y + 2) >= MAZE_Y || maze[y + 2][x] == 0 {
                    continue;
                }
                maze[y + 1][x] = 0;
                maze[y + 1][x + 1] = 0;
                maze[y + 2][x] = 0;
                maze[y + 2][x + 1] = 0;
                y += 2;
            }
            Direction::Left => {
                let dx = x.checked_sub(4);
                if dx.is_none() || maze[y][dx.unwrap()] == 0 {
                    continue;
                }
                maze[y][x - 1] = 0;
                maze[y][x - 2] = 0;
                maze[y][x - 3] = 0;
                maze[y][x - 4] = 0;
                x -= 4;
            }
        }
        draw_maze(maze).unwrap();
        std::thread::sleep(TIME);
        dig_maze(x, y, maze)?;
    }
    Ok(())
}

fn draw_maze(maze: &[[usize; MAZE_X]; MAZE_Y]) -> Result<()> {
    for y in 0..MAZE_Y {
        for x in 0..MAZE_X {
            if maze[y][x] == 0 {
                queue!(
                    stdout(),
                    MoveTo(x as u16, y as u16),
                    PrintStyledContent(WALL.blue())
                )?;
            } else {
                queue!(
                    stdout(),
                    MoveTo(x as u16, y as u16),
                    PrintStyledContent(WALL.black())
                )?;
            }
        }
    }
    stdout().flush()?;
    Ok(())
}
