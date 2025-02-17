use std::{error::Error, io};

use ratatui::{
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    }, prelude::{Backend, CrosstermBackend}, style::{palette::tailwind::SLATE, Color, Modifier, Style, Stylize}, symbols, text::{Line, Span}, widgets::{Block, Borders, HighlightSpacing, List, ListItem}, Frame, Terminal
};

struct App {
    systems: Vec<String>,
}

impl App {
    fn new() -> App {
        App { systems: vec![] }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout(); // This is a special case. Normally using stdout is fine
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }

            match key.code {
                event::KeyCode::Char('q') => return Ok(true),
                event::KeyCode::Char('a') => return Ok(true),
                _ => {}
            }
        }
    }
}

const NORMAL_ROW_BG: Color = SLATE.c950;
const TODO_HEADER_STYLE: Style = Style::new().fg(Color::LightCyan).bg(Color::DarkGray);

pub fn ui(frame: &mut Frame, app: &App) {
    let block = Block::new()
        .title(Line::raw("TODO List").centered())
        .borders(Borders::TOP)
        .border_set(symbols::border::EMPTY)
        .border_style(TODO_HEADER_STYLE)
        .bg(NORMAL_ROW_BG);

    let list = List::new(vec![
        ListItem::new(Span::styled(
            "Item 1",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        ListItem::new(Span::raw("Item 2")),
        ListItem::new(Span::raw("Item 3")),
    ])
    .block(block)
    .highlight_symbol(">")
    .highlight_spacing(HighlightSpacing::Always);

    frame.render_widget(list, frame.size());
}
