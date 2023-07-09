use crossterm::{
    event::{poll, read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Terminal,
};

use crate::structs::Database;

pub fn run_ui(database: &mut Database) -> io::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create and run app
    let res = run_app(&mut terminal, database);

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;

    // check for error
    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, database: &mut Database) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let main_area = frame.size();

            // Main block
            let block = Block::default()
                .borders(Borders::ALL)
                .title("Todo App")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded);
            frame.render_widget(block, main_area);

            // Main layout
            let main_layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(main_area);

            // Todolist
            let mut items: Vec<ListItem> = Vec::new();
            for todo in database.todos.iter() {
                items.push(ListItem::new(todo.to_string()));
            }
            let todo_list = List::new(items);
            frame.render_widget(todo_list, main_layout[0]);
        })?;

        // check for events
        if poll(Duration::from_millis(1))? {
            match read()? {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Esc => break,
                    _ => (),
                },
                _ => (),
            }
        }
    }

    Ok(())
}
