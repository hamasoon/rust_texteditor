use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};

use std::io::{stdout, Write};

use super::point::Point;
use super::size::Size;

pub struct Terminal {}

impl Terminal {
    pub fn init() -> Result<(), std::io::Error> {
        Self::execute()?;
        enable_raw_mode()?;
        Ok(())
    }

    pub fn exit() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Self::clear_screen(ClearType::All)?;
        Self::move_cursor_to(Point { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Hide)
    }

    pub fn show_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Show)
    }

    pub fn clear_screen(clear_target: ClearType) -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(clear_target))
    }

    pub fn print(point: Point, text: &str) -> Result<(), std::io::Error> {
        let Point { x, y } = point;
        queue!(stdout(), MoveTo(x as u16, y as u16), Print(text))
    }

    pub fn move_cursor_to(Point { x, y }: Point) -> Result<(), std::io::Error> {
        queue!(stdout(), MoveTo(x as u16, y as u16))
    }

    pub fn execute() -> Result<(), std::io::Error> {
        stdout().flush()
    }

    pub fn size() -> Result<Size, std::io::Error> {
        let (w, h) = size()?;
        Ok(Size::new(w as usize, h as usize))
    }
}
