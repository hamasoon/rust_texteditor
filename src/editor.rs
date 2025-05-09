// event
use core::cmp::min;

use crossterm::event::Event::{self, Key};
use crossterm::event::KeyCode::{self, Char};
use crossterm::event::{read, KeyEvent, KeyEventKind};
use crossterm::terminal::ClearType;

use crate::terminal::terminal::Terminal;
use crate::types::point::Point;
use crate::types::size::Size;

const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    should_clear_screen: bool,
    cursor_pos: Point,
}

impl Editor {
    pub fn default() -> Self {
        Editor {
            should_quit: false,
            should_clear_screen: true,
            cursor_pos: Point::new(0, 0),
        }
    }

    pub fn run(&mut self) {
        self.init().unwrap();
        let result = self.repl();
        Self::exit().unwrap();
        result.unwrap();
    }

    fn init(&mut self) -> Result<(), std::io::Error> {
        Terminal::init()
    }

    fn exit() -> Result<(), std::io::Error> {
        Terminal::exit()
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        Self::draw_rows()?;
        self.print_welcome()?;
        self.move_cursor_to(Point { x: 0, y: 0 })?;
        Terminal::show_cursor()?;
        Terminal::execute()
    }

    fn clear_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        if self.should_clear_screen {
            self.should_clear_screen = false;
            Terminal::clear_screen(ClearType::All)?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()
    }

    fn move_cursor_to(&mut self, point: Point) -> Result<(), std::io::Error> {
        self.cursor_pos = point;
        Terminal::move_cursor_to(point)
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let height = Terminal::size()?.height;
        for current_row in 0..height {
            Terminal::clear_screen(ClearType::CurrentLine)?;
            Terminal::print(Point::new(0, current_row), "~")?;
            if current_row + 1 < height {
                Terminal::print(Point::new(0, current_row), "\r\n")?;
            }
        }
        Ok(())
    }

    // Read-Eval-Print-Loop
    fn repl(&mut self) -> Result<(), std::io::Error> {
        self.clear_screen()?;
        self.refresh_screen()?;
        loop {
            let event = read()?;
            let key_event = self.parsing_keyevent(event)?;
            self.evaluate_event(&key_event)?;

            self.clear_screen()?;
            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn print_welcome(&mut self) -> Result<(), std::io::Error> {
        let message = format!("{}'s {} -- version {}", AUTHOR, NAME, VERSION);
        let Size { width, height } = Terminal::size()?;
        let len = message.len();
        let x_padding = (width - len) / 2;
        let y_padding = height / 2;

        Terminal::print(Point { x: 0, y: y_padding }, &" ".repeat(x_padding))?;
        Terminal::print(
            Point {
                x: x_padding,
                y: y_padding,
            },
            &message,
        )?;
        Terminal::execute()
    }

    fn parsing_keyevent(&mut self, event: Event) -> Result<KeyEvent, std::io::Error> {
        if let Key(key_event) = event {
            Ok(key_event)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Invalid key event",
            ))
        }
    }

    fn evaluate_event(&mut self, event: &KeyEvent) -> Result<(), std::io::Error> {
        match event {
            KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            } => match code {
                KeyCode::Esc => {
                    self.should_quit = true;
                }
                Char('`') => {
                    self.should_clear_screen = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::End
                | KeyCode::Home => {
                    self.move_cursor_event(*code)?;
                }
                _ => {}
            },
            _ => {}
        }

        Terminal::execute()
    }

    fn move_cursor_event(&mut self, code: KeyCode) -> Result<(), std::io::Error> {
        let (mut x, mut y) = (self.cursor_pos.x, self.cursor_pos.y);
        let Size { height, width } = Terminal::size()?;

        match code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = min(height.saturating_sub(1), y.saturating_add(1));
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            _ => (),
        }

        self.cursor_pos = Point { x, y };
        self.move_cursor_to(self.cursor_pos)
    }
}
