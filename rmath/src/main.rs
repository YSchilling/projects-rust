//TODO implement with a syntax tree (compiler bau vorlesung)

use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

#[derive(Parser)]
struct Cli {
    term1: f64,
    operand: char,
    term2: f64,
}

struct Calculation {}

fn main() {
    // Global Variables
    let mut calculationHistory: Vec<Calculation> = Vec::new();
    let mut currentCalculation: Calculation;

    // INIT TUI
    enable_raw_mode().expect("can't enable raw mode");
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout)).unwrap();

    // DRAWING

    terminal.draw(|frame| {
        let widget = Block::new;
        let area = frame.size();
        frame.render_widget(widget, area);
    });

    thread::sleep(Duration::from_millis(5000));

    // DE-INIT TUI

    disable_raw_mode().expect("can't disable raw mode");
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();
    terminal.show_cursor().unwrap();
}
