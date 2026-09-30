use anyhow::Result;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::{
    io::{self, BufWriter, Write},
    ops::Range,
};

pub struct TerminalGuard;
impl TerminalGuard {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen, Hide)?;
        Ok(Self)
    }
}
impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        if !cfg!(debug_assertions) {
            let _ = execute!(io::stdout(), Show, LeaveAlternateScreen);
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    None,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Modifier {
    None,
    Bold,
    Italic,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub underline_color: Color,
    pub modifier: Modifier,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub style: Style,
    pub symbol: char,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            fg: Color::None,
            bg: Color::None,
            underline_color: Color::None,
            modifier: Modifier::None,
        }
    }
}

impl Cell {
    pub fn simple(symbol: char) -> Self {
        Self {
            style: Style::default(),
            symbol,
        }
    }
}

pub struct Buffer {
    pub cells: Vec<Cell>,
    pub width: u16,
    pub height: u16,
}

pub struct BufferCollection {
    _guard: TerminalGuard,
    buffers: [Buffer; 2],
    iteration: bool,
    width: usize,
    height: usize,
    redraw: bool,
}

impl Buffer {
    pub fn splat(cell: Cell, width: u16, height: u16) -> Self {
        Self {
            cells: vec![cell; width as usize * height as usize],
            width,
            height,
        }
    }
}

impl BufferCollection {
    pub fn new(width: u16, height: u16) -> Result<Self> {
        Ok(Self {
            _guard: TerminalGuard::new()?,
            buffers: [
                Buffer::splat(Cell::simple('`'), width, height),
                Buffer::splat(Cell::simple('`'), width, height),
            ],
            iteration: false,
            width: width as usize,
            height: height as usize,
            redraw: true,
        })
    }

    pub fn next(&mut self) {
        self.iteration = !self.iteration;
    }
    pub fn last_buffer(&self) -> &Buffer {
        &self.buffers[!self.iteration as usize]
    }
    pub fn current_buffer(&self) -> &Buffer {
        &self.buffers[self.iteration as usize]
    }
    pub fn current_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.iteration as usize]
    }
    // pub fn width(&self) -> usize {
    //     self.width
    // }
    // pub fn height(&self) -> usize {
    //     self.height
    // }
    pub fn full_redraw(&mut self) {
        self.redraw = true;
    }
    pub fn diff(&self) -> Vec<Range<usize>> {
        const MAX_GAP: usize = 6;

        let last_buffer = self.last_buffer();
        let current_buffer = self.current_buffer();
        let mut ranges = Vec::new();

        let mut start_idx: Option<usize> = None;
        let mut last_diff_idx: Option<usize> = None;

        for i in 0..current_buffer.cells.len() {
            let is_row_start = i % self.width == 0;
            let is_diff = last_buffer.cells[i] != current_buffer.cells[i];

            if is_row_start {
                if let (Some(start), Some(last_diff)) = (start_idx, last_diff_idx) {
                    ranges.push(start..last_diff + 1);
                    start_idx = None;
                    last_diff_idx = None;
                }
            }

            if is_diff {
                if start_idx.is_none() {
                    start_idx = Some(i);
                }
                last_diff_idx = Some(i);
            } else if let (Some(start), Some(last_diff)) = (start_idx, last_diff_idx) {
                if i - last_diff > MAX_GAP {
                    ranges.push(start..last_diff + 1);
                    start_idx = None;
                    last_diff_idx = None;
                }
            }
        }

        if let (Some(start), Some(last_diff)) = (start_idx, last_diff_idx) {
            ranges.push(start..last_diff + 1);
        }

        ranges
    }
    pub fn draw(&mut self) -> Result<()> {
        let stdout = io::stdout();
        let handle = stdout.lock();
        let mut writer = BufWriter::with_capacity(self.width * self.height * 2, handle);
        let buffer = self.current_buffer();

        if self.redraw {
            write!(writer, "\x1b[1;1H")?;
            writer.write_all(
                &buffer
                    .cells
                    .iter()
                    .map(|c| c.symbol as u8)
                    .collect::<Vec<u8>>(),
            )?;
            self.redraw = false;
        } else {
            let diff = self.diff();
            for range in diff {
                let col = (range.start % self.width) + 1;
                let row = (range.start / self.width) + 1;
                write!(writer, "\x1b[{row};{col}H")?;
                for cell in &buffer.cells[range] {
                    write!(writer, "{}", cell.symbol)?;
                }
            }
        }

        writer.flush()?;
        self.next();
        let slice = self.last_buffer().cells.clone();
        self.current_buffer_mut().cells.copy_from_slice(&slice);
        Ok(())
    }
}
