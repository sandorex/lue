use std::io::{Write, stdout};
use crossterm::{cursor, style::{self, Attribute, Color, ContentStyle, PrintStyledContent, Stylize}, terminal, QueueableCommand};

pub use anyhow::Result;

pub const FULL_VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "-", env!("VERGEN_GIT_DESCRIBE"));

#[derive(Debug, PartialEq, Clone)]
pub struct Cell {
    pub ch: char,
    pub style: ContentStyle,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            style: ContentStyle::default(),
        }
    }
}

#[derive(Debug)]
pub struct Buffer {
    pub data: Vec<Vec<Cell>>
}

impl Buffer {
    fn new(columns: usize, rows: usize) -> Self {
        let data = vec![
            vec![Cell::default(); rows]; columns
        ];

        Buffer {
            data,
        }
    }

    fn set_cell(&mut self, rows: usize, cols: usize, cell: Cell) {
        self.data[cols][rows] = cell;
    }

    // TODO kinda unsafe
    fn get_cell(&self, rows: usize, cols: usize) -> &Cell {
        &self.data[cols][rows]
    }

    fn get_cell_mut(&mut self, rows: usize, cols: usize) -> &mut Cell {
        &mut self.data[cols][rows]
    }

    fn resize(&mut self, columns: usize, rows: usize) {
        self.data.resize(columns, vec![Cell::default(); rows]);
    }

    fn queue(&self, command: &mut impl QueueableCommand) -> Result<(), std::io::Error> {
        command.queue(terminal::BeginSynchronizedUpdate)?;

        for row in &self.data {
            for cell in row {
                command.queue(style::SetStyle(cell.style))?;
                command.queue(style::Print(cell.ch))?;
            }
        }

        // command.queue();
        command.queue(terminal::EndSynchronizedUpdate)?;

        Ok(())
    }
}

fn main() -> Result<()> {
    let (cols, rows) = terminal::size()?;

    terminal::enable_raw_mode()?;

    let mut stdout = stdout();

    stdout.queue(terminal::SetTitle(format!("{} {} {}x{}", env!("CARGO_BIN_NAME"), FULL_VERSION, cols, rows)))?;
    stdout.queue(terminal::EnterAlternateScreen)?;
    stdout.queue(cursor::Hide)?;
    stdout.queue(cursor::MoveTo(0, 0))?;

    let mut buffer = Buffer::new(cols.into(), rows.into());
    let mut pos_x = 0;

    loop {
        if pos_x > 20 {
            break;
        }

        buffer.set_cell(pos_x, 4, Cell { ch: '?', ..Default::default() });

        buffer.queue(&mut stdout)?;
        stdout.flush()?;

        pos_x += 1;
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    std::thread::sleep(std::time::Duration::from_secs(1));

    stdout.queue(terminal::LeaveAlternateScreen)?;
    stdout.queue(cursor::Show)?;
    stdout.flush()?;

    terminal::disable_raw_mode()?;

    Ok(())
}
